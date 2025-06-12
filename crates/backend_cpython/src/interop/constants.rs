use crate::converter::constant_value_to_value;
use crate::{Interop, render};
use interoptopus::Error;
use interoptopus::backend::IndentWriter;
use std::collections::HashMap;

pub fn write_constants(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    let constants = i
        .inventory
        .constants()
        .into_iter()
        .map(|c| (c.name(), constant_value_to_value(c.value())))
        .collect::<HashMap<_, _>>();

    render!(w, "constants.py", ("constants", &constants))
}
