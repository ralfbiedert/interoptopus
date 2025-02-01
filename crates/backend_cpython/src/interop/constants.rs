use crate::converter::constant_value_to_value;
use crate::Interop;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_constants(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for c in i.inventory.constants() {
        indented!(w, r"{} = {}", c.name(), constant_value_to_value(c.value()))?;
    }

    Ok(())
}
