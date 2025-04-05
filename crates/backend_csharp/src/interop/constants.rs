use crate::Interop;
use crate::converter::{const_value, rval_to_type_sync};
use crate::interop::docs::write_documentation;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::Constant;
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
    let rval = rval_to_type_sync(&constant.the_type());
    let name = constant.name();
    let value = const_value(constant.value());

    write_documentation(w, constant.meta().docs())?;
    indented!(w, r"public const {} {} = ({}) {};", rval, name, rval, value)
}
