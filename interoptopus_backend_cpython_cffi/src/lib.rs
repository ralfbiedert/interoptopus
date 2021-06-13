use interoptopus::writer::IndentWriter;
use interoptopus::{Error, Library};

#[derive(Debug)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
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

    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_imports(w)?;
        w.newline()?;

        self.write_c_header(w)?;
        w.newline()?;
        w.newline()?;

        self.write_api_load_fuction(w)?;

        Ok(())
    }

    fn write_imports(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"from cffi import FFI"#))?;
        Ok(())
    }

    fn write_api_load_fuction(&self, w: &mut IndentWriter) -> Result<(), Error> {
        w.indented(|w| writeln!(w, r#"def api_from_dll(dll):"#))?;
        w.indent();

        w.indented(|w| writeln!(w, r#"ffibuilder = FFI()"#))?;
        w.indented(|w| writeln!(w, r#"ffibuilder.cdef(api_definition)"#))?;
        w.indented(|w| writeln!(w, r#"return ffibuilder.dlopen(dll)"#))?;

        w.unindent();
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
