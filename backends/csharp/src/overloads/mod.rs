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

use interoptopus::lang::c::{CType, CompositeType, Documentation, Field, Function, Parameter, PrimitiveType};
use interoptopus::patterns::service::Service;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

mod dotnet;
mod unity;

use crate::{CSharpTypeConverter, Config};
use crate::converter::FunctionNameFlavor;
pub use dotnet::DotNet;
pub use unity::Unity;

#[doc(hidden)]
pub struct Helper<'a> {
    pub config: &'a Config,
    pub converter: &'a dyn CSharpTypeConverter,
}

#[doc(hidden)]
pub trait OverloadWriter {
    fn write_imports(&self, w: &mut IndentWriter, h: Helper) -> Result<(), Error>;

    fn write_field_decorators(&self, w: &mut IndentWriter, h: Helper, field: &Field, strct: &CompositeType) -> Result<(), Error>;

    fn write_function_overload(&self, w: &mut IndentWriter, h: Helper, function: &Function) -> Result<(), Error>;

    fn write_service_method_overload(&self, w: &mut IndentWriter, h: Helper, class: &Service, function: &Function, fn_pretty: &str) -> Result<(), Error>;

    fn write_pattern_slice_overload(&self, w: &mut IndentWriter, h: Helper, context_type_name: &str, type_string: &str) -> Result<(), Error>;

    fn write_pattern_slice_mut_overload(&self, w: &mut IndentWriter, h: Helper, context_type_name: &str, type_string: &str) -> Result<(), Error>;

    fn write_pattern_slice_unsafe_copied_fragment(&self, w: &mut IndentWriter, h: Helper, type_string: &str) -> Result<(), Error>;

    fn write_documentation(&self, w: &mut IndentWriter, documentation: &Documentation) -> Result<(), Error> {
        for line in documentation.lines() {
            indented!(w, r#"///{}"#, line)?;
        }

        Ok(())
    }
}

/// Writes common error handling based on a call's return type.
#[rustfmt::skip]
fn write_function_overloaded_invoke_with_error_handling(w: &mut IndentWriter, function: &Function, fn_call: &str) -> Result<(), Error> {

    match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
            indented!(w, [_], r#"var rval = {};"#, fn_call)?;
            indented!(w, [_], r#"if (rval != {}.{})"#, e.the_enum().rust_name(), e.success_variant().name())?;
            indented!(w, [_], r#"{{"#)?;
            indented!(w, [_ _], r#"throw new InteropException<{}>(rval);"#, e.the_enum().rust_name())?;
            indented!(w, [_], r#"}}"#)?;
        }
        CType::Pattern(TypePattern::AsciiPointer) => {
            indented!(w, [_], r#"var s = {};"#, fn_call)?;
            indented!(w, [_], r#"return Marshal.PtrToStringAnsi(s);"#)?;
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

/// Writes common service overload code
fn write_common_service_method_overload<FPatternMap: Fn(&Helper, &Parameter) -> String>(
    w: &mut IndentWriter,
    h: Helper,
    function: &Function,
    fn_pretty: &str,
    f_pattern: FPatternMap,
) -> Result<(), Error> {
    let mut names = Vec::new();
    let mut to_invoke = Vec::new();
    let mut types = Vec::new();

    // Write checked method. These are "normal" methods that accept
    // common C# types.
    let rval = match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
        CType::Pattern(TypePattern::AsciiPointer) => "string".to_string(),
        _ => h.converter.to_typespecifier_in_rval(function.signature().rval()),
    };

    // For every parameter except the first, figure out how we should forward
    // it to the invocation we perform.
    for p in function.signature().params().iter().skip(1) {
        let name = p.name();

        // If we call the checked function we want to resolve a `SliceU8` to a `byte[]`,
        // but if we call the unchecked version we want to keep that `Sliceu8` in our signature.
        // let native = self.to_typespecifier_in_param(p.the_type());
        let native = f_pattern(&h, p);

        // Forward `ref` and `out` accordingly.
        if native.contains("out ") {
            to_invoke.push(format!("out {}", name.to_string()));
        } else if native.contains("ref ") {
            to_invoke.push(format!("ref {}", name.to_string()));
        } else {
            to_invoke.push(name.to_string());
        }

        names.push(name);
        types.push(native);
    }

    let method_to_invoke = h.converter.function_name_to_csharp_name(function, match h.config.rename_symbols {
        true => FunctionNameFlavor::CSharpMethodNameWithClass,
        false => FunctionNameFlavor::RawFFIName
    });
    let extra_args = if to_invoke.is_empty() {
        "".to_string()
    } else {
        format!(", {}", to_invoke.join(", "))
    };

    // Assemble actual function call.
    let context = "_context";
    let arg_tokens = names.iter().zip(types.iter()).map(|(n, t)| format!("{} {}", t, n)).collect::<Vec<_>>();
    let fn_call = format!(r#"{}.{}({}{})"#, h.config.class, method_to_invoke, context, extra_args);

    // Write signature.
    indented!(w, r#"public {} {}({})"#, rval, fn_pretty, arg_tokens.join(", "))?;
    indented!(w, r#"{{"#)?;

    match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(_)) => {
            indented!(w, [_], r#"{};"#, fn_call)?;
        }
        CType::Primitive(PrimitiveType::Void) => {
            indented!(w, [_], r#"{};"#, fn_call)?;
        }
        _ => {
            indented!(w, [_], r#"return {};"#, fn_call)?;
        }
    }

    indented!(w, r#"}}"#)?;

    Ok(())
}
