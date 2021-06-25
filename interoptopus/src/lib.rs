#![cfg_attr(docsrs, feature(doc_cfg))] // does this work?
//!
//! [![Latest Version]][crates.io]
//! [![docs]][docs.rs]
//! ![MIT]
//! [![Rust](https://img.shields.io/badge/rust-1.53%2B-blue.svg?maxAge=3600)](https://github.com/ralfbiedert/interoptopus)
//! [![Rust](https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml)
//!
//! # Interoptopus
//!
//! C#, Python, C, ... → 🐙 → 🦀
//!
//! FFI from your favorite language to Rust. Escape hatch ets included. 🪓
//!
//!
//! ## Overview
//!
//! If you ...
//!
//! - are **about to write** an `extern "C"` API in Rust,
//! - **need** C#, Python, C, ... bindings to your library,
//! - prefer having **fine-grained-control** over your API and interop generation,
//! - would like to use **quality-of-life [patterns](crate::patterns)** on **both sides** (e.g., [options](crate::patterns::option), [slices](crate::patterns::slice), '[classes](crate::patterns::class)') where feasible,
//! - might need to swiftly **support a new language**,
//! - think your FFI [**single source of truth**](https://en.wikipedia.org/wiki/Single_source_of_truth) should be living Rust code,
//!
//! ... then Interoptopus might be for you.
//!
//!
//! ## Known Limitations
//!
//! - not yet used in production
//! - a bit verbose if you don't own your types (still possible, just more work)
//! - if you target only a single language and don't care about your FFI layer other solutions might be better
//!
//!
//! ## Rust Code You Write
//!
//! This is code you would write:
//!
//! ```rust
//! use interoptopus::{ffi_function, ffi_type};
//!
//! #[ffi_type]
//! #[repr(C)]
//! pub struct Vec3 {
//!     pub x: f32,
//!     pub y: f32,
//!     pub z: f32,
//! }
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn my_function(input: Vec3) -> Vec3 {
//!     Vec3 { x: 2.0, y: 4.0, z: input.z }
//! }
//!
//! interoptopus::inventory_function!(ffi_inventory, [], [my_function], []);
//! ```
//!
//! ## Generated Code
//!
//! Once you've written the code above you use one of these backends to generate interop code:
//!
//! | Language | Crate | Sample Output |
//! | --- | --- | --- |
//! | C# (incl. Unity) | [**interoptopus_backend_csharp**](https://crates.io/crates/interoptopus_backend_csharp) | [Interop.cs](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_csharp/tests/output/Interop.cs) |
//! | C | [**interoptopus_backend_c**](https://crates.io/crates/interoptopus_backend_c) | [my_header.h](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_c/tests/output/my_header.h) |
//! | Python [CFFI](https://cffi.readthedocs.io/en/latest/index.html) | [**interoptopus_backend_cpython_cffi**](https://crates.io/crates/interoptopus_backend_cpython_cffi) | [reference.py](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_cpython_cffi/tests/output/reference_project.py) |
//! | Your language | Write your own backend! | - |
//!
//! ## Features
//!
//! See the [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/interoptopus_reference_project/src) lists all supported constructs including:
//! - [functions](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_reference_project/src/functions.rs) (`extern "C"` functions and delegates)
//! - [types](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_reference_project/src/types.rs) (primitives, composite, enums (numeric only), opaques, references, pointers, ...)
//! - [constants](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_reference_project/src/constants.rs) (primitive constants; results of const evaluation)
//! - [patterns](https://github.com/ralfbiedert/interoptopus/tree/master/interoptopus_reference_project/src/patterns) (ASCII pointers, options, slices, classes, ...)
//!
//! As a rule of thumb we recommend to be slightly conservative with your signatures and always "think C", since other languages don't track lifetimes
//! well and it's is easy to accidentally pass an outlived pointer or doubly alias a `&mut X` on reentrant functions.
//!
//!
//! ## Current Status
//!
//! - June 20, 2021 - Alpha. Has generated simple working<sup>TM</sup> bindings for a few projects for a week now, many things missing.
//! - June 13, 2021 - Pre-alpha. Has generated C#, C, Python-CFFI bindings at least once, many things missing, untested.
//!
//!
//! ## FAQ
//!
//! - [FAQ and Safety Guides](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md).
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
/// # Usage
///
/// This macro must be invoked with exactly 4 parameters:
///
/// ```ignore
/// # use interoptopus::inventory_function;
/// inventory_function!(symbol, consts, functions, patterns);
/// ```
///
/// Where
/// - `symbol` - the name of the exported inventory function producing a [`Library`],
/// - `consts` - a list of [`#[ffi_constant]`](crate::ffi_constant) constants to include `[C1, C2, ...]`,
/// - `functions` - a list of [`#[ffi_function]`](crate::ffi_function) functions to include `[f1, f2, ...]`,
/// - `patterns` - a list of [`LibraryPattern`](crate::patterns::LibraryPattern) to include `[p1, ...]`,
///
/// Any of `consts`, `functions` or `patters` can be an empty list `[]` instead. Types are always
/// inferred automatically based on the used functions.
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
                    use $const as user_constant;
                    constants.push(user_constant::constant_info());
                }
            )*

            let mut functions: Vec<$crate::lang::c::Function> = Vec::new();
            $(
                {
                    use $function as user_function;
                    functions.push(user_function::function_info());
                }
            )*

            let mut patterns: Vec<$crate::patterns::LibraryPattern> = Vec::new();
            $(
                {
                    let pattern: $crate::patterns::LibraryPattern = $pattern().into();

                    match &pattern {
                        $crate::patterns::LibraryPattern::Class(class) => {
                            functions.push(class.constructor().clone());
                            functions.push(class.destructor().clone());
                            functions.extend(class.methods().iter().cloned());
                        }
                    }

                    patterns.push(pattern);
                }
            )*

            $crate::Library::new(functions, constants, patterns)
        }
    };
}
