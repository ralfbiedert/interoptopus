#![cfg_attr(docsrs, feature(doc_cfg))] // does this work?
//!
//! [![Latest Version]][crates.io]
//! [![docs]][docs.rs]
//! ![MIT]
//!
//! # Interoptopus
//!
//! 🦀  →  🐙 →  Python, C#, C, ...
//!
//! FFI bindings to your favorite language. Composable. Sane. Escape hatches included.
//!
//!
//! ## Overview
//!
//! If you ...
//!
//! - wrote an `extern "C"` API in Rust
//! - need C#, Python, C, ... bindings to your library, all at the same time
//! - prefer having fine-grained control over your API and interop generation
//! - would like to use quality-of-life [patterns](crate::patterns) on both sides (e.g., [options](crate::patterns::option), [slices](crate::patterns::slice), '[classes](crate::patterns::class)') where feasible
//! - create your own bindings for a not-yet supported language
//! - want all your binding-related information (e.g., documentation) in Rust code
//!
//! ... then Interoptopus might be for you.
//!
//!
//! ## Known limitations
//!
//! - not yet used in production
//! - somewhat verbose if you don't own most of your types (still possible, just more work)
//! - if you target only a single language and don't care about your FFI layer other solutions might be better
//!
//!
//! ## Supported Languages & Example
//!
//! Assume you have written this Rust FFI code:
//!
//! ```rust
//! use interoptopus::{ffi_function, ffi_type};
//!
//! #[ffi_type]
//! #[repr(C)]
//! pub struct Vec3f32 {
//!     pub x: f32,
//!     pub y: f32,
//!     pub z: f32,
//! }
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn my_game_function(input: Option<&Vec3f32>) -> Vec3f32 {
//!     Vec3f32 { x: 2.0, y: 4.0, z: 6.0 }
//! }
//!
//! interoptopus::inventory_function!(ffi_inventory, [], [my_game_function], []);
//! ```
//!
//! You can now use one of these backends to generate interop code:
//!
//! | Language | Crate | Comment |
//! | --- | --- | --- |
//! | C# (incl. Unity) | [**interoptopus_backend_csharp**](https://crates.io/crates/interoptopus_backend_csharp) |  Built-in. |
//! | C | [**interoptopus_backend_c**](https://crates.io/crates/interoptopus_backend_c) | Built-in. |
//! | Python [CFFI](https://cffi.readthedocs.io/en/latest/index.html) | [**interoptopus_backend_cpython_cffi**](https://crates.io/crates/interoptopus_backend_cpython_cffi) | Built-in. |
//! | Your language | Write your own backend! | See existing backends for what to do. |
//!
//!
//! ## Current Status
//!
//! - June 13, 2021 - Pre-alpha. Has generated C#, C, Python-CFFI bindings at least once, many things missing, untested.
//!
//!
//! ## FAQ
//!
//! - [FAQ and Safety Guides](https://github.com/ralfbiedert/interoptopus/blob/master/TODO.md).
//!
//! ## Contributing
//!
//! PRs are welcome.
//!
//! - Bug fixes can be submitted directly. Major changes should be filed as issues
//! first.
//!
//! - Anything that would make previously working bindings change behavior or stop compiling
//! is a major change; which doesn't mean we're opposed to breaking stuff before 1.0, just that
//! we'd like to talk about it before it happens.
//!
//! - New features or patterns must be materialized in the reference project and accompanied by
//! an interop test (i.e., a backend test running C# / Python against a DLL invoking that code)
//! in at least one included backend.
//!
//! [Latest Version]: https://img.shields.io/crates/v/interoptopus.svg
//! [crates.io]: https://crates.io/crates/interoptopus
//! [MIT]: https://img.shields.io/badge/license-MIT-blue.svg
//! [docs]: https://docs.rs/interoptopus/badge.svg
//! [docs.rs]: https://docs.rs/interoptopus/

pub use error::Error;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))] // does this work?
pub use interoptopus_proc::{ffi_constant, ffi_function, ffi_type};

pub use crate::core::Library;
pub use generators::Interop;

mod core;
mod error;
mod generators;
pub mod patterns;
#[cfg(feature = "testing")]
#[cfg_attr(docsrs, doc(cfg(feature = "testing")))] // does this work?
pub mod testing;
pub mod util;
pub mod writer;

pub mod lang {
    //! Abstractions for authors of backends.
    //!
    //! A a rule of thumb, types in the [`rust`](rust) module generate instances
    //! of types in the [`c`](c) module.
    //!
    //! Normal users of Interoptopus probably won't have to concern
    //! themselves with any of the items in this module.
    //! A notable exception to this rule is having to manually implement
    //! a [`CType`](`c::CType`) surrogate for un-owned types. See the
    //! [surrogates section in `ffi_type`](crate::ffi_type#surrogates).
    pub mod c;
    pub mod rust;
}

/// **The** macro to define your library, ties everything together!
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
/// const MY_CONSTANT: u8 = 1;
///
/// #[ffi_function]
/// #[no_mangle]
/// pub extern "C" fn f(_x: u8) {}
///
/// interoptopus::inventory_function!(
///     my_inventory_function,
///     [ MY_CONSTANT ],
///     [ f ],
///     []
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
        ],
        [
        $(
            $pattern:path
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

            let mut patterns: Vec<$crate::patterns::LibraryPattern> = Vec::new();
            $(
                {
                    patterns.push($pattern().into());
                }
            )*

            $crate::Library::new(functions, constants, patterns)
        }
    };
}
