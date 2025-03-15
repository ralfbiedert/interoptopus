use crate::Interop;
use crate::converter::{fnpointer_to_typename, to_typespecifier_in_param, to_typespecifier_in_sync_fn_rval};
use interoptopus::backend::IndentWriter;
use interoptopus::lang::FnPointerType;
use interoptopus::{Error, indented};

pub fn write_type_definition_fn_pointer(i: &Interop, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
    i.debug(w, "write_type_definition_fn_pointer")?;
    write_type_definition_fn_pointer_annotation(w, the_type)?;
    write_type_definition_fn_pointer_body(i, w, the_type)?;
    Ok(())
}

pub fn write_type_definition_fn_pointer_annotation(w: &mut IndentWriter, _the_type: &FnPointerType) -> Result<(), Error> {
    indented!(w, r"[UnmanagedFunctionPointer(CallingConvention.Cdecl)]")
}

pub fn write_type_definition_fn_pointer_body(i: &Interop, w: &mut IndentWriter, the_type: &FnPointerType) -> Result<(), Error> {
    let rval = to_typespecifier_in_sync_fn_rval(the_type.signature().rval());
    let name = fnpointer_to_typename(the_type);
    let visibility = i.visibility_types.to_access_modifier();
    let needs_wrapper = i.has_custom_marshalled_types(the_type.signature());

    let mut params = Vec::new();
    let mut native_params = Vec::new();
    for (param_index, param) in the_type.signature().params().iter().enumerate() {
        params.push(format!("{} x{}", to_typespecifier_in_param(param.the_type()), param_index));
        native_params.push(format!("{} {}", i.to_native_callback_typespecifier(param.the_type()), param.name()));
    }

    indented!(w, r"{} delegate {} {}({});", visibility, rval, name, params.join(", "))?;
    if needs_wrapper {
        indented!(w, r"{} delegate {} {}_native({});", visibility, i.to_native_callback_typespecifier(the_type.signature().rval()), name, native_params.join(", "))?;
    }

    Ok(())
}
