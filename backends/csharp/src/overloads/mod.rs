//! Overloads provide additional methods for .NET and Unity.
//!
//! We recommend to **always add the [`DotNet`](crate::overloads::DotNet) writer** (unless you want to handle overloads yourself), **even when targeting Unity**.
//! The [`Unity`](crate::overloads::Unity) writer should only be added when Unity support is needed, and will require a [`Config::use_unsafe`](crate::Config::use_unsafe)
//! setting of [`Unsafe::UnsafeKeyword`](crate::Unsafe::UnsafeKeyword) or higher.
//!
//! # Background
//!
//! Normally the C# backend only emits simple signatures. These mostly contain primitive types, structs, `IntPtr`, and all of the above mixed with `ref` our `out`:
//!
//!
//! ```csharp
//! public static extern uint my_function(Sliceu32 slice);
//! ```
//!
//! Overload writers provide additional convenience methods. For example the [`DotNet`](crate::overloads::DotNet) writer might add:
//!
//! ```csharp
//! public static uint my_function(uint[] slice) { ... }
//! ```
//!
//! While the [`Unity`](crate::overloads::Unity) writer could add:
//!
//! ```csharp
//! public static uint my_function(NativeArray<uint> slice) { ... }
//! ```
//!
//!
//! # Example
//!
//! Overloads are passed to a [`Generator`](crate::Generator) like this:
//!
//! ```
//! # use interoptopus::util::NamespaceMappings;
//! # use interoptopus::{Error, Interop};
//!
//! #[test]
//! fn bindings_csharp() -> Result<(), Error> {
//!     use interoptopus_backend_csharp::{Generator, Config};
//!     use interoptopus_backend_csharp::overloads::{Unity, DotNet};
//!
//!     let config = Config::default();
//!
//!     Generator::new(config, example_library_ffi::my_inventory())
//!         .add_overload_writer(Unity::new())
//!         .add_overload_writer(DotNet::new())
//!         .write_file("bindings/csharp/Interop.cs")?;
//!
//!     Ok(())
//! }
//!

use interoptopus::lang::c::{CType, Function, PrimitiveType};
use interoptopus::patterns::service::Service;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

mod dotnet;
mod unity;

use crate::{CSharpTypeConverter, Config};
pub use dotnet::DotNet;
use interoptopus::patterns::TypePattern;
pub use unity::Unity;

#[doc(hidden)]
pub struct Helper<'a> {
    pub config: &'a Config,
    pub converter: &'a dyn CSharpTypeConverter,
}

#[doc(hidden)]
pub trait OverloadWriter {
    fn write_imports(&self, w: &mut IndentWriter, h: Helper) -> Result<(), Error>;

    fn write_function_overload(&self, w: &mut IndentWriter, h: Helper, function: &Function) -> Result<(), Error>;

    fn write_service_method_overload(&self, w: &mut IndentWriter, h: Helper, class: &Service, function: &Function, fn_pretty: &str) -> Result<(), Error>;

    fn write_pattern_slice_overload(&self, w: &mut IndentWriter, h: Helper, context_type_name: &str, type_string: &str) -> Result<(), Error>;

    fn write_pattern_slice_unsafe_copied_fragment(&self, w: &mut IndentWriter, h: Helper, type_string: &str) -> Result<(), Error>;
}

#[rustfmt::skip]
fn write_function_overloaded_invoke_with_error_handling(w: &mut IndentWriter, function: &Function, fn_call: &str) -> Result<(), Error> {

    match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
            indented!(w, [_], r#"var rval = {};"#, fn_call)?;
            indented!(w, [_], r#"if (rval != {}.{})"#, e.the_enum().rust_name(), e.success_variant().name())?;
            indented!(w, [_], r#"{{"#)?;
            indented!(w, [_ _], r#"throw new Exception($"Something went wrong: {{rval}}");"#)?;
            indented!(w, [_], r#"}}"#)?;
        }
        CType::Primitive(PrimitiveType::Void) => {
            indented!(w, [_], r#"{};"#, fn_call)?;
        }
        _ => {
            indented!(w, [_], r#"return {};"#, fn_call)?;
        }
    }

    Ok(())
}
