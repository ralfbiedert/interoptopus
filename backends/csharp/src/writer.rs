use crate::config::{Config, Unsafe, WriteTypes};
use crate::converter::{CSharpTypeConverter, Converter};
use interoptopus::lang::c::{CType, CompositeType, Constant, Documentation, EnumType, Field, FnPointerType, Function, Meta, PrimitiveType, Variant, Visibility};
use interoptopus::patterns::api_guard::library_hash;
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{is_global_type, longest_common_prefix, IdPrettifier};
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error, Library};

/// Writes the C# file format, `impl` this trait to customize output.
pub trait CSharpWriter {
    /// Returns the user config.
    fn config(&self) -> &Config;

    /// Returns the library to produce bindings for.
    fn library(&self) -> &Library;

    fn converter(&self) -> &Converter;

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
        indented!(w, r#"using System;"#)?;
        indented!(w, r#"using System.Collections;"#)?;
        indented!(w, r#"using System.Collections.Generic;"#)?;
        indented!(w, r#"using System.Runtime.InteropServices;"#)?;

        if self.config().use_unsafe == Unsafe::UnsafeCompilerService {
            indented!(w, r#"using System.Runtime.CompilerServices;"#)?;
        }

        for namespace_id in self.library().namespaces() {
            let namespace = self
                .config()
                .namespace_mappings
                .get(namespace_id)
                .unwrap_or_else(|| panic!("Must have namespace for '{}' ID", namespace_id));

            indented!(w, r#"using {};"#, namespace)?;
        }

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
            .library()
            .functions()
            .iter()
            .find(|x| matches!(x.signature().rval(), CType::Pattern(TypePattern::APIVersion)))
        {
            let version = library_hash(self.library());
            indented!(w, [_], r#"var api_version = {}.{}();"#, self.config().class, api_guard.name())?;
            indented!(w, [_], r#"if (api_version != {}ul)"#, version)?;
            indented!(w, [_], r#"{{"#)?;
            indented!(w, [_ _], r#"throw new Exception($"API reports hash {{api_version}} which differs from hash in bindings ({}). You probably forgot to update / copy either the bindings or the library.");"#, version)?;
            indented!(w, [_], r#"}}"#)?;
        }

        indented!(w, r#"}}"#)?;

        Ok(())
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for constant in self.library().constants() {
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
        for function in self.library().functions() {
            if self.should_emit_by_meta(function.meta()) {
                self.write_function(w, function)?;
                w.newline()?;
            }
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        self.debug(w, "write_function")?;
        self.write_documentation(w, function.meta().documentation())?;
        self.write_function_annotation(w, function)?;
        self.write_function_declaration(w, function)?;

        w.newline()?;

        self.write_function_overloaded(w, function)?;

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
        let name = self.converter().function_name_to_csharp_name(function);

        let mut params = Vec::new();
        for (_, p) in function.signature().params().iter().enumerate() {
            let the_type = self.converter().function_parameter_to_csharp_typename(p, function);
            let name = p.name();

            params.push(format!("{} {}", the_type, name));
        }

        indented!(w, r#"public static extern {} {}({});"#, rval, name, params.join(", "))
    }

    #[rustfmt::skip]
    fn write_function_overloaded(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        self.debug(w, "write_function_overloaded")?;

        // If there is nothing to write, don't do it
        if !self.converter().has_overloadable(function.signature()) {
            return Ok(());
        }

        let mut to_pin_name = Vec::new();
        let mut to_pin_slice_type = Vec::new();
        let mut to_invoke = Vec::new();
        let rval = self.converter().function_rval_to_csharp_typename(function);
        let name = self.converter().function_name_to_csharp_name(function);

        let mut params = Vec::new();
        for (_, p) in function.signature().params().iter().enumerate() {
            let name = p.name();
            let native = self.converter().pattern_to_native_in_signature(p, function.signature());
            let the_type = self.converter().function_parameter_to_csharp_typename(p, function);

            match p.the_type() {
                CType::Pattern(TypePattern::Slice(_) | TypePattern::SliceMut(_)) => {
                    to_pin_name.push(name);
                    to_pin_slice_type.push(the_type);
                    to_invoke.push(format!("{}_slice", name));
                }
                _ => {
                    if native.contains("out ") {
                        to_invoke.push(format!("out {}", name.to_string()));
                    } else if native.contains("ref ") {
                        to_invoke.push(format!("ref {}", name.to_string()));
                    } else {
                        to_invoke.push(name.to_string());
                    }
                }
            }

            params.push(format!("{} {}", native, name));
        }


        let return_stmt = if function.signature().rval().is_void() { "" } else { "return " };

        indented!(w, r#"public static {} {}({}) {{"#, rval, name, params.join(", "))?;

        if self.config().use_unsafe.any_unsafe() {

            // unsafe
            //     {
            //         fixed (Vec3f32* ptr = ffi_slice)
            //         {
            //             var ffi_slice_slice = new SliceVec3f32(new IntPtr(ptr), (ulong)ffi_slice.Length);
            //             return pattern_ffi_slice_2(ffi_slice_slice, i);
            //         }
            //     }

            indented!(w, [_], r#"unsafe"#)?;
            indented!(w, [_], r#"{{"#)?;
            w.indent();

            for (pin_var, slice_struct) in to_pin_name.iter().zip(to_pin_slice_type.iter()) {
                indented!(w, [_], r#"fixed (void* ptr_{} = {})"#, pin_var, pin_var)?;
                indented!(w, [_], r#"{{"#)?;
                indented!(w, [_ _], r#"var {}_slice = new {}(new IntPtr(ptr_{}), (ulong) {}.Length);"#, pin_var, slice_struct, pin_var, pin_var)?;
                w.indent();
            }

            indented!(w, [_], r#"{}{}({});"#, return_stmt, name, to_invoke.join(", "))?;
            for _ in to_pin_name.iter() {
                w.unindent();
                indented!(w, [_], r#"}}"#)?;
            }

            w.unindent();
            indented!(w, [_], r#"}}"#)?;
        } else {
            for (pin_var, slice_struct) in to_pin_name.iter().zip(to_pin_slice_type.iter()) {
                indented!(w, [_], r#"var {}_pinned = GCHandle.Alloc({}, GCHandleType.Pinned);"#, pin_var, pin_var)?;
                indented!(w, [_], r#"var {}_slice = new {}({}_pinned, (ulong) {}.Length);"#, pin_var, slice_struct, pin_var, pin_var)?;
            }

            indented!(w, [_], r#"try"#)?;
            indented!(w, [_], r#"{{"#)?;
            indented!(w, [_ _], r#"{}{}({});"#, return_stmt, name, to_invoke.join(", "))?;
            indented!(w, [_], r#"}}"#)?;
            indented!(w, [_], r#"finally"#)?;
            indented!(w, [_], r#"{{"#)?;
            for pin in &to_pin_name {
                indented!(w, [_ _], r#"{}_pinned.Free();"#, pin)?;
            }
            indented!(w, [_], r#"}}"#)?;
        }

        indented!(w, r#"}}"#)
    }

    fn write_type_definitions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for the_type in self.library().ctypes() {
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
                self.write_type_definition_enum(w, e)?;
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
                TypePattern::AsciiPointer => {}
                TypePattern::FFIErrorEnum(e) => {
                    self.write_type_definition_enum(w, e.the_enum())?;
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
        indented!(w, r#"public partial struct {}"#, type_name)?;
        indented!(w, r#"{{"#)?;
        indented!(w, [_], r#"byte value;"#)?;
        indented!(w, r#"}}"#)?;
        w.newline()?;

        indented!(w, r#"public partial struct {}"#, type_name)?;
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
        Ok(())
    }

    fn write_type_definition_named_callback_body(&self, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
        let rval = self.converter().to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
        let name = self.converter().named_callback_to_typename(the_type);

        let mut params = Vec::new();
        for (i, param) in the_type.fnpointer().signature().params().iter().enumerate() {
            params.push(format!("{} x{}", self.converter().to_typespecifier_in_param(param.the_type()), i));
        }

        indented!(w, r#"public delegate {} {}({});"#, rval, name, params.join(", "))
    }

    fn write_type_definition_fn_pointer_annotation(&self, w: &mut IndentWriter, _the_type: &FnPointerType) -> Result<(), Error> {
        indented!(w, r#"[UnmanagedFunctionPointer(CallingConvention.Cdecl)]"#)
    }

    fn write_type_definition_fn_pointer_body(&self, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
        let rval = self.converter().to_typespecifier_in_rval(the_type.signature().rval());
        let name = self.converter().fnpointer_to_typename(the_type);

        let mut params = Vec::new();
        for (i, param) in the_type.signature().params().iter().enumerate() {
            params.push(format!("{} x{}", self.converter().to_typespecifier_in_param(param.the_type()), i));
        }

        indented!(w, r#"public delegate {} {}({});"#, rval, name, params.join(", "))
    }

    fn write_type_definition_enum(&self, w: &mut IndentWriter, the_type: &EnumType) -> Result<(), Error> {
        self.debug(w, "write_type_definition_enum")?;
        self.write_documentation(w, the_type.meta().documentation())?;
        indented!(w, r#"public enum {}"#, the_type.rust_name())?;
        indented!(w, r#"{{"#)?;
        w.indent();

        for variant in the_type.variants() {
            self.write_type_definition_enum_variant(w, variant, the_type)?;
        }

        w.unindent();
        indented!(w, r#"}}"#)
    }

    fn write_type_definition_enum_variant(&self, w: &mut IndentWriter, variant: &Variant, _the_type: &EnumType) -> Result<(), Error> {
        let variant_name = variant.name();
        let variant_value = variant.value();
        self.write_documentation(w, variant.documentation())?;
        indented!(w, r#"{} = {},"#, variant_name, variant_value)
    }

    fn write_type_definition_composite(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        self.debug(w, "write_type_definition_composite")?;
        self.write_documentation(w, the_type.meta().documentation())?;
        self.write_type_definition_composite_annotation(w, the_type)?;
        self.write_type_definition_composite_body(w, the_type)
    }

    fn write_type_definition_composite_annotation(&self, w: &mut IndentWriter, _the_type: &CompositeType) -> Result<(), Error> {
        indented!(w, r#"[Serializable]"#)?;
        indented!(w, r#"[StructLayout(LayoutKind.Sequential)]"#)
    }

    fn write_type_definition_composite_body(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        indented!(w, r#"public partial struct {}"#, the_type.rust_name())?;
        indented!(w, r#"{{"#)?;
        w.indent();

        for field in the_type.fields() {
            self.write_documentation(w, field.documentation())?;
            self.write_type_definition_composite_body_field(w, field, the_type)?;
        }

        w.unindent();
        indented!(w, r#"}}"#)
    }

    fn write_type_definition_composite_body_field(&self, w: &mut IndentWriter, field: &Field, the_type: &CompositeType) -> Result<(), Error> {
        match field.the_type() {
            CType::Array(a) => {
                if !self.config().unroll_struct_arrays {
                    panic!("Unable to generate bindings for arrays in fields if `unroll_struct_arrays` is not enabled.");
                }

                let field_name = field.name();
                let type_name = self.converter().to_typespecifier_in_field(a.array_type(), field, the_type);
                let visibility = match field.visibility() {
                    Visibility::Public => "public ",
                    Visibility::Private => "",
                };

                for i in 0..a.len() {
                    indented!(w, r#"{}{} {}{};"#, visibility, type_name, field_name, i)?;
                }

                Ok(())
            }
            _ => {
                let field_name = field.name();
                let type_name = self.converter().to_typespecifier_in_field(field.the_type(), field, the_type);
                let visibility = match field.visibility() {
                    Visibility::Public => "public ",
                    Visibility::Private => "",
                };

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

    fn write_class_context(&self, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        self.debug(w, "write_class_context")?;
        indented!(w, r#"public static partial class {}"#, self.config().class)?;
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

    fn should_emit_by_meta(&self, meta: &Meta) -> bool {
        let rval = meta.namespace() == self.config().namespace_id;
        rval
    }

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
                TypePattern::AsciiPointer => true,
                TypePattern::APIVersion => true,
                TypePattern::FFIErrorEnum(x) => self.should_emit_by_meta(x.the_enum().meta()),
                TypePattern::Slice(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::SliceMut(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::Option(x) => self.should_emit_by_meta(x.meta()),
                TypePattern::Bool => self.config().write_types == WriteTypes::NamespaceAndInteroptopusGlobal,
                TypePattern::NamedCallback(_) => true,
            },
        }
    }

    fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.library().patterns() {
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

        indented!(w, r#"public partial struct {}"#, context_type_name)?;
        indented!(w, r#"{{"#)?;

        // FromNullable
        indented!(w, [_], r#"public static {} FromNullable({}? nullable)"#, context_type_name, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"var result = new {}();"#, context_type_name)?;
        indented!(w, [_ _], r#"if (nullable.HasValue)"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"result.is_some = 1;"#)?;
        indented!(w, [_ _ _], r#"result.t = nullable.Value;"#)?;
        indented!(w, [_ _], r#"}}"#)?;
        w.newline()?;
        indented!(w, [_ _], r#"return result;"#)?;
        indented!(w, [_], r#"}}"#)?;
        w.newline()?;

        // ToNullable
        indented!(w, [_], r#"public {}? ToNullable()"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"return this.is_some == 1 ? this.t : ({}?)null;"#, type_string)?;
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
            .deref_pointer()
            .expect("data must be a pointer type");

        let type_string = self.converter().to_typespecifier_in_rval(data_type);

        indented!(w, r#"public partial struct {} : IEnumerable<{}>"#, context_type_name, type_string)?;
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
        indented!(w, [_], r#"}}"#)?;

        // Copied
        indented!(w, [_], r#"public {}[] Copied"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"get"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"var rval = new {}[len];"#, type_string)?;

        if self.config().use_unsafe == Unsafe::UnsafeCompilerService {
            indented!(w, [_ _ _], r#"unsafe"#)?;
            indented!(w, [_ _ _], r#"{{"#)?;
            indented!(w, [_ _ _ _ ], r#"fixed (void* dst = rval)"#)?;
            indented!(w, [_ _ _ _ ], r#"{{"#)?;
            indented!(w, [_ _ _ _ _], r#"Unsafe.CopyBlock(dst, data.ToPointer(), (uint)len);"#)?;
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
            .deref_pointer()
            .expect("data must be a pointer type");

        let type_string = self.converter().to_typespecifier_in_rval(data_type);

        indented!(w, r#"public partial struct {} : IEnumerable<{}>"#, context_type_name, type_string)?;
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

        if self.config().use_unsafe == Unsafe::UnsafeCompilerService {
            indented!(w, [_ _ _], r#"unsafe"#)?;
            indented!(w, [_ _ _], r#"{{"#)?;
            indented!(w, [_ _ _ _ ], r#"fixed (void* dst = rval)"#)?;
            indented!(w, [_ _ _ _ ], r#"{{"#)?;
            indented!(w, [_ _ _ _ _], r#"Unsafe.CopyBlock(dst, data.ToPointer(), (uint)len);"#)?;
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
        let mut all_functions = vec![class.constructor().clone(), class.destructor().clone()];
        all_functions.extend_from_slice(class.methods());

        let context_type_name = class.the_type().rust_name();
        let common_prefix = longest_common_prefix(&all_functions);

        self.write_documentation(w, class.the_type().meta().documentation())?;
        indented!(w, r#"public partial class {} : IDisposable"#, context_type_name)?;
        indented!(w, r#"{{"#)?;
        w.indent();
        indented!(w, r#"private IntPtr _context;"#)?;

        // Ctor
        let args = self.pattern_class_args_without_first_to_string(class.constructor(), true);
        self.write_documentation(w, class.constructor().meta().documentation())?;
        indented!(w, r#"public {}({})"#, context_type_name, args)?;
        indented!(w, r#"{{"#)?;
        w.indent();
        self.write_pattern_service_success_enum_aware_rval(w, class, class.constructor(), false)?;
        w.unindent();
        indented!(w, r#"}}"#)?;
        w.newline()?;

        // Dtor
        indented!(w, r#"public void Dispose()"#)?;
        indented!(w, r#"{{"#)?;
        w.indent();
        self.write_pattern_service_success_enum_aware_rval(w, class, class.destructor(), false)?;
        w.unindent();
        indented!(w, r#"}}"#)?;
        w.newline()?;

        for function in class.methods() {
            // Main function
            let args = self.pattern_class_args_without_first_to_string(function, true);
            let without_common_prefix = function.name().replace(&common_prefix, "");
            let prettified = IdPrettifier::from_rust_lower(&without_common_prefix);
            let fn_name = prettified.to_camel_case();
            let rval = match function.signature().rval() {
                CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
                _ => self.converter().to_typespecifier_in_rval(function.signature().rval()),
            };

            self.write_documentation(w, function.meta().documentation())?;

            indented!(w, r#"public {} {}({})"#, rval, fn_name, &args)?;
            indented!(w, r#"{{"#)?;
            w.indent();
            self.write_pattern_service_success_enum_aware_rval(w, class, function, true)?;
            w.unindent();
            indented!(w, r#"}}"#)?;
            w.newline()?;

            // Overloaded
            if self.converter().has_overloadable(function.signature()) {
                self.write_documentation(w, function.meta().documentation())?;
                self.write_pattern_service_method_overload(w, class, function, &rval, &fn_name)?;
                w.newline()?;
            }
        }

        indented!(w, r#"public IntPtr Context => _context;"#)?;

        w.unindent();
        indented!(w, r#"}}"#)?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_pattern_service_method_overload(&self, w: &mut IndentWriter, _class: &Service, function: &Function, rval: &str, fn_name: &str) -> Result<(), Error> {
        self.debug(w, "write_pattern_service_method_overload")?;
        let mut names = Vec::new();
        let mut to_invoke = Vec::new();
        let mut types = Vec::new();

        for p in function.signature().params().iter().skip(1) {
            let name = p.name();
            let native = self.converter().pattern_to_native_in_signature(p, function.signature());

            if native.contains("out ") {
                to_invoke.push(format!("out {}", name.to_string()));
            } else if native.contains("ref ") {
                to_invoke.push(format!("ref {}", name.to_string()));
            } else {
                to_invoke.push(name.to_string());
            }

            names.push(name);
            types.push(native);
        }

        let arg_tokens = names.iter().zip(types.iter()).map(|(n, t)| format!("{} {}", t, n)).collect::<Vec<_>>();
        let fn_call = format!(r#"{}.{}(_context, {})"#, self.config().class, function.name(), to_invoke.join(", "));

        indented!(w, r#"public {} {}({})"#, rval, fn_name, arg_tokens.join(", "))?;
        indented!(w, r#"{{"#)?;

        match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
                indented!(w, [_], r#"var rval = {};"#, fn_call)?;
                indented!(w, [_], r#"if (rval != {}.{})"#, e.the_enum().rust_name(), e.success_variant().name())?;
                indented!(w, [_], r#"{{"#)?;
                indented!(w, [_ _], r#"throw new Exception($"Something went wrong: {{rval}}");"#)?;
                indented!(w, [_], r#"}}"#)?;
            }
            CType::Primitive(PrimitiveType::Void) => {
                indented!(w, [_], r#"{};"#, fn_call)?;
            }
            _ => {
                indented!(w, [_], r#"return {};"#, fn_call)?;
            }
        }

        indented!(w, r#"}}"#)?;

        Ok(())
    }

    fn write_pattern_service_success_enum_aware_rval(&self, w: &mut IndentWriter, _class: &Service, function: &Function, deref_context: bool) -> Result<(), Error> {
        self.debug(w, "write_pattern_service_success_enum_aware_rval")?;
        let mut args = self.pattern_class_args_without_first_to_string(function, false);

        // Make sure we don't have a `,` when only single parameter
        if !args.is_empty() {
            args = format!(", {}", args);
        }

        let context = if deref_context { "_context".to_string() } else { "ref _context".to_string() };

        match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
                indented!(w, r#"var rval = {}.{}({} {});"#, self.config().class, function.name(), context, args)?;
                indented!(w, r#"if (rval != {}.{})"#, e.the_enum().rust_name(), e.success_variant().name())?;
                indented!(w, r#"{{"#)?;
                indented!(w, [_], r#"throw new Exception($"Something went wrong {{rval}}");"#)?;
                indented!(w, r#"}}"#)?;
            }
            CType::Primitive(PrimitiveType::Void) => {
                indented!(w, r#"{}.{}({}{});"#, self.config().class, function.name(), context, args)?;
            }
            _ => {
                indented!(w, r#"return {}.{}({}{});"#, self.config().class, function.name(), context, args)?;
            }
        }

        Ok(())
    }

    fn pattern_class_args_without_first_to_string(&self, function: &Function, with_types: bool) -> String {
        function
            .signature()
            .params()
            .iter()
            .skip(1)
            .map(|x| {
                let with_type = self.converter().to_typespecifier_in_param(x.the_type());

                let name = if with_types {
                    x.name().to_string()
                } else if with_type.contains("ref ") {
                    format!("ref {}", x.name().to_string())
                } else if with_type.contains("out ") {
                    format!("out {}", x.name().to_string())
                } else {
                    x.name().to_string()
                };

                format!("{}{}", if with_types { format!("{} ", with_type) } else { "".to_string() }, name)
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn write_all(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_file_header_comments(w)?;
        w.newline()?;

        self.write_imports(w)?;
        w.newline()?;

        self.write_namespace_context(w, |w| {
            if self.has_emittable_functions(self.library().functions()) {
                self.write_class_context(w, |w| {
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

            w.newline()?;
            self.write_type_definitions(w)?;

            w.newline()?;
            self.write_patterns(w)?;

            Ok(())
        })?;

        Ok(())
    }
}