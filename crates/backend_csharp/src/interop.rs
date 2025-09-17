pub mod builtins;
pub mod class;
pub mod constants;
pub mod docs;
pub mod functions;
pub mod imports;
pub mod namespace;
pub mod patterns;
pub mod types;
pub mod wires;

use crate::converter::param_to_type;
use crate::interop::builtins::write_builtins;
use crate::interop::class::{write_class_context, write_native_lib_string};
use crate::interop::constants::write_constants;
use crate::interop::docs::write_file_header_comments;
use crate::interop::functions::write_functions;
use crate::interop::imports::write_imports;
use crate::interop::namespace::write_namespace_context;
use crate::interop::patterns::abi_guard::write_abi_guard;
use crate::interop::patterns::asynk::write_pattern_async_trampoline_initializers;
use crate::interop::patterns::write_patterns;
use crate::interop::types::write_type_definitions;
use crate::interop::wires::write_wire_helpers;
use derive_builder::Builder;
use interoptopus::inventory::Inventory;
use interoptopus::lang::util::is_global_type;
use interoptopus::lang::{Constant, Function, Meta, NamespaceMappings, Signature, Type, WirePayload};
use interoptopus::pattern::TypePattern;
use interoptopus_backend_utils::{Error, IndentWriter, indented};
use std::fs::File;
use std::marker::PhantomData;
use std::path::Path;

/// How to convert function names from Rust to C#
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FunctionNameFlavor<'a> {
    /// Takes the name as it is written in Rust
    RawFFIName,
    /// Converts the name to camel case
    CSharpMethodWithClass,
    /// Converts the name to camel case and removes the class name
    CSharpMethodWithoutClass(&'a str),
}

/// The types to write for the given recorder.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WriteTypes {
    /// Only write items defined in the library for this namespace.
    Namespace,
    /// Write types in this namespace and global interoptopus types (e.g., `FFIBool`)
    NamespaceAndInteroptopusGlobal,
    /// Write every type in the library, regardless of namespace association.
    All,
}

impl WriteTypes {
    #[must_use]
    pub const fn write_interoptopus_globals(self) -> bool {
        !matches!(self, Self::Namespace)
    }
}

/// The access modifiers for generated `CSharp` types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Visibility {
    /// Mimics Rust visibility.
    AsDeclared,
    /// Generates all types as `public class` / `public struct`.
    ForcePublic,
    /// Generates all types as `internal class` / `internal struct`.
    ForceInternal,
}

impl Visibility {
    #[must_use]
    pub const fn to_access_modifier(self) -> &'static str {
        match self {
            // TODO: `AsDeclared` should ultimately use the declared visibility but for now copy the previous
            //        behavior which is to make everything public.
            Self::AsDeclared => "public",
            Self::ForcePublic => "public",
            Self::ForceInternal => "internal",
        }
    }
}

impl Default for Interop {
    fn default() -> Self {
        Self {
            inventory: Inventory::default(),
            file_header_comment: None,
            class: "Interop".to_string(),
            class_constants: None,
            dll_name: "library".to_string(),
            namespace_mappings: NamespaceMappings::new("My.Company"),
            namespace_id: String::new(),
            visibility_types: Visibility::AsDeclared,
            write_types: WriteTypes::NamespaceAndInteroptopusGlobal,
            debug: false,
            doc_hints: true,
            decorate_fn: vec![],
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DecorateFn<'a> {
    _phantom: PhantomData<&'a ()>,
}

/// Generates C# interop files, **get this with [`Interop::builder()`]**.🐙
#[derive(Builder)]
#[builder(pattern = "owned", default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Interop {
    /// The file header, e.g., `// (c) My Company`.
    #[builder(setter(into))]
    file_header_comment: Option<String>,
    /// Name of static class for Interop methods, e.g., `Interop`.
    #[builder(setter(into))]
    class: String,
    /// Name of static class for Interop constants, e.g., `Interop`. If [None] then [Self.class] is used
    #[builder(setter(into))]
    class_constants: Option<String>,
    /// DLL to load, e.g., `my_library`.
    #[builder(setter(into))]
    dll_name: String,
    /// Maps which namespace id belongs into which FQN (e.g., "common" => "MyCompany.Common").
    #[builder(setter(into))]
    namespace_mappings: NamespaceMappings,
    /// Namespace ID of _this_ namespace to write (default "").
    #[builder(setter(into))]
    namespace_id: String,
    /// Sets the visibility access modifiers for generated types.
    #[builder(setter(into))]
    pub(crate) visibility_types: Visibility,
    /// Which types to write.
    #[builder(setter(into))]
    write_types: WriteTypes,
    /// Also generate markers for easier debugging.
    debug: bool,
    /// Enrich user-provided item documentation with safety warnings and proper API use hints.
    doc_hints: bool,
    pub(crate) inventory: Inventory,
    // A list of function decorators
    #[builder(setter(custom))]
    #[allow(clippy::type_complexity)]
    decorate_fn: Vec<Box<dyn Fn(DecorateFn) -> String>>,
}

impl InteropBuilder {
    #[must_use]
    pub fn decorate_fn(mut self, f: impl Fn(DecorateFn) -> String + 'static) -> Self {
        match &mut self.decorate_fn {
            Some(vec) => vec.push(Box::new(f)),
            None => self.decorate_fn = Some(vec![Box::new(f)]),
        }
        self
    }
}

impl std::fmt::Debug for Interop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interop")
            .field("file_header_comment", &self.file_header_comment)
            .field("class", &self.class)
            .field("class_constants", &self.class_constants)
            .field("dll_name", &self.dll_name)
            .field("namespace_mappings", &self.namespace_mappings)
            .field("namespace_id", &self.namespace_id)
            .field("visibility_types", &self.visibility_types)
            .field("write_types", &self.write_types)
            .field("debug", &self.debug)
            .field("doc_hints", &self.doc_hints)
            .field("inventory", &self.inventory)
            .field("decorate_fn", &format!("Vec<Box<dyn FnMut() -> String>> (length: {})", self.decorate_fn.len()))
            .finish()
    }
}

#[allow(clippy::unused_self)]
impl Interop {
    /// Creates a new [`InteropBuilder`].
    #[must_use]
    pub fn builder() -> InteropBuilder {
        InteropBuilder::new()
    }

    fn debug(&self, w: &mut IndentWriter, marker: &str) -> Result<(), Error> {
        if !self.debug {
            return Ok(());
        }

        indented!(w, r"// Debug - {} ", marker)?;

        Ok(())
    }

    #[must_use]
    fn namespace_for_id(&self, id: &str) -> String {
        self.namespace_mappings
            .get(id)
            .unwrap_or_else(|| panic!("Found a namespace not mapped '{id}'. You should specify this one in the config."))
            .to_string()
    }

    pub(crate) fn inline_hint(&self, w: &mut IndentWriter, indents: usize) -> Result<(), Error> {
        for _ in 0..indents {
            w.indent();
        }

        indented!(w, r"[MethodImpl(MethodImplOptions.AggressiveOptimization)]")?;

        for _ in 0..indents {
            w.unindent();
        }

        Ok(())
    }

    #[must_use]
    #[allow(dead_code)] // TODO?
    fn should_emit_delegate(&self) -> bool {
        match self.write_types {
            WriteTypes::Namespace => false,
            WriteTypes::NamespaceAndInteroptopusGlobal => self.namespace_id.is_empty(),
            WriteTypes::All => true,
        }
    }

    #[must_use]
    #[allow(clippy::match_like_matches_macro)]
    pub fn should_emit_marshaller(&self, ctype: &Type) -> bool {
        match ctype {
            Type::Array(_) => true,
            Type::Composite(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn has_emittable_marshallers(&self, types: &[Type]) -> bool {
        types.iter().any(|x| self.should_emit_marshaller(x))
    }

    fn has_emittable_functions(&self, functions: &[Function]) -> bool {
        functions.iter().any(|x| self.should_emit_by_meta(x.meta()))
    }

    #[must_use]
    fn has_emittable_constants(&self, constants: &[Constant]) -> bool {
        constants.iter().any(|x| self.should_emit_by_meta(x.meta()))
    }

    #[must_use]
    fn has_emittable_wired_types(&self) -> bool {
        self.inventory.wire_types().iter().any(|t| match t {
            Type::Wire(_) => {
                matches!(t, Type::Wire(w) if self.should_emit_by_meta(w.meta()))
            }
            _ => false,
        })
    }

    /// Given a wire payload type in `c_types`, look up a corresponding Wire type in `wire_types` and return it if it exists.
    #[must_use]
    fn wired_counterpart(&self, kind: &Type) -> Option<Type> {
        let kind_name = kind.name_within_lib();
        if !self.inventory.c_types().iter().any(|t| t.name_within_lib() == kind_name) {
            return None;
        }

        self.inventory
            .wire_types()
            .iter()
            .find(|t| matches!(t, Type::Wire(composite) if composite.rust_name() == kind_name))
            .cloned()
    }

    #[must_use]
    fn should_emit_by_meta(&self, meta: &Meta) -> bool {
        meta.module() == self.namespace_id
    }

    fn is_custom_marshalled(&self, x: &Type) -> bool {
        self.should_emit_marshaller(x)
            || match x {
                Type::FnPointer(y) => self.has_custom_marshalled_delegate(y.signature()),
                Type::Pattern(y) => match y {
                    TypePattern::NamedCallback(z) => self.has_custom_marshalled_delegate(z.fnpointer().signature()),
                    TypePattern::Slice(_) => true,
                    TypePattern::SliceMut(_) => true,
                    _ => false,
                },
                _ => false,
            }
    }

    fn has_custom_marshalled_types(&self, signature: &Signature) -> bool {
        let mut types = signature.params().iter().map(|x| x.the_type().clone()).collect::<Vec<_>>();
        types.push(signature.rval().clone());

        types.iter().any(|x| self.is_custom_marshalled(x))
    }

    fn has_custom_marshalled_delegate(&self, signature: &Signature) -> bool {
        let mut types = signature.params().iter().map(|x| x.the_type().clone()).collect::<Vec<_>>();
        types.push(signature.rval().clone());

        types.iter().any(|x| match x {
            Type::FnPointer(y) => self.has_custom_marshalled_types(y.signature()),
            Type::Pattern(TypePattern::NamedCallback(z)) => self.has_custom_marshalled_types(z.fnpointer().signature()),
            _ => false,
        })
    }

    fn to_native_callback_typespecifier(&self, t: &Type) -> String {
        match t {
            Type::Pattern(TypePattern::Slice(_)) => format!("{}.Unmanaged", param_to_type(t)),
            Type::Pattern(TypePattern::SliceMut(_)) => format!("{}.Unmanaged", param_to_type(t)),
            Type::Pattern(TypePattern::Utf8String(_)) => format!("{}.Unmanaged", param_to_type(t)),
            Type::Composite(_) => format!("{}.Unmanaged", param_to_type(t)),
            Type::Enum(_) => format!("{}.Unmanaged", param_to_type(t)),
            _ => param_to_type(t),
        }
    }

    #[allow(clippy::match_like_matches_macro)]
    fn has_overloadable(&self, signature: &Signature) -> bool {
        signature.params().iter().any(|x| match x.the_type() {
            Type::Pattern(p) => match p {
                TypePattern::NamedCallback(_) => true,
                TypePattern::AsyncCallback(_) => true,
                _ => false,
            },
            _ => false,
        })
    }

    /// Checks whether for the given type and the current file a type definition should be emitted.
    #[must_use]
    fn should_emit_by_type(&self, t: &Type) -> bool {
        if self.write_types == WriteTypes::All {
            return true;
        }

        if is_global_type(t) {
            return self.write_types == WriteTypes::NamespaceAndInteroptopusGlobal;
        }

        match t {
            Type::Primitive(_) => self.write_types == WriteTypes::NamespaceAndInteroptopusGlobal, // need wire wrappers for primitives!
            Type::Array(_) => false,
            Type::Enum(x) => self.should_emit_by_meta(x.meta()),
            Type::Opaque(x) => self.should_emit_by_meta(x.meta()),
            Type::Composite(x) => self.should_emit_by_meta(x.meta()),
            Type::Wire(x) => self.should_emit_by_meta(x.meta()),
            Type::WirePayload(dom) => match dom {
                WirePayload::Composite(x) => self.should_emit_by_meta(x.meta()),
                WirePayload::Enum(x) => self.should_emit_by_meta(x.meta()),
                WirePayload::String => todo!(),
                WirePayload::Vec(x) => self.should_emit_by_type(x),
                WirePayload::Option(x) => self.should_emit_by_type(x),
                WirePayload::Map(_, _) => todo!(),
            },
            Type::FnPointer(_) => true,
            Type::ReadPointer(_) => false,
            Type::ReadWritePointer(_) => false,
            Type::Pattern(x) => match x {
                TypePattern::CStrPointer => true,
                TypePattern::APIVersion => true,
                TypePattern::Slice(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::SliceMut(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::Option(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::Result(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::Bool => self.write_types == WriteTypes::NamespaceAndInteroptopusGlobal,
                TypePattern::CChar => false,
                TypePattern::NamedCallback(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::AsyncCallback(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::Vec(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::Utf8String(_) => false,
            },
        }
    }

    #[must_use]
    pub fn fn_decorations(&self) -> Vec<String> {
        self.decorate_fn
            .iter()
            .map(|decorator| {
                let util = DecorateFn::default();
                decorator(util)
            })
            .collect()
    }

    /// Generates FFI binding code and writes them to the [`IndentWriter`].
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        write_file_header_comments(self, w)?;
        w.newline()?;

        write_imports(self, w)?;
        w.newline()?;

        write_namespace_context(self, w, |w| {
            if self.class_constants.is_none() || self.class_constants == Some(self.class.clone()) {
                if self.has_emittable_functions(self.inventory.functions()) || self.has_emittable_constants(self.inventory.constants()) {
                    write_class_context(self, &self.class, w, |w| {
                        write_native_lib_string(self, w)?;
                        w.newline()?;

                        write_abi_guard(self, w)?;
                        w.newline()?;

                        write_pattern_async_trampoline_initializers(self, w)?;
                        w.newline()?;

                        write_constants(self, w)?;
                        w.newline()?;

                        write_functions(self, w)?;
                        Ok(())
                    })?;
                }
            } else {
                if self.has_emittable_constants(self.inventory.constants()) {
                    write_class_context(self, self.class_constants.as_ref().unwrap(), w, |w| {
                        write_constants(self, w)?;
                        w.newline()?;

                        Ok(())
                    })?;
                }

                if self.has_emittable_functions(self.inventory.functions()) {
                    w.newline()?;
                    write_class_context(self, &self.class, w, |w| {
                        write_native_lib_string(self, w)?;
                        w.newline()?;

                        write_abi_guard(self, w)?;
                        w.newline()?;

                        write_functions(self, w)?;
                        Ok(())
                    })?;
                }
            }

            w.newline()?;
            write_type_definitions(self, w)?;

            w.newline()?;
            write_patterns(self, w)?;

            w.newline()?;
            write_builtins(self, w)?;

            if self.has_emittable_wired_types() {
                w.newline()?;
                write_wire_helpers(self, w)?;
            }

            Ok(())
        })?;

        Ok(())
    }

    /// Convenience method to write FFI bindings to the specified file with default indentation.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn write_file<P: AsRef<Path>>(&self, file_name: P) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        let mut writer = IndentWriter::new(&mut file);

        self.write_to(&mut writer)
    }

    /// Convenience method to write FFI bindings to a string.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn to_string(&self) -> Result<String, Error> {
        let mut vec = Vec::new();
        let mut writer = IndentWriter::new(&mut vec);
        self.write_to(&mut writer)?;
        Ok(String::from_utf8(vec)?)
    }
}

impl InteropBuilder {
    /// Creates a new builder instance, **start here**.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
