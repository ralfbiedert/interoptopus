use crate::interop::functions::write_documentation;
use crate::Interop;
use interoptopus::lang::c::{EnumType, Variant};
use interoptopus::writer::{IndentWriter, WriteFor};
use interoptopus::{indented, Error};

pub fn write_type_definition_enum(i: &Interop, w: &mut IndentWriter, the_type: &EnumType, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_type_definition_enum")?;
    if write_for == WriteFor::Code {
        write_documentation(w, the_type.meta().documentation())?;
    }
    indented!(w, r"public enum {}", the_type.rust_name())?;
    indented!(w, r"{{")?;
    w.indent();

    for variant in the_type.variants() {
        write_type_definition_enum_variant(i, w, variant, the_type, write_for)?;
    }

    w.unindent();
    indented!(w, r"}}")
}

pub fn write_type_definition_enum_variant(_i: &Interop, w: &mut IndentWriter, variant: &Variant, _the_type: &EnumType, write_for: WriteFor) -> Result<(), Error> {
    let variant_name = variant.name();
    let variant_value = variant.value();
    if write_for == WriteFor::Code {
        write_documentation(w, variant.documentation())?;
    }
    indented!(w, r"{} = {},", variant_name, variant_value)
}
