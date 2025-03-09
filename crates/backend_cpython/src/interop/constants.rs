use crate::Interop;
use crate::converter::constant_value_to_value;
use interoptopus::writer::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_constants(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for c in i.inventory.constants() {
        indented!(w, r"{} = {}", c.name(), constant_value_to_value(c.value()))?;
    }

    Ok(())
}
