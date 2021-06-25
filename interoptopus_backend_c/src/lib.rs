//! Generates C bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
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
//! fn generate_c_bindings() {
//!     use interoptopus::Interop;
//!     use interoptopus_backend_c::{Generator, Writer, Config};
//!
//!     // Converts an `ffi_inventory()` into Python interop definitions.
//!     Generator::new(Config::default(), ffi_inventory()).write_to("module.h")
//! }
//! ```
//!
//! And we might produce something like this:
//!
//! ```c
//!
//! #ifndef module
//! #define module
//!
//! #ifdef __cplusplus
//! extern "C" {
//! #endif
//!
//! #include <stdint.h>
//! #include <stdbool.h>
//!
//! typedef struct Vec3f32
//! {
//!     float x;
//!     float y;
//!     float z;
//! } Vec3f32;
//!
//! Vec3f32 my_game_function(Vec3f32* input);
//!
//! #ifdef __cplusplus
//! }
//! #endif
//!
//! #endif /* module */
//!
//! ```

use interoptopus::writer::IndentWriter;
use interoptopus::Interop;
use interoptopus::{Error, Library};

mod config;
mod converter;
mod writer;

pub use crate::config::Config;
pub use converter::{Converter, TypeConverter};
pub use writer::CWriter;

/// Helper type implementing [`InteropC`] and [`Interop`].
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

impl CWriter for Generator {
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
