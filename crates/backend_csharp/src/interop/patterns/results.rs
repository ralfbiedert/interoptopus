use crate::converter::to_typespecifier_in_rval;
use crate::Interop;
use interoptopus::lang::c::{CType, CompositeType};
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_pattern_result(i: &Interop, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
    i.debug(w, "write_pattern_result")?;

    let ffi_error = i
        .inventory
        .ctypes()
        .iter()
        .filter_map(|x| match x {
            CType::Pattern(TypePattern::FFIErrorEnum(e)) => Some(e),
            _ => None,
        })
        .next()
        .expect("When using result type there must be an FFIError in the inventory with an `ok` variant.");

    let context_type_name = slice.rust_name();
    let data_type = slice
        .fields()
        .iter()
        .find(|x| x.name().eq("t"))
        .expect("Option must contain field called 't'.")
        .the_type();

    let type_string = to_typespecifier_in_rval(data_type);

    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), context_type_name)?;
    indented!(w, r"{{")?;

    // FromNullable
    indented!(w, [()], r"public {} Ok()", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"if (err == {})", ffi_error.success_variant().value())?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"return t;")?;
    indented!(w, [()()], r"}}")?;
    indented!(w, [()()], r"throw new InteropException<{}>(err);", ffi_error.the_enum().rust_name())?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}
