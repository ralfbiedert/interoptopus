#![cfg_attr(docsrs, feature(doc_cfg))] // does this work?
//!
//! ## Interoptopus
//!
//! Create FFI bindings to your favorite language. Composable. Escape hatches included.
//!
//!
//!
//! ## Overview
//!
//! - you wrote an `extern "C"` API in Rust
//! - the types at the FFI boundary are (mostly) owned by yourself
//! - you prefer to keep all your binding-related information (e.g., documentation) in Rust code
//!
//! Known limitations
//! - not used in production yet
//! - somewhat verbose if you don't own most of your types (still possible, just more work)
//! - if you target only a single language and don't care about your FFI layer other solutions might be better
//!
//! ## Supported Languages
//!
//! | Language | Crate | Comment |
//! | --- | --- | --- |
//! | C# | `interoptopus_backend_csharp` |  Built-in. |
//! | C | `interoptopus_backend_c` | Built-in. |
//! | Python CFFI | `interoptopus_backend_cpython_cffi` | Built-in. |
//! | Your language | Write your own backend! | See existing backends for what to do.* |
//!
//! (*) Ok, right now I don't really recommend writing a new backend just yet as lots of internals might change. That said, it should only take a few hours and feedback is more than welcome.
//!
//!
//!
//! ## Example
//!
//! Slightly abridged, see the `examples/hello_world` for full code:
//!
//! ```rust
//! use interoptopus::{ffi_function, ffi_type};
//!
//! #[ffi_type]
//! #[repr(C)]
//! pub struct Vec2f32 {
//!     pub x: f32,
//!     pub y: f32,
//!     pub z: f32,
//! }
//!
//! /// A function which does something with the vector.
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn my_game_function(input: Option<&Vec2f32>) -> Vec2f32 {
//!     Vec2f32 { x: 2.0, y: 4.0, z: 6.0 }
//! }
//!
//! // This ultimately defines our FFI exports, all functions have to be listed here.
//! interoptopus::inventory_function!(ffi_inventory, [], [my_game_function]);
//!
//! #[test]
//! fn generate_csharp_bindings() {
//!     use interoptopus_backend_csharp::InteropCSharp;
//!     use interoptopus::writer::IndentWriter;
//!
//!     let library = ffi_inventory();
//!
//!     let config = interoptopus_backend_csharp::Config {
//!         namespace: "My.Company".to_string(),
//!         class: "InteropClass".to_string(),
//!         dll_name: "hello_world".to_string(),
//!         ..interoptopus_backend_csharp::Config::default()
//!     };
//!
//!     let generator = interoptopus_backend_csharp::Generator::new(config, library);
//!
//!     generator.write_to(my_file)?;
//! }
//! ```
//!
//! With a Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! interoptopus = { version = "0.1", features = ["derive"] }
//! interoptopus_backend_csharp = "0.1"
//! ```
//!
//!
//! Will produce:
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
//!         public static extern Vec2f32 my_game_function(ref Vec2f32 input);
//!     }
//!
//!     [Serializable]
//!     [StructLayout(LayoutKind.Sequential)]
//!     public partial struct Vec2f32
//!     {
//!         public float x;
//!         public float y;
//!         public float z;
//!     }
//! }
//! ```
//!
//! For other languages (Python, C, ...) see `examples` folder.
//!

pub use error::Error;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))] // does this work?
pub use interoptopus_proc::{ffi_constant, ffi_function, ffi_type};

pub use crate::core::Library;

mod core;
mod error;
pub mod generators;
pub mod patterns;
#[cfg(feature = "testing")]
#[cfg_attr(docsrs, doc(cfg(feature = "testing")))] // does this work?
pub mod testing;
pub mod util;
pub mod writer;

pub mod lang {
    //! Abstractions for authors of backends.
    pub mod c;
    pub mod rust;
}

/// **The** macro to define your library, start here!
///
/// This macro produces an "inventory function", which can be ingested by backends. The function
/// will have the signature `fn f() -> Library`, where [`Library`] represents all functions,
/// types, constants and documentation exported by this crate over the FFI boundary.
///
/// Constants and functions must be declared; types are determined automatically.
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi_function, ffi_constant};
///
/// #[ffi_constant]
/// const C1: u8 = 1;
///
/// #[ffi_constant]
/// const C2: u8 = 2;
///
/// #[ffi_function]
/// #[no_mangle]
/// pub extern "C" fn f(_x: u8) {}
///
/// interoptopus::inventory_function!(
///     my_inventory_function,
///     [ C1, C2 ],
///     [ f ]
/// );
/// ```
///
/// You can then use `my_inventory_function`, which will return a [`Library`], in a backend to
/// produce bindings to your language.
///
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))] // does this work?
#[macro_export]
macro_rules! inventory_function {
    (
        $export_function:ident,
        [
        $(
            $const:path
        ),*
        ],
        [
        $(
            $function:path
        ),*
        ]
    ) => {
        #[doc(hidden)]
        pub fn $export_function() -> $crate::Library {
            use $crate::lang::rust::FunctionInfo;
            use $crate::lang::rust::ConstantInfo;

            let mut constants: Vec<$crate::lang::c::Constant> = Vec::new();
            $(
                {
                    use $const as x;
                    constants.push(x::constant_info());
                }
            )*

            let mut functions: Vec<$crate::lang::c::Function> = Vec::new();
            $(
                {
                    use $function as x;
                    functions.push(x::function_info());
                }
            )*

            $crate::Library::new(functions, constants)
        }
    };
}
