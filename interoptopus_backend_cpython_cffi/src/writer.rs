use crate::config::Config;
use crate::converter::{Converter, TypeConverter};
use interoptopus::lang::c::{CType, CompositeType, EnumType, Function};
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, safe_name};
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error, Interop, Library};

/// Contains all Python generators, create sub-trait to customize.
pub trait PythonWriter {
    /// Returns the user config.
    fn config(&self) -> &Config;

    /// Returns the library to produce bindings for.
    fn library(&self) -> &Library;

    /// Returns the library to produce bindings for.
    fn converter(&self) -> &Converter;

    /// Returns the C-generator
    fn c_generator(&self) -> &interoptopus_backend_c::Generator;

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"from cffi import FFI"#)?;
        Ok(())
    }

    fn write_api_load_fuction(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"{} = FFI()"#, self.config().ffi_attribute)?;
        indented!(w, r#"{}.cdef(api_definition)"#, self.config().ffi_attribute)?;
        indented!(w, r#"_api = None"#)?;
        w.newline()?;
        w.newline()?;

        indented!(w, r#"def {}(dll):"#, self.config().init_api_function_name)?;
        indented!(w, [_], r#""""Initializes this library, call with path to DLL.""""#)?;
        indented!(w, [_], r#"global _api"#)?;
        indented!(w, [_], r#"_api = {}.dlopen(dll)"#, self.config().ffi_attribute)?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for constant in self.library().constants() {
            let docs = constant
                .meta()
                .documentation()
                .lines()
                .iter()
                .map(|x| format!("# {}", x))
                .collect::<Vec<_>>()
                .join("\n");
            indented!(w, r#"{}"#, docs)?;
            indented!(w, r#"{} = {}"#, constant.name(), self.converter().constant_value_to_value(constant.value()))?;
        }

        Ok(())
    }

    fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for the_type in self.library().ctypes() {
            match the_type {
                CType::Enum(e) => self.write_enum(w, e)?,
                CType::Composite(c) => self.write_struct(w, c)?,
                CType::Pattern(TypePattern::SuccessEnum(e)) => self.write_enum(w, e.the_enum())?,
                _ => {}
            }
        }

        Ok(())
    }

    fn write_struct(&self, w: &mut IndentWriter, e: &CompositeType) -> Result<(), Error> {
        indented!(w, r#"class {}(object):"#, e.rust_name())?;
        indented!(w, [_], r#"{}"#, self.converter().documentation(e.meta().documentation()))?;

        // Ctor
        indented!(w, [_], r#"def __init__(self):"#)?;
        indented!(w, [_ _], r#"global _api, ffi"#)?;
        indented!(w, [_ _], r#"self._ctx = ffi.new("{}[]", 1)"#, e.rust_name())?;

        w.newline()?;

        // Array constructor
        indented!(w, [_], r#"def array(n):"#)?;
        indented!(w, [_ _], r#"global _api, ffi"#)?;
        indented!(w, [_ _], r#"return ffi.new("{}[]", n)"#, e.rust_name())?;
        w.newline()?;

        for field in e.fields() {
            indented!(w, [_], r#"@property"#)?;
            indented!(w, [_], r#"def {}(self):"#, field.name())?;
            indented!(w, [_ _], r#"{}"#, self.converter().documentation(field.documentation()))?;
            indented!(w, [_ _], r#"return self._ctx[0].{}"#, field.name())?;
            w.newline()?;

            let _docs = field.documentation().lines().join("\n");
            indented!(w, [_], r#"@{}.setter"#, field.name())?;
            indented!(w, [_], r#"def {}(self, value):"#, field.name())?;
            // We also write _ptr to hold on to any allocated object created by CFFI. If we do not
            // then we might assign a pointer in the _ctx line below, but once the parameter (the CFFI handle)
            // leaves this function the handle might become deallocated and therefore the pointer
            // becomes invalid
            indented!(w, [_ _], r#"self._ptr_{} = value"#, field.name())?;
            indented!(w, [_ _], r#"self._ctx[0].{} = value"#, field.name())?;
            w.newline()?;
        }

        Ok(())
    }

    fn write_enum(&self, w: &mut IndentWriter, e: &EnumType) -> Result<(), Error> {
        indented!(w, r#"class {}:"#, e.rust_name())?;
        indented!(w, [_], r#"{}"#, self.converter().documentation(e.meta().documentation()))?;
        for v in e.variants() {
            indented!(w, [_], r#"{} = {}"#, v.name(), v.value())?;
        }

        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_callback_helpers(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"class {}:"#, self.config().callback_namespace)?;
        indented!(w, [_], r#""""Helpers to define `@ffi.callback`-style callbacks.""""#)?;

        for callback in self.library().ctypes().iter().filter_map(|x| match x {
            CType::FnPointer(x) => Some(x),
            _ => None,
        }) {
            indented!(
                w,
                [_],
                r#"{} = "{}""#,
                safe_name(&callback.internal_name()),
                self.converter().type_fnpointer_to_typename(callback)
            )?;
        }

        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_function_proxies(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"class {}:"#, self.config().raw_fn_namespace)?;
        indented!(w, [_], r#""""Raw access to all exported functions.""""#)?;

        for function in self.library().functions() {
            let args = function.signature().params().iter().map(|x| x.name().to_string()).collect::<Vec<_>>().join(", ");

            indented!(w, [_], r#"def {}({}):"#, function.name(), &args)?;
            indented!(w, [_ _], r#"{}"#, self.converter().documentation(function.meta().documentation()))?;
            indented!(w, [_ _], r#"global _api"#)?;

            // Determine if the function was called with a wrapper we produced which as a private `_ctx`.
            // If so, use that instead. Otherwise, just pass parameters and hope for the best.
            for param in function.signature().params() {
                indented!(w, [_ _], r#"if hasattr({}, "_ctx"):"#, param.name())?;
                indented!(w, [_ _ _], r#"{} = {}._ctx[0]"#, param.name(), param.name())?;
            }

            indented!(w, [_ _], r#"return _api.{}({})"#, function.name(), &args)?;

            w.newline()?;
        }

        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_patterns(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for pattern in self.library().patterns() {
            match pattern {
                LibraryPattern::Class(cls) => self.write_pattern_class(w, cls)?,
            }
        }

        Ok(())
    }

    fn pattern_class_args_without_first_to_string(&self, function: &Function) -> String {
        function
            .signature()
            .params()
            .iter()
            .skip(1)
            .map(|x| x.name().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn write_pattern_class_success_enum_aware_rval(&self, w: &mut IndentWriter, _class: &Service, function: &Function, deref_ctx: bool) -> Result<(), Error> {
        let args = self.pattern_class_args_without_first_to_string(function);

        let ctx = if deref_ctx { "self.ctx[0]" } else { "self.ctx" };

        match function.signature().rval() {
            CType::Pattern(TypePattern::SuccessEnum(e)) => {
                indented!(w, [_ _], r#"rval = {}.{}({}, {})"#, self.config().raw_fn_namespace, function.name(), ctx, &args)?;
                indented!(w, [_ _], r#"if rval == {}.{}:"#, e.the_enum().rust_name(), e.success_variant().name())?;
                indented!(w, [_ _ _], r#"return None"#)?;
                indented!(w, [_ _], r#"else:"#)?;
                indented!(w, [_ _ _], r#"raise Exception(f"return value ${{rval}}")"#)?;
            }
            _ => {
                indented!(w, [_ _], r#"return _api.{}(self.ctx[0], {})"#, function.name(), &args)?;
            }
        }

        Ok(())
    }

    fn write_pattern_class(&self, w: &mut IndentWriter, class: &Service) -> Result<(), Error> {
        let context_type_name = class.the_type().rust_name();

        let mut all_functions = vec![class.constructor().clone(), class.destructor().clone()];
        all_functions.extend_from_slice(class.methods());
        let common_prefix = longest_common_prefix(&all_functions);

        let ctor_args = self.pattern_class_args_without_first_to_string(class.constructor());
        indented!(w, r#"class {}(object):"#, context_type_name)?;
        indented!(w, [_], r#"def __init__(self, {}):"#, ctor_args)?;
        indented!(w, [_ _], r#"{}"#, self.converter().documentation(class.constructor().meta().documentation()))?;
        indented!(w, [_ _], r#"global _api, ffi"#)?;
        for param in class.constructor().signature().params().iter().skip(1) {
            indented!(w, [_ _ ], r#"if hasattr({}, "_ctx"):"#, param.name())?;
            indented!(w, [_ _ _], r#"{} = {}._ctx"#, param.name(), param.name())?;
        }

        indented!(w, [_ _], r#"self.ctx = ffi.new("{}**")"#, context_type_name)?;
        self.write_pattern_class_success_enum_aware_rval(w, class, class.constructor(), false)?;
        w.newline()?;

        // Dtor
        indented!(w, [_], r#"def __del__(self):"#)?;
        indented!(w, [_ _], r#"global _api, ffi"#)?;
        self.write_pattern_class_success_enum_aware_rval(w, class, class.destructor(), false)?;
        w.newline()?;

        for function in class.methods() {
            let args = self.pattern_class_args_without_first_to_string(function);

            indented!(w, [_], r#"def {}(self, {}):"#, function.name().replace(&common_prefix, ""), &args)?;
            indented!(w, [_ _], r#"{}"#, self.converter().documentation(function.meta().documentation()))?;
            indented!(w, [_ _], r#"global {}"#, self.config().raw_fn_namespace)?;
            self.write_pattern_class_success_enum_aware_rval(w, class, function, true)?;
            w.newline()?;
        }

        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_c_header(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"api_definition = """"#)?;
        self.c_generator().write_to(w)?;
        indented!(w, r#"""""#)?;
        Ok(())
    }

    fn write_utils(&self, w: &mut IndentWriter) -> Result<(), Error> {
        indented!(w, r#"def ascii_string(x):"#)?;
        indented!(w, [_], r#""""Must be called with a b"my_string".""""#)?;
        indented!(w, [_], r#"global ffi"#)?;
        indented!(w, [_], r#"return ffi.new("char[]", x)"#)?;
        w.newline()?;
        Ok(())
    }

    fn write_all(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_imports(w)?;
        w.newline()?;

        self.write_c_header(w)?;
        w.newline()?;
        w.newline()?;

        self.write_api_load_fuction(w)?;
        w.newline()?;
        w.newline()?;

        self.write_constants(w)?;
        w.newline()?;
        w.newline()?;

        self.write_types(w)?;
        w.newline()?;
        w.newline()?;

        self.write_callback_helpers(w)?;
        w.newline()?;
        w.newline()?;

        self.write_function_proxies(w)?;
        w.newline()?;
        w.newline()?;

        self.write_patterns(w)?;
        w.newline()?;
        w.newline()?;

        self.write_utils(w)?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }
}
