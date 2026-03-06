use crate::Interop;
use crate::converter::rval_to_type_sync;
use interoptopus::lang::Composite;
use interoptopus_backend_utils::{Error, IndentWriter, indented};

// TODO: Write helpers for options
pub fn _write_pattern_option(i: &Interop, w: &mut IndentWriter, slice: &Composite) -> Result<(), Error> {
    i.debug(w, "write_pattern_option")?;

    let name = slice.rust_name();
    let data_type = slice
        .fields()
        .iter()
        .find(|x| x.name().eq("t"))
        .expect("Option must contain field called 't'.")
        .the_type();

    let type_string = rval_to_type_sync(data_type);
    let is_some = "is_some";

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
