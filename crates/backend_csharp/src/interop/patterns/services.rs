use crate::converter::{function_name_to_csharp_name, pattern_to_native_in_signature, to_typespecifier_in_param, to_typespecifier_in_rval};
use crate::interop::functions::write_documentation;
use crate::{FunctionNameFlavor, Interop};
use interoptopus::lang::c::{CType, Function, PrimitiveType};
use interoptopus::patterns::service::ServiceDefinition;
use interoptopus::patterns::TypePattern;
use interoptopus::util::longest_common_prefix;
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, Error};

pub fn write_pattern_service(i: &Interop, w: &mut IndentWriter, class: &ServiceDefinition) -> Result<(), Error> {
    i.debug(w, "write_pattern_service")?;
    let mut all_functions = class.constructors().to_vec();
    all_functions.extend_from_slice(class.methods());
    all_functions.push(class.destructor().clone());

    let context_type_name = class.the_type().rust_name();
    let common_prefix = longest_common_prefix(&all_functions);

    write_documentation(w, class.the_type().meta().documentation())?;
    indented!(w, r"{} partial class {} : IDisposable", i.visibility_types.to_access_modifier(), context_type_name)?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"private IntPtr _context;")?;
    w.newline()?;
    indented!(w, r"private {}() {{}}", context_type_name)?;
    w.newline()?;

    for ctor in class.constructors() {
        // Ctor
        let fn_name = function_name_to_csharp_name(ctor, FunctionNameFlavor::CSharpMethodNameWithoutClass(&common_prefix));
        let rval = format!("static {context_type_name}");

        write_documentation(w, ctor.meta().documentation())?;
        write_pattern_service_method(i, w, class, ctor, &rval, &fn_name, true, true, WriteFor::Code)?;
        w.newline()?;
    }

    // Dtor
    write_pattern_service_method(i, w, class, class.destructor(), "void", "Dispose", true, false, WriteFor::Code)?;
    w.newline()?;

    for function in class.methods() {
        // Main function
        let fn_name = function_name_to_csharp_name(function, FunctionNameFlavor::CSharpMethodNameWithoutClass(&common_prefix));

        // Write checked method. These are "normal" methods that accept
        // common C# types.
        let rval = match function.signature().rval() {
            CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
            CType::Pattern(TypePattern::CStrPointer) => "string".to_string(),
            _ => to_typespecifier_in_rval(function.signature().rval()),
        };
        write_documentation(w, function.meta().documentation())?;
        write_pattern_service_method(i, w, class, function, &rval, &fn_name, false, false, WriteFor::Code)?;
        write_service_method_overload(i, w, class, function, &fn_name, WriteFor::Code)?;

        w.newline()?;
    }

    indented!(w, r"public IntPtr Context => _context;")?;

    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;
    w.newline()?;

    Ok(())
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn write_pattern_service_method(
    i: &Interop,
    w: &mut IndentWriter,
    class: &ServiceDefinition,
    function: &Function,
    rval: &str,
    fn_name: &str,
    write_contxt_by_ref: bool,
    is_ctor: bool,
    write_for: WriteFor,
) -> Result<(), Error> {
    i.debug(w, "write_pattern_service_method")?;

    let async_rval = function.async_rval();

    let mut rval = rval.to_string();
    let mut names = Vec::new();
    let mut to_invoke = Vec::new();
    let mut types = Vec::new();
    // let mut to_wrap_delegates = Vec::new();
    // let mut to_wrap_delegate_types = Vec::new();

    // For every parameter except the first, figure out how we should forward
    // it to the invocation we perform.
    for p in function.signature().params().iter().skip(1) {
        let name = p.name();

        // If we call the checked function we want to resolve a `SliceU8` to a `byte[]`,
        // but if we call the unchecked version we want to keep that `Sliceu8` in our signature.
        let mut native = to_typespecifier_in_param(p.the_type());

        match p.the_type() {
            CType::Pattern(TypePattern::NamedCallback(callback)) => {
                let _ = callback.fnpointer().signature().rval();
                if native.contains("out ") {
                    to_invoke.push(format!("out {name}"));
                } else if native.contains("ref ") {
                    to_invoke.push(format!("ref {name}"));
                } else {
                    to_invoke.push(name.to_string());
                }
            }
            CType::Pattern(TypePattern::Utf8String(_)) => {
                native = "string".to_string();
                to_invoke.push(name.to_string());
            }
            _ => {
                // Forward `ref` and `out` accordingly.
                if native.contains("out ") {
                    to_invoke.push(format!("out {name}"));
                } else if native.contains("ref ") {
                    to_invoke.push(format!("ref {name}"));
                } else {
                    to_invoke.push(name.to_string());
                }
            }
        }

        names.push(name);
        types.push(native);
    }

    if let Some(x) = async_rval {
        names.pop();
        types.pop();
        to_invoke.pop();
        rval = format!("Task<{}>", to_typespecifier_in_param(x));
    }

    let method_to_invoke = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );
    let extra_args = if to_invoke.is_empty() {
        String::new()
    } else {
        format!(", {}", to_invoke.join(", "))
    };

    // Assemble actual function call.
    let context = if write_contxt_by_ref {
        if is_ctor {
            "ref self._context"
        } else {
            "ref _context"
        }
    } else {
        "_context"
    };
    let arg_tokens = names.iter().zip(types.iter()).map(|(n, t)| format!("{t} {n}")).collect::<Vec<_>>();
    let fn_call = format!(r"{}.{}({}{})", i.class, method_to_invoke, context, extra_args);

    // Write signature.
    let signature = format!(r"public {} {}({})", rval, fn_name, arg_tokens.join(", "));
    if write_for == WriteFor::Docs {
        indented!(w, r"{};", signature)?;
        return Ok(());
    }

    indented!(w, "{}", signature)?;
    indented!(w, r"{{")?;

    if is_ctor {
        indented!(w, [()], r"var self = new {}();", class.the_type().rust_name())?;
    }

    // Determine return value behavior and write function call.
    match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(e)) if async_rval.is_none() => {
            indented!(w, [()], r"var rval = {};", fn_call)?;
            // for name in to_wrap_delegates {
            //     indented!(w, [()], r"{}_safe_delegate.Rethrow();", name)?;
            // }
            indented!(w, [()], r"if (rval != {}.{})", e.the_enum().rust_name(), e.success_variant().name())?;
            indented!(w, [()], r"{{")?;
            indented!(w, [()()], r"throw new InteropException<{}>(rval);", e.the_enum().rust_name())?;
            indented!(w, [()], r"}}")?;
        }
        CType::Pattern(TypePattern::FFIErrorEnum(_)) if async_rval.is_some() => {
            indented!(w, [()], r"return {};", fn_call)?;
        }
        CType::Pattern(TypePattern::CStrPointer) => {
            indented!(w, [()], r"var s = {};", fn_call)?;
            indented!(w, [()], r"return Marshal.PtrToStringAnsi(s);")?;
        }
        CType::Primitive(PrimitiveType::Void) => {
            indented!(w, [()], r"{};", fn_call)?;
        }
        _ => {
            indented!(w, [()], r"return {};", fn_call)?;
        }
    }

    if is_ctor {
        indented!(w, [()], r"return self;")?;
    }

    indented!(w, r"}}")?;

    Ok(())
}

pub fn write_service_method_overload(
    i: &Interop,
    w: &mut IndentWriter,
    _class: &ServiceDefinition,
    function: &Function,
    fn_pretty: &str,
    write_for: WriteFor,
) -> Result<(), Error> {
    i.debug(w, "write_service_method_overload")?;

    if !i.has_overloadable(function.signature()) || function.async_rval().is_some() {
        return Ok(());
    }

    if write_for == WriteFor::Code {
        w.newline()?;
        write_documentation(w, function.meta().documentation())?;
    }

    write_common_service_method_overload(i, w, function, fn_pretty, write_for)?;

    Ok(())
}

/// Writes common service overload code
pub fn write_common_service_method_overload(i: &Interop, w: &mut IndentWriter, function: &Function, fn_pretty: &str, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_common_service_method_overload")?;

    let mut names = Vec::new();
    let mut to_invoke = Vec::new();
    let mut types = Vec::new();

    // Write checked method. These are "normal" methods that accept
    // common C# types.
    let rval = match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
        CType::Pattern(TypePattern::CStrPointer) => "string".to_string(),
        _ => to_typespecifier_in_rval(function.signature().rval()),
    };

    // For every parameter except the first, figure out how we should forward
    // it to the invocation we perform.
    for p in function.signature().params().iter().skip(1) {
        let name = p.name();

        // If we call the checked function we want to resolve a `SliceU8` to a `byte[]`,
        // but if we call the unchecked version we want to keep that `Sliceu8` in our signature.
        // let native = i.to_typespecifier_in_param(p.the_type());
        let native = pattern_to_native_in_signature(i, p);

        // Forward `ref` and `out` accordingly.
        if native.contains("out ") {
            to_invoke.push(format!("out {name}"));
        } else if native.contains("ref ") {
            to_invoke.push(format!("ref {name}"));
        } else {
            to_invoke.push(name.to_string());
        }

        names.push(name);
        types.push(native);
    }

    let method_to_invoke = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );
    let extra_args = if to_invoke.is_empty() {
        String::new()
    } else {
        format!(", {}", to_invoke.join(", "))
    };

    // Assemble actual function call.
    let context = "_context";
    let arg_tokens = names.iter().zip(types.iter()).map(|(n, t)| format!("{t} {n}")).collect::<Vec<_>>();
    let fn_call = format!(r"{}.{}({}{})", i.class, method_to_invoke, context, extra_args);

    let signature = format!(r"public {} {}({})", rval, fn_pretty, arg_tokens.join(", "));
    if write_for == WriteFor::Docs {
        indented!(w, "{};", signature)?;
        return Ok(());
    }

    // Write signature.
    indented!(w, "{}", signature)?;
    indented!(w, r"{{")?;

    match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(_)) => {
            indented!(w, [()], r"{};", fn_call)?;
        }
        CType::Primitive(PrimitiveType::Void) => {
            indented!(w, [()], r"{};", fn_call)?;
        }
        _ => {
            indented!(w, [()], r"return {};", fn_call)?;
        }
    }

    indented!(w, r"}}")?;

    Ok(())
}
