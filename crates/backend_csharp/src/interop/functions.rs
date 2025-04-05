use crate::converter::{function_name, param_to_type, pattern_to_native_in_signature, rval_to_type_async, rval_to_type_sync};
use crate::interop::docs::write_documentation;
use crate::utils::sugared_return_type;
use crate::{FunctionNameFlavor, Interop};
use interoptopus::backend::{IndentWriter, WriteFor};
use interoptopus::lang::{Function, Primitive, SugaredReturnType, Type};
use interoptopus::pattern::TypePattern;
use interoptopus::{Error, indented};
use std::iter::zip;

pub fn write_functions(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for function in i.inventory.functions() {
        if i.should_emit_by_meta(function.meta()) {
            write_function(i, w, function, WriteFor::Code)?;
            w.newline()?;
        }
    }

    Ok(())
}

pub fn write_function(i: &Interop, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_function")?;
    if write_for == WriteFor::Code {
        write_documentation(w, function.meta().docs())?;
        write_function_annotation(i, w, function)?;
    }
    write_function_declaration(i, w, function, false)?;
    w.newline()?;
    write_function_overload(i, w, function, write_for)?;

    Ok(())
}

pub fn write_function_annotation(_i: &Interop, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
    indented!(w, r#"[LibraryImport(NativeLib, EntryPoint = "{}")]"#, function.name())?;

    if *function.signature().rval() == Type::Primitive(Primitive::Bool) {
        indented!(w, r"[return: MarshalAs(UnmanagedType.U1)]")?;
    }

    Ok(())
}

pub fn write_function_declaration(i: &Interop, w: &mut IndentWriter, function: &Function, has_body: bool) -> Result<(), Error> {
    i.debug(w, "write_function_declaration")?;

    let rval = rval_to_type_sync(function.signature().rval());
    let name = function_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );

    let mut params = Vec::new();

    let native = i.has_custom_marshalled_delegate(function.signature());
    let visibility = "public ";

    for p in function.signature().params() {
        let the_type = param_to_type(p.the_type());
        let name = p.name();

        if native && matches!(p.the_type(), Type::FnPointer(_) | Type::Pattern(TypePattern::NamedCallback(_))) {
            let suffix = if matches!(p.the_type(), Type::FnPointer(_)) { "_native" } else { "" };
            params.push(format!("{the_type}{suffix} {name}"));
        } else {
            params.push(format!("{the_type} {name}"));
        }
    }

    let line_ending = if has_body { "" } else { ";" };
    let partial = if has_body { "" } else { "partial " };

    i.inline_hint(w, 0)?;
    indented!(w, r"{}static {}{} {}({}){}", visibility, partial, rval, name, params.join(", "), line_ending)
}

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
pub fn write_function_overload(i: &Interop, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_function_overload")?;

    let has_overload = i.has_overloadable(function.signature());

    // If there is nothing to write, don't do it
    if !has_overload {
        i.debug(w, &format!("no overload for {}", function.name()))?;
        return Ok(());
    }

    let async_rval = sugared_return_type(function);

    let mut to_invoke = Vec::new();
    let mut to_wrap_name = Vec::new();
    let mut to_wrap_type = Vec::new();

    let raw_name = function_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );

    let rval = rval_to_type_async(&sugared_return_type(function));

    let mut params = Vec::new();
    for p in function.signature().params() {
        let name = p.name();
        let native = pattern_to_native_in_signature(i, p);
        let the_type = param_to_type(p.the_type());

        let mut fallback = || {
            if native.contains("ref ") {
                to_invoke.push(format!("ref {name}"));
            } else {
                to_invoke.push(name.to_string());
            }
        };
        match p.the_type() {
            Type::Pattern(TypePattern::NamedCallback(_)) => {
                to_wrap_name.push(name);
                to_wrap_type.push(param_to_type(p.the_type()));
                to_invoke.push(format!("{name}_wrapped"));
            }
            _ => fallback(),
        }

        params.push(format!("{native} {name}"));
    }

    if matches!(async_rval, SugaredReturnType::Async(_)) {
        params.pop();
        to_invoke.pop();
        to_invoke.push("_cb".to_string());
    }

    let signature = format!(r"public static unsafe {} {}({})", rval, raw_name, params.join(", "));
    if write_for == WriteFor::Docs {
        indented!(w, r"{};", signature)?;
        return Ok(());
    }

    if write_for == WriteFor::Code {
        write_documentation(w, function.meta().docs())?;
    }

    i.inline_hint(w, 0)?;
    indented!(w, "{}", signature)?;
    indented!(w, r"{{")?;

    if let SugaredReturnType::Async(ref x) = async_rval {
        let trampoline = format!("_trampoline{}", param_to_type(x));
        indented!(w, [()], r"var (_cb, _cs) = {trampoline}.NewCall();")?;
    }

    for (n, t) in zip(&to_wrap_name, &to_wrap_type) {
        indented!(w, [()], r"var {}_wrapped = new {}({});", n, t, n)?;
    }

    indented!(w, [()], r"try")?;
    indented!(w, [()], r"{{")?;

    let fn_name = function_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );

    let call = format!(r"{}({})", fn_name, to_invoke.join(", "));

    match function.signature().rval() {
        Type::Pattern(TypePattern::CStrPointer) => {
            indented!(w, [()()], r"var _s = {};", call)?;
            indented!(w, [()()], r"return Marshal.PtrToStringAnsi(_s);")?;
        }
        Type::Primitive(Primitive::Void) => {
            indented!(w, [()()], r"{};", call)?;
        }
        _ if matches!(async_rval, SugaredReturnType::Async(_)) => {
            indented!(w, [()()], r"{call}.AsOk();")?;
            indented!(w, [()()], r"return _cs;")?;
        }
        _ => {
            indented!(w, [()()], r"return {call};")?;
        }
    }
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"finally")?;
    indented!(w, [()], r"{{")?;
    for n in to_wrap_name {
        indented!(w, [()()], r"{}_wrapped.Dispose();", n)?;
    }
    indented!(w, [()], r"}}")?;

    if matches!(async_rval, SugaredReturnType::Async(_)) {
        indented!(w, [()], r"return _cs;")?;
    }

    indented!(w, r"}}")
}
