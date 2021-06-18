use interoptopus::generators::Interop;
use interoptopus::lang::c::{CType, ConstantValue, EnumType, FnPointerType, Function, PrimitiveValue};
use interoptopus::patterns::class::Class;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, safe_name};
use interoptopus::writer::IndentWriter;
use interoptopus::{Error, Library};
use interoptopus_backend_c::InteropC;

#[derive(Clone, Debug)]
pub struct Config {
    init_api_function_name: String,
    ffi_attribute: String,
    raw_fn_namespace: String,
    callback_namespace: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            init_api_function_name: "init_api".to_string(),
            ffi_attribute: "ffi".to_string(),
            raw_fn_namespace: "raw".to_string(),
            callback_namespace: "callbacks".to_string(),
        }
    }
}

pub struct Generator {
    c_generator: interoptopus_backend_c::Generator,
    config: Config,
    library: Library,
}

impl Generator {
    pub fn new(config: Config, library: Library) -> Self {
        let c_generator = interoptopus_backend_c::Generator::new(
            interoptopus_backend_c::Config {
                directives: false,
                imports: false,
                file_header_comment: "".to_string(),
                ..interoptopus_backend_c::Config::default()
            },
            library.clone(),
        );

        Self { c_generator, config, library }
    }
}

pub trait InteropCPythonCFFI: Interop {
    /// Returns the user config.
    fn config(&self) -> &Config;

    /// Returns the library to produce bindings for.
    fn library(&self) -> &Library;

    /// Returns the C-generator
    fn c_generator(&self) -> &interoptopus_backend_c::Generator;

    fn constant_value_to_value(&self, value: &ConstantValue) -> String {
        match value {
            ConstantValue::Primitive(x) => match x {
                PrimitiveValue::Bool(x) => format!("{}", x),
                PrimitiveValue::U8(x) => format!("{}", x),
                PrimitiveValue::U16(x) => format!("{}", x),
                PrimitiveValue::U32(x) => format!("{}", x),
                PrimitiveValue::U64(x) => format!("{}", x),
                PrimitiveValue::I8(x) => format!("{}", x),
                PrimitiveValue::I16(x) => format!("{}", x),
                PrimitiveValue::I32(x) => format!("{}", x),
                PrimitiveValue::I64(x) => format!("{}", x),
                PrimitiveValue::F32(x) => format!("{}", x),
                PrimitiveValue::F64(x) => format!("{}", x),
            },
        }
    }

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"from cffi import FFI"#))?;
        Ok(())
    }

    fn write_api_load_fuction(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"{} = FFI()"#, self.config().ffi_attribute))?;
        w.indented(|w| writeln!(w, r#"{}.cdef(api_definition)"#, self.config().ffi_attribute))?;
        w.indented(|w| writeln!(w, r#"_api = None"#))?;
        w.newline()?;
        w.newline()?;

        w.indented(|w| writeln!(w, r#"def {}(dll):"#, self.config().init_api_function_name))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""Initializes this library, call with path to DLL.""""#))?;
        w.indented(|w| writeln!(w, r#"global _api"#))?;
        w.indented(|w| writeln!(w, r#"_api = {}.dlopen(dll)"#, self.config().ffi_attribute))?;
        w.unindent();
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_constants(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for constant in self.library().constants() {
            let docs = constant.documentation().lines().iter().map(|x| format!("# {}", x)).collect::<Vec<_>>().join("\n");
            w.indented(|w| writeln!(w, r#"{}"#, docs))?;
            w.indented(|w| writeln!(w, r#"{} = {}"#, constant.name(), self.constant_value_to_value(constant.value())))?;
        }

        Ok(())
    }

    fn write_types(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for the_type in self.library().ctypes() {
            match the_type {
                CType::Enum(e) => self.write_enum(w, e)?,
                CType::Pattern(TypePattern::SuccessEnum(e)) => self.write_enum(w, e.the_enum())?,
                _ => {}
            }
        }

        Ok(())
    }

    fn write_enum(&self, w: &mut IndentWriter, e: &EnumType) -> Result<(), Error> {
        let docs = e.documentation().lines().join("\n");

        w.indented(|w| writeln!(w, r#"class {}:"#, e.name()))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""{}""""#, docs))?;
        for v in e.variants() {
            w.indented(|w| writeln!(w, r#"{} = {}"#, v.name(), v.value()))?;
        }
        w.unindent();
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_callback_helpers(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"class {}:"#, self.config().callback_namespace))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""Helpers to define `@ffi.callback`-style callbacks.""""#))?;

        for callback in self.library().ctypes().iter().filter_map(|x| match x {
            CType::FnPointer(x) => Some(x),
            _ => None,
        }) {
            w.indented(|w| writeln!(w, r#"{} = "{}""#, safe_name(&callback.internal_name()), self.type_fnpointer_to_typename(callback)))?;
        }

        w.unindent();
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn type_fnpointer_to_typename(&self, fn_pointer: &FnPointerType) -> String {
        let rval = self.c_generator().type_to_type_specifier(&fn_pointer.signature().rval());
        let params = fn_pointer
            .signature()
            .params()
            .iter()
            .map(|x| self.c_generator().type_to_type_specifier(x.the_type()))
            .collect::<Vec<_>>()
            .join(",");

        format!("{}({})", rval, params)
    }

    fn write_function_proxies(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"class {}:"#, self.config().raw_fn_namespace))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""Raw access to all exported functions.""""#))?;

        for function in self.library().functions() {
            let args = function.signature().params().iter().map(|x| x.name().to_string()).collect::<Vec<_>>().join(", ");
            let docs = function.documentation().lines().join("\n");

            w.indented(|w| writeln!(w, r#"def {}({}):"#, function.name(), &args))?;
            w.indent();
            w.indented(|w| writeln!(w, r#""""{}""""#, docs))?;
            w.indented(|w| writeln!(w, r#"global _api"#))?;
            w.indented(|w| writeln!(w, r#"return _api.{}({})"#, function.name(), &args))?;
            w.unindent();

            w.newline()?;
        }

        w.unindent();
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

    fn write_pattern_class_success_enum_aware_rval(&self, w: &mut IndentWriter, _class: &Class, function: &Function, deref_ctx: bool) -> Result<(), Error> {
        let args = self.pattern_class_args_without_first_to_string(function);

        let ctx = if deref_ctx { "self.ctx[0]" } else { "self.ctx" };

        match function.signature().rval() {
            CType::Pattern(TypePattern::SuccessEnum(e)) => {
                w.indented(|w| writeln!(w, r#"rval = _api.{}({}, {})"#, function.name(), ctx, &args))?;
                w.indented(|w| writeln!(w, r#"if rval == {}.{}:"#, e.the_enum().name(), e.success_variant().name()))?;
                w.indent();
                w.indented(|w| writeln!(w, r#"return None"#))?;
                w.unindent();
                w.indented(|w| writeln!(w, r#"else:"#))?;
                w.indent();
                w.indented(|w| writeln!(w, r#"raise Exception(f"return value ${{rval}}")"#))?;
                w.unindent();
            }
            _ => {
                w.indented(|w| writeln!(w, r#"return _api.{}(self.ctx[0], {})"#, function.name(), &args))?;
            }
        }

        Ok(())
    }

    fn write_pattern_class(&self, w: &mut IndentWriter, class: &Class) -> Result<(), Error> {
        let context_type_name = class.the_type().name();

        let mut all_functions = vec![class.constructor().clone(), class.destructor().clone()];
        all_functions.extend_from_slice(class.methods());
        let common_prefix = longest_common_prefix(&all_functions);

        w.indented(|w| writeln!(w, r#"class {}(object):"#, context_type_name))?;
        w.indent();

        // Ctor
        let args = self.pattern_class_args_without_first_to_string(class.constructor());
        let docs = class.constructor().documentation().lines().join("\n");
        w.indented(|w| writeln!(w, r#"def __init__(self, {}):"#, args))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""{}""""#, docs))?;
        w.indented(|w| writeln!(w, r#"global _api, ffi"#))?;
        w.indented(|w| writeln!(w, r#"self.ctx = ffi.new("{}**")"#, context_type_name))?;
        self.write_pattern_class_success_enum_aware_rval(w, class, class.constructor(), false)?;
        w.unindent();
        w.newline()?;
        w.newline()?;

        // Dtor
        let docs = class.destructor().documentation().lines().join("\n");
        w.indented(|w| writeln!(w, r#"def __del__(self):"#))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""{}""""#, docs))?;
        w.indented(|w| writeln!(w, r#"global _api, ffi"#))?;
        self.write_pattern_class_success_enum_aware_rval(w, class, class.destructor(), false)?;
        w.unindent();
        w.newline()?;
        w.newline()?;

        for function in class.methods() {
            let args = self.pattern_class_args_without_first_to_string(function);
            let docs = function.documentation().lines().join("\n");

            w.indented(|w| writeln!(w, r#"def {}(self, {}):"#, function.name().replace(&common_prefix, ""), &args))?;
            w.indent();
            w.indented(|w| writeln!(w, r#""""{}""""#, docs))?;
            w.indented(|w| writeln!(w, r#"global _api"#))?;

            self.write_pattern_class_success_enum_aware_rval(w, class, function, true)?;

            w.unindent();
            w.newline()?;
        }

        w.unindent();
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_c_header(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"api_definition = """"#))?;
        self.c_generator().write_to(w)?;
        w.indented(|w| writeln!(w, r#"""""#))?;
        Ok(())
    }
}

impl Interop for Generator {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
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

        Ok(())
    }
}

impl InteropCPythonCFFI for Generator {
    fn config(&self) -> &Config {
        &self.config
    }

    fn library(&self) -> &Library {
        &self.library
    }

    fn c_generator(&self) -> &interoptopus_backend_c::Generator {
        &self.c_generator
    }
}
