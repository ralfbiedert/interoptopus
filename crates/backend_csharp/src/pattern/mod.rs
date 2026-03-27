//! C#-specific backend patterns for transparent exception mapping.
//!
//! The central type here is [`Try<T>`], a transparent exception converter for plugins.
//! When the C# backend encounters a function returning `Try<T>`, it automatically
//! captures exceptions and converts them into an [`ExceptionError`].
//!
//! # Example
//!
//! ```rust
//! # use interoptopus::plugin;
//! plugin!(MyPlugin {
//!     fn foo() -> Try<u32>;
//! });
//!
//! # fn foo(plugin: &MyPlugin) {
//! plugin.foo().ok()?;
//! # }
//! ```
//!
//! On the C# side the following plugin code is emitted that allows callees to just return a result.
//! Should an exception happen it will be automatically caught and converted.
//!
//! ```csharp
//! partial class Plugin : IPlugin
//! {
//!     public static uint Foo() { };
//! }
//! ```
//!
//! # Registering exceptions
//!
//! You can tell the builder which C# exception to map (besides `System.Exception`) with
//! [`DotnetLibrary::builder::exception()`]:
//!
//! ```rust
//! # use interoptopus::inventory::ForeignInventory;
//! # fn foo(inventory: &ForeignInventory) -> Result<(), Box<dyn std::error::Error>> {
//! let output = DotnetLibrary::builder(inventory)
//!     .exception(Exception::new("System.IO.FileNotFoundException"))
//!     .build()
//!     .process()?;
//! # }
//! ```
//!
//!
//! # `Try` - `Result` conversion
//!
//! Because `Try<T>` is effectively an `ffi::Result` it cannot be used with Rust's `?` operator
//! directly. [`TryExtension`] converts it into a standard result for ergonomic error propagation:
//!
//! ```rust
//! use interoptopus_csharp::pattern::TryExtension;
//! # fn foo() -> Try<u32> {}
//! let _ = foo(42).ok()?;
//! ```
use interoptopus::ffi;

mod exception;
pub use exception::{Exception, ExceptionError};

/// Return type that enables transparent C# exception mapping.
///
/// This is an alias for `ffi::Result<T, ExceptionError>`. When the C# backend
/// sees this as a function's return type it generates typed `catch` blocks for
/// each registered [`Exception`] and unwraps the `Result` in user-facing
/// interfaces so callers work with `T` directly.
///
/// See the [module-level docs](self) for the full story.
pub type Try<T> = ffi::Result<T, ExceptionError>;

/// Converts a [`Try<T>`] into a standard `Result<T, ExceptionError>`.
///
/// This is a workaround for the fact that `ffi::Result` is a 4-variant enum
/// (`Ok`, `Err`, `Panic`, `Null`) and therefore incompatible with the `?`
/// operator.  Both `Panic` and `Null` are mapped to
/// [`ExceptionError::unknown()`].
pub trait TryExtension<T> {
    /// Unwrap this `Try` into a standard `Result`.
    fn ok(self) -> Result<T, ExceptionError>;
}

impl<T> TryExtension<T> for Try<T> {
    fn ok(self) -> Result<T, ExceptionError> {
        match self {
            Self::Ok(t) => Ok(t),
            Self::Err(e) => Err(e),
            Self::Panic => Err(ExceptionError::unknown()),
            Self::Null => Err(ExceptionError::unknown()),
        }
    }
}

/// Checks whether a string looks like `System.Exception` or `System.IO.IOException`.
const fn assert_looks_like_exception_name(fqp: &str) {
    let bytes = fqp.as_bytes();
    assert!(!bytes.is_empty() && bytes[0].is_ascii_uppercase(), "Exceptions must look like `System.Exception` or `Company.System.OtherException`");
    let mut has_dot = false;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'.' {
            has_dot = true;
            assert!(i + 1 < bytes.len() && bytes[i + 1].is_ascii_uppercase(), "Exceptions must look like `System.Exception` or `Company.System.OtherException`");
        }
        i += 1;
    }
    assert!(has_dot, "fqp must contain at least one dot");
}
