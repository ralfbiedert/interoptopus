use crate::Interop;
use crate::converter::param_to_type;
use interoptopus::lang::Type;
use interoptopus::pattern::TypePattern;
use interoptopus_backend_utils::{Error, IndentWriter, indented};

pub fn write_type_definition_ffibool(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_type_definition_ffibool")?;

    let type_name = param_to_type(&Type::Pattern(TypePattern::Bool));

    indented!(w, r"[Serializable]")?;
    indented!(w, r"[StructLayout(LayoutKind.Sequential)]")?;
    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), type_name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"byte value;")?;
    indented!(w, r"}}")?;
    w.newline()?;

    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), type_name)?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"public static readonly {} True = new Bool {{ value =  1 }};", type_name)?;
    indented!(w, [()], r"public static readonly {} False = new Bool {{ value =  0 }};", type_name)?;
    indented!(w, [()], r"public Bool(bool b)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"value = (byte) (b ? 1 : 0);")?;
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"public bool Is => value == 1;")?;
    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}
