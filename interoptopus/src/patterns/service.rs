//! Bundles function with common receiver into a service or 'class' in object oriented languages.
//!
//! Services provide a convenient way to manage state and memory between method invocations.
//! They are similar to classes in object oriented languages, but we refrained from naming them
//! as such because for lifetime and memory-safety reasons it would be best practice to only have
//! "a few" well-defined instances around at any given time, not millions with arbitrary
//! inter-dependencies.
//!
//! That said, services usually translate to classes in languages supporting them, automatically
//! guard against panics (prevent them from bubbling into C which would be undefined behavior)
//! and can provide transparent error handling.
//!
//! In short, if your library provides a "service", the _service pattern_ might provide a noticeable
//! quality of life improvement.
//!
//! # C API
//!
//! Services reflect a common pattern in C APIs, where an opaque type `Service` is created with a
//! `service_init(*s)` function. A set of functions can then be invoked on `s`, such as
//! `service_compute(s, x)`. Once the service is no longer needed it will be removed
//! with `service_destroy(*s)`.
//!
//!
//! # Defining Services
//!
//! To define a service you need the following parts:
//!
//! - An `opaque` type; the instance of a service
//! - A Rust `Error` type mappable to an [FFIError](crate::patterns::result::FFIError) enum via `From<Error>`
//! - Some methods on the opaque type.
//!
//! # Example
//!
//! In this example we define a service called `SimpleService` with a constructor and two methods.
//! The type `MyFFIError` is not shown, but implemented as in the [FFIError](crate::patterns::result::FFIError) example.
//!
//! ```
//! # use std::fmt::{Display, Formatter};
//! #
//! # #[derive(Debug)]
//! # pub enum Error {
//! #     Bad,
//! # }
//! #
//! # impl Display for Error {
//! #     fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
//! #         Ok(())
//! #     }
//! # }
//! #
//! # impl std::error::Error for Error {}
//! #
//! # #[ffi_type(patterns(ffi_error))]
//! # #[repr(C)]
//! # pub enum MyFFIError {
//! #     Ok = 0,
//! #     NullPassed = 1,
//! #     Panic = 2,
//! #     OtherError = 3,
//! # }
//! #
//! # impl FFIError for MyFFIError {
//! #     const SUCCESS: Self = Self::Ok;
//! #     const NULL: Self = Self::NullPassed;
//! #     const PANIC: Self = Self::Panic;
//! # }
//! #
//! # impl From<Error> for MyFFIError {
//! #     fn from(x: Error) -> Self {
//! #         match x {
//! #             Error::Bad => Self::OtherError,
//! #         }
//! #     }
//! # }
//! #
//! use interoptopus::{ffi_type, ffi_service, ffi_service_ctor, ffi_service_method};
//! use interoptopus::patterns::result::FFIError;
//!
//! #[ffi_type(opaque)]
//! pub struct SimpleService {
//!     pub some_value: u32,
//! }
//!
//! #[ffi_service(error = "MyFFIError", prefix = "simple_service_")]
//! impl SimpleService {
//!
//!     #[ffi_service_ctor]
//!     pub fn new_with(some_value: u32) -> Result<Self, Error> {
//!         Ok(Self { some_value })
//!     }
//!
//!     pub fn maybe_fails(&self, x: u32) -> Result<(), Error> {
//!         Ok(())
//!     }
//!
//!     #[ffi_service_method(direct)]
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

use crate::lang::c::{CType, Function, OpaqueType};
use crate::patterns::TypePattern;
use std::fmt::Debug;

/// Combines a receiver, constructor, destructor and multiple methods in one entity.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Service {
    the_type: OpaqueType,
    constructor: Function,
    destructor: Function,
    methods: Vec<Function>,
}

impl Service {
    pub fn new(constructor: Function, destructor: Function, methods: Vec<Function>) -> Self {
        let the_type = extract_obvious_opaque_from_parameter(
            constructor
                .signature()
                .params()
                .first()
                .expect("Constructor must have at least one parameter")
                .the_type(),
        )
        .expect("First parameter must point to opaque.");

        Self {
            the_type,
            constructor,
            destructor,
            methods,
        }
    }

    /// Checks if the signature of this service is compatible with the `Service` pattern, panic with
    /// error message otherwise.
    ///
    /// This function is mainly called during compile time therefore panicking with a good error
    /// message is beneficial.
    pub fn assert_valid(&self) {
        let constructor_fist_parameter = self
            .constructor
            .signature()
            .params()
            .get(0)
            .expect("Constructor for must have at least one parameter.");

        match &constructor_fist_parameter.the_type() {
            CType::ReadWritePointer(x) => match **x {
                CType::ReadWritePointer(ref x) => match **x {
                    CType::Opaque(_) => {}
                    _ => panic!("First parameter must be opaque type"),
                },
                _ => panic!("First parameter must be opaque type"),
            },
            CType::Opaque(_) => {}
            _ => panic!("First parameter must be RwPointer(RwPointer(Opaque)) type"),
        }

        let destructor_first_parameter = self
            .destructor
            .signature()
            .params()
            .get(0)
            .expect("Constructor for must have at least one parameter.");

        match &destructor_first_parameter.the_type() {
            CType::ReadWritePointer(x) => match **x {
                CType::ReadWritePointer(ref x) => match **x {
                    CType::Opaque(_) => {}
                    _ => panic!("First parameter must be opaque type"),
                },
                _ => panic!("First parameter must be opaque type"),
            },
            CType::Opaque(_) => {}
            _ => panic!("First parameter must be RwPointer(RwPointer(Opaque)) type"),
        }

        match self.constructor.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => {}
            _ => panic!("Constructor must return a `ffi_error` type pattern."),
        }

        match self.destructor.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => {}
            _ => panic!("Destructor must return a `ffi_error` type pattern."),
        }
    }

    pub fn the_type(&self) -> &OpaqueType {
        &self.the_type
    }

    pub fn constructor(&self) -> &Function {
        &self.constructor
    }

    pub fn destructor(&self) -> &Function {
        &self.destructor
    }

    pub fn methods(&self) -> &[Function] {
        &self.methods
    }
}

/// Walks the type until it finds the first "obvious" Opaque.
///
/// An Opaque is obvious if it is at a singular position (e.g., `*const Opaque`),
/// but not within the fields of a struct.
fn extract_obvious_opaque_from_parameter(param: &CType) -> Option<OpaqueType> {
    match param {
        CType::Primitive(_) => None,
        CType::Enum(_) => None,
        CType::Opaque(x) => Some(x.clone()),
        CType::Composite(_) => None,
        CType::FnPointer(_) => None,
        CType::ReadPointer(x) => extract_obvious_opaque_from_parameter(x),
        CType::ReadWritePointer(x) => extract_obvious_opaque_from_parameter(x),
        CType::Pattern(_) => None,
        CType::Array(_) => None,
    }
}
