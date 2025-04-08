//! Like a regular [`Result`](std::result::Result), but FFI safe.
//!
//!
//! # Examples
//!
//! Functions returning a [`Result`] might receive special treatment in backends supporting
//! exception handling. For example, a [`service`](crate::pattern::service) method defined
//! as:
//!
//! ```
//! # use interoptopus::Error;
//! #
//! pub fn my_method() -> Result<(), Error> {
//!     Ok(())
//! }
//! ```
//!
//! might receive a binding helper equivalent to:
//!
//! ```csharp
//! public void MyMethod()
//! {
//!     var rval = Interop.simple_service_my_method(_context);
//!     if (rval != FFIError.Ok)
//!     {
//!         throw new Exception($"Something went wrong {rval}");
//!     }
//! }
//! ```

use crate::backend::capitalize_first_letter;
use crate::lang::{Docs, Enum, Layout, Meta, Primitive, Representation, Type, Variant};
use crate::lang::{TypeInfo, VariantKind};
use crate::pattern::TypePattern;
use std::any::Any;
use std::fmt::Debug;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Extracts a string message from a panic unwind.
pub fn get_panic_message(pan: &(dyn Any + Send)) -> &str {
    match pan.downcast_ref::<&'static str>() {
        Some(s) => s,
        None => match pan.downcast_ref::<String>() {
            Some(s) => s,
            None => "Any { .. }",
        },
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
    /// Internal variant used when a panic occurred.
    Panic,
    /// Internal variant used when null was passed where it shouldn't.
    Null,
}

impl<T, E> ResultAsPtr for Result<T, E> {
    type AsPtr = Result<*const T, E>;
}

impl<T, E> ResultAsUnitT for Result<T, E> {
    type AsUnitT = Result<(), E>;
}

impl<T, E> Result<T, E>
where
    T: TypeInfo,
    E: TypeInfo,
{
    /// Returns `true` if the result is `Ok`.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            Self::Err(_) => false,
            Self::Panic => false,
            Self::Null => false,
        }
    }

    /// Returns the `Ok` variant if it exists, otherwise panics.
    pub fn unwrap(self) -> T {
        if let Self::Ok(t) = self {
            t
        } else {
            panic!("Called `unwrap` on an `FFIResult` that is not `Ok`.")
        }
    }

    /// Returns the `Err` variant if it exists, otherwise panics.
    pub fn unwrap_err(self) -> E {
        if let Self::Err(err) = self {
            err
        } else {
            panic!("Called `unwrap_err` on an `FFIResult` that is not `Err`.")
        }
    }
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E>
where
    T: TypeInfo,
    E: TypeInfo,
{
    fn from(x: std::result::Result<T, E>) -> Self {
        match x {
            std::result::Result::Ok(t) => Self::Ok(t),
            std::result::Result::Err(err) => Self::Err(err),
        }
    }
}

unsafe impl<T, E> TypeInfo for Result<T, E>
where
    T: TypeInfo,
    E: TypeInfo,
{
    fn type_info() -> Type {
        let doc_t = Docs::from_line("Element if err is `Ok`.");
        let doc_err = Docs::from_line("Error value.");

        let variants = vec![
            Variant::new("Ok".to_string(), VariantKind::Typed(0, Box::new(T::type_info())), doc_t),
            Variant::new("Err".to_string(), VariantKind::Typed(1, Box::new(E::type_info())), doc_err),
            Variant::new("Panic".to_string(), VariantKind::Unit(2), Docs::new()),
            Variant::new("Null".to_string(), VariantKind::Unit(3), Docs::new()),
        ];

        let doc = Docs::from_line("Result that contains value or an error.");
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_module_docs(T::type_info().namespace().map_or_else(String::new, std::convert::Into::into), doc);
        let t_name = capitalize_first_letter(T::type_info().name_within_lib().as_str());
        let e_name = capitalize_first_letter(E::type_info().name_within_lib().as_str());
        let name = match T::type_info() {
            Type::Primitive(Primitive::Void) => format!("Result{e_name}"),
            _ => format!("Result{t_name}{e_name}"),
        };
        let the_enum = Enum::new(name, variants, meta, repr);
        let result_enum = ResultType::new(the_enum);
        Type::Pattern(TypePattern::Result(result_enum))
    }
}

pub trait IntoFFIResult {
    type FFIResult;
}

impl<T, E> IntoFFIResult for Result<T, E> {
    type FFIResult = Self;
}

///
/// At some point we want to get rid of these once `Try` ([try_trait_v2](https://github.com/rust-lang/rust/issues/84277)) stabilizes.
pub fn result_to_ffi<T: TypeInfo, E: TypeInfo>(f: impl FnOnce() -> std::result::Result<T, E>) -> Result<T, E> {
    f().into()
}

/// At some point we want to get rid of these once `Try` ([try_trait_v2](https://github.com/rust-lang/rust/issues/84277)) stabilizes.
pub async fn result_to_ffi_async<T: TypeInfo, E: TypeInfo>(f: impl std::ops::AsyncFnOnce() -> std::result::Result<T, E>) -> Result<T, E> {
    f().await.into()
}

/// Converts a panic to a [`Result::Panic`].
pub fn panic_to_result<T: TypeInfo, E: TypeInfo>(f: impl FnOnce() -> Result<T, E>) -> Result<T, E> {
    catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|_| Result::Panic)
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ResultType {
    the_enum: Enum,
}

impl ResultType {
    #[must_use]
    pub fn new(the_enum: Enum) -> Self {
        Self { the_enum }
    }

    #[must_use]
    pub fn meta(&self) -> &Meta {
        self.the_enum.meta()
    }

    #[must_use]
    pub fn t(&self) -> &Type {
        self.the_enum.variants()[0].kind().as_typed().unwrap()
    }

    #[must_use]
    pub fn e(&self) -> &Type {
        self.the_enum.variants()[1].kind().as_typed().unwrap()
    }

    #[must_use]
    pub fn the_enum(&self) -> &Enum {
        &self.the_enum
    }
}

pub trait ResultAsPtr {
    type AsPtr;
}

pub trait ResultAsUnitT {
    type AsUnitT;
}
