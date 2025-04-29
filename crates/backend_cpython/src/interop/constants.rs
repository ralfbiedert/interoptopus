use crate::Interop;
use crate::converter::constant_value_to_value;
use interoptopus::{Error, backend::IndentWriter, render};
use std::collections::HashMap;

pub fn write_constants(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    let constants = i
        .inventory
        .constants()
        .iter()
        .map(|c| (c.name(), constant_value_to_value(c.value())))
        .collect::<HashMap<_, _>>();

    render!(w, "constants.py", ("constants", &constants))
}
