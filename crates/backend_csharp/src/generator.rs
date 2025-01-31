use crate::config::{Config, Unsupported, WriteTypes};
use crate::converter::{CSharpTypeConverter, Converter, FunctionNameFlavor};
use interoptopus::lang::c::{
    ArrayType, CType, CompositeType, Constant, Documentation, EnumType, Field, FnPointerType, Function, Layout, Meta, Parameter, PrimitiveType, Variant, Visibility,
};
use interoptopus::patterns::api_guard::inventory_hash;
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{is_global_type, longest_common_prefix};
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, Error, Generate, Inventory};
use std::iter::zip;

/// **Start here**, main converter implementing [`Generate`].
pub struct Generator {
    config: Config,
    inventory: Inventory,
    converter: Converter,
}

/// Writes the C# file format, `impl` this trait to customize output.
impl Generator {
    #[must_use]
    pub const fn new(config: Config, inventory: Inventory) -> Self {
        Self {
            config,
            inventory,
            converter: Converter {},
        }
    }

    pub fn write_file_header_comments(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r"{}", &self.config.file_header_comment)?;
        Ok(())
    }

    pub fn debug(&self, w: &mut IndentWriter, marker: &str) -> Result<(), Error> {
        if !self.config.debug {
            return Ok(());
        }

        indented!(w, r"// Debug - {} ", marker)
    }

    pub fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.debug(w, "write_imports")?;

        indented!(w, r"#pragma warning disable 0105")?;
        indented!(w, r"using System;")?;
        indented!(w, r"using System.Text;")?;
        indented!(w, r"using System.Collections;")?;
        indented!(w, r"using System.Collections.Generic;")?;
        indented!(w, r"using System.Runtime.InteropServices;")?;
        indented!(w, r"using System.Runtime.InteropServices.Marshalling;")?;
        indented!(w, r"using System.Runtime.CompilerServices;")?;

        for namespace_id in self.inventory.namespaces() {
            let namespace = self
                .config
                .namespace_mappings
                .get(namespace_id)
                .unwrap_or_else(|| panic!("Must have namespace for '{namespace_id}' ID"));

            indented!(w, r"using {};", namespace)?;
        }
        indented!(w, r"#pragma warning restore 0105")?;

        Ok(())
    }

    pub fn write_native_lib_string(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.debug(w, "write_native_lib_string")?;
        indented!(w, r#"public const string NativeLib = "{}";"#, self.config.dll_name)
    }

    pub fn write_abi_guard(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.debug(w, "write_abi_guard")?;

        indented!(w, r"static {}()", self.config.class)?;
        indented!(w, r"{{")?;

        // Check if there is a API version marker for us to write
        if let Some(api_guard) = self
            .inventory
            .functions()
            .iter()
            .find(|x| matches!(x.signature().rval(), CType::Pattern(TypePattern::APIVersion)))
        {
            let version = inventory_hash(&self.inventory);
            let flavor = if self.config.rename_symbols {
                FunctionNameFlavor::CSharpMethodNameWithClass
            } else {
                FunctionNameFlavor::RawFFIName
            };
            let fn_call = self.converter.function_name_to_csharp_name(api_guard, flavor);
            indented!(w, [()], r"var api_version = {}.{}();", self.config.class, fn_call)?;
            indented!(w, [()], r"if (api_version != {}ul)", version)?;
            indented!(w, [()], r"{{")?;
            indented!(
                w,
                [()()],
                r#"throw new TypeLoadException($"API reports hash {{api_version}} which differs from hash in bindings ({}). You probably forgot to update / copy either the bindings or the library.");"#,
                version
            )?;
            indented!(w, [()], r"}}")?;
        }

        indented!(w, r"}}")?;

        Ok(())
    }

    pub fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for constant in self.inventory.constants() {
            if self.should_emit_by_meta(constant.meta()) {
                self.write_constant(w, constant)?;
                w.newline()?;
            }
        }

        Ok(())
    }

    pub fn write_constant(&self, w: &mut IndentWriter, constant: &Constant) -> Result<(), Error> {
        self.debug(w, "write_constant")?;
        let rval = self.converter.to_typespecifier_in_rval(&constant.the_type());
        let name = constant.name();
        let value = self.converter.constant_value_to_value(constant.value());

        self.write_documentation(w, constant.meta().documentation())?;
        indented!(w, r"public const {} {} = ({}) {};", rval, name, rval, value)
    }

    pub fn write_functions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for function in self.inventory.functions() {
            if self.should_emit_by_meta(function.meta()) {
                self.write_function(w, function, WriteFor::Code)?;
                w.newline()?;
            }
        }

        Ok(())
    }

    pub fn write_function(&self, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
        self.debug(w, "write_function")?;
        if write_for == WriteFor::Code {
            self.write_documentation(w, function.meta().documentation())?;
            self.write_function_annotation(w, function)?;
        }
        self.write_function_declaration(w, function)?;
        self.write_function_overload(w, function, write_for)?;

        Ok(())
    }

    pub fn write_documentation(&self, w: &mut IndentWriter, documentation: &Documentation) -> Result<(), Error> {
        for line in documentation.lines() {
            indented!(w, r"///{}", line)?;
        }

        Ok(())
    }

    pub fn write_function_annotation(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        indented!(w, r#"[LibraryImport(NativeLib, EntryPoint = "{}")]"#, function.name())?;

        if *function.signature().rval() == CType::Primitive(PrimitiveType::Bool) {
            indented!(w, r"[return: MarshalAs(UnmanagedType.U1)]")?;
        }

        Ok(())
    }

    pub fn write_function_declaration(&self, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
        let rval = self.converter.function_rval_to_csharp_typename(function);
        let name = self.converter.function_name_to_csharp_name(
            function,
            if self.config.rename_symbols {
                FunctionNameFlavor::CSharpMethodNameWithClass
            } else {
                FunctionNameFlavor::RawFFIName
            },
        );

        let mut params = Vec::new();
        for p in function.signature().params() {
            let the_type = self.converter.function_parameter_to_csharp_typename(p);
            let name = p.name();

            params.push(format!("{the_type} {name}"));
        }

        indented!(w, r"public static partial {} {}({});", rval, name, params.join(", "))
    }

    #[allow(clippy::too_many_lines)]
    pub fn write_function_overload(&self, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
        let has_overload = self.converter.has_overloadable(function.signature());
        let has_error_enum = self.converter.has_ffi_error_rval(function.signature());

        // If there is nothing to write, don't do it
        if !has_overload && !has_error_enum {
            return Ok(());
        }

        let mut to_pin_name = Vec::new();
        let mut to_pin_slice_type = Vec::new();
        let mut to_invoke = Vec::new();
        let mut to_wrap_delegates = Vec::new();
        let mut to_wrap_delegate_types = Vec::new();

        let raw_name = self.converter.function_name_to_csharp_name(
            function,
            if self.config.rename_symbols {
                FunctionNameFlavor::CSharpMethodNameWithClass
            } else {
                FunctionNameFlavor::RawFFIName
            },
        );
        let this_name = if has_error_enum && !has_overload {
            format!("{raw_name}_checked")
        } else {
            raw_name
        };

        let rval = match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
            CType::Pattern(TypePattern::CStrPointer) => "string".to_string(),
            _ => self.converter.to_typespecifier_in_rval(function.signature().rval()),
        };

        let mut params = Vec::new();
        for p in function.signature().params() {
            let name = p.name();
            let native = self.pattern_to_native_in_signature(p);
            let the_type = self.converter.function_parameter_to_csharp_typename(p);

            let mut fallback = || {
                if native.contains("out ") {
                    to_invoke.push(format!("out {name}"));
                } else if native.contains("ref ") {
                    to_invoke.push(format!("ref {name}"));
                } else {
                    to_invoke.push(name.to_string());
                }
            };

            match p.the_type() {
                CType::Pattern(TypePattern::Slice(_) | TypePattern::SliceMut(_)) => {
                    to_pin_name.push(name);
                    to_pin_slice_type.push(the_type);
                    to_invoke.push(format!("{name}_slice"));
                }
                CType::Pattern(TypePattern::NamedCallback(callback)) => match callback.fnpointer().signature().rval() {
                    CType::Pattern(TypePattern::FFIErrorEnum(_)) if self.config.work_around_exception_in_callback_no_reentry => {
                        to_wrap_delegates.push(name);
                        to_wrap_delegate_types.push(self.converter.to_typespecifier_in_param(p.the_type()));
                        to_invoke.push(format!("{name}_safe_delegate.Call"));
                    }
                    _ => fallback(),
                },
                CType::ReadPointer(x) | CType::ReadWritePointer(x) => match &**x {
                    CType::Pattern(x) => match x {
                        TypePattern::Slice(_) => {
                            to_pin_name.push(name);
                            to_pin_slice_type.push(the_type.replace("ref ", ""));
                            to_invoke.push(format!("ref {name}_slice"));
                        }
                        TypePattern::SliceMut(_) => {
                            to_pin_name.push(name);
                            to_pin_slice_type.push(the_type.replace("ref ", ""));
                            to_invoke.push(format!("ref {name}_slice"));
                        }
                        _ => fallback(),
                    },
                    _ => fallback(),
                },
                _ => fallback(),
            }

            params.push(format!("{native} {name}"));
        }

        let signature = format!(r"public static unsafe {} {}({})", rval, this_name, params.join(", "));
        if write_for == WriteFor::Docs {
            indented!(w, r"{};", signature)?;
            return Ok(());
        }

        w.newline()?;

        if write_for == WriteFor::Code {
            self.write_documentation(w, function.meta().documentation())?;
        }

        indented!(w, "{}", signature)?;
        indented!(w, r"{{")?;

        for (name, ty) in zip(&to_wrap_delegates, &to_wrap_delegate_types) {
            indented!(w, [()], r"var {}_safe_delegate = new {}ExceptionSafe({});", name, ty, name)?;
        }

        if !to_pin_name.is_empty() {
            for (pin_var, slice_struct) in to_pin_name.iter().zip(to_pin_slice_type.iter()) {
                indented!(w, [()], r"fixed (void* ptr_{} = {})", pin_var, pin_var)?;
                indented!(w, [()], r"{{")?;
                indented!(
                    w,
                    [()()],
                    r"var {}_slice = new {}(new IntPtr(ptr_{}), (ulong) {}.Length);",
                    pin_var,
                    slice_struct,
                    pin_var,
                    pin_var
                )?;
                w.indent();
            }
        }

        let fn_name = self.converter.function_name_to_csharp_name(
            function,
            if self.config.rename_symbols {
                FunctionNameFlavor::CSharpMethodNameWithClass
            } else {
                FunctionNameFlavor::RawFFIName
            },
        );
        let call = format!(r"{}({});", fn_name, to_invoke.join(", "));

        self.write_function_overloaded_invoke_with_error_handling(w, function, &call, to_wrap_delegates.as_slice())?;

        if !to_pin_name.is_empty() {
            for _ in &to_pin_name {
                w.unindent();
                indented!(w, [()], r"}}")?;
            }
        }

        indented!(w, r"}}")
    }

    /// Writes common error handling based on a call's return type.
    pub fn write_function_overloaded_invoke_with_error_handling(
        &self,
        w: &mut IndentWriter,
        function: &Function,
        fn_call: &str,
        rethrow_delegates: &[&str],
    ) -> Result<(), Error> {
        match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
                indented!(w, [()], r"var rval = {};", fn_call)?;
                for name in rethrow_delegates {
                    indented!(w, [()], r"{}_safe_delegate.Rethrow();", name)?;
                }
                indented!(w, [()], r"if (rval != {}.{})", e.the_enum().rust_name(), e.success_variant().name())?;
                indented!(w, [()], r"{{")?;
                indented!(w, [()()], r"throw new InteropException<{}>(rval);", e.the_enum().rust_name())?;
                indented!(w, [()], r"}}")?;
            }
            CType::Pattern(TypePattern::CStrPointer) => {
                indented!(w, [()], r"var s = {};", fn_call)?;
                indented!(w, [()], r"return Marshal.PtrToStringAnsi(s);")?;
            }
            CType::Primitive(PrimitiveType::Void) => {
                indented!(w, [()], r"{};", fn_call)?;
            }
            _ => {
                indented!(w, [()], r"return {};", fn_call)?;
            }
        }

        Ok(())
    }

    pub fn write_type_definitions(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for the_type in self.inventory.ctypes() {
            self.write_type_definition(w, the_type)?;
        }

        Ok(())
    }

    pub fn write_type_definition(&self, w: &mut IndentWriter, the_type: &CType) -> Result<(), Error> {
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
                _ => panic!("Pattern not explicitly handled"),
            },
        }
        Ok(())
    }

    pub fn write_type_definition_ffibool(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.debug(w, "write_type_definition_ffibool")?;

        let type_name = self.converter.to_typespecifier_in_param(&CType::Pattern(TypePattern::Bool));

        indented!(w, r"[Serializable]")?;
        indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
        indented!(w, r"{} partial struct {}", self.config.visibility_types.to_access_modifier(), type_name)?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"byte value;")?;
        indented!(w, r"}}")?;
        w.newline()?;

        indented!(w, r"{} partial struct {}", self.config.visibility_types.to_access_modifier(), type_name)?;
        indented!(w, r"{{")?;
        indented!(w, [()], r"public static readonly {} True = new Bool {{ value =  1 }};", type_name)?;
        indented!(w, [()], r"public static readonly {} False = new Bool {{ value =  0 }};", type_name)?;
        indented!(w, [()], r"public Bool(bool b)")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"value = (byte) (b ? 1 : 0);")?;
        indented!(w, [()], r"}}")?;
        indented!(w, [()], r"public bool Is => value == 1;")?;
        indented!(w, r"}}")?;
        w.newline()?;
        Ok(())
    }

    pub fn write_type_definition_fn_pointer(&self, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
        self.debug(w, "write_type_definition_fn_pointer")?;
        self.write_type_definition_fn_pointer_annotation(w, the_type)?;
        self.write_type_definition_fn_pointer_body(w, the_type)?;
        Ok(())
    }

    pub fn write_type_definition_named_callback(&self, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
        self.debug(w, "write_type_definition_named_callback")?;
        self.write_type_definition_fn_pointer_annotation(w, the_type.fnpointer())?;
        self.write_type_definition_named_callback_body(w, the_type)?;
        self.write_callback_overload(w, the_type)?;
        Ok(())
    }

    pub fn write_callback_overload(&self, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
        if !self.config.work_around_exception_in_callback_no_reentry {
            return Ok(());
        }

        let CType::Pattern(TypePattern::FFIErrorEnum(ffi_error)) = the_type.fnpointer().signature().rval() else {
            return Ok(());
        };

        let name = format!("{}ExceptionSafe", the_type.name());
        let rval = self.converter.to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
        let mut function_signature = Vec::new();
        let mut function_param_names = Vec::new();

        for p in the_type.fnpointer().signature().params() {
            let name = p.name();
            let the_type = self.converter.function_parameter_to_csharp_typename(p);

            let x = format!("{the_type} {name}");
            function_signature.push(x);
            function_param_names.push(name);
        }

        w.newline()?;
        indented!(w, "// Internal helper that works around an issue where exceptions in callbacks don't reenter Rust.")?;
        indented!(w, "{} class {} {{", self.config.visibility_types.to_access_modifier(), name)?;
        indented!(w, [()], "private Exception failure = null;")?;
        indented!(w, [()], "private readonly {} _callback;", the_type.name())?;
        w.newline()?;
        indented!(w, [()], "public {}({} original)", name, the_type.name())?;
        indented!(w, [()], "{{")?;
        indented!(w, [()()], "_callback = original;")?;
        indented!(w, [()], "}}")?;
        w.newline()?;
        indented!(w, [()], "public {} Call({})", rval, function_signature.join(", "))?;
        indented!(w, [()], "{{")?;
        indented!(w, [()()], "try")?;
        indented!(w, [()()], "{{")?;
        indented!(w, [()()()], "return _callback({});", function_param_names.join(", "))?;
        indented!(w, [()()], "}}")?;
        indented!(w, [()()], "catch (Exception e)")?;
        indented!(w, [()()], "{{")?;
        indented!(w, [()()()], "failure = e;")?;
        indented!(w, [()()()], "return {}.{};", rval, ffi_error.panic_variant().name())?;
        indented!(w, [()()], "}}")?;
        indented!(w, [()], "}}")?;
        w.newline()?;
        indented!(w, [()], "public void Rethrow()")?;
        indented!(w, [()], "{{")?;
        indented!(w, [()()], "if (this.failure != null)")?;
        indented!(w, [()()], "{{")?;
        indented!(w, [()()()], "throw this.failure;")?;
        indented!(w, [()()], "}}")?;
        indented!(w, [()], "}}")?;
        indented!(w, "}}")?;

        Ok(())
    }

    pub fn write_type_definition_named_callback_body(&self, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
        let rval = self.converter.to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
        let name = self.converter.named_callback_to_typename(the_type);
        let visibility = self.config.visibility_types.to_access_modifier();

        let mut params = Vec::new();
        for param in the_type.fnpointer().signature().params() {
            params.push(format!("{} {}", self.converter.to_typespecifier_in_param(param.the_type()), param.name()));
        }

        indented!(w, r"{} delegate {} {}({});", visibility, rval, name, params.join(", "))
    }

    pub fn write_type_definition_fn_pointer_annotation(&self, w: &mut IndentWriter, _the_type: &FnPointerType) -> Result<(), Error> {
        indented!(w, r"[UnmanagedFunctionPointer(CallingConvention.Cdecl)]")
    }

    pub fn write_type_definition_fn_pointer_body(&self, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
        let rval = self.converter.to_typespecifier_in_rval(the_type.signature().rval());
        let name = self.converter.fnpointer_to_typename(the_type);
        let visibility = self.config.visibility_types.to_access_modifier();

        let mut params = Vec::new();
        for (i, param) in the_type.signature().params().iter().enumerate() {
            params.push(format!("{} x{}", self.converter.to_typespecifier_in_param(param.the_type()), i));
        }

        indented!(w, r"{} delegate {} {}({});", visibility, rval, name, params.join(", "))
    }

    pub fn write_type_definition_enum(&self, w: &mut IndentWriter, the_type: &EnumType, write_for: WriteFor) -> Result<(), Error> {
        self.debug(w, "write_type_definition_enum")?;
        if write_for == WriteFor::Code {
            self.write_documentation(w, the_type.meta().documentation())?;
        }
        indented!(w, r"public enum {}", the_type.rust_name())?;
        indented!(w, r"{{")?;
        w.indent();

        for variant in the_type.variants() {
            self.write_type_definition_enum_variant(w, variant, the_type, write_for)?;
        }

        w.unindent();
        indented!(w, r"}}")
    }

    pub fn write_type_definition_enum_variant(&self, w: &mut IndentWriter, variant: &Variant, _the_type: &EnumType, write_for: WriteFor) -> Result<(), Error> {
        let variant_name = variant.name();
        let variant_value = variant.value();
        if write_for == WriteFor::Code {
            self.write_documentation(w, variant.documentation())?;
        }
        indented!(w, r"{} = {},", variant_name, variant_value)
    }

    pub fn write_type_definition_composite(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        self.debug(w, "write_type_definition_composite")?;
        self.write_documentation(w, the_type.meta().documentation())?;
        self.write_type_definition_composite_annotation(w, the_type)?;
        self.write_type_definition_composite_body(w, the_type, WriteFor::Code)?;
        self.write_type_definition_composite_marshaller(w, the_type)
    }

    fn write_type_definition_composite_marshaller(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        self.debug(w, "write_type_marshaller")?;

        if self.should_emit_marshaller_for_composite(the_type) {
            w.newline()?;
            indented!(
                w,
                r"[CustomMarshaller(typeof({}), MarshalMode.Default, typeof({}Marshaller))]",
                the_type.rust_name(),
                the_type.rust_name()
            )?;
            indented!(w, r"internal static class {}Marshaller", the_type.rust_name())?;
            indented!(w, r"{{")?;
            w.indent();
            self.write_type_definition_composite_layout_annotation(w, the_type)?;
            indented!(w, r"public unsafe struct Unmanaged")?;
            indented!(w, r"{{")?;
            w.indent();
            for field in the_type.fields() {
                self.write_type_definition_composite_unmanaged_body_field(w, field, the_type)?;
            }
            w.unindent();
            indented!(w, r"}}")?;
            w.unindent();
            w.newline()?;
            w.indent();
            indented!(w, r"public static Unmanaged ConvertToUnmanaged({} managed)", the_type.rust_name())?;
            indented!(w, r"{{")?;
            w.indent();
            indented!(w, r"var result = new Unmanaged")?;
            indented!(w, r"{{")?;
            w.indent();
            for field in the_type.fields().iter().filter(|t| !matches!(t.the_type(), CType::Array(_))) {
                self.write_type_definition_composite_to_unmanaged_inline_field(w, field)?;
            }
            w.unindent();
            indented!(w, r"}};")?;
            w.newline()?;
            indented!(w, r"unsafe")?;
            indented!(w, r"{{")?;
            w.indent();
            for (i, field) in the_type.fields().iter().filter(|t| matches!(t.the_type(), CType::Array(_))).enumerate() {
                if i > 0 {
                    w.newline()?;
                }
                if let CType::Array(a) = field.the_type() {
                    self.write_type_definition_composite_to_unmanaged_marshal_field(w, the_type, field, a)?;
                }
            }
            w.unindent();
            indented!(w, r"}}")?;
            w.newline()?;
            indented!(w, r"return result;")?;
            w.unindent();
            indented!(w, r"}}")?;
            w.newline()?;
            indented!(w, r"public static {0} ConvertToManaged(Unmanaged unmanaged)", the_type.rust_name())?;
            indented!(w, r"{{")?;
            w.indent();
            indented!(w, r"var result = new {0}()", the_type.rust_name())?;
            indented!(w, r"{{")?;
            w.indent();
            for field in the_type.fields().iter().filter(|t| !matches!(t.the_type(), CType::Array(_))) {
                self.write_type_definition_composite_to_managed_inline_field(w, field)?;
            }
            w.unindent();
            indented!(w, r"}};")?;
            w.newline()?;
            indented!(w, r"unsafe")?;
            indented!(w, r"{{")?;
            w.indent();
            for (i, field) in the_type.fields().iter().filter(|t| matches!(t.the_type(), CType::Array(_))).enumerate() {
                if i > 0 {
                    w.newline()?;
                }
                if let CType::Array(a) = field.the_type() {
                    self.write_type_definition_composite_to_managed_marshal_field(w, the_type, field, a)?;
                }
            }
            w.unindent();
            indented!(w, r"}}")?;
            w.newline()?;
            indented!(w, r"return result;")?;
            w.unindent();
            indented!(w, r"}}")?;
            w.unindent();
            indented!(w, r"}}")?;
            w.newline()?;
        }

        Ok(())
    }

    fn write_type_definition_composite_to_managed_marshal_field(
        &self,
        w: &mut IndentWriter,
        the_type: &CompositeType,
        field: &Field,
        a: &ArrayType,
    ) -> Result<(), Error> {
        let field_name = self.converter().field_name_to_csharp_name(field, self.config().rename_symbols);
        let type_name = self.converter().to_typespecifier_in_field(a.array_type(), field, the_type);
        if self.config().unroll_struct_arrays {
            for i in 0..a.len() {
                indented!(w, r"result.{0}{1} = unmanaged.{0}[{1}];", field_name, i)?;
            }
        } else if matches!(a.array_type(), CType::Pattern(TypePattern::CChar)) {
            indented!(w, r"var source = new ReadOnlySpan<byte>(unmanaged.{}, {});", field_name, a.len())?;
            indented!(w, r"var terminatorIndex = source.IndexOf<byte>(0);")?;
            indented!(
                w,
                r"result.{} = Encoding.UTF8.GetString(source.Slice(0, terminatorIndex == -1 ? Math.Min(source.Length, {}) : terminatorIndex));",
                field_name,
                a.len()
            )?;
        } else {
            indented!(w, r"var source = new Span<{}>(unmanaged.{}, {});", type_name, field_name, a.len())?;
            indented!(w, r"var arr_{} = new {}[{}];", field_name, type_name, a.len())?;
            indented!(w, r"source.CopyTo(arr_{}.AsSpan());", field_name)?;
            indented!(w, r"result.{0} = arr_{0};", field_name)?;
        }
        Ok(())
    }

    fn write_type_definition_composite_to_managed_inline_field(&self, w: &mut IndentWriter, field: &Field) -> Result<(), Error> {
        let field_name = self.converter().field_name_to_csharp_name(field, self.config().rename_symbols);
        match field.the_type() {
            CType::Primitive(PrimitiveType::Bool) => {
                indented!(w, r"{0} = Convert.ToBoolean(unmanaged.{0}),", field_name)?;
            }
            CType::Composite(composite) if self.should_emit_marshaller_for_composite(composite) => {
                indented!(w, r"{0} = {1}Marshaller.ConvertToManaged(unmanaged.{0}),", field_name, composite.rust_name())?;
            }
            _ => {
                indented!(w, r"{0} = unmanaged.{0},", field_name)?;
            }
        }
        Ok(())
    }

    fn write_type_definition_composite_to_unmanaged_marshal_field(
        &self,
        w: &mut IndentWriter,
        the_type: &CompositeType,
        field: &Field,
        a: &ArrayType,
    ) -> Result<(), Error> {
        let field_name = self.converter().field_name_to_csharp_name(field, self.config().rename_symbols);
        let type_name = self.converter().to_typespecifier_in_field(a.array_type(), field, the_type);
        if self.config().unroll_struct_arrays {
            for i in 0..a.len() {
                indented!(w, r"result.{0}[{1}] = managed.{0}{1};", field_name, i)?;
            }
        } else {
            indented!(w, r"if(managed.{} != null)", field_name)?;
            indented!(w, r"{{")?;
            w.indent();
            if matches!(a.array_type(), CType::Pattern(TypePattern::CChar)) {
                indented!(w, "fixed(char* s = managed.{})", field_name)?;
                indented!(w, "{{")?;
                w.indent();
                indented!(
                    w,
                    r"if(Encoding.UTF8.GetByteCount(managed.{0}, 0, managed.{0}.Length) + 1 > {1})",
                    field_name,
                    a.len()
                )?;
                indented!(w, r"{{")?;
                w.indent();
                indented!(
                    w,
                    r#"throw new InvalidOperationException($"The managed string field '{{nameof({0}.{1})}}' cannot be encoded to fit the fixed size array of {2}.");"#,
                    the_type.rust_name(),
                    field_name,
                    a.len()
                )?;
                w.unindent();
                indented!(w, r"}}")?;
                indented!(
                    w,
                    r"var written = Encoding.UTF8.GetBytes(s, managed.{0}.Length, result.{0}, {1});",
                    field_name,
                    a.len() - 1
                )?;
                indented!(w, r"result.{}[written] = 0;", field_name)?;
                w.unindent();
                indented!(w, r"}}")?;
            } else {
                indented!(w, r"if(managed.{}.Length > {})", field_name, a.len())?;
                indented!(w, r"{{")?;
                w.indent();
                indented!(
                    w,
                    r#"throw new InvalidOperationException($"The managed array field '{{nameof({0}.{1})}}' has {{managed.{1}.Length}} elements, exceeding the fixed size array of {2}.");"#,
                    the_type.rust_name(),
                    field_name,
                    a.len()
                )?;
                w.unindent();
                indented!(w, r"}}")?;
                indented!(w, r"var source = new ReadOnlySpan<{0}>(managed.{1}, 0, managed.{1}.Length);", type_name, field_name)?;
                indented!(w, r"var dest = new Span<{0}>(result.{1}, {2});", type_name, field_name, a.len())?;
                indented!(w, r"source.CopyTo(dest);")?;
            }
            w.unindent();
            indented!(w, r"}}")?;
        }
        Ok(())
    }

    fn write_type_definition_composite_to_unmanaged_inline_field(&self, w: &mut IndentWriter, field: &Field) -> Result<(), Error> {
        let field_name = self.converter().field_name_to_csharp_name(field, self.config().rename_symbols);
        match field.the_type() {
            CType::Primitive(PrimitiveType::Bool) => {
                indented!(w, r"{0} = Convert.ToSByte(managed.{0}),", field_name)?;
            }
            CType::Composite(composite) if self.should_emit_marshaller_for_composite(composite) => {
                indented!(w, r"{0} = {1}Marshaller.ConvertToUnmanaged(managed.{0}),", field_name, composite.rust_name())?;
            }
            _ => {
                indented!(w, r"{0} = managed.{0},", field_name)?;
            }
        }
        Ok(())
    }

    fn write_type_definition_composite_unmanaged_body_field(&self, w: &mut IndentWriter, field: &Field, the_type: &CompositeType) -> Result<(), Error> {
        let field_name = self.converter().field_name_to_csharp_name(field, self.config().rename_symbols);
        match field.the_type() {
            CType::Array(a) => {
                let type_name = self.converter().to_typespecifier_in_field(a.array_type(), field, the_type);
                let size = a.len();
                if matches!(a.array_type(), CType::Pattern(TypePattern::CChar)) {
                    indented!(w, r"public fixed byte {}[{}];", field_name, size)?;
                } else {
                    indented!(w, r"public fixed {} {}[{}];", type_name, field_name, size)?;
                }
            }
            CType::Primitive(PrimitiveType::Bool) => {
                indented!(w, r"public sbyte {};", field_name)?;
            }
            CType::Composite(composite) => {
                if self.should_emit_marshaller_for_composite(composite) {
                    indented!(w, r"public {}Marshaller.Unmanaged {};", composite.rust_name(), field_name)?;
                } else {
                    indented!(w, r"public {} {};", composite.rust_name(), field_name)?;
                }
            }
            _ => {
                let type_name = self.converter().to_typespecifier_in_field(field.the_type(), field, the_type);
                indented!(w, r"public {} {};", type_name, field_name)?;
            }
        }
        Ok(())
    }

    pub fn write_type_definition_composite_annotation(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        indented!(w, r"[Serializable]")?;

        if the_type.repr().alignment().is_some() {
            let comment = r"// THIS STRUCT IS BROKEN - C# does not support alignment of entire Rust types that do #[repr(align(...))]";
            match self.config.unsupported {
                Unsupported::Panic => panic!("{}", comment),
                Unsupported::Comment => indented!(w, "{}", comment)?,
            }
        }

        if self.should_emit_marshaller_for_composite(the_type) {
            indented!(w, r"[NativeMarshalling(typeof({}Marshaller))]", the_type.rust_name())?;
        } else {
            self.write_type_definition_composite_layout_annotation(w, the_type)?;
        }

        Ok(())
    }

    fn write_type_definition_composite_layout_annotation(&self, w: &mut IndentWriter, the_type: &CompositeType) -> Result<(), Error> {
        match the_type.repr().layout() {
            Layout::C | Layout::Transparent | Layout::Opaque => indented!(w, r"[StructLayout(LayoutKind.Sequential)]"),
            Layout::Packed => indented!(w, r"[StructLayout(LayoutKind.Sequential, Pack = 1)]"),
            Layout::Primitive(_) => panic!("Primitive layout not supported for structs."),
        }
    }

    pub fn write_type_definition_composite_body(&self, w: &mut IndentWriter, the_type: &CompositeType, write_for: WriteFor) -> Result<(), Error> {
        indented!(w, r"{} partial struct {}", self.config.visibility_types.to_access_modifier(), the_type.rust_name())?;
        indented!(w, r"{{")?;
        w.indent();

        for field in the_type.fields() {
            if write_for == WriteFor::Code {
                self.write_documentation(w, field.documentation())?;
            }

            self.write_type_definition_composite_body_field(w, field, the_type)?;
        }

        w.unindent();
        indented!(w, r"}}")
    }

    pub fn write_type_definition_composite_body_field(&self, w: &mut IndentWriter, field: &Field, the_type: &CompositeType) -> Result<(), Error> {
        let field_name = self.converter.field_name_to_csharp_name(field, self.config.rename_symbols);
        let visibility = match field.visibility() {
            Visibility::Public => "public ",
            Visibility::Private if self.should_emit_marshaller_for_composite(the_type) => "internal ",
            Visibility::Private => "",
        };

        match field.the_type() {
            CType::Array(a) => {
                if self.config().unroll_struct_arrays {
                    let type_name = self.converter().to_typespecifier_in_field(a.array_type(), field, the_type);
                    for i in 0..a.len() {
                        indented!(w, r"{}{} {}{};", visibility, type_name, field_name, i)?;
                    }
                } else {
                    assert!(self.converter().is_blittable(a.array_type()), "Array type is not blittable: {:?}", a.array_type());

                    let type_name = if matches!(a.array_type(), CType::Pattern(TypePattern::CChar)) {
                        "string".to_string()
                    } else {
                        format!("{}[]", self.converter().to_typespecifier_in_field(a.array_type(), field, the_type))
                    };

                    indented!(w, r"{}{} {};", visibility, type_name, field_name)?;
                }

                Ok(())
            }
            CType::Primitive(PrimitiveType::Bool) => {
                let type_name = self.converter().to_typespecifier_in_field(field.the_type(), field, the_type);
                if !self.should_emit_marshaller_for_composite(the_type) {
                    indented!(w, r"[MarshalAs(UnmanagedType.I1)]")?;
                }
                indented!(w, r"{}{} {};", visibility, type_name, field_name)
            }
            _ => {
                let type_name = self.converter.to_typespecifier_in_field(field.the_type(), field, the_type);
                indented!(w, r"{}{} {};", visibility, type_name, field_name)
            }
        }
    }

    #[must_use]
    pub fn namespace_for_id(&self, id: &str) -> String {
        self.config
            .namespace_mappings
            .get(id)
            .unwrap_or_else(|| panic!("Found a namespace not mapped '{id}'. You should specify this one in the config."))
            .to_string()
    }

    pub fn write_namespace_context(&self, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        self.debug(w, "write_namespace_context")?;
        indented!(w, r"namespace {}", self.namespace_for_id(&self.config.namespace_id))?;
        indented!(w, r"{{")?;
        w.indent();

        f(w)?;

        w.unindent();

        indented!(w, r"}}")
    }

    pub fn write_class_context(&self, class_name: &str, w: &mut IndentWriter, f: impl FnOnce(&mut IndentWriter) -> Result<(), Error>) -> Result<(), Error> {
        self.debug(w, "write_class_context")?;
        indented!(w, r"{} static partial class {}", self.config.visibility_types.to_access_modifier(), class_name)?;
        indented!(w, r"{{")?;
        w.indent();

        f(w)?;

        w.unindent();
        indented!(w, r"}}")
    }

    #[must_use]
    pub fn should_emit_delegate(&self) -> bool {
        match self.config.write_types {
            WriteTypes::Namespace => false,
            WriteTypes::NamespaceAndInteroptopusGlobal => self.config.namespace_id.is_empty(),
            WriteTypes::All => true,
        }
    }

    fn should_emit_marshaller_for_composite(&self, composite: &CompositeType) -> bool {
        composite
            .fields()
            .iter()
            .any(|f| matches!(f.the_type(), CType::Composite(_)) || self.should_emit_marshaller(f.the_type()))
    }

    fn should_emit_marshaller(&self, ctype: &CType) -> bool {
        match ctype {
            CType::Array(_) => !self.config().unroll_struct_arrays,
            CType::Composite(x) => self.should_emit_marshaller_for_composite(x),
            _ => false,
        }
    }

    fn has_emittable_marshallers(&self, types: &[CType]) -> bool {
        types.iter().any(|x| self.should_emit_marshaller(x))
    }

    fn has_emittable_functions(&self, functions: &[Function]) -> bool {
        functions.iter().any(|x| self.should_emit_by_meta(x.meta()))
    }

    #[must_use]
    pub fn has_emittable_constants(&self, constants: &[Constant]) -> bool {
        constants.iter().any(|x| self.should_emit_by_meta(x.meta()))
    }

    pub fn has_ffi_error(&self, functions: &[Function]) -> bool {
        functions.iter().any(interoptopus::lang::c::Function::returns_ffi_error)
    }

    #[must_use]
    pub fn should_emit_by_meta(&self, meta: &Meta) -> bool {
        let rval = meta.namespace() == self.config.namespace_id;
        rval
    }

    /// Checks whether for the given type and the current file a type definition should be emitted.
    #[must_use]
    pub fn should_emit_by_type(&self, t: &CType) -> bool {
        if self.config.write_types == WriteTypes::All {
            return true;
        }

        if is_global_type(t) {
            return self.config.write_types == WriteTypes::NamespaceAndInteroptopusGlobal;
        }

        match t {
            CType::Primitive(_) => self.config.write_types == WriteTypes::NamespaceAndInteroptopusGlobal,
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
                TypePattern::Bool => self.config.write_types == WriteTypes::NamespaceAndInteroptopusGlobal,
                TypePattern::CChar => false,
                TypePattern::NamedCallback(x) => self.should_emit_by_meta(x.meta()),
                _ => panic!("Pattern not explicitly handled"),
            },
        }
    }

    pub fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.inventory.patterns() {
            match pattern {
                LibraryPattern::Service(cls) => {
                    if self.should_emit_by_meta(cls.the_type().meta()) {
                        self.write_pattern_service(w, cls)?;
                    }
                }
                _ => panic!("Pattern not explicitly handled"),
            }
        }

        Ok(())
    }

    pub fn write_pattern_option(&self, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
        self.debug(w, "write_pattern_option")?;

        let context_type_name = slice.rust_name();
        let data_type = slice
            .fields()
            .iter()
            .find(|x| x.name().eq("t"))
            .expect("Option must contain field called 't'.")
            .the_type();

        let type_string = self.converter.to_typespecifier_in_rval(data_type);
        let is_some = if self.config.rename_symbols { "isSome" } else { "is_some" };

        indented!(w, r"{} partial struct {}", self.config.visibility_types.to_access_modifier(), context_type_name)?;
        indented!(w, r"{{")?;

        // FromNullable
        indented!(w, [()], r"public static {} FromNullable({}? nullable)", context_type_name, type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"var result = new {}();", context_type_name)?;
        indented!(w, [()()], r"if (nullable.HasValue)")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"result.{} = 1;", is_some)?;
        indented!(w, [()()()], r"result.t = nullable.Value;")?;
        indented!(w, [()()], r"}}")?;
        w.newline()?;
        indented!(w, [()()], r"return result;")?;
        indented!(w, [()], r"}}")?;
        w.newline()?;

        // ToNullable
        indented!(w, [()], r"public {}? ToNullable()", type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"return this.{} == 1 ? this.t : ({}?)null;", is_some, type_string)?;
        indented!(w, [()], r"}}")?;

        indented!(w, r"}}")?;
        w.newline()?;
        Ok(())
    }

    pub fn write_pattern_slice(&self, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
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

        let type_string = self.converter.to_typespecifier_in_rval(data_type);
        let is_blittable = self.converter.is_blittable(data_type);

        indented!(
            w,
            r"{} partial struct {} : IEnumerable<{}>",
            self.config.visibility_types.to_access_modifier(),
            context_type_name,
            type_string
        )?;
        indented!(w, r"{{")?;

        // Ctor
        indented!(w, [()], r"public {}(GCHandle handle, ulong count)", context_type_name)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"this.data = handle.AddrOfPinnedObject();")?;
        indented!(w, [()()], r"this.len = count;")?;
        indented!(w, [()], r"}}")?;

        // Ctor
        indented!(w, [()], r"public {}(IntPtr handle, ulong count)", context_type_name)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"this.data = handle;")?;
        indented!(w, [()()], r"this.len = count;")?;
        indented!(w, [()], r"}}")?;

        self.write_pattern_slice_overload(w, context_type_name, &type_string)?;

        // Getter
        indented!(w, [()], r"public unsafe {} this[int i]", type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"get")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;

        if is_blittable {
            indented!(w, [()()()], r"var d = ({}*) data.ToPointer();", type_string)?;
            indented!(w, [()()()], r"return d[i];")?;
        } else {
            indented!(w, [()()()], r"var size = Marshal.SizeOf(typeof({}));", type_string)?;
            indented!(w, [()()()], r"var ptr = new IntPtr(data.ToInt64() + i * size);")?;
            indented!(w, [()()()], r"return Marshal.PtrToStructure<{}>(ptr);", type_string)?;
        }

        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;

        // Copied
        indented!(w, [()], r"public unsafe {}[] Copied", type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"get")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"var rval = new {}[len];", type_string)?;

        if is_blittable {
            indented!(w, [()()()], r"fixed (void* dst = rval)")?;
            indented!(w, [()()()], r"{{")?;
            indented!(
                w,
                [()()()()],
                r"Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof({}));",
                type_string
            )?;
            indented!(w, [()()()()], r"for (var i = 0; i < (int) len; i++) {{")?;
            indented!(w, [()()()()()], r"rval[i] = this[i];")?;
            indented!(w, [()()()()], r"}}")?;
            indented!(w, [()()()], r"}}")?;
        } else {
            indented!(w, [()()()], r"for (var i = 0; i < (int) len; i++) {{")?;
            indented!(w, [()()()()], r"rval[i] = this[i];")?;
            indented!(w, [()()()], r"}}")?;
        }
        indented!(w, [()()()], r"return rval;")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;

        // Count
        indented!(w, [()], r"public int Count => (int) len;")?;

        // GetEnumerator
        indented!(w, [()], r"public IEnumerator<{}> GetEnumerator()", type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"for (var i = 0; i < (int)len; ++i)")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"yield return this[i];")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;

        // The other GetEnumerator
        indented!(w, [()], r"IEnumerator IEnumerable.GetEnumerator()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"return this.GetEnumerator();")?;
        indented!(w, [()], r"}}")?;

        indented!(w, r"}}")?;
        w.newline()?;

        Ok(())
    }

    pub fn write_pattern_slice_overload(&self, w: &mut IndentWriter, _context_type_name: &str, type_string: &str) -> Result<(), Error> {
        indented!(w, [()], r"public unsafe ReadOnlySpan<{}> ReadOnlySpan", type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"get")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"unsafe")?;
        indented!(w, [()()()], r"{{")?;
        indented!(w, [()()()()], r"return new ReadOnlySpan<{}>(this.data.ToPointer(), (int) this.len);", type_string)?;
        indented!(w, [()()()], r"}}")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;
        Ok(())
    }

    pub fn write_pattern_slice_mut_overload(&self, w: &mut IndentWriter, _context_type_name: &str, type_string: &str) -> Result<(), Error> {
        indented!(w, [()], r"public unsafe Span<{}> Span", type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"get")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"unsafe")?;
        indented!(w, [()()()], r"{{")?;
        indented!(w, [()()()()], r"return new Span<{}>(this.data.ToPointer(), (int) this.len);", type_string)?;
        indented!(w, [()()()], r"}}")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;
        Ok(())
    }

    pub fn write_pattern_slice_mut(&self, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
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

        let type_string = self.converter.to_typespecifier_in_rval(data_type);

        indented!(
            w,
            r"{} partial struct {} : IEnumerable<{}>",
            self.config.visibility_types.to_access_modifier(),
            context_type_name,
            type_string
        )?;
        indented!(w, r"{{")?;

        // Ctor
        indented!(w, [()], r"public {}(GCHandle handle, ulong count)", context_type_name)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"this.data = handle.AddrOfPinnedObject();")?;
        indented!(w, [()()], r"this.len = count;")?;
        indented!(w, [()], r"}}")?;

        // Ctor
        indented!(w, [()], r"public {}(IntPtr handle, ulong count)", context_type_name)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"this.data = handle;")?;
        indented!(w, [()()], r"this.len = count;")?;
        indented!(w, [()], r"}}")?;

        self.write_pattern_slice_overload(w, context_type_name, &type_string)?;
        self.write_pattern_slice_mut_overload(w, context_type_name, &type_string)?;

        // Getter
        indented!(w, [()], r"public unsafe {} this[int i]", type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"get")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
        indented!(w, [()()()], r"var d = ({}*) data.ToPointer();", type_string)?;
        indented!(w, [()()()], r"return d[i];")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()()], r"set")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"if (i >= Count) throw new IndexOutOfRangeException();")?;
        indented!(w, [()()()], r"var d = ({}*) data.ToPointer();", type_string)?;
        indented!(w, [()()()], r"d[i] = value;")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;

        // Copied
        indented!(w, [()], r"public unsafe {}[] Copied", type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"get")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"var rval = new {}[len];", type_string)?;
        indented!(w, [()()()], r"fixed (void* dst = rval)")?;
        indented!(w, [()()()], r"{{")?;
        indented!(
            w,
            [()()()()],
            r"Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof({}));",
            type_string
        )?;
        indented!(w, [()()()()], r"for (var i = 0; i < (int) len; i++) {{")?;
        indented!(w, [()()()()()], r"rval[i] = this[i];")?;
        indented!(w, [()()()()], r"}}")?;
        indented!(w, [()()()], r"}}")?;
        indented!(w, [()()()], r"return rval;")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;

        // Count
        indented!(w, [()], r"public int Count => (int) len;")?;

        // GetEnumerator
        indented!(w, [()], r"public IEnumerator<{}> GetEnumerator()", type_string)?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"for (var i = 0; i < (int)len; ++i)")?;
        indented!(w, [()()], r"{{")?;
        indented!(w, [()()()], r"yield return this[i];")?;
        indented!(w, [()()], r"}}")?;
        indented!(w, [()], r"}}")?;

        // The other GetEnumerator
        indented!(w, [()], r"IEnumerator IEnumerable.GetEnumerator()")?;
        indented!(w, [()], r"{{")?;
        indented!(w, [()()], r"return this.GetEnumerator();")?;
        indented!(w, [()], r"}}")?;

        indented!(w, r"}}")?;
        w.newline()?;

        Ok(())
    }

    pub fn write_pattern_service(&self, w: &mut IndentWriter, class: &Service) -> Result<(), Error> {
        self.debug(w, "write_pattern_service")?;
        let mut all_functions = class.constructors().to_vec();
        all_functions.extend_from_slice(class.methods());
        all_functions.push(class.destructor().clone());

        let context_type_name = class.the_type().rust_name();
        let common_prefix = longest_common_prefix(&all_functions);

        self.write_documentation(w, class.the_type().meta().documentation())?;
        indented!(
            w,
            r"{} partial class {} : IDisposable",
            self.config.visibility_types.to_access_modifier(),
            context_type_name
        )?;
        indented!(w, r"{{")?;
        w.indent();
        indented!(w, r"private IntPtr _context;")?;
        w.newline()?;
        indented!(w, r"private {}() {{}}", context_type_name)?;
        w.newline()?;

        for ctor in class.constructors() {
            // Ctor
            let fn_name = self
                .converter
                .function_name_to_csharp_name(ctor, FunctionNameFlavor::CSharpMethodNameWithoutClass(&common_prefix));
            let rval = format!("static {context_type_name}");

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
                .converter
                .function_name_to_csharp_name(function, FunctionNameFlavor::CSharpMethodNameWithoutClass(&common_prefix));

            // Write checked method. These are "normal" methods that accept
            // common C# types.
            let rval = match function.signature().rval() {
                CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
                CType::Pattern(TypePattern::CStrPointer) => "string".to_string(),
                _ => self.converter.to_typespecifier_in_rval(function.signature().rval()),
            };
            self.write_documentation(w, function.meta().documentation())?;
            self.write_pattern_service_method(w, class, function, &rval, &fn_name, false, false, WriteFor::Code)?;
            self.write_service_method_overload(w, class, function, &fn_name, WriteFor::Code)?;

            w.newline()?;
        }

        indented!(w, r"public IntPtr Context => _context;")?;

        w.unindent();
        indented!(w, r"}}")?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub fn write_pattern_service_method(
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
            let native = self.converter.to_typespecifier_in_param(p.the_type());

            match p.the_type() {
                CType::Pattern(TypePattern::NamedCallback(callback)) => match callback.fnpointer().signature().rval() {
                    CType::Pattern(TypePattern::FFIErrorEnum(_)) if self.config.work_around_exception_in_callback_no_reentry => {
                        to_wrap_delegates.push(name);
                        to_wrap_delegate_types.push(self.converter.to_typespecifier_in_param(p.the_type()));
                        to_invoke.push(format!("{name}_safe_delegate.Call"));
                    }
                    _ => {
                        // Forward `ref` and `out` accordingly.
                        if native.contains("out ") {
                            to_invoke.push(format!("out {name}"));
                        } else if native.contains("ref ") {
                            to_invoke.push(format!("ref {name}"));
                        } else {
                            to_invoke.push(name.to_string());
                        }
                    }
                },

                _ => {
                    // Forward `ref` and `out` accordingly.
                    if native.contains("out ") {
                        to_invoke.push(format!("out {name}"));
                    } else if native.contains("ref ") {
                        to_invoke.push(format!("ref {name}"));
                    } else {
                        to_invoke.push(name.to_string());
                    }
                }
            }

            names.push(name);
            types.push(native);
        }

        let method_to_invoke = self.converter.function_name_to_csharp_name(
            function,
            if self.config.rename_symbols {
                FunctionNameFlavor::CSharpMethodNameWithClass
            } else {
                FunctionNameFlavor::RawFFIName
            },
        );
        let extra_args = if to_invoke.is_empty() {
            String::new()
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
        let arg_tokens = names.iter().zip(types.iter()).map(|(n, t)| format!("{t} {n}")).collect::<Vec<_>>();
        let fn_call = format!(r"{}.{}({}{})", self.config.class, method_to_invoke, context, extra_args);

        // Write signature.
        let signature = format!(r"public {} {}({})", rval, fn_name, arg_tokens.join(", "));
        if write_for == WriteFor::Docs {
            indented!(w, r"{};", signature)?;
            return Ok(());
        }

        indented!(w, "{}", signature)?;
        indented!(w, r"{{")?;

        if is_ctor {
            indented!(w, [()], r"var self = new {}();", class.the_type().rust_name())?;
        }

        for (name, ty) in zip(&to_wrap_delegates, &to_wrap_delegate_types) {
            indented!(w, [()], r"var {}_safe_delegate = new {}ExceptionSafe({});", name, ty, name)?;
        }

        // Determine return value behavior and write function call.
        match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
                indented!(w, [()], r"var rval = {};", fn_call)?;
                for name in to_wrap_delegates {
                    indented!(w, [()], r"{}_safe_delegate.Rethrow();", name)?;
                }
                indented!(w, [()], r"if (rval != {}.{})", e.the_enum().rust_name(), e.success_variant().name())?;
                indented!(w, [()], r"{{")?;
                indented!(w, [()()], r"throw new InteropException<{}>(rval);", e.the_enum().rust_name())?;
                indented!(w, [()], r"}}")?;
            }
            CType::Pattern(TypePattern::CStrPointer) => {
                indented!(w, [()], r"var s = {};", fn_call)?;
                indented!(w, [()], r"return Marshal.PtrToStringAnsi(s);")?;
            }
            CType::Primitive(PrimitiveType::Void) => {
                indented!(w, [()], r"{};", fn_call)?;
            }
            _ => {
                indented!(w, [()], r"return {};", fn_call)?;
            }
        }

        if is_ctor {
            indented!(w, [()], r"return self;")?;
        }

        indented!(w, r"}}")?;

        Ok(())
    }

    /// Writes common service overload code
    pub fn write_common_service_method_overload<FPatternMap: Fn(&Parameter) -> String>(
        &self,
        w: &mut IndentWriter,
        function: &Function,
        fn_pretty: &str,
        f_pattern: FPatternMap,
        write_for: WriteFor,
    ) -> Result<(), Error> {
        let mut names = Vec::new();
        let mut to_invoke = Vec::new();
        let mut types = Vec::new();

        // Write checked method. These are "normal" methods that accept
        // common C# types.
        let rval = match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
            CType::Pattern(TypePattern::CStrPointer) => "string".to_string(),
            _ => self.converter.to_typespecifier_in_rval(function.signature().rval()),
        };

        // For every parameter except the first, figure out how we should forward
        // it to the invocation we perform.
        for p in function.signature().params().iter().skip(1) {
            let name = p.name();

            // If we call the checked function we want to resolve a `SliceU8` to a `byte[]`,
            // but if we call the unchecked version we want to keep that `Sliceu8` in our signature.
            // let native = self.to_typespecifier_in_param(p.the_type());
            let native = f_pattern(p);

            // Forward `ref` and `out` accordingly.
            if native.contains("out ") {
                to_invoke.push(format!("out {name}"));
            } else if native.contains("ref ") {
                to_invoke.push(format!("ref {name}"));
            } else {
                to_invoke.push(name.to_string());
            }

            names.push(name);
            types.push(native);
        }

        let method_to_invoke = self.converter.function_name_to_csharp_name(
            function,
            if self.config.rename_symbols {
                FunctionNameFlavor::CSharpMethodNameWithClass
            } else {
                FunctionNameFlavor::RawFFIName
            },
        );
        let extra_args = if to_invoke.is_empty() {
            String::new()
        } else {
            format!(", {}", to_invoke.join(", "))
        };

        // Assemble actual function call.
        let context = "_context";
        let arg_tokens = names.iter().zip(types.iter()).map(|(n, t)| format!("{t} {n}")).collect::<Vec<_>>();
        let fn_call = format!(r"{}.{}({}{})", self.config.class, method_to_invoke, context, extra_args);

        let signature = format!(r"public {} {}({})", rval, fn_pretty, arg_tokens.join(", "));
        if write_for == WriteFor::Docs {
            indented!(w, "{};", signature)?;
            return Ok(());
        }

        // Write signature.
        indented!(w, "{}", signature)?;
        indented!(w, r"{{")?;

        match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => {
                indented!(w, [()], r"{};", fn_call)?;
            }
            CType::Primitive(PrimitiveType::Void) => {
                indented!(w, [()], r"{};", fn_call)?;
            }
            _ => {
                indented!(w, [()], r"return {};", fn_call)?;
            }
        }

        indented!(w, r"}}")?;

        Ok(())
    }

    #[must_use]
    pub fn pattern_to_native_in_signature(&self, param: &Parameter) -> String {
        let slice_type_name = |mutable: bool, element_type: &CType| -> String {
            if mutable {
                format!("System.Span<{}>", self.converter.to_typespecifier_in_param(element_type))
            } else {
                format!("System.ReadOnlySpan<{}>", self.converter.to_typespecifier_in_param(element_type))
            }
        };
        match param.the_type() {
            CType::Pattern(p) => match p {
                TypePattern::Slice(p) => {
                    let element_type = p.try_deref_pointer().expect("Must be pointer");
                    slice_type_name(false, &element_type)
                }
                TypePattern::SliceMut(p) => {
                    let element_type = p.try_deref_pointer().expect("Must be pointer");
                    slice_type_name(true, &element_type)
                }
                _ => self.converter.to_typespecifier_in_param(param.the_type()),
            },
            CType::ReadPointer(x) | CType::ReadWritePointer(x) => match &**x {
                CType::Pattern(x) => match x {
                    TypePattern::Slice(p) => {
                        let element_type = p.try_deref_pointer().expect("Must be pointer");
                        slice_type_name(false, &element_type)
                    }
                    TypePattern::SliceMut(p) => {
                        let element_type = p.try_deref_pointer().expect("Must be pointer");
                        slice_type_name(true, &element_type)
                    }
                    _ => self.converter.to_typespecifier_in_param(param.the_type()),
                },
                _ => self.converter.to_typespecifier_in_param(param.the_type()),
            },

            x => self.converter.to_typespecifier_in_param(x),
        }
    }

    pub fn write_service_method_overload(&self, w: &mut IndentWriter, _class: &Service, function: &Function, fn_pretty: &str, write_for: WriteFor) -> Result<(), Error> {
        if !self.converter.has_overloadable(function.signature()) {
            return Ok(());
        }

        if write_for == WriteFor::Code {
            w.newline()?;
            self.write_documentation(w, function.meta().documentation())?;
        }

        self.write_common_service_method_overload(w, function, fn_pretty, |p| self.pattern_to_native_in_signature(p), write_for)?;

        Ok(())
    }

    pub fn write_builtins(&self, w: &mut IndentWriter) -> Result<(), Error> {
        if self.config.write_types.write_interoptopus_globals() && self.has_ffi_error(self.inventory.functions()) {
            let error_text = &self.config.error_text;

            indented!(w, r"public class InteropException<T> : Exception")?;
            indented!(w, r"{{")?;
            indented!(w, [()], r"public T Error {{ get; private set; }}")?;
            w.newline()?;
            indented!(w, [()], r#"public InteropException(T error): base($"{error_text}")"#)?;
            indented!(w, [()], r"{{")?;
            indented!(w, [()()], r"Error = error;")?;
            indented!(w, [()], r"}}")?;
            indented!(w, r"}}")?;
            w.newline()?;
        }

        Ok(())
    }

    pub fn write_all(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_file_header_comments(w)?;
        w.newline()?;

        self.write_imports(w)?;
        w.newline()?;

        self.write_namespace_context(w, |w| {
            if self.config.class_constants.is_none() || self.config.class_constants == Some(self.config.clone().class) {
                if self.has_emittable_functions(self.inventory.functions()) || self.has_emittable_constants(self.inventory.constants()) {
                    self.write_class_context(&self.config.class, w, |w| {
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
                if self.has_emittable_constants(self.inventory.constants()) {
                    self.write_class_context(self.config.class_constants.as_ref().unwrap(), w, |w| {
                        self.write_constants(w)?;
                        w.newline()?;

                        Ok(())
                    })?;
                }

                if self.has_emittable_functions(self.inventory.functions()) {
                    w.newline()?;
                    self.write_class_context(&self.config.class, w, |w| {
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

    #[must_use]
    pub fn converter(&self) -> &Converter {
        &self.converter
    }

    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Generate for Generator {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_all(w)
    }
}
