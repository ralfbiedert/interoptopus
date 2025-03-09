use crate::converter::{
    function_name_to_csharp_name, pattern_to_native_in_signature, to_typespecifier_in_async_fn_rval, to_typespecifier_in_param, to_typespecifier_in_sync_fn_rval,
};
use crate::interop::docs::write_documentation;
use crate::{FunctionNameFlavor, Interop};
use interoptopus::lang::c::{AsyncRval, CType, Function, PrimitiveType};
use interoptopus::patterns::service::ServiceDefinition;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, Error};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MethodType {
    Ctor,
    Dtor,
    Regular,
}

pub fn write_pattern_service(i: &Interop, w: &mut IndentWriter, class: &ServiceDefinition) -> Result<(), Error> {
    i.debug(w, "write_pattern_service")?;

    let context_type_name = class.the_type().rust_name();

    write_documentation(w, class.the_type().meta().documentation())?;
    indented!(w, r"{} partial class {} : IDisposable", i.visibility_types.to_access_modifier(), context_type_name)?;
    indented!(w, r"{{")?;
    w.indent();
    indented!(w, r"private IntPtr _context;")?;
    w.newline()?;
    indented!(w, r"private {}() {{}}", context_type_name)?;
    w.newline()?;

    for ctor in class.constructors() {
        write_documentation(w, ctor.meta().documentation())?;
        write_pattern_service_method(i, w, class, ctor, MethodType::Ctor, WriteFor::Code)?;
        w.newline()?;
    }

    // Dtor
    write_pattern_service_method(i, w, class, class.destructor(), MethodType::Dtor, WriteFor::Code)?;
    w.newline()?;

    for function in class.methods() {
        write_documentation(w, function.meta().documentation())?;
        write_pattern_service_method(i, w, class, function, MethodType::Regular, WriteFor::Code)?;
        write_service_method_overload(i, w, class, function, WriteFor::Code)?;
        w.newline()?;
    }

    indented!(w, r"public IntPtr Context => _context;")?;

    w.unindent();
    indented!(w, r"}}")?;
    w.newline()?;
    w.newline()?;

    Ok(())
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines, clippy::cognitive_complexity)]
pub fn write_pattern_service_method(
    i: &Interop,
    w: &mut IndentWriter,
    class: &ServiceDefinition,
    function: &Function,
    method_type: MethodType,
    write_for: WriteFor,
) -> Result<(), Error> {
    i.debug(w, "write_pattern_service_method")?;

    let common_prefix = class.common_prefix();
    let mut names = Vec::new();
    let mut to_invoke = Vec::new();
    let mut types = Vec::new();
    let async_rval = function.async_rval();

    // For every parameter except the first, figure out how we should forward
    // it to the invocation we perform.
    let skip_params = match method_type {
        MethodType::Ctor => 0,
        MethodType::Dtor => 1,
        MethodType::Regular => 1,
    };

    for p in function.signature().params().iter().skip(skip_params) {
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

    let fn_name = match method_type {
        MethodType::Ctor => function_name_to_csharp_name(function, FunctionNameFlavor::CSharpMethodNameWithoutClass(&common_prefix)),
        MethodType::Regular => function_name_to_csharp_name(function, FunctionNameFlavor::CSharpMethodNameWithoutClass(&common_prefix)),
        MethodType::Dtor => "Dispose".to_string(),
    };

    let mut static_prefix = "";

    let rval = match async_rval {
        // let fn_name = function_name_to_csharp_name(ctor, FunctionNameFlavor::CSharpMethodNameWithoutClass(&common_prefix));
        // let rval = format!("static {context_type_name}");
        AsyncRval::Sync(_) => match method_type {
            MethodType::Ctor => {
                static_prefix = "static ";
                class.the_type().rust_name().to_string()
            }
            MethodType::Regular => match function.signature().rval() {
                CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
                CType::Pattern(TypePattern::CStrPointer) => "string".to_string(),
                x => to_typespecifier_in_sync_fn_rval(x),
            },
            MethodType::Dtor => "void".to_string(),
        },
        AsyncRval::Async(CType::Pattern(TypePattern::Result(_))) => {
            names.pop();
            types.pop();
            to_invoke.pop();
            to_typespecifier_in_async_fn_rval(&async_rval)
        }
        AsyncRval::Async(_) => {
            names.pop();
            types.pop();
            to_invoke.pop();
            to_typespecifier_in_async_fn_rval(&async_rval)
        }
    };

    let method_to_invoke = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );

    // Assemble actual function call.
    let invoke_args = match method_type {
        MethodType::Ctor => {
            if to_invoke.is_empty() {
                String::new()
            } else {
                to_invoke.join(", ")
            }
        }
        MethodType::Dtor => "_context".to_string(),
        MethodType::Regular => {
            if to_invoke.is_empty() {
                "_context".to_string()
            } else {
                format!("_context, {}", to_invoke.join(", "))
            }
        }
    };

    let arg_tokens = names.iter().zip(types.iter()).map(|(n, t)| format!("{t} {n}")).collect::<Vec<_>>();
    let fn_call = format!(r"{}.{}({})", i.class, method_to_invoke, invoke_args);

    // Write signature.
    let signature = format!(r"public {static_prefix}{rval} {fn_name}({})", arg_tokens.join(", "));
    if write_for == WriteFor::Docs {
        indented!(w, r"{};", signature)?;
        return Ok(());
    }

    indented!(w, "{}", signature)?;
    indented!(w, r"{{")?;

    if matches!(method_type, MethodType::Ctor) {
        indented!(w, [()], r"var self = new {}();", class.the_type().rust_name())?;
    }

    // Determine return value behavior and write function call.
    match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(_)) if async_rval.is_sync() => {
            indented!(w, [()], r"{fn_call}.Ok();")?;
        }
        CType::Pattern(TypePattern::FFIErrorEnum(_)) if async_rval.is_async() => {
            indented!(w, [()], r"return {fn_call};")?;
        }
        CType::Pattern(TypePattern::CStrPointer) => {
            indented!(w, [()], r"var s = {fn_call};")?;
            indented!(w, [()], r"return Marshal.PtrToStringAnsi(s);")?;
        }
        CType::Primitive(PrimitiveType::Void) => {
            indented!(w, [()], r"{fn_call};",)?;
        }
        _ if matches!(method_type, MethodType::Ctor) => {
            indented!(w, [()], r"self._context = {fn_call}.Ok();")?;
        }
        _ if matches!(method_type, MethodType::Dtor) => {
            indented!(w, [()], r"{fn_call}.Ok();")?;
        }
        _ => {
            indented!(w, [()], r"return {fn_call};")?;
        }
    }

    if matches!(method_type, MethodType::Ctor) {
        indented!(w, [()], r"return self;")?;
    }

    indented!(w, r"}}")?;

    Ok(())
}

pub fn write_service_method_overload(i: &Interop, w: &mut IndentWriter, class: &ServiceDefinition, function: &Function, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_service_method_overload")?;

    if !i.has_overloadable(function.signature()) || function.async_rval().is_async() {
        return Ok(());
    }

    if write_for == WriteFor::Code {
        w.newline()?;
        write_documentation(w, function.meta().documentation())?;
    }

    write_common_service_method_overload(i, w, class, function, write_for)?;

    Ok(())
}

/// Writes common service overload code
pub fn write_common_service_method_overload(i: &Interop, w: &mut IndentWriter, class: &ServiceDefinition, function: &Function, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_common_service_method_overload")?;

    let mut names = Vec::new();
    let mut to_invoke = Vec::new();
    let mut types = Vec::new();

    let fn_name = function_name_to_csharp_name(function, FunctionNameFlavor::CSharpMethodNameWithoutClass(&class.common_prefix()));

    // Write checked method. These are "normal" methods that accept
    // common C# types.
    let rval = match function.signature().rval() {
        // CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
        CType::Pattern(TypePattern::CStrPointer) => "string".to_string(),
        _ => to_typespecifier_in_sync_fn_rval(function.signature().rval()),
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

    let signature = format!(r"public {} {}({})", rval, fn_name, arg_tokens.join(", "));
    if write_for == WriteFor::Docs {
        indented!(w, "{};", signature)?;
        return Ok(());
    }

    // Write signature.
    indented!(w, "{}", signature)?;
    indented!(w, r"{{")?;

    match function.signature().rval() {
        // CType::Pattern(TypePattern::FFIErrorEnum(_)) => {
        //     indented!(w, [()], r"{};", fn_call)?;
        // }
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
