//! For return enums with defined `Ok` variants; may translate to exceptions if not met.
//!
//!
//! # Examples
//!
//! Functions returning a [`FFIError`] might receive special treatment in backends supporting
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
use crate::lang::{Composite, Documentation, Enum, Layout, Meta, Primitive, Representation, Type, Variant};
use crate::lang::{TypeInfo, VariantKind};
use crate::pattern::TypePattern;
use std::any::Any;
use std::fmt::Debug;

/// A trait you should implement for enums that signal errors in FFI calls.
///
/// Once implemented, the enum can be used in [services](crate::pattern::service) to automatically
/// convert `Result<(), E>` types to FFI enums.
///
/// # Example
///
/// ```
/// use interoptopus::pattern::result::FFIError;
/// use interoptopus::ffi_type;
///
/// // Some Error used in your application.
/// pub enum Error {
///     Bad,
/// }
///
/// // The error FFI users should see
/// #[ffi_type(error)]
/// #[derive(PartialOrd, PartialEq)]
/// enum MyFFIError {
///     Ok = 0,
///     NullPassed = 1,
///     Panic = 2,
///     OtherError = 3,
/// }
///
/// // Gives special meaning to some of your error variants.
/// impl FFIError for MyFFIError {
///     const SUCCESS: Self = Self::Ok;
///     const NULL: Self = Self::NullPassed;
///     const PANIC: Self = Self::Panic;
/// }
///
/// // How to map an `Error` to an `MyFFIError`.
/// impl From<Error> for MyFFIError {
///     fn from(x: Error) -> Self {
///         match x {
///             Error::Bad => Self::OtherError,
///         }
///     }
/// }
///
/// ```
pub trait FFIError: PartialEq + Sized {
    /// The variant to return when everything went OK, usually the variant with value `0`.
    const SUCCESS: Self;
    /// Signals a null pointer was passed where an actual element was needed.
    const NULL: Self;
    /// This can indicate one of two things:
    /// - Returned from Rust function this indicates a panic. Once this is observed no further calls
    ///   should be attempted.
    /// - Returned from a callback, this indicates "an unusual code flow like a panic" happened
    ///   in hosting process (e.g., some callback code threw an exception). In that case
    ///   you should probably attempt to return early and indicate an error.
    const PANIC: Self;

    // fn ok(self) -> Result<(), E>;
}

/// Internal helper derived for enums that are an [`FFIError`].
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FFIErrorEnum {
    the_enum: Enum,
    success_variant: Variant,
    panic_variant: Variant,
}

impl FFIErrorEnum {
    #[must_use]
    pub const fn new(the_enum: Enum, success_variant: Variant, panic_variant: Variant) -> Self {
        Self { the_enum, success_variant, panic_variant }
    }

    #[must_use]
    pub const fn the_enum(&self) -> &Enum {
        &self.the_enum
    }

    #[must_use]
    pub const fn success_variant(&self) -> &Variant {
        &self.success_variant
    }

    #[must_use]
    pub const fn panic_variant(&self) -> &Variant {
        &self.panic_variant
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FFIResultType {
    composite_type: Composite,
}

impl FFIResultType {
    #[must_use]
    pub const fn new(composite_type: Composite) -> Self {
        Self { composite_type }
    }

    #[must_use]
    pub fn t(&self) -> &Type {
        self.composite_type.fields()[0].the_type()
    }

    #[must_use]
    pub fn e(&self) -> &Type {
        self.composite_type.fields()[1].the_type()
    }

    #[must_use]
    pub const fn composite(&self) -> &Composite {
        &self.composite_type
    }

    #[must_use]
    pub fn meta(&self) -> &Meta {
        self.composite_type.meta()
    }
}

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

pub trait FFIResultAsPtr {
    type AsPtr;
}

pub trait FFIResultAsUnitT {
    type AsUnitT;
}

#[repr(u32)]
#[derive(Debug)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
    Panic,
    Null,
}

impl<T, E> FFIResultAsPtr for Result<T, E> {
    type AsPtr = Result<*const T, E>;
}

impl<T, E> FFIResultAsUnitT for Result<T, E> {
    type AsUnitT = Result<(), E>;
}

impl<T, E> Result<T, E>
where
    T: TypeInfo,
    E: TypeInfo + FFIError,
{
    // pub const fn ok(t: T) -> Self {
    //     Self { t: MaybeUninit::new(t), err: E::SUCCESS }
    // }
    //
    // pub const fn err(err: E) -> Self {
    //     Self { t: MaybeUninit::uninit(), err }
    // }
    //
    // #[must_use]
    // pub const fn panic() -> Self {
    //     Self { t: MaybeUninit::uninit(), err: E::PANIC }
    // }
    //
    // #[must_use]
    // pub const fn null() -> Self {
    //     Self { t: MaybeUninit::uninit(), err: E::NULL }
    // }

    #[must_use]
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            Self::Err(_) => false,
            Self::Panic => false,
            Self::Null => false,
        }
    }

    pub fn unwrap(self) -> T {
        if let Self::Ok(t) = self {
            t
        } else {
            panic!("Called `unwrap` on an `FFIResult` that is not `Ok`.")
        }
    }

    pub fn unwrap_err(&self) -> &E {
        if let Self::Err(err) = self {
            err
        } else {
            panic!("Called `unwrap_err` on an `FFIResult` that is not `Err`.")
        }
    }
}

impl<T, E> Result<T, E>
where
    T: TypeInfo,
    E: TypeInfo,
{
    // pub fn error(err: E) -> Self {
    //     Self { t: MaybeUninit::uninit(), err }
    // }
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E>
where
    T: TypeInfo,
    E: TypeInfo + FFIError,
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
    E: TypeInfo + FFIError,
{
    fn type_info() -> Type {
        let doc_t = Documentation::from_line("Element if err is `Ok`.");
        let doc_err = Documentation::from_line("Error value.");

        let variants = vec![
            Variant::new("Ok".to_string(), VariantKind::Typed(0, Box::new(T::type_info())), doc_t),
            Variant::new("Err".to_string(), VariantKind::Typed(1, Box::new(E::type_info())), doc_err),
            Variant::new("Panic".to_string(), VariantKind::Unit(2), Documentation::new()),
            Variant::new("Null".to_string(), VariantKind::Unit(3), Documentation::new()),
        ];

        let doc = Documentation::from_line("Result that contains value or an error.");
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_namespace_documentation(T::type_info().namespace().map_or_else(String::new, std::convert::Into::into), doc);
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

impl<T, E: FFIError> IntoFFIResult for Result<T, E> {
    type FFIResult = Self;
}

///
/// At some point we want to get rid of these once `Try` ([try_trait_v2](https://github.com/rust-lang/rust/issues/84277)) stabilizes.
pub fn result_to_ffi<T: TypeInfo, E: TypeInfo + crate::pattern::result::FFIError>(f: impl FnOnce() -> std::result::Result<T, E>) -> Result<T, E> {
    match f() {
        std::result::Result::Ok(x) => Result::Ok(x),
        std::result::Result::Err(e) => Result::Err(e),
    }
}

/// At some point we want to get rid of these once `Try` ([try_trait_v2](https://github.com/rust-lang/rust/issues/84277)) stabilizes.
pub async fn result_to_ffi_async<T: TypeInfo, E: TypeInfo>(f: impl std::ops::AsyncFnOnce() -> std::result::Result<T, E>) -> Result<T, E> {
    match f().await {
        std::result::Result::Ok(x) => Result::Ok(x),
        std::result::Result::Err(e) => Result::Err(e),
    }
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
