use crate::Interop;
use crate::converter::fnpointer_to_typename;
use interoptopus::backend::IndentWriter;
use interoptopus::backend::safe_name;
use interoptopus::lang::Type;
use interoptopus::pattern::TypePattern;
use interoptopus::{Error, indented};

pub fn write_callback_helpers(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, r"class {}:", i.callback_namespace)?;
    indented!(w, [()], r#""""Helpers to define callbacks.""""#)?;

    for callback in i.inventory.c_types().iter().filter_map(|x| match x {
        Type::FnPointer(x) => Some(x),
        Type::Pattern(TypePattern::NamedCallback(x)) => Some(x.fnpointer()),
        _ => None,
    }) {
        indented!(w, [()], r"{} = {}", safe_name(&callback.internal_name()), fnpointer_to_typename(callback))?;
    }

    Ok(())
}
