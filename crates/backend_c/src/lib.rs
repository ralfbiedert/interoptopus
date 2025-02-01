//! Generates C bindings for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! # Usage
//!
//! Assuming you have written a crate containing your FFI logic called `example_library_ffi` and
//! want to generate **C bindings**, follow the instructions below.
//!
//! ### Inside Your Library
//!
//! Add [**Interoptopus**](https://crates.io/crates/interoptopus) attributes to the library you have
//! written, and define an inventory function listing all symbols you wish to export. An overview of all
//! supported constructs can be found in the
//! [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/crates/reference_project/src).
//!
//! ```rust
//! use interoptopus::{ffi_function, ffi_type, Inventory, InventoryBuilder, function};
//!
//! #[ffi_type]
//! pub struct Vec2 {
//!     pub x: f32,
//!     pub y: f32,
//! }
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub fn my_function(input: Vec2) -> Vec2 {
//!     input
//! }
//!
//! pub fn my_inventory() -> Inventory {
//!     InventoryBuilder::new()
//!         .register(function!(my_function))
//!         .validate()
//!         .inventory()
//! }
//! ```
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
//! interoptopus_backend_c = "..."
//! ```
//!
//! Create a unit test in `tests/bindings.rs` which will generate your bindings when run
//! with `cargo test`. In real projects you might want to add this code to another crate instead:
//!
//! ```
//! use interoptopus::util::NamespaceMappings;
//! use interoptopus::{Error, Bindings};
//!
//! #[test]
//! fn bindings_c() -> Result<(), Error> {
//!     use interoptopus_backend_c::{Config, Interop};
//!
//!     Interop::new(
//!         Config {
//!             ifndef: "example_library".to_string(),
//!             ..Config::default()
//!         },
//!         example_library_ffi::my_inventory(),
//!     ).write_file("bindings/c/example_complex.h")?;
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
//! ```c
//! // Automatically generated by Interoptopus.
//!
//! #ifndef example_library
//! #define example_library
//!
//! #ifdef __cplusplus
//! extern "C" {
//! #endif
//!
//! #include <stdint.h>
//! #include <stdbool.h>
//!
//! typedef struct vec2
//!     {
//!     float x
//!     float y;
//!     } vec2;
//!
//!
//! vec2 my_function(vec2 input);
//!
//! #ifdef __cplusplus
//! }
//! #endif
//!
//! #endif /* example_library */
//!
//! ```
#![allow(clippy::test_attr_in_doctest)]

mod converters;
mod docs;
mod generator;

pub use docs::Documentation;
pub use generator::{DocStyle, Functions, Indentation, Interop, InteropBuilder, NameCase};
