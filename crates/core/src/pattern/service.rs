//! Becomes a service class in object oriented languages.
//!
//! Services provide a convenient way to manage state and memory between method invocations.
//! They are similar to classes in object oriented languages, but we refrained from naming them
//! as such because for lifetime and memory-safety reasons it would be best practice to only have
//! "a few" well-defined instances around at any given time, not millions with arbitrary
//! inter-dependencies.
//!
//! That said, services usually translate to classes in languages supporting them, automatically
//! guard against panics (preventing them from bubbling into C which would be undefined behavior)
//! and can provide transparent error handling.
//!
//! In short, if your library offers a "service", the _service pattern_ might give you a noticeable
//! quality of life improvement.
//!
//! # Defining Services
//!
//! To define a service you need the following parts:
//!
//! - An `opaque` type; the instance of a service
//! - A service implementation with at least one constructor (returning `ffi::Result<Self, _>`)
//!
//! # Example
//!
//! In this example we define a service called `SimpleService` with a constructor and two methods.
//!
//! ```
//! # use std::fmt::{Display, Formatter};
//! #
//! # impl Display for Error {
//! #     fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
//! #         Ok(())
//! #     }
//! # }
//! #
//! # impl std::error::Error for Error {}
//! #
//! # #[ffi_type]
//! # #[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
//! # pub enum Error {
//! #     Bad = 3,
//! # }
//! #
//! use interoptopus::{ffi, ffi_type, ffi_service, ffi_service_method};
//!
//! #[ffi_type(opaque)]
//! pub struct SimpleService {
//!     pub some_value: u32,
//! }
//!
//! #[ffi_service]
//! impl SimpleService {
//!
//!     pub fn new_with(some_value: u32) -> ffi::Result<Self, Error> {
//!         ffi::Ok(Self { some_value })
//!     }
//!
//!     pub fn maybe_fails(&self, x: u32) -> ffi::Result<(), Error> {
//!         ffi::Ok(())
//!     }
//!
//!     pub fn just_return_value(&self) -> u32 {
//!         self.some_value
//!     }
//! }
//! ```
//!
//! In languages supporting this pattern bindings will be generated allowing the service to be
//! instantiated and called like this:
//!
//! ```csharp
//! var x = new SimpleService(123);
//! x.MaybeFails(456);
//! x.JustReturnValue();
//! x.Dispose();
//! ```

//!
//! In other languages and on the C FFI level the following methods would be emitted:
//!
//! ```c
//! myffierror simple_service_new_with(simpleservice** context, uint32_t some_value);
//! myffierror simple_service_destroy(simpleservice** context);
//! myffierror simple_service_maybe_fails(simpleservice* context, uint32_t x);
//! uint32_t simple_service_just_return_value(simpleservice* context);
//! ```
//!

use crate::pattern::result::ResultAsPtr;

pub trait ServiceInfo {
    type CtorResult: ResultAsPtr;
}
