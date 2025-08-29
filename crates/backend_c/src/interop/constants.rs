use crate::converters::{const_name_to_name, constant_value_to_value, primitive_to_typename};
use crate::interop::docs::write_documentation;
use crate::{DocStyle, Interop};
use interoptopus::lang::{Constant, Type};
use interoptopus_backend_utils::{Error, IndentWriter, indented};

pub fn write_constants(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for constant in i.inventory.constants() {
        write_constant(i, w, constant)?;
    }

    Ok(())
}

pub fn write_constant(i: &Interop, w: &mut IndentWriter, constant: &Constant) -> Result<(), Error> {
    let name = const_name_to_name(i, constant);
    let the_type = match constant.the_type() {
        Type::Primitive(x) => primitive_to_typename(x),
        _ => return Err(Error::Null),
    };

    if i.documentation == DocStyle::Inline {
        write_documentation(w, constant.meta().docs())?;
    }

    indented!(w, r"const {} {} = {};", the_type, name, constant_value_to_value(constant.value()))?;

    Ok(())
}
