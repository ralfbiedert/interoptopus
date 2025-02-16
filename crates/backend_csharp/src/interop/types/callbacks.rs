use crate::converter::{function_parameter_to_csharp_typename, named_callback_to_typename, to_typespecifier_in_param, to_typespecifier_in_rval};
use crate::interop::types::fnptrs::write_type_definition_fn_pointer_annotation;
use crate::Interop;
use interoptopus::lang::c::CType;
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_type_definition_named_callback(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    i.debug(w, "write_type_definition_named_callback")?;
    write_type_definition_fn_pointer_annotation(w, the_type.fnpointer())?;
    write_type_definition_named_callback_body(i, w, the_type)?;
    write_callback_overload(i, w, the_type)?;
    Ok(())
}

pub fn write_callback_overload(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    if !i.work_around_exception_in_callback_no_reentry {
        return Ok(());
    }

    let CType::Pattern(TypePattern::FFIErrorEnum(ffi_error)) = the_type.fnpointer().signature().rval() else {
        return Ok(());
    };

    let name = format!("{}ExceptionSafe", the_type.name());
    let rval = to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
    let mut function_signature = Vec::new();
    let mut function_param_names = Vec::new();

    for p in the_type.fnpointer().signature().params() {
        let name = p.name();
        let the_type = function_parameter_to_csharp_typename(p);

        let x = format!("{the_type} {name}");
        function_signature.push(x);
        function_param_names.push(name);
    }

    w.newline()?;
    indented!(w, "// Internal helper that works around an issue where exceptions in callbacks don't reenter Rust.")?;
    indented!(w, "{} class {} {{", i.visibility_types.to_access_modifier(), name)?;
    indented!(w, [()], "private Exception failure = null;")?;
    indented!(w, [()], "private readonly {} _callback;", the_type.name())?;
    w.newline()?;
    indented!(w, [()], "public {}({} original)", name, the_type.name())?;
    indented!(w, [()], "{{")?;
    indented!(w, [()()], "_callback = original;")?;
    indented!(w, [()], "}}")?;
    w.newline()?;
    indented!(w, [()], "public {} Call({})", rval, function_signature.join(", "))?;
    indented!(w, [()], "{{")?;
    indented!(w, [()()], "try")?;
    indented!(w, [()()], "{{")?;
    indented!(w, [()()()], "return _callback({});", function_param_names.join(", "))?;
    indented!(w, [()()], "}}")?;
    indented!(w, [()()], "catch (Exception e)")?;
    indented!(w, [()()], "{{")?;
    indented!(w, [()()()], "failure = e;")?;
    indented!(w, [()()()], "return {}.{};", rval, ffi_error.panic_variant().name())?;
    indented!(w, [()()], "}}")?;
    indented!(w, [()], "}}")?;
    w.newline()?;
    indented!(w, [()], "public void Rethrow()")?;
    indented!(w, [()], "{{")?;
    indented!(w, [()()], "if (this.failure != null)")?;
    indented!(w, [()()], "{{")?;
    indented!(w, [()()()], "throw this.failure;")?;
    indented!(w, [()()], "}}")?;
    indented!(w, [()], "}}")?;
    indented!(w, "}}")?;

    Ok(())
}

pub fn write_type_definition_named_callback_body(i: &Interop, w: &mut IndentWriter, the_type: &NamedCallback) -> Result<(), Error> {
    let rval = to_typespecifier_in_rval(the_type.fnpointer().signature().rval());
    let name = named_callback_to_typename(the_type);
    let visibility = i.visibility_types.to_access_modifier();
    let needs_wrapper = i.has_custom_marshalled_types(the_type.fnpointer().signature());

    let mut params = Vec::new();
    let mut native_params = Vec::new();
    for param in the_type.fnpointer().signature().params() {
        params.push(format!("{} {}", to_typespecifier_in_param(param.the_type()), param.name()));
        native_params.push(format!("{} {}", i.to_native_callback_typespecifier(param.the_type()), param.name()));
    }

    indented!(w, r"{} delegate {} {}({});", visibility, rval, name, params.join(", "))?;
    if needs_wrapper {
        indented!(
            w,
            r"delegate {} {}Native({});",
            i.to_native_callback_typespecifier(the_type.fnpointer().signature().rval()),
            name,
            native_params.join(", ")
        )?;
    }
    Ok(())
}
