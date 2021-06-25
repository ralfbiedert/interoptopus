//! Generates CPython CFFI bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! ## Usage
//!
//! In your library or a support project add this:
//!
//! ```
//! # mod my_crate { use interoptopus::{Library}; pub fn ffi_inventory() -> Library { todo!() } }
//! use my_crate::ffi_inventory;
//!
//! #[test]
//! fn generate_python_bindings() {
//!     use interoptopus::Interop;
//!     use interoptopus_backend_cpython_cffi::{Generator, PythonWriter, Config};
//!
//!     // Converts an `ffi_inventory()` into Python interop definitions.
//!     Generator::new(Config::default(), ffi_inventory()).write_to("module.py")
//! }
//! ```
//!
//! And we might produce something like this:
//!
//! ```python
//! from cffi import FFI
//!
//! api_definition = """
//! typedef struct Vec3f32
//!     {
//!     float x;
//!     float y;
//!     float z;
//!     } Vec2f32;
//!
//! Vec3f32 my_game_function(Vec3f32* input);
//! """
//!
//!
//! ffi = FFI()
//! ffi.cdef(api_definition)
//! _api = None
//!
//!
//! def init_api(dll):
//!     """Initializes this library, call with path to DLL."""
//!     global _api
//!     _api = ffi.dlopen(dll)
//!
//!
//! class raw:
//!     """Raw access to all exported functions."""
//!
//!     def my_game_function(input):
//!     global _api
//!     return _api.my_game_function(input)
//! ```

use crate::converter::Converter;
use crate::writer::PythonWriter;
use interoptopus::lang::c::{CType, CompositeType, ConstantValue, Documentation, EnumType, FnPointerType, Function, PrimitiveValue};
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, safe_name};
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Interop};
use interoptopus::{Error, Library};
use interoptopus_backend_c::{CWriter, TypeConverter};

mod config;
mod converter;
mod writer;

pub use config::Config;

/// Helper type implementing [`InteropCPythonCFFI`] and [`Interop`].
pub struct Generator {
    c_generator: interoptopus_backend_c::Generator,
    config: Config,
    library: Library,
    converter: Converter,
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

        let c_converter = *c_generator.converter();

        Self {
            c_generator,
            config,
            library,
            converter: Converter { c_converter },
        }
    }
}

impl Interop for Generator {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_all(w)
    }
}

impl PythonWriter for Generator {
    fn config(&self) -> &Config {
        &self.config
    }

    fn library(&self) -> &Library {
        &self.library
    }

    fn converter(&self) -> &Converter {
        &self.converter
    }

    fn c_generator(&self) -> &interoptopus_backend_c::Generator {
        &self.c_generator
    }
}
