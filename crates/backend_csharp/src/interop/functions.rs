use crate::converter::{
    function_name_to_csharp_name, function_parameter_to_csharp_typename, function_rval_to_csharp_typename, get_slice_type, has_ffi_error_rval, to_slice_marshaller,
    to_typespecifier_in_param, to_typespecifier_in_rval,
};
use crate::interop::patterns::pattern_to_native_in_signature;
use crate::{FunctionNameFlavor, Interop};
use interoptopus::lang::c::{CType, Documentation, Function, FunctionSignature, Parameter, PrimitiveType};
use interoptopus::patterns::TypePattern;
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, Error};
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
        write_documentation(w, function.meta().documentation())?;
        write_function_annotation(i, w, function)?;
    }

    if i.has_custom_marshalled_delegate(function.signature()) {
        write_function_declaration(i, w, function, true, false)?;
        write_function_declaration(i, w, function, false, true)?;
        write_function_native_wrapper_body(i, w, function)?;
    } else {
        write_function_declaration(i, w, function, false, false)?;
    }
    write_function_overload(i, w, function, write_for)?;

    Ok(())
}

pub fn write_documentation(w: &mut IndentWriter, documentation: &Documentation) -> Result<(), Error> {
    for line in documentation.lines() {
        indented!(w, r"///{}", line)?;
    }

    Ok(())
}

pub fn write_function_annotation(_i: &Interop, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
    indented!(w, r#"[LibraryImport(NativeLib, EntryPoint = "{}")]"#, function.name())?;

    if *function.signature().rval() == CType::Primitive(PrimitiveType::Bool) {
        indented!(w, r"[return: MarshalAs(UnmanagedType.U1)]")?;
    }

    Ok(())
}

pub fn write_function_native_wrapper_body(i: &Interop, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
    let name = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );

    let ret = if matches!(function.signature().rval(), CType::Primitive(PrimitiveType::Void)) {
        ""
    } else {
        "return "
    };

    indented!(w, r"{{")?;
    indented!(w, [()], r"{ret}{name}(")?;
    w.indent();
    w.indent();
    write_function_wrapper_call_params(i, w, function.signature().params())?;
    w.unindent();
    w.unindent();
    indented!(w, [()], r");")?;
    indented!(w, r"}}")?;
    Ok(())
}

pub fn write_function_wrapper_call_params(i: &Interop, w: &mut IndentWriter, params: &[Parameter]) -> Result<(), Error> {
    for (index, p) in params.iter().enumerate() {
        let name = p.name();
        let sep = if index + 1 < params.len() { ", " } else { "" };
        match p.the_type() {
            CType::FnPointer(x) => {
                write_function_wrapper_call_delegate_param_body(i, w, name, sep, x.signature())?;
            }
            CType::Pattern(TypePattern::NamedCallback(x)) => {
                write_function_wrapper_call_delegate_param_body(i, w, name, sep, x.fnpointer().signature())?;
            }
            _ => {
                let modifier = match p.the_type() {
                    CType::ReadPointer(x) | CType::ReadWritePointer(x) => match &**x {
                        CType::Pattern(x) => match x {
                            TypePattern::Slice(_) => "ref ",
                            TypePattern::SliceMut(_) => "ref ",
                            _ => "",
                        },
                        _ => "",
                    },
                    _ => "",
                };
                indented!(w, r"{modifier}{name}{sep}")?;
            }
        }
    }
    Ok(())
}

pub fn write_function_wrapper_call_delegate_param_body(
    i: &Interop,
    w: &mut IndentWriter,
    delegate_param_name: &str,
    sep: &str,
    delegate_signature: &FunctionSignature,
) -> Result<(), Error> {
    let mut lambda_params = Vec::new();
    for p in delegate_signature.params() {
        lambda_params.push(format!("{} {}_native", i.to_native_callback_typespecifier(p.the_type()), p.name()));
    }

    indented!(w, r"({}) => {{", lambda_params.join(", "))?;

    // write custom marshalling for composite types
    for p in delegate_signature.params() {
        match p.the_type() {
            CType::Composite(x) => {
                if i.should_emit_marshaller_for_composite(x) {
                    indented!(
                        w,
                        [()],
                        r"var {0}_managed = {1}Marshaller.ConvertToManaged({0}_native);",
                        p.name(),
                        to_typespecifier_in_param(p.the_type())
                    )?;
                }
            }
            CType::Pattern(TypePattern::Slice(_) | TypePattern::SliceMut(_)) => {
                indented!(w, [()], r"var {}_marshaller = new {}.Marshaller();", p.name(), to_slice_marshaller(p.the_type()))?;
                indented!(w, [()], r"{0}_marshaller.FromUnmanaged({0}_native);", p.name())?;
                indented!(w, [()], r"var {0}_managed = {0}_marshaller.ToManaged();", p.name())?;
            }
            _ => {}
        }
    }

    let returns = !matches!(delegate_signature.rval(), CType::Primitive(PrimitiveType::Void));

    if returns {
        indented!(w, [()], r"var result = {}(", delegate_param_name)?;
    } else {
        indented!(w, [()], r"{}(", delegate_param_name)?;
    }

    for (index, p) in delegate_signature.params().iter().enumerate() {
        let sep = if index + 1 < delegate_signature.params().len() { ", " } else { "" };
        match p.the_type() {
            CType::Composite(x) => {
                if i.should_emit_marshaller_for_composite(x) {
                    indented!(w, [()()], r"{}_managed{}", p.name(), sep)?;
                }
            }
            CType::Pattern(TypePattern::Slice(_) | TypePattern::SliceMut(_)) => {
                indented!(w, [()()], r"{}_managed{}", p.name(), sep)?;
            }
            _ => {
                indented!(w, [()()], r"{}{}", p.name(), sep)?;
            }
        }
    }
    indented!(w, [()], r");")?;

    for p in delegate_signature.params() {
        if let CType::Pattern(TypePattern::Slice(_) | TypePattern::SliceMut(_)) = p.the_type() {
            if matches!(p.the_type(), CType::Pattern(TypePattern::SliceMut(_))) {
                indented!(w, [()], r"{}_marshaller.OnInvoked();", p.name())?;
            }
            indented!(w, [()], r"{}_marshaller.Free();", p.name())?;
        }
    }

    if returns {
        indented!(w, [()], r"return result;")?;
    }

    indented!(w, r"}}{sep}")?;

    Ok(())
}

pub fn write_function_declaration(i: &Interop, w: &mut IndentWriter, function: &Function, native: bool, has_body: bool) -> Result<(), Error> {
    let rval = function_rval_to_csharp_typename(function);
    let name = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );
    let visibility = if native { "private " } else { "public " };

    let mut params = Vec::new();
    for p in function.signature().params() {
        let the_type = function_parameter_to_csharp_typename(p);
        let name = p.name();

        if native && matches!(p.the_type(), CType::FnPointer(_) | CType::Pattern(TypePattern::NamedCallback(_))) {
            let suffix = if matches!(p.the_type(), CType::FnPointer(_)) { "_native" } else { "Native" };
            params.push(format!("{the_type}{suffix} {name}"));
        } else {
            params.push(format!("{the_type} {name}"));
        }
    }

    let line_ending = if has_body { "" } else { ";" };

    let partial = if has_body { "" } else { "partial " };

    indented!(w, r"{}static {}{} {}({}){}", visibility, partial, rval, name, params.join(", "), line_ending)
}

#[allow(clippy::too_many_lines)]
pub fn write_function_overload(i: &Interop, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
    let has_overload = i.has_overloadable(function.signature());
    let has_error_enum = has_ffi_error_rval(function.signature());

    // If there is nothing to write, don't do it
    if !has_overload && !has_error_enum {
        return Ok(());
    }

    let mut to_pin_name = Vec::new();
    let mut to_pin_slice_type = Vec::new();
    let mut to_invoke = Vec::new();
    let mut to_wrap_delegates = Vec::new();
    let mut to_wrap_delegate_types = Vec::new();

    let raw_name = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );
    let this_name = if has_error_enum && !has_overload {
        format!("{raw_name}_checked")
    } else {
        raw_name
    };

    let rval = match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(_)) => "void".to_string(),
        CType::Pattern(TypePattern::CStrPointer) => "string".to_string(),
        _ => to_typespecifier_in_rval(function.signature().rval()),
    };

    let mut params = Vec::new();
    for p in function.signature().params() {
        let name = p.name();
        let native = pattern_to_native_in_signature(p);
        let the_type = function_parameter_to_csharp_typename(p);

        let mut fallback = || {
            if native.contains("out ") {
                to_invoke.push(format!("out {name}"));
            } else if native.contains("ref ") {
                to_invoke.push(format!("ref {name}"));
            } else {
                to_invoke.push(name.to_string());
            }
        };

        match p.the_type() {
            CType::Pattern(TypePattern::Slice(x) | TypePattern::SliceMut(x)) => {
                if !i.should_emit_marshaller(&get_slice_type(x)) {
                    to_pin_name.push(name);
                    to_pin_slice_type.push(the_type);
                    to_invoke.push(format!("{name}_slice"));
                }
            }
            CType::Pattern(TypePattern::NamedCallback(callback)) => match callback.fnpointer().signature().rval() {
                CType::Pattern(TypePattern::FFIErrorEnum(_)) if i.work_around_exception_in_callback_no_reentry => {
                    to_wrap_delegates.push(name);
                    to_wrap_delegate_types.push(to_typespecifier_in_param(p.the_type()));
                    to_invoke.push(format!("{name}_safe_delegate.Call"));
                }
                _ => fallback(),
            },
            CType::ReadPointer(x) | CType::ReadWritePointer(x) => match &**x {
                CType::Pattern(x) => match x {
                    TypePattern::Slice(x) => {
                        if !i.should_emit_marshaller(&get_slice_type(x)) {
                            to_pin_name.push(name);
                            to_pin_slice_type.push(the_type.replace("ref ", ""));
                            to_invoke.push(format!("ref {name}_slice"));
                        }
                    }
                    TypePattern::SliceMut(x) => {
                        if !i.should_emit_marshaller(&get_slice_type(x)) {
                            to_pin_name.push(name);
                            to_pin_slice_type.push(the_type.replace("ref ", ""));
                            to_invoke.push(format!("ref {name}_slice"));
                        }
                    }
                    _ => fallback(),
                },
                _ => fallback(),
            },
            _ => fallback(),
        }

        params.push(format!("{native} {name}"));
    }

    let signature = format!(r"public static unsafe {} {}({})", rval, this_name, params.join(", "));
    if write_for == WriteFor::Docs {
        indented!(w, r"{};", signature)?;
        return Ok(());
    }

    w.newline()?;

    if write_for == WriteFor::Code {
        write_documentation(w, function.meta().documentation())?;
    }

    indented!(w, "{}", signature)?;
    indented!(w, r"{{")?;

    for (name, ty) in zip(&to_wrap_delegates, &to_wrap_delegate_types) {
        indented!(w, [()], r"var {}_safe_delegate = new {}ExceptionSafe({});", name, ty, name)?;
    }

    if !to_pin_name.is_empty() {
        for (pin_var, slice_struct) in to_pin_name.iter().zip(to_pin_slice_type.iter()) {
            indented!(w, [()], r"fixed (void* ptr_{} = {})", pin_var, pin_var)?;
            indented!(w, [()], r"{{")?;
            indented!(
                w,
                [()()],
                r"var {}_slice = new {}(new IntPtr(ptr_{}), (ulong) {}.Length);",
                pin_var,
                slice_struct,
                pin_var,
                pin_var
            )?;
            w.indent();
        }
    }

    let fn_name = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );
    let call = format!(r"{}({});", fn_name, to_invoke.join(", "));

    write_function_overloaded_invoke_with_error_handling(i, w, function, &call, to_wrap_delegates.as_slice())?;

    if !to_pin_name.is_empty() {
        for _ in &to_pin_name {
            w.unindent();
            indented!(w, [()], r"}}")?;
        }
    }

    indented!(w, r"}}")
}

/// Writes common error handling based on a call's return type.
pub fn write_function_overloaded_invoke_with_error_handling(
    _i: &Interop,
    w: &mut IndentWriter,
    function: &Function,
    fn_call: &str,
    rethrow_delegates: &[&str],
) -> Result<(), Error> {
    match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
            indented!(w, [()], r"var rval = {};", fn_call)?;
            for name in rethrow_delegates {
                indented!(w, [()], r"{}_safe_delegate.Rethrow();", name)?;
            }
            indented!(w, [()], r"if (rval != {}.{})", e.the_enum().rust_name(), e.success_variant().name())?;
            indented!(w, [()], r"{{")?;
            indented!(w, [()()], r"throw new InteropException<{}>(rval);", e.the_enum().rust_name())?;
            indented!(w, [()], r"}}")?;
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

    Ok(())
}
