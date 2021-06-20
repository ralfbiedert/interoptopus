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
//! ## Safety, Soundness, Undefined Behavior
//!
//! This library naturally does "unsafe" things and any journey into FFI-land is a little adventure.
//! That said, here are some assumptions and quality standards this project is based on:
//!
//! - Safe Rust calling safe Rust code must always be sound, with soundness boundaries
//! on the module level, although smaller scopes are preferred. For example, creating a `FFISlice`
//! from Rust and directly using it from Rust must never cause UB.
//!
//! - We must never willingly generate broken bindings. For _low level types_ we must never
//! generate bindings which "cannot be used correctly" (e.g., map a `u8` to a `float`), for
//! _patterns_ we must generate bindings that are "correct if used according to specification".
//!
//! - There are situations where the (Rust) soundness of a binding invocation depends on conditions outside
//! our control. In these cases we trust foreign code will invoke the generated functions
//! correctly. For example, if a function is called with an `AsciiPointer` type we consider it _safe and sound_
//! to obtain a `str` from this pointer as `AsciiPointer`'s contract specifies it must point to
//! ASCII data.
//!
//! - Related to the previous point we generally assume functions and types on both sides are used _appropriately_ w.r.t.
//! Rust's FFI requirements and we trust you do that, e.g., you must declare types `#[repr(C)]` (or similar)
//! and functions `extern "C"`.
//!
//! - Any `unsafe` code in any abstraction we provide should be "well contained", properly documented
//! and reasonably be auditable.
//!
//! - If unsound Rust types or bindings were ever needed (e.g., because of a lack of Rust specification,
//! like 'safely' mapping a trait's vtable) such bindings should be gated behind a feature flag
//! (e.g., `unsound`) and only enabled via an explicit opt-in. Right now there are none, but this is
//! to set expectations around discussions.
//!
//!
//! ## FAQ
//!
//! - **Why do I get `error[E0658]: macro attributes in #[derive] output are unstable`?**
//!
//!     This happens when `#[ffi_type]` appears after `#derive[...]`. Just switch their order.
//!
//!
//! - **How do I support a new language?**
//!
//!     1) create a new crate, like `my_language`
//!     1) check which existing backend comes closest, copy that code
//!     1) start from trait `Interop::write_to` producing some output, fix errors as they appear
//!     1) create a UI test against `interoptopus_reference_project` to ensure your bindings are stable
//!
//!
//! - **How does it actually work?**
//!
//!     As  [answered by Alex Hirsekorn](https://www.quora.com/How-does-an-octopus-eat-a-crab-without-getting-cuts?share=1):
//!     - When a GPO [Giant Pacific Octopus] finds a crab it does something called a “flaring web-over” which uses the webbing between the arms to engulf the crab while simultaneously immobilizing the crab’s claws with its suckers.
//!     - With the crab in what amounts to a sealed bag the GPO spits one of its two types of saliva into that space. This first saliva is called cephalotoxin and acts as a sedative, rendering the crab unconscious but still alive. [If the crab is taken away from the GPO at this point it will wake up and run away.]
//!     - The GPO then spits the other kind of saliva into the crab; that saliva is a powerful digestive enzyme. Since the crab is still alive at this point it pumps that enzyme throughout its body and basically digests itself on the GPO’s behalf. The octopus typically takes a nap during this stage.
//!     - After some period of time (I’ve seen this vary from 1.5 to 3 hours) the GPO wakes up, disassembles the crab, and licks out what amounts to crab meat Jell-O with a tongue-like structure called a radula. As Kathleen said the GPO does the disassembly with its suckers but it doesn’t just open the carapace: It will also take the claws and legs apart at the various joints.
//!     - When the meal is finished and the shell parts tossed out the GPO will take another nap until it’s hungry again. [Studies have shown that a GPO spends as much as 70% of its time sleeping in its den.]
//!
//!     Occasionally a GPO will get minor injuries from capturing the crab but, for the most part they are too careful and too skilled for that to be much of an issue.
//!
//!     After the GPO has rested, FFI bindings are produced.
//!
//! ## Contributing
//!
//! PRs are welcome.
//!
//! - Bug fixes can be submitted directly. major changes should be filed as issues
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
//! Also, please read the next section.
//!
//!
//! ## License
//!
//! [MIT](https://opensource.org/licenses/MIT)
//!
//! This license only applies to code **in** this repository, not code generated **by** this repository. We do not claim copyright for code produced by backends included here; even if said code was based on a template in this repository.
//! For the avoidance of doubt, anything produced by `Interop::write_to` or any item emitted by a proc macro is considered "generated by".
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
