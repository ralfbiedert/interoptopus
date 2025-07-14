use crate::Interop;
use crate::interop::FfiTransType;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Composite, Type};
use interoptopus::{Error, render};

#[expect(dead_code)]
pub fn write_wires(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for t in i.inventory.c_types() {
        match t {
            Type::Wired(wired) => write_type_definitions_wired(i, w, wired)?,
            _ => (),
        }
    }
    Ok(())
}

pub fn write_type_definitions_wired(_i: &Interop, w: &mut IndentWriter, wired: &Composite) -> Result<(), Error> {
    render!(w, "wire.cs", ("type", wired.trans_type_name()))
}
