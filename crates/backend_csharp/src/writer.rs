use crate::config::{Config, Unsafe, Unsupported, WriteTypes};
use crate::converter::{CSharpTypeConverter, Converter, FunctionNameFlavor};
use crate::overloads::{Helper, OverloadWriter};
use interoptopus::lang::c::{CType, CompositeType, Constant, Documentation, EnumType, Field, FnPointerType, Function, Layout, Meta, PrimitiveType, Variant, Visibility};
use interoptopus::patterns::api_guard::inventory_hash;
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{is_global_type, longest_common_prefix};
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, Error, Inventory};
use std::iter::zip;

/// Writes the C# file format, `impl` this trait to customize output.
pub trait CSharpWriter {
    /// Returns the user config.
    fn config(&self) -> &Config;

    /// Returns the library to produce bindings for.
    fn inventory(&self) -> &Inventory;

    fn converter(&self) -> &Converter;

    fn overloads(&self) -> &[Box<dyn OverloadWriter>];

    fn helper(&self) -> Helper {
        Helper {
            config: self.config(),
            converter: self.converter(),
        }
    }

    fn write_file_header_comments(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"{}"#, &self.config().file_header_comment)?;
        Ok(())
    }

    fn debug(&self, w: &mut IndentWriter, marker: &str) -> Result<(), Error> {
        if !self.config().debug {
            return Ok(());
        }

        indented!(w, r#"// Debug - {} "#, marker)
    }

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.debug(w, "write_imports")?;

        indented!(w, r#"#pragma warning disable 0105"#)?;
        indented!(w, r#"using System;"#)?;
        indented!(w, r#"using System.Collections;"#)?;
        indented!(w, r#"using System.Collections.Generic;"#)?;
        indented!(w, r#"using System.Runtime.InteropServices;"#)?;

        for overload in self.overloads() {
            overload.write_imports(w, self.helper())?;
        }

        for namespace_id in self.inventory().namespaces() {
            let namespace = self
                .config()
                .namespace_mappings
                .get(namespace_id)
                .unwrap_or_else(|| panic!("Must have namespace for '{}' ID", namespace_id));

            indented!(w, r#"using {};"#, namespace)?;
        }
        indented!(w, r#"#pragma warning restore 0105"#)?;

        Ok(())
    }

    fn write_native_lib_string(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.debug(w, "write_native_lib_string")?;
        indented!(w, r#"public const string NativeLib = "{}";"#, self.config().dll_name)
    }

    fn write_abi_guard(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.debug(w, "write_abi_guard")?;

        indented!(w, r#"static {}()"#, self.config().class)?;
        indented!(w, r#"{{"#)?;

        // Check if there is a API version marker for us to write
        if let Some(api_guard) = self
            .inventory()
            .functions()
            .iter()
            .find(|x| matches!(x.signature().rval(), CType::Pattern(TypePattern::APIVersion)))
        {
            let version = inventory_hash(self.inventory());
            let flavor = match self.config().rename_symbols {
                true => FunctionNameFlavor::CSharpMethodNameWithClass,
                false => FunctionNameFlavor::RawFFIName,
            };
            let fn_call = self.converter().function_name_to_csharp_name(api_guard, flavor);
            indented!(w, [_], r#"var api_version = {}.{}();"#, self.config().class, fn_call)?;
            indented!(w, [_], r#"if (api_version != {}ul)"#, version)?;
            indented!(w, [_], r#"{{"#)?;
            indented!(w, [_ _], r#"throw new TypeLoadException($"API reports hash {{api_version}} which differs from hash in bindings ({}). You probably forgot to update / copy either the bindings or the library.");"#, version)?;
            indented!(w, [_], r#"}}"#)?;
        }

        indented!(w, r#"}}"#)?;

        Ok(())
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for constant in self.inventory().constants() {
            if self.should_emit_by_meta(constant.meta()) {
                self.write_constant(w, constant)?;
                w.newline()?;
            }
        }

        Ok(())
    }

    fn write_constant(&self, w: &mut IndentWriter, constant: &Constant) -> Result<(), Error> {
        self.debug(w, "write_constant")?;
        let rval = self.converter().to_typespecifier_in_rval(&constant.the_type());
        let name = constant.name();
        let value = self.converter().constant_value_to_value(constant.value());

        self.write_documentation(w, constant.meta().documentation())?;
        indented!(w, r#"public const {} {} = ({}) {};"#, rval, name, rval, value)
    }

    fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for function in self.inventory().functions() {
            if self.should_emit_by_meta(function.meta()) {
                self.write_function(w, function, WriteFor::Code)?;
                w.newline()?;
            }
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
        self.debug(w, "write_function")?;
        if write_for == WriteFor::Code {
            self.write_documentation(w, function.meta().documentation())?;
            self.write_function_annotation(w, function)?;
        }
        self.write_function_declaration(w, function)?;

        for overload in self.overloads() {
            overload.write_function_overload(w, self.helper(), function, write_for)?;
        }

        Ok(())
    }

    fn write_documentation(&self, w: &mut IndentWriter, documentation: &Documentation) -> Result<(), Error> {
        for line in documentation.lines() {
            indented!(w, r#"///{}"#, line)?;
        }

        Ok(())
    }

    fn write_function_annotation(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        indented!(
            w,
            r#"[DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "{}")]"#,
            function.name()
        )
    }

    fn write_function_declaration(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        let rval = self.converter().function_rval_to_csharp_typename(function);
        let name = self.converter().function_name_to_csharp_name(
            function,
            match self.config().rename_symbols {
                true => FunctionNameFlavor::CSharpMethodNameWithClass,
                false => FunctionNameFlavor::RawFFIName,
            },
        );

        let mut params = Vec::new();
        for (_, p) in function.signature().params().iter().enumerate() {
            let the_type = self.converter().function_parameter_to_csharp_typename(p);
            let name = p.name();

            params.push(format!("{} {}", the_type, name));
        }

        indented!(w, r#"public static extern {} {}({});"#, rval, name, params.join(", "))
    }

    fn write_type_definitions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for the_type in self.inventory().ctypes() {
            self.write_type_definition(w, the_type)?;
        }

        Ok(())
    }

    fn write_type_definition(&self, w: &mut IndentWriter, the_type: &CType) -> Result<(), Error> {
        if !self.should_emit_by_type(the_type) {
            return Ok(());
        }

        match the_type {
            CType::Primitive(_) => {}
            CType::Array(_) => {}
            CType::Enum(e) => {
                self.write_type_definition_enum(w, e, WriteFor::Code)?;
                w.newline()?;
            }
            CType::Opaque(_) => {}
            CType::Composite(c) => {
                self.write_type_definition_composite(w, c)?;
                w.newline()?;
            }
            CType::FnPointer(f) => {
                self.write_type_definition_fn_pointer(w, f)?;
                w.newline()?;
            }
            CType::ReadPointer(_) => {}
            CType::ReadWritePointer(_) => {}
            CType::Pattern(x) => match x {
                TypePattern::CStrPointer => {}
                TypePattern::FFIErrorEnum(e) => {
                    self.write_type_definition_enum(w, e.the_enum(), WriteFor::Code)?;
                    w.newline()?;
                }
                TypePattern::Slice(x) => {
                    self.write_type_definition_composite(w, x)?;
                    w.newline()?;
                    self.write_pattern_slice(w, x)?;
                    w.newline()?;
                }
                TypePattern::SliceMut(x) => {
                    self.write_type_definition_composite(w, x)?;
                    w.newline()?;
                    self.write_pattern_slice_mut(w, x)?;
                    w.newline()?;
                }
                TypePattern::Option(x) => {
                    self.write_type_definition_composite(w, x)?;
                    w.newline()?;
                    self.write_pattern_option(w, x)?;
                    w.newline()?;
                }
                TypePattern::NamedCallback(x) => {
                    // Handle this better way
                    self.write_type_definition_named_callback(w, x)?;
                    w.newline()?;
                }
                TypePattern::Bool => {
                    self.write_type_definition_ffibool(w)?;
                    w.newline()?;
                }

                TypePattern::CChar => {}
                TypePattern::APIVersion => {}
            },
        }
        Ok(())
    }

    fn write_type_definition_ffibool(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.debug(w, "write_type_definition_ffibool")?;

        let type_name = self.converter().to_typespecifier_in_param(&CType::Pattern(TypePattern::Bool));

        indented!(w, r#"[Serializable]"#)?;
        indented!(w, r#"[StructLayout(LayoutKind.Sequential)]"#)?;
        indented!(w, r#"{} partial struct {}"#, self.config().visibility_types.to_access_modifier(), type_name)?;
        indented!(w, r#"{{"#)?;
        indented!(w, [_], r#"byte value;"#)?;
        indented!(w, r#"}}"#)?;
        w.newline()?;

        indented!(w, r#"{} partial struct {}"#, self.config().visibility_types.to_access_modifier(), type_name)?;
        indented!(w, r#"{{"#)?;
        indented!(w, [_], r#"public static readonly {} True = new Bool {{ value =  1 }};"#, type_name)?;
        indented!(w, [_], r#"public static readonly {} False = new Bool {{ value =  0 }};"#, type_name)?;
        indented!(w, [_], r#"public Bool(bool b)"#)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _ ], r#"value = (byte) (b ? 1 : 0);"#)?;
        indented!(w, [_], r#"}}"#)?;
        indented!(w, [_], r#"public bool Is => value == 1;"#)?;
        indented!(w, r#"}}"#)?;
        w.newline()?;
        Ok(())
    }

    fn write_type_definition_fn_pointer(&self, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
        self.debug(w, "write_type_definition_fn_pointer")?;
        self.write_type_definition_fn_pointer_annotation(w, the_type)?;
        self.write_type_definition_fn_pointer_body(w, the_type)?;
        Ok(())
    }

    fn write_type_definition_named_callback(&self, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
        self.debug(w, "write_type_definition_named_callback")?;
        self.write_type_definition_fn_pointer_annotation(w, the_type.fnpointer())?;
        self.write_type_definition_named_callback_body(w, the_type)?;

        for overload in self.overloads() {
            overload.write_callback_overload(w, self.helper(), the_type)?;
        }

        Ok(())
    }

    fn write_type_definition_named_callback_body(&self, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
        let rval = self.converter().to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
        let name = self.converter().named_callback_to_typename(the_type);
        let visibility = self.config().visibility_types.to_access_modifier();

        let mut params = Vec::new();
        for param in the_type.fnpointer().signature().params().iter() {
            params.push(format!("{} {}", self.converter().to_typespecifier_in_param(param.the_type()), param.name()));
        }

        indented!(w, r#"{} delegate {} {}({});"#, visibility, rval, name, params.join(", "))
    }

    fn write_type_definition_fn_pointer_annotation(&self, w: &mut IndentWriter, _the_type: &FnPointerType) -> Result<(), Error> {
        indented!(w, r#"[UnmanagedFunctionPointer(CallingConvention.Cdecl)]"#)
    }

    fn write_type_definition_fn_pointer_body(&self, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
        let rval = self.converter().to_typespecifier_in_rval(the_type.signature().rval());
        let name = self.converter().fnpointer_to_typename(the_type);
        let visibility = self.config().visibility_types.to_access_modifier();

        let mut params = Vec::new();
        for (i, param) in the_type.signature().params().iter().enumerate() {
            params.push(format!("{} x{}", self.converter().to_typespecifier_in_param(param.the_type()), i));
        }

        indented!(w, r#"{} delegate {} {}({});"#, visibility, rval, name, params.join(", "))
    }

    fn write_type_definition_enum(&self, w: &mut IndentWriter, the_type: &EnumType, write_for: WriteFor) -> Result<(), Error> {
        self.debug(w, "write_type_definition_enum")?;
        if write_for == WriteFor::Code {
            self.write_documentation(w, the_type.meta().documentation())?;
        }
        indented!(w, r#"public enum {}"#, the_type.rust_name())?;
        indented!(w, r#"{{"#)?;
        w.indent();

        for variant in the_type.variants() {
            self.write_type_definition_enum_variant(w, variant, the_type, write_for)?;
        }

        w.unindent();
        indented!(w, r#"}}"#)
    }

    fn write_type_definition_enum_variant(&self, w: &mut IndentWriter, variant: &Variant, _the_type: &EnumType, write_for: WriteFor) -> Result<(), Error> {
        let variant_name = variant.name();
        let variant_value = variant.value();
        if write_for == WriteFor::Code {
            self.write_documentation(w, variant.documentation())?;
        }
        indented!(w, r#"{} = {},"#, variant_name, variant_value)
    }

    fn write_type_definition_composite(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        self.debug(w, "write_type_definition_composite")?;
        self.write_documentation(w, the_type.meta().documentation())?;
        self.write_type_definition_composite_annotation(w, the_type)?;
        self.write_type_definition_composite_body(w, the_type, WriteFor::Code)
    }

    fn write_type_definition_composite_annotation(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        indented!(w, r#"[Serializable]"#)?;

        if the_type.repr().alignment().is_some() {
            let comment = r#"// THIS STRUCT IS BROKEN - C# does not support alignment of entire Rust types that do #[repr(align(...))]"#;
            match self.config().unsupported {
                Unsupported::Panic => panic!("{}", comment),
                Unsupported::Comment => indented!(w, "{}", comment)?,
            }
        };

        match the_type.repr().layout() {
            Layout::C | Layout::Transparent | Layout::Opaque => indented!(w, r#"[StructLayout(LayoutKind.Sequential)]"#),
            Layout::Packed => indented!(w, r#"[StructLayout(LayoutKind.Sequential, Pack = 1)]"#),
            Layout::Primitive(_) => panic!("Primitive layout not supported for structs."),
        }
    }

    fn write_type_definition_composite_body(&self, w: &mut IndentWriter, the_type: &CompositeType, write_for: WriteFor) -> Result<(), Error> {
        indented!(
            w,
            r#"{} partial struct {}"#,
            self.config().visibility_types.to_access_modifier(),
            the_type.rust_name()
        )?;
        indented!(w, r#"{{"#)?;
        w.indent();

        for field in the_type.fields() {
            if write_for == WriteFor::Code {
                self.write_documentation(w, field.documentation())?;

                for overload in self.overloads() {
                    overload.write_field_decorators(w, self.helper(), field, the_type)?;
                }
            }

            self.write_type_definition_composite_body_field(w, field, the_type)?;
        }

        w.unindent();
        indented!(w, r#"}}"#)
    }

    fn write_type_definition_composite_body_field(&self, w: &mut IndentWriter, field: &Field, the_type: &CompositeType) -> Result<(), Error> {
        let field_name = self.converter().field_name_to_csharp_name(field, self.config().rename_symbols);
        let visibility = match field.visibility() {
            Visibility::Public => "public ",
            Visibility::Private => "",
        };

        match field.the_type() {
            CType::Array(a) => {
                if !self.config().unroll_struct_arrays {
                    panic!("Unable to generate bindings for arrays in fields if `unroll_struct_arrays` is not enabled.");
                }

                let type_name = self.converter().to_typespecifier_in_field(a.array_type(), field, the_type);
                for i in 0..a.len() {
                    indented!(w, r#"{}{} {}{};"#, visibility, type_name, field_name, i)?;
                }

                Ok(())
            }
            CType::Primitive(PrimitiveType::Bool) => {
                let type_name = self.converter().to_typespecifier_in_field(field.the_type(), field, the_type);
                indented!(w, r#"[MarshalAs(UnmanagedType.I1)]"#)?;
                indented!(w, r#"{}{} {};"#, visibility, type_name, field_name)
            }
            _ => {
                let type_name = self.converter().to_typespecifier_in_field(field.the_type(), field, the_type);
                indented!(w, r#"{}{} {};"#, visibility, type_name, field_name)
            }
        }
    }

    fn namespace_for_id(&self, id: &str) -> String {
        self.config()
            .namespace_mappings
            .get(id)
            .unwrap_or_else(|| panic!("Found a namespace not mapped '{}'. You should specify this one in the config.", id))
            .to_string()
    }

    fn write_namespace_context(&self, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        self.debug(w, "write_namespace_context")?;
        indented!(w, r#"namespace {}"#, self.namespace_for_id(&self.config().namespace_id))?;
        indented!(w, r#"{{"#)?;
        w.indent();

        f(w)?;

        w.unindent();

        indented!(w, r#"}}"#)
    }

    fn write_class_context(&self, class_name: &str, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        self.debug(w, "write_class_context")?;
        indented!(w, r#"{} static partial class {}"#, self.config().visibility_types.to_access_modifier(), class_name)?;
        indented!(w, r#"{{"#)?;
        w.indent();

        f(w)?;

        w.unindent();
        indented!(w, r#"}}"#)
    }

    fn should_emit_delegate(&self) -> bool {
        match self.config().write_types {
            WriteTypes::Namespace => false,
            WriteTypes::NamespaceAndInteroptopusGlobal => self.config().namespace_id.is_empty(),
            WriteTypes::All => true,
        }
    }

    fn has_emittable_functions(&self, functions: &[Function]) -> bool {
        functions.iter().any(|x| self.should_emit_by_meta(x.meta()))
    }

    fn has_emittable_constants(&self, constants: &[Constant]) -> bool {
        constants.iter().any(|x| self.should_emit_by_meta(x.meta()))
    }

    fn has_ffi_error(&self, functions: &[Function]) -> bool {
        functions.iter().any(|x| x.returns_ffi_error())
    }

    fn should_emit_by_meta(&self, meta: &Meta) -> bool {
        let rval = meta.namespace() == self.config().namespace_id;
        rval
    }

    /// Checks whether for the given type and the current file a type definition should be emitted.
    fn should_emit_by_type(&self, t: &CType) -> bool {
        if self.config().write_types == WriteTypes::All {
            return true;
        }

        if is_global_type(t) {
            return self.config().write_types == WriteTypes::NamespaceAndInteroptopusGlobal;
        }

        match t {
            CType::Primitive(_) => self.config().write_types == WriteTypes::NamespaceAndInteroptopusGlobal,
            CType::Array(_) => false,
            CType::Enum(x) => self.should_emit_by_meta(x.meta()),
            CType::Opaque(x) => self.should_emit_by_meta(x.meta()),
            CType::Composite(x) => self.should_emit_by_meta(x.meta()),
            CType::FnPointer(_) => true,
            CType::ReadPointer(_) => false,
            CType::ReadWritePointer(_) => false,
            CType::Pattern(x) => match x {
                TypePattern::CStrPointer => true,
                TypePattern::APIVersion => true,
                TypePattern::FFIErrorEnum(x) => self.should_emit_by_meta(x.the_enum().meta()),
                TypePattern::Slice(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::SliceMut(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::Option(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::Bool => self.config().write_types == WriteTypes::NamespaceAndInteroptopusGlobal,
                TypePattern::CChar => false,
                TypePattern::NamedCallback(x) => self.should_emit_by_meta(x.meta()),
            },
        }
    }

    fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.inventory().patterns() {
            match pattern {
                LibraryPattern::Service(cls) => {
                    if self.should_emit_by_meta(cls.the_type().meta()) {
                        self.write_pattern_service(w, cls)?
                    }
                }
            }
        }

        Ok(())
    }

    fn write_pattern_option(&self, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
        self.debug(w, "write_pattern_option")?;

        let context_type_name = slice.rust_name();
        let data_type = slice
            .fields()
            .iter()
            .find(|x| x.name().eq("t"))
            .expect("Option must contain field called 't'.")
            .the_type();

        let type_string = self.converter().to_typespecifier_in_rval(data_type);
        let is_some = if self.config().rename_symbols { "isSome" } else { "is_some" };

        indented!(w, r#"{} partial struct {}"#, self.config().visibility_types.to_access_modifier(), context_type_name)?;
        indented!(w, r#"{{"#)?;

        // FromNullable
        indented!(w, [_], r#"public static {} FromNullable({}? nullable)"#, context_type_name, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"var result = new {}();"#, context_type_name)?;
        indented!(w, [_ _], r#"if (nullable.HasValue)"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"result.{} = 1;"#, is_some)?;
        indented!(w, [_ _ _], r#"result.t = nullable.Value;"#)?;
        indented!(w, [_ _], r#"}}"#)?;
        w.newline()?;
        indented!(w, [_ _], r#"return result;"#)?;
        indented!(w, [_], r#"}}"#)?;
        w.newline()?;

        // ToNullable
        indented!(w, [_], r#"public {}? ToNullable()"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"return this.{} == 1 ? this.t : ({}?)null;"#, is_some, type_string)?;
        indented!(w, [_], r#"}}"#)?;

        indented!(w, r#"}}"#)?;
        w.newline()?;
        Ok(())
    }

    fn write_pattern_slice(&self, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
        self.debug(w, "write_pattern_slice")?;

        let context_type_name = slice.rust_name();
        let data_type = slice
            .fields()
            .iter()
            .find(|x| x.name().contains("data"))
            .expect("Slice must contain field called 'data'.")
            .the_type()
            .try_deref_pointer()
            .expect("data must be a pointer type");

        let type_string = self.converter().to_typespecifier_in_rval(data_type);
        let is_blittable = self.converter().is_blittable(data_type);

        indented!(
            w,
            r#"{} partial struct {} : IEnumerable<{}>"#,
            self.config().visibility_types.to_access_modifier(),
            context_type_name,
            type_string
        )?;
        indented!(w, r#"{{"#)?;

        // Ctor
        indented!(w, [_], r#"public {}(GCHandle handle, ulong count)"#, context_type_name)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"this.data = handle.AddrOfPinnedObject();"#)?;
        indented!(w, [_ _], r#"this.len = count;"#)?;
        indented!(w, [_], r#"}}"#)?;

        // Ctor
        indented!(w, [_], r#"public {}(IntPtr handle, ulong count)"#, context_type_name)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"this.data = handle;"#)?;
        indented!(w, [_ _], r#"this.len = count;"#)?;
        indented!(w, [_], r#"}}"#)?;

        for overload in self.overloads() {
            overload.write_pattern_slice_overload(w, self.helper(), context_type_name, &type_string)?;
        }

        // Getter
        indented!(w, [_], r#"public {} this[int i]"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"get"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"if (i >= Count) throw new IndexOutOfRangeException();"#)?;

        if self.config().use_unsafe.any_unsafe() && is_blittable {
            indented!(w, [_ _ _], r#"unsafe"#)?;
            indented!(w, [_ _ _], r#"{{"#)?;
            indented!(w, [_ _ _ _], r#"var d = ({}*) data.ToPointer();"#, type_string)?;
            indented!(w, [_ _ _ _], r#"return d[i];"#)?;
            indented!(w, [_ _ _], r#"}}"#)?;
        } else {
            indented!(w, [_ _ _], r#"var size = Marshal.SizeOf(typeof({}));"#, type_string)?;
            indented!(w, [_ _ _], r#"var ptr = new IntPtr(data.ToInt64() + i * size);"#)?;
            indented!(w, [_ _ _], r#"return Marshal.PtrToStructure<{}>(ptr);"#, type_string)?;
        }

        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_], r#"}}"#)?;

        // Copied
        indented!(w, [_], r#"public {}[] Copied"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"get"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"var rval = new {}[len];"#, type_string)?;

        if self.config().use_unsafe == Unsafe::UnsafePlatformMemCpy && is_blittable {
            indented!(w, [_ _ _], r#"unsafe"#)?;
            indented!(w, [_ _ _], r#"{{"#)?;
            indented!(w, [_ _ _ _ ], r#"fixed (void* dst = rval)"#)?;
            indented!(w, [_ _ _ _ ], r#"{{"#)?;
            indented!(w, [_ _ _ _ _], r#"#if __INTEROPTOPUS_NEVER"#)?;

            for overload in self.overloads() {
                overload.write_pattern_slice_unsafe_copied_fragment(w, self.helper(), &type_string)?;
            }

            indented!(w, [_ _ _ _ _], r#"#else"#)?;
            indented!(w, [_ _ _ _ _], r#"for (var i = 0; i < (int) len; i++) {{"#)?;
            indented!(w, [_ _ _ _ _ _], r#"rval[i] = this[i];"#)?;
            indented!(w, [_ _ _ _ _], r#"}}"#)?;
            indented!(w, [_ _ _ _ _], r#"#endif"#)?;
            indented!(w, [_ _ _ _ ], r#"}}"#)?;
            indented!(w, [_ _ _], r#"}}"#)?;
        } else {
            indented!(w, [_ _ _], r#"for (var i = 0; i < (int) len; i++) {{"#)?;
            indented!(w, [_ _ _ _], r#"rval[i] = this[i];"#)?;
            indented!(w, [_ _ _], r#"}}"#)?;
        }
        indented!(w, [_ _ _], r#"return rval;"#)?;
        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_], r#"}}"#)?;

        // Count
        indented!(w, [_], r#"public int Count => (int) len;"#)?;

        // GetEnumerator
        indented!(w, [_], r#"public IEnumerator<{}> GetEnumerator()"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"for (var i = 0; i < (int)len; ++i)"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"yield return this[i];"#)?;
        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_], r#"}}"#)?;

        // The other GetEnumerator
        indented!(w, [_], r#"IEnumerator IEnumerable.GetEnumerator()"#)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"return this.GetEnumerator();"#)?;
        indented!(w, [_], r#"}}"#)?;

        indented!(w, r#"}}"#)?;
        w.newline()?;

        Ok(())
    }

    fn write_pattern_slice_mut(&self, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
        self.debug(w, "write_pattern_slice_mut")?;
        let context_type_name = slice.rust_name();
        let data_type = slice
            .fields()
            .iter()
            .find(|x| x.name().contains("data"))
            .expect("Slice must contain field called 'data'.")
            .the_type()
            .try_deref_pointer()
            .expect("data must be a pointer type");

        let type_string = self.converter().to_typespecifier_in_rval(data_type);

        indented!(
            w,
            r#"{} partial struct {} : IEnumerable<{}>"#,
            self.config().visibility_types.to_access_modifier(),
            context_type_name,
            type_string
        )?;
        indented!(w, r#"{{"#)?;

        // Ctor
        indented!(w, [_], r#"public {}(GCHandle handle, ulong count)"#, context_type_name)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"this.data = handle.AddrOfPinnedObject();"#)?;
        indented!(w, [_ _], r#"this.len = count;"#)?;
        indented!(w, [_], r#"}}"#)?;

        // Ctor
        indented!(w, [_], r#"public {}(IntPtr handle, ulong count)"#, context_type_name)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"this.data = handle;"#)?;
        indented!(w, [_ _], r#"this.len = count;"#)?;
        indented!(w, [_], r#"}}"#)?;

        for overload in self.overloads() {
            overload.write_pattern_slice_overload(w, self.helper(), context_type_name, &type_string)?;
        }

        for overload in self.overloads() {
            overload.write_pattern_slice_mut_overload(w, self.helper(), context_type_name, &type_string)?;
        }

        // Getter
        indented!(w, [_], r#"public {} this[int i]"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"get"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"if (i >= Count) throw new IndexOutOfRangeException();"#)?;
        if self.config().use_unsafe.any_unsafe() {
            indented!(w, [_ _ _], r#"unsafe"#)?;
            indented!(w, [_ _ _], r#"{{"#)?;
            indented!(w, [_ _ _ _], r#"var d = ({}*) data.ToPointer();"#, type_string)?;
            indented!(w, [_ _ _ _], r#"return d[i];"#)?;
            indented!(w, [_ _ _], r#"}}"#)?;
        } else {
            indented!(w, [_ _ _], r#"var size = Marshal.SizeOf(typeof({}));"#, type_string)?;
            indented!(w, [_ _ _], r#"var ptr = new IntPtr(data.ToInt64() + i * size);"#)?;
            indented!(w, [_ _ _], r#"return Marshal.PtrToStructure<{}>(ptr);"#, type_string)?;
        }
        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_ _], r#"set"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"if (i >= Count) throw new IndexOutOfRangeException();"#)?;
        if self.config().use_unsafe.any_unsafe() {
            indented!(w, [_ _ _], r#"unsafe"#)?;
            indented!(w, [_ _ _], r#"{{"#)?;
            indented!(w, [_ _ _ _], r#"var d = ({}*) data.ToPointer();"#, type_string)?;
            indented!(w, [_ _ _ _], r#"d[i] = value;"#)?;
            indented!(w, [_ _ _], r#"}}"#)?;
        } else {
            indented!(w, [_ _ _], r#"var size = Marshal.SizeOf(typeof({}));"#, type_string)?;
            indented!(w, [_ _ _], r#"var ptr = new IntPtr(data.ToInt64() + i * size);"#)?;
            indented!(w, [_ _ _], r#"Marshal.StructureToPtr<{}>(value, ptr, false);"#, type_string)?;
        }
        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_], r#"}}"#)?;

        // Copied
        indented!(w, [_], r#"public {}[] Copied"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"get"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"var rval = new {}[len];"#, type_string)?;

        if self.config().use_unsafe == Unsafe::UnsafePlatformMemCpy {
            indented!(w, [_ _ _], r#"unsafe"#)?;
            indented!(w, [_ _ _], r#"{{"#)?;
            indented!(w, [_ _ _ _ ], r#"fixed (void* dst = rval)"#)?;
            indented!(w, [_ _ _ _ ], r#"{{"#)?;
            indented!(w, [_ _ _ _ _], r#"#if __FALSE"#)?;

            for overload in self.overloads() {
                overload.write_pattern_slice_unsafe_copied_fragment(w, self.helper(), &type_string)?;
            }

            indented!(w, [_ _ _ _ _], r#"#else"#)?;
            indented!(w, [_ _ _ _ _], r#"for (var i = 0; i < (int) len; i++) {{"#)?;
            indented!(w, [_ _ _ _ _ _], r#"rval[i] = this[i];"#)?;
            indented!(w, [_ _ _ _ _], r#"}}"#)?;
            indented!(w, [_ _ _ _ _], r#"#endif"#)?;
            indented!(w, [_ _ _ _ ], r#"}}"#)?;
            indented!(w, [_ _ _], r#"}}"#)?;
        } else {
            indented!(w, [_ _ _], r#"for (var i = 0; i < (int) len; i++) {{"#)?;
            indented!(w, [_ _ _ _], r#"rval[i] = this[i];"#)?;
            indented!(w, [_ _ _], r#"}}"#)?;
        }

        indented!(w, [_ _ _], r#"return rval;"#)?;
        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_], r#"}}"#)?;

        // Count
        indented!(w, [_], r#"public int Count => (int) len;"#)?;

        // GetEnumerator
        indented!(w, [_], r#"public IEnumerator<{}> GetEnumerator()"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"for (var i = 0; i < (int)len; ++i)"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"yield return this[i];"#)?;
        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_], r#"}}"#)?;

        // The other GetEnumerator
        indented!(w, [_], r#"IEnumerator IEnumerable.GetEnumerator()"#)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"return this.GetEnumerator();"#)?;
        indented!(w, [_], r#"}}"#)?;

        indented!(w, r#"}}"#)?;
        w.newline()?;

        Ok(())
    }

    fn write_pattern_service(&self, w: &mut IndentWriter, class: &Service) -> Result<(), Error> {
        self.debug(w, "write_pattern_service")?;
        let mut all_functions = class.constructors().to_vec();
        all_functions.extend_from_slice(class.methods());
        all_functions.push(class.destructor().clone());

        let context_type_name = class.the_type().rust_name();
        let common_prefix = longest_common_prefix(&all_functions);

        self.write_documentation(w, class.the_type().meta().documentation())?;
        indented!(
            w,
            r#"{} partial class {} : IDisposable"#,
            self.config().visibility_types.to_access_modifier(),
            context_type_name
        )?;
        indented!(w, r#"{{"#)?;
        w.indent();
        indented!(w, r#"private IntPtr _context;"#)?;
        w.newline()?;
        indented!(w, r#"private {}() {{}}"#, context_type_name)?;
        w.newline()?;

        for ctor in class.constructors() {
            // Ctor
            let fn_name = self
                .converter()
                .function_name_to_csharp_name(ctor, FunctionNameFlavor::CSharpMethodNameWithoutClass(&common_prefix));
            let rval = format!("static {}", context_type_name);

            self.write_documentation(w, ctor.meta().documentation())?;
            self.write_pattern_service_method(w, class, ctor, &rval, &fn_name, true, true, WriteFor::Code)?;
            w.newline()?;
        }

        // Dtor
        self.write_pattern_service_method(w, class, class.destructor(), "void", "Dispose", true, false, WriteFor::Code)?;
        w.newline()?;

        for function in class.methods() {
            // Main function
            let fn_name = self
                .converter()
                .function_name_to_csharp_name(function, FunctionNameFlavor::CSharpMethodNameWithoutClass(&common_prefix));

            // Write checked method. These are "normal" methods that accept
            // common C# types.
            let rval = match function.signature().rval() {
                CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
                CType::Pattern(TypePattern::CStrPointer) => "string".to_string(),
                _ => self.converter().to_typespecifier_in_rval(function.signature().rval()),
            };
            self.write_documentation(w, function.meta().documentation())?;
            self.write_pattern_service_method(w, class, function, &rval, &fn_name, false, false, WriteFor::Code)?;

            for overload in self.overloads() {
                overload.write_service_method_overload(w, self.helper(), class, function, &fn_name, WriteFor::Code)?;
            }

            w.newline()?;
        }

        indented!(w, r#"public IntPtr Context => _context;"#)?;

        w.unindent();
        indented!(w, r#"}}"#)?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn write_pattern_service_method(
        &self,
        w: &mut IndentWriter,
        class: &Service,
        function: &Function,
        rval: &str,
        fn_name: &str,
        write_contxt_by_ref: bool,
        is_ctor: bool,
        write_for: WriteFor,
    ) -> Result<(), Error> {
        self.debug(w, "write_pattern_service_method")?;

        let mut names = Vec::new();
        let mut to_invoke = Vec::new();
        let mut types = Vec::new();
        let mut to_wrap_delegates = Vec::new();
        let mut to_wrap_delegate_types = Vec::new();

        // For every parameter except the first, figure out how we should forward
        // it to the invocation we perform.
        for p in function.signature().params().iter().skip(1) {
            let name = p.name();

            // If we call the checked function we want to resolve a `SliceU8` to a `byte[]`,
            // but if we call the unchecked version we want to keep that `Sliceu8` in our signature.
            let native = self.converter().to_typespecifier_in_param(p.the_type());

            match p.the_type() {
                CType::Pattern(TypePattern::NamedCallback(callback)) => match callback.fnpointer().signature().rval() {
                    CType::Pattern(TypePattern::FFIErrorEnum(_)) if self.config().work_around_exception_in_callback_no_reentry => {
                        to_wrap_delegates.push(name);
                        to_wrap_delegate_types.push(self.helper().converter.to_typespecifier_in_param(p.the_type()));
                        to_invoke.push(format!("{}_safe_delegate.Call", name));
                    }
                    _ => {
                        // Forward `ref` and `out` accordingly.
                        if native.contains("out ") {
                            to_invoke.push(format!("out {}", name));
                        } else if native.contains("ref ") {
                            to_invoke.push(format!("ref {}", name));
                        } else {
                            to_invoke.push(name.to_string());
                        }
                    }
                },

                _ => {
                    // Forward `ref` and `out` accordingly.
                    if native.contains("out ") {
                        to_invoke.push(format!("out {}", name));
                    } else if native.contains("ref ") {
                        to_invoke.push(format!("ref {}", name));
                    } else {
                        to_invoke.push(name.to_string());
                    }
                }
            }

            names.push(name);
            types.push(native);
        }

        let method_to_invoke = self.converter().function_name_to_csharp_name(
            function,
            match self.config().rename_symbols {
                true => FunctionNameFlavor::CSharpMethodNameWithClass,
                false => FunctionNameFlavor::RawFFIName,
            },
        );
        let extra_args = if to_invoke.is_empty() {
            "".to_string()
        } else {
            format!(", {}", to_invoke.join(", "))
        };

        // Assemble actual function call.
        let context = if write_contxt_by_ref {
            if is_ctor {
                "ref self._context"
            } else {
                "ref _context"
            }
        } else {
            "_context"
        };
        let arg_tokens = names.iter().zip(types.iter()).map(|(n, t)| format!("{} {}", t, n)).collect::<Vec<_>>();
        let fn_call = format!(r#"{}.{}({}{})"#, self.config().class, method_to_invoke, context, extra_args);

        // Write signature.
        let signature = format!(r#"public {} {}({})"#, rval, fn_name, arg_tokens.join(", "));
        if write_for == WriteFor::Docs {
            indented!(w, r#"{};"#, signature)?;
            return Ok(());
        }

        indented!(w, "{}", signature)?;
        indented!(w, r#"{{"#)?;

        if is_ctor {
            indented!(w, [_], r#"var self = new {}();"#, class.the_type().rust_name())?;
        }

        for (name, ty) in zip(&to_wrap_delegates, &to_wrap_delegate_types) {
            indented!(w, [_], r#"var {}_safe_delegate = new {}ExceptionSafe({});"#, name, ty, name)?;
        }

        // Determine return value behavior and write function call.
        match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
                indented!(w, [_], r#"var rval = {};"#, fn_call)?;
                for name in to_wrap_delegates {
                    indented!(w, [_], r#"{}_safe_delegate.Rethrow();"#, name)?;
                }
                indented!(w, [_], r#"if (rval != {}.{})"#, e.the_enum().rust_name(), e.success_variant().name())?;
                indented!(w, [_], r#"{{"#)?;
                indented!(w, [_ _], r#"throw new InteropException<{}>(rval);"#, e.the_enum().rust_name())?;
                indented!(w, [_], r#"}}"#)?;
            }
            CType::Pattern(TypePattern::CStrPointer) => {
                indented!(w, [_], r#"var s = {};"#, fn_call)?;
                indented!(w, [_], r#"return Marshal.PtrToStringAnsi(s);"#)?;
            }
            CType::Primitive(PrimitiveType::Void) => {
                indented!(w, [_], r#"{};"#, fn_call)?;
            }
            _ => {
                indented!(w, [_], r#"return {};"#, fn_call)?;
            }
        }

        if is_ctor {
            indented!(w, [_], r#"return self;"#)?;
        }

        indented!(w, r#"}}"#)?;

        Ok(())
    }

    fn write_builtins(&self, w: &mut IndentWriter) -> Result<(), Error> {
        if self.config().write_types.write_interoptopus_globals() && self.has_ffi_error(self.inventory().functions()) {
            let error_text = &self.config().error_text;

            indented!(w, r#"public class InteropException<T> : Exception"#)?;
            indented!(w, r#"{{"#)?;
            indented!(w, [_], r#"public T Error {{ get; private set; }}"#)?;
            w.newline()?;
            indented!(w, [_], r#"public InteropException(T error): base($"{error_text}")"#)?;
            indented!(w, [_], r#"{{"#)?;
            indented!(w, [_ _], r#"Error = error;"#)?;
            indented!(w, [_], r#"}}"#)?;
            indented!(w, r#"}}"#)?;
            w.newline()?;
        }

        Ok(())
    }

    fn write_all(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_file_header_comments(w)?;
        w.newline()?;

        self.write_imports(w)?;
        w.newline()?;

        self.write_namespace_context(w, |w| {
            if self.config().class_constants.is_none() || self.config().class_constants == Some(self.config().clone().class) {
                if self.has_emittable_functions(self.inventory().functions()) || self.has_emittable_constants(self.inventory().constants()) {
                    self.write_class_context(&self.config().class, w, |w| {
                        self.write_native_lib_string(w)?;
                        w.newline()?;

                        self.write_abi_guard(w)?;
                        w.newline()?;

                        self.write_constants(w)?;
                        w.newline()?;

                        self.write_functions(w)?;
                        Ok(())
                    })?;
                }
            } else {
                if self.has_emittable_constants(self.inventory().constants()) {
                    self.write_class_context(self.config().class_constants.as_ref().unwrap(), w, |w| {
                        self.write_constants(w)?;
                        w.newline()?;

                        Ok(())
                    })?;
                }

                if self.has_emittable_functions(self.inventory().functions()) {
                    w.newline()?;
                    self.write_class_context(&self.config().class, w, |w| {
                        self.write_native_lib_string(w)?;
                        w.newline()?;

                        self.write_abi_guard(w)?;
                        w.newline()?;

                        self.write_functions(w)?;
                        Ok(())
                    })?;
                }
            }

            w.newline()?;
            self.write_type_definitions(w)?;

            w.newline()?;
            self.write_patterns(w)?;

            w.newline()?;
            self.write_builtins(w)?;

            Ok(())
        })?;

        Ok(())
    }
}
