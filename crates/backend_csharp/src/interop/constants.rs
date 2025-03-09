use crate::Interop;
use crate::converter::{constant_value_to_value, to_typespecifier_in_sync_fn_rval};
use crate::interop::docs::write_documentation;
use interoptopus::lang::c::Constant;
use interoptopus::writer::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_constants(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for constant in i.inventory.constants() {
        if i.should_emit_by_meta(constant.meta()) {
            write_constant(i, w, constant)?;
            w.newline()?;
        }
    }

    Ok(())
}

pub fn write_constant(i: &Interop, w: &mut IndentWriter, constant: &Constant) -> Result<(), Error> {
    i.debug(w, "write_constant")?;
    let rval = to_typespecifier_in_sync_fn_rval(&constant.the_type());
    let name = constant.name();
    let value = constant_value_to_value(constant.value());

    write_documentation(w, constant.meta().documentation())?;
    indented!(w, r"public const {} {} = ({}) {};", rval, name, rval, value)
}
