use crate::config::Config;
use crate::converter::{CSharpTypeConverter, Converter};
use interoptopus::lang::c::{CType, CompositeType, Constant, Documentation, EnumType, Field, FnPointerType, Function, Meta, PrimitiveType, Variant, Visibility};
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, IdPrettifier};
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

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"using System;"#)?;
        indented!(w, r#"using System.Collections;"#)?;
        indented!(w, r#"using System.Collections.Generic;"#)?;
        indented!(w, r#"using System.Runtime.InteropServices;"#)?;

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
        indented!(w, r#"public const string NativeLib = "{}";"#, self.config().dll_name)
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for constant in self.library().constants() {
            if self.should_emit(constant.meta()) {
                self.write_constant(w, constant)?;
                w.newline()?;
            }
        }

        Ok(())
    }

    fn write_constant(&self, w: &mut IndentWriter, constant: &Constant) -> Result<(), Error> {
        let rval = self.converter().to_typespecifier_in_rval(&constant.the_type());
        let name = constant.name();
        let value = self.converter().constant_value_to_value(constant.value());

        self.write_documentation(w, constant.meta().documentation())?;
        indented!(w, r#"public const {} {} = ({}) {};"#, rval, name, rval, value)
    }

    fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for function in self.library().functions() {
            if self.should_emit(function.meta()) {
                self.write_function(w, function)?;
                w.newline()?;
            }
        }

        Ok(())
    }

    fn write_function(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        self.write_documentation(w, function.meta().documentation())?;
        self.write_function_annotation(w, function)?;
        self.write_function_declaration(w, function)?;

        w.newline()?;

        self.write_function_overloaded(w, function)?;

        Ok(())
    }

    fn write_documentation(&self, w: &mut IndentWriter, documentation: &Documentation) -> Result<(), Error> {
        for line in documentation.lines() {
            indented!(w, r#"/// {}"#, line)?;
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
                CType::Pattern(TypePattern::Slice(_)) => {
                    to_pin_name.push(name);
                    to_pin_slice_type.push(the_type);
                    to_invoke.push(format!("{}_slice", name));
                }
                _ => {
                    to_invoke.push(name.to_string());
                }
            }

            params.push(format!("{} {}", native, name));
        }

        indented!(w, r#"public static {} {}({}) {{"#, rval, name, params.join(", "))?;

        for (pin_var, slice_struct) in to_pin_name.iter().zip(to_pin_slice_type.iter()) {
            indented!(w, [_], r#"var {}_pinned = GCHandle.Alloc({}, GCHandleType.Pinned);"#, pin_var, pin_var)?;
            indented!(w, [_], r#"var {}_slice = new {}({}_pinned, (ulong) {}.Length);"#, pin_var, slice_struct, pin_var, pin_var)?;
        }

        indented!(w, [_], r#"try"#)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"return {}({});"#, name, to_invoke.join(", "))?;
        indented!(w, [_], r#"}}"#)?;
        indented!(w, [_], r#"finally"#)?;
        indented!(w, [_], r#"{{"#)?;
        for pin in &to_pin_name {
            indented!(w, [_ _], r#"{}_pinned.Free();"#, pin)?;
        }
        indented!(w, [_], r#"}}"#)?;
        indented!(w, r#"}}"#)
    }

    fn write_type_definitions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for the_type in self.library().ctypes() {
            self.write_type_definition(w, the_type)?;
        }

        Ok(())
    }

    fn write_type_definition(&self, w: &mut IndentWriter, the_type: &CType) -> Result<(), Error> {
        match the_type {
            CType::Primitive(_) => {}
            CType::Enum(e) => {
                if self.should_emit(e.meta()) {
                    self.write_type_definition_enum(w, e)?;
                    w.newline()?;
                }
            }
            CType::Opaque(_) => {}
            CType::Composite(c) => {
                if self.should_emit(c.meta()) {
                    self.write_type_definition_composite(w, c)?;
                    w.newline()?;
                }
            }
            CType::FnPointer(f) => {
                if self.should_emit_delegate() {
                    self.write_type_definition_fn_pointer(w, f)?;
                    w.newline()?;
                }
            }
            CType::ReadPointer(_) => {}
            CType::ReadWritePointer(_) => {}
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => {}
                TypePattern::SuccessEnum(e) => {
                    if self.should_emit(e.the_enum().meta()) {
                        self.write_type_definition_enum(w, e.the_enum())?;
                        w.newline()?;
                    }
                }
                TypePattern::Slice(x) => {
                    if self.should_emit(x.meta()) {
                        self.write_type_definition_composite(w, x)?;
                        w.newline()?;
                        self.write_pattern_slice(w, x)?;
                        w.newline()?;
                    }
                }
                TypePattern::Option(x) => {
                    if self.should_emit(x.meta()) {
                        self.write_type_definition_composite(w, x)?;
                        w.newline()?;
                    }
                }
                TypePattern::NamedCallback(x) => {
                    // Handle this better way
                    if self.should_emit(&Meta::new()) {
                        self.write_type_definition_named_callback(w, x)?;
                        w.newline()?;
                    }
                }
            },
        }
        Ok(())
    }

    fn write_type_definition_fn_pointer(&self, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
        self.write_type_definition_fn_pointer_annotation(w, the_type)?;
        self.write_type_definition_fn_pointer_body(w, the_type)?;
        Ok(())
    }

    fn write_type_definition_named_callback(&self, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
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
        let field_name = field.name();
        let type_name = self.converter().to_typespecifier_in_field(field.the_type(), field, the_type);
        let visibility = match field.visibility() {
            Visibility::Public => "public ",
            Visibility::Private => "",
        };

        indented!(w, r#"{}{} {};"#, visibility, type_name, field_name)
    }

    fn namespace_for_id(&self, id: &str) -> String {
        self.config()
            .namespace_mappings
            .get(id)
            .unwrap_or_else(|| panic!("Found a namespace not mapped '{}'. You should specify this one in the config.", id))
            .to_string()
    }

    fn write_namespace_context(&self, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        indented!(w, r#"namespace {}"#, self.namespace_for_id(&self.config().namespace_id))?;
        indented!(w, r#"{{"#)?;
        w.indent();

        f(w)?;

        w.unindent();

        indented!(w, r#"}}"#)
    }

    fn write_class_context(&self, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        indented!(w, r#"public static partial class {}"#, self.config().class)?;
        indented!(w, r#"{{"#)?;
        w.indent();

        f(w)?;

        w.unindent();
        indented!(w, r#"}}"#)
    }

    fn should_emit_delegate(&self) -> bool {
        self.config().namespace_id.is_empty()
    }

    fn has_emittable_functions(&self, functions: &[Function]) -> bool {
        functions.iter().any(|x| self.should_emit(x.meta()))
    }

    fn should_emit(&self, meta: &Meta) -> bool {
        let rval = meta.namespace() == self.config().namespace_id;
        rval
    }

    fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.library().patterns() {
            match pattern {
                LibraryPattern::Service(cls) => {
                    if self.should_emit(cls.the_type().meta()) {
                        self.write_pattern_class(w, cls)?
                    }
                }
            }
        }

        Ok(())
    }

    fn write_pattern_slice(&self, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
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

        // Getter
        indented!(w, [_], r#"public {} this[int i]"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"get"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"var size = Marshal.SizeOf(typeof({}));"#, type_string)?;
        indented!(w, [_ _ _], r#"var ptr = new IntPtr(data.ToInt64() + i * size);"#)?;
        indented!(w, [_ _ _], r#"return Marshal.PtrToStructure<{}>(ptr);"#, type_string)?;
        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_], r#"}}"#)?;

        // Copied
        indented!(w, [_], r#"public {}[] Copied"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"get"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"var rval = new {}[len];"#, type_string)?;
        indented!(w, [_ _ _], r#"for (var i = 0; i < (int) len; i++) {{"#)?;
        indented!(w, [_ _ _ _], r#"rval[i] = this[i];"#)?;
        indented!(w, [_ _ _], r#"}}"#)?;
        indented!(w, [_ _ _], r#"return rval;"#)?;
        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_], r#"}}"#)?;

        // Count
        indented!(w, [_], r#"public int Count"#)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"get"#)?;
        indented!(w, [_ _], r#"{{"#)?;
        indented!(w, [_ _ _], r#"return (int) len;"#)?;
        indented!(w, [_ _], r#"}}"#)?;
        indented!(w, [_], r#"}}"#)?;

        // GetEnumerator
        indented!(w, [_], r#"public IEnumerator<{}> GetEnumerator()"#, type_string)?;
        indented!(w, [_], r#"{{"#)?;
        indented!(w, [_ _], r#"for (int i = 0; i < (int)len; ++i)"#)?;
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

    fn write_pattern_class(&self, w: &mut IndentWriter, class: &Service) -> Result<(), Error> {
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
        self.write_pattern_class_success_enum_aware_rval(w, class, class.constructor(), false)?;
        w.unindent();
        indented!(w, r#"}}"#)?;
        w.newline()?;

        // Dtor
        indented!(w, r#"public void Dispose()"#)?;
        indented!(w, r#"{{"#)?;
        w.indent();
        self.write_pattern_class_success_enum_aware_rval(w, class, class.destructor(), false)?;
        w.unindent();
        indented!(w, r#"}}"#)?;
        w.newline()?;

        for function in class.methods() {
            let args = self.pattern_class_args_without_first_to_string(function, true);
            let without_common_prefix = function.name().replace(&common_prefix, "");
            let prettified = IdPrettifier::from_rust_lower(&without_common_prefix);
            let rval = match function.signature().rval() {
                CType::Pattern(TypePattern::SuccessEnum(_)) => "void".to_string(),
                _ => self.converter().to_typespecifier_in_rval(function.signature().rval()),
            };

            self.write_documentation(w, function.meta().documentation())?;

            indented!(w, r#"public {} {}({})"#, rval, prettified.to_camel_case(), &args)?;
            indented!(w, r#"{{"#)?;
            w.indent();
            self.write_pattern_class_success_enum_aware_rval(w, class, function, true)?;
            w.unindent();
            indented!(w, r#"}}"#)?;
            w.newline()?;
        }

        w.unindent();
        indented!(w, r#"}}"#)?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_pattern_class_success_enum_aware_rval(&self, w: &mut IndentWriter, _class: &Service, function: &Function, deref_context: bool) -> Result<(), Error> {
        let mut args = self.pattern_class_args_without_first_to_string(function, false);

        // Make sure we don't have a `,` when only single parameter
        if !args.is_empty() {
            args = format!(", {}", args);
        }

        let context = if deref_context { "_context".to_string() } else { "out _context".to_string() };

        match function.signature().rval() {
            CType::Pattern(TypePattern::SuccessEnum(e)) => {
                indented!(w, r#"var rval = {}.{}({} {});"#, self.config().class, function.name(), context, args)?;
                indented!(w, r#"if (rval != {}.{})"#, e.the_enum().rust_name(), e.success_variant().name())?;
                indented!(w, r#"{{"#)?;
                indented!(w, [_], r#"throw new Exception("Something went wrong");"#)?;
                indented!(w, r#"}}"#)?;
            }
            CType::Primitive(PrimitiveType::Void) => {
                indented!(w, r#"{}.{}({} {});"#, self.config().class, function.name(), context, args)?;
            }
            _ => {
                indented!(w, r#"return {}.{}({} {});"#, self.config().class, function.name(), context, args)?;
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
                format!(
                    "{} {}",
                    if with_types {
                        self.converter().to_typespecifier_in_param(x.the_type())
                    } else {
                        "".to_string()
                    },
                    x.name().to_string()
                )
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
