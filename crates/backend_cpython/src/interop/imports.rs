use crate::Interop;
use interoptopus::Error;
use interoptopus::backend::IndentWriter;
use tera::Context;

pub fn write_imports(_i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    let context = Context::new();
    crate::TEMPLATES.render_to("imports.py", &context, w.writer()).unwrap();
    Ok(())
}
