use crate::converter::{fnpointer_to_typename, to_typespecifier_in_param, to_typespecifier_in_rval};
use crate::Interop;
use interoptopus::lang::c::FnPointerType;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

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
    let rval = to_typespecifier_in_rval(the_type.signature().rval());
    let name = fnpointer_to_typename(the_type);
    let visibility = i.visibility_types.to_access_modifier();

    let mut params = Vec::new();
    for (i, param) in the_type.signature().params().iter().enumerate() {
        params.push(format!("{} x{}", to_typespecifier_in_param(param.the_type()), i));
    }

    indented!(w, r"{} delegate {} {}({});", visibility, rval, name, params.join(", "))
}
