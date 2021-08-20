//! Generates CPython CFFI bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! # Usage
//!
//! Assuming you have written a crate containing your FFI logic called `example_library_ffi` and
//! want to generate **CPython CFFI bindings** for Python 3.7+, follow the instructions below.
//!
//! ### Inside Your Library
//!
//! Add [**Interoptopus**](https://crates.io/crates/interoptopus) attributes to the library you have
//! written, and define an [**inventory**](https://docs.rs/interoptopus/latest/interoptopus/macro.inventory.html)
//! function listing all symbols you wish to export. An overview of all supported constructs can be found in the
//! [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/interoptopus_reference_project/src).
//!
//! ```rust
//! use interoptopus::{ffi_function, ffi_type};
//!
//! #[ffi_type]
//! #[repr(C)]
//! pub struct Vec2 {
//!     pub x: f32,
//!     pub y: f32,
//! }
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn my_function(input: Vec2) -> Vec2 {
//!     input
//! }
//!
//! interoptopus::inventory!(my_inventory, [], [my_function], [], []);
//! ```
//!
//!
//! Add these to your `Cargo.toml` so the attributes and the binding generator can be found
//! (replace `...` with the latest version):
//!
//! ```toml
//! [lib]
//! crate-type = ["cdylib", "rlib"]
//!
//! [dependencies]
//! interoptopus = "..."
//! interoptopus_backend_cpython_cffi = "..."
//! ```
//!
//! Create a unit test in `tests/bindings.rs` which will generate your bindings when run
//! with `cargo test`. In real projects you might want to add this code to another crate instead:
//!
//! ```
//! use interoptopus::util::NamespaceMappings;
//! use interoptopus::{Error, Interop};
//!
//! #[test]
//! fn bindings_cpython_cffi() -> Result<(), Error> {
//!     use interoptopus_backend_cpython_cffi::{Config, Generator};
//!
//!     let library = example_library_ffi::my_inventory();
//!
//!     Generator::new(Config::default(), library)
//!         .write_file("bindings/python/example_library.py")?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Now run `cargo test`.
//!
//! If anything is unclear you can find a [**working sample on Github**](https://github.com/ralfbiedert/interoptopus/tree/master/examples/hello_world).
//!
//! ### Generated Output
//!
//! The output below is what this backend might generate. Have a look at the [`Config`] struct
//! if you want to customize something. If you really don't like how something is generated it is
//! easy to [**create your own**](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md#new-backends).
//!
//! ```python
//! from cffi import FFI
//!
//! api_definition = """
//! typedef struct cffi_vec2
//!     {
//!     float x;
//!     float y;
//!     } cffi_vec2;
//!
//!
//! cffi_vec2 my_function(cffi_vec2 input);
//! """
//!
//! ffi = FFI()
//! ffi.cdef(api_definition)
//! _api = None
//!
//! def init_api(dll):
//!     """Initializes this library, call with path to DLL."""
//!     global _api
//!     _api = ffi.dlopen(dll)
//!
//!
//! class Vec2(object):
//!     """ A simple type in our FFI layer."""
//!     def __init__(self):
//!         global _api, ffi
//!         self._ctx = ffi.new("cffi_vec2[]", 1)
//!
//!     def array(n):
//!         global _api, ffi
//!         return ffi.new("cffi_vec2[]", n)
//!
//!     def ptr(self):
//!         return self._ctx
//!
//!     @property
//!     def x(self):
//!         """"""
//!         return self._ctx[0].x
//!
//!     @x.setter
//!     def x(self, value):
//!         self._ptr_x = value
//!         self._ctx[0].x = value
//!
//!     @property
//!     def y(self):
//!         """"""
//!         return self._ctx[0].y
//!
//!     @y.setter
//!     def y(self, value):
//!         self._ptr_y = value
//!         self._ctx[0].y = value
//!
//!
//! class raw:
//!     """Raw access to all exported functions."""
//!     def my_function(input):
//!         """ Function using the type."""
//!         global _api
//!         if hasattr(input, "_ctx"):
//!             input = input._ctx[0]
//!
//!         return _api.my_function(input)
//!
//! ```

use interoptopus::writer::IndentWriter;
use interoptopus::Interop;
use interoptopus::{Error, Library};
use interoptopus_backend_c::CWriter;

mod config;
mod converter;
mod testing;
mod writer;

pub use config::Config;
pub use converter::{Converter, PythonTypeConverter};
pub use testing::run_python_if_installed;
pub use writer::PythonWriter;

/// **Start here**, main converter implementing [`Interop`].
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
                prefix: "cffi_".to_string(),
                ..interoptopus_backend_c::Config::default()
            },
            library.clone(),
        );

        let c_converter = c_generator.converter().clone();

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
