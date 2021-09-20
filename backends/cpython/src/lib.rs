//! Generates CPython bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! # Usage
//!
//! Assuming you have written a crate containing your FFI logic called `example_library_ffi` and
//! want to generate **CPython bindings** for Python 3.7+, follow the instructions below.
//!
//! ### Inside Your Library
//!
//! Add [**Interoptopus**](https://crates.io/crates/interoptopus) attributes to the library you have
//! written, and define an [**inventory**](https://docs.rs/interoptopus/latest/interoptopus/macro.inventory.html)
//! function listing all symbols you wish to export. An overview of all supported constructs can be found in the
//! [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/reference_project/src).
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
//! interoptopus_backend_cpython = "..."
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
//!     use interoptopus_backend_cpython::{Config, Generator};
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
//! TODO !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//!
//! ```

use interoptopus::writer::IndentWriter;
use interoptopus::Interop;
use interoptopus::{Error, Library};

mod config;
mod converter;
mod testing;
mod writer;

pub use config::Config;
pub use converter::Converter;
pub use testing::run_python_if_installed;
pub use writer::PythonWriter;

/// **Start here**, main converter implementing [`Interop`].
pub struct Generator {
    config: Config,
    library: Library,
    converter: Converter,
}

impl Generator {
    pub fn new(config: Config, library: Library) -> Self {
        Self {
            config,
            library,
            converter: Converter {},
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
}
