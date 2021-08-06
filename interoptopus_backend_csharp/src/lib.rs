//! Generates C# bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
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
//! fn generate_csharp_bindings() {
//!     use interoptopus::Interop;
//!     use interoptopus_backend_csharp::{Generator, CSharpWriter, Config};
//!
//!     // Converts an `ffi_inventory()` into C# interop definitions.
//!     Generator::new(Config::default(), ffi_inventory()).write_to("Interop.cs")
//! }
//! ```
//!
//! And we might produce something like this:
//!
//! ```cs
//! using System;
//! using System.Runtime.InteropServices;
//!
//! namespace My.Company
//! {
//!     public static class InteropClass
//!     {
//!         public const string NativeLib = "hello_world";
//!
//!         /// A function which does something with the vector.
//!         [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "my_game_function")]
//!         public static extern Vec3f32 my_game_function(ref Vec3f32 input);
//!     }
//!
//!     [Serializable]
//!     [StructLayout(LayoutKind.Sequential)]
//!     public partial struct Vec3f32
//!     {
//!         public float x;
//!         public float y;
//!         public float z;
//!     }
//! }
//! ```

use interoptopus::writer::IndentWriter;
use interoptopus::Interop;
use interoptopus::{Error, Library};

mod config;
mod converter;
mod writer;

pub use crate::config::{Config, WriteTypes};
pub use crate::converter::{CSharpTypeConverter, Converter};
pub use crate::writer::CSharpWriter;

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

impl CSharpWriter for Generator {
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
