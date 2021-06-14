use interoptopus::writer::IndentWriter;
use interoptopus::{Error, Library};
use interoptopus::lang::c::{ConstantValue, PrimitiveValue, CType, EnumType};

#[derive(Clone, Debug)]
pub struct Config {
    init_api_function_name: String,
    ffi_attribute: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { init_api_function_name: "init_api".to_string(), ffi_attribute: "ffi".to_string() }
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

pub trait Interop {
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

        self.write_function_proxies(w)?;
        w.newline()?;
        w.newline()?;

        Ok(())
    }

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"from cffi import FFI"#))?;
        Ok(())
    }

    fn write_api_load_fuction(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"{} = FFI()"#, self.config().ffi_attribute))?;
        w.indented(|w| writeln!(w, r#"{}.cdef(api_definition)"#, self.config().ffi_attribute))?;
        w.indented(|w| writeln!(w, r#"__api = None"#))?;
        w.newline()?;
        w.newline()?;

        w.indented(|w| writeln!(w, r#"def {}(dll):"#, self.config().init_api_function_name))?;
        w.indent();
        w.indented(|w| writeln!(w, r#""""Initializes this library, call with path to DLL.""""#))?;
        w.indented(|w| writeln!(w, r#"global __api"#))?;
        w.indented(|w| writeln!(w, r#"__api = {}.dlopen(dll)"#, self.config().ffi_attribute))?;
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
        for the_type in self.library().types() {
            match the_type {
                CType::Enum(e) => self.write_enum(w, e)?,
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



    fn write_function_proxies(&self, w: &mut IndentWriter) -> Result<(), Error> {
        for function in self.library().functions() {
            let args = function.signature().params().iter().map(|x| x.name().to_string()).collect::<Vec<_>>().join(", ");
            let docs = function.documentation().lines().join("\n");

            w.indented(|w| writeln!(w, r#"def {}({}):"#, function.name(), &args))?;
            w.indent();
            w.indented(|w| writeln!(w, r#""""{}""""#, docs))?;
            w.indented(|w| writeln!(w, r#"return __api.{}({})"#, function.name(), &args))?;
            w.unindent();

            w.newline()?;
            w.newline()?;
        }

        Ok(())
    }


    fn write_c_header(&self, w: &mut IndentWriter) -> Result<(), Error> {
        use interoptopus_backend_c::Interop as _;

        w.indented(|w| writeln!(w, r#"api_definition = """"#))?;
        self.c_generator().write_to(w)?;
        w.indented(|w| writeln!(w, r#"""""#))?;
        Ok(())
    }
}

impl Interop for Generator {
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
