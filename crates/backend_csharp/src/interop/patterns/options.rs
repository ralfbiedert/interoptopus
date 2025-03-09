use crate::Interop;
use crate::converter::to_typespecifier_in_sync_fn_rval;
use interoptopus::lang::c::CompositeType;
use interoptopus::writer::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_pattern_option(i: &Interop, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
    i.debug(w, "write_pattern_option")?;

    let name = slice.rust_name();
    let data_type = slice
        .fields()
        .iter()
        .find(|x| x.name().eq("t"))
        .expect("Option must contain field called 't'.")
        .the_type();

    let type_string = to_typespecifier_in_sync_fn_rval(data_type);
    let is_some = if i.rename_symbols { "isSome" } else { "is_some" };

    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), name)?;
    indented!(w, r"{{")?;

    // FromNullable
    indented!(w, [()], r"public static {name} FromNullable({type_string}? nullable)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var result = new {}();", name)?;
    indented!(w, [()()], r"if (nullable.HasValue)")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"result.{is_some} = 1;")?;
    indented!(w, [()()()], r"result.t = nullable.Value;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"return result;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    // ToNullable
    indented!(w, [()], r"public {type_string}? ToNullable()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"return this.{is_some} == 1 ? this.t : ({type_string}?)null;")?;
    indented!(w, [()], r"}}")?;

    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}
