//! Generates C# bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! # Usage
//!
//! Assuming you have written a crate containing your FFI logic called `example_library_ffi` and
//! want to generate **C# bindings**, follow the instructions below.
//!
//! ### Inside Your Library
//!
//! Add [**Interoptopus**](https://crates.io/crates/interoptopus) attributes to the library you have
//! written, and define an inventory function listing all symbols you wish to export. An overview of all
//! supported constructs can be found in the
//! [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/reference_project/src).
//!
//! ```rust
//! use interoptopus::{ffi_function, ffi_type, Library, LibraryBuilder, function};
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
//! pub fn my_inventory() -> Library {
//!     LibraryBuilder::new()
//!         .register(function!(my_function))
//!         .library()
//! }
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
//! interoptopus_backend_csharp = "..."
//! ```
//!
//! Create a unit test in `tests/bindings.rs` which will generate your bindings when run
//! with `cargo test`. In real projects you might want to add this code to another crate instead:
//!
//!```
//! use interoptopus::util::NamespaceMappings;
//! use interoptopus::{Error, Interop};
//!
//! #[test]
//! fn bindings_csharp() -> Result<(), Error> {
//!     use interoptopus_backend_csharp::{Config, Generator};
//!     use interoptopus_backend_csharp::overloads::{DotNet, Unity};
//!
//!     let config = Config {
//!         dll_name: "example_library".to_string(),
//!         namespace_mappings: NamespaceMappings::new("My.Company"),
//!         ..Config::default()
//!     };
//!
//!     Generator::new(config, example_library_ffi::my_inventory())
//!         .add_overload_writer(DotNet::new())
//!         //.add_overload_writer(Unity::new())
//!         .write_file("bindings/csharp/Interop.cs")?;
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
//! ```csharp
//!using System;
//!using System.Collections;
//!using System.Collections.Generic;
//!using System.Runtime.InteropServices;
//!using My.Company;
//!
//! namespace My.Company
//! {
//!     public static partial class InteropClass
//!     {
//!         public const string NativeLib = "example_library";
//!
//!         /// Function using the type.
//!         [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "my_function")]
//!         public static extern Vec2 my_function(Vec2 input);
//!     }
//!
//!     /// A simple type in our FFI layer.
//!     [Serializable]
//!     [StructLayout(LayoutKind.Sequential)]
//!     public partial struct Vec2
//!     {
//!         public float x;
//!         public float y;
//!     }
//! }
//! ```

use interoptopus::writer::IndentWriter;
use interoptopus::Interop;
use interoptopus::{Error, Library};
use overloads::OverloadWriter;

mod config;
mod converter;
pub mod overloads;
mod testing;
mod writer;

pub use config::{CSharpVisibility, Config, Unsafe, WriteTypes};
pub use converter::{CSharpTypeConverter, Converter};
pub use testing::run_dotnet_command_if_installed;
pub use writer::CSharpWriter;

/// **Start here**, main converter implementing [`Interop`].
pub struct Generator {
    config: Config,
    library: Library,
    converter: Converter,
    overload_writer: Vec<Box<dyn OverloadWriter>>,
}

impl Generator {
    pub fn new(config: Config, library: Library) -> Self {
        Self {
            config,
            library,
            converter: Converter {},
            overload_writer: vec![],
        }
    }

    pub fn add_overload_writer(&mut self, writer: Box<dyn OverloadWriter>) -> &mut Self {
        self.overload_writer.push(writer);
        self
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

    fn overloads(&self) -> &[Box<dyn OverloadWriter>] {
        self.overload_writer.as_slice()
    }
}
