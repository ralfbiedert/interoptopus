use crate::Interop;
use crate::converter::constant_value_to_value;
use interoptopus::Error;
use interoptopus::backend::IndentWriter;
use std::collections::HashMap;
use tera::Context;

pub fn write_constants(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    let constants = i
        .inventory
        .constants()
        .into_iter()
        .map(|c| (c.name(), constant_value_to_value(c.value())))
        .collect::<HashMap<_, _>>();

    //render!(w, "constants.py", ("constants", &constants));
    let mut context = Context::new();
    context.insert("constants", &constants);
    crate::TEMPLATES.render_to("constants.py", &context, w.writer()).unwrap();
    Ok(())
}
