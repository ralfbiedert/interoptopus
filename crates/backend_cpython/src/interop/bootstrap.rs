use crate::Interop;
use crate::converter::to_ctypes_name;
use interoptopus::lang::c::CType;
use interoptopus::patterns::TypePattern;
use interoptopus::writer::IndentWriter;
use interoptopus::{Error, indented};

pub fn write_api_load_fuction(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    indented!(w, r"c_lib = None")?;
    w.newline()?;
    indented!(w, r"def init_lib(path):")?;
    indented!(w, [()], r#""""Initializes the native library. Must be called at least once before anything else.""""#)?;
    indented!(w, [()], r"global c_lib")?;
    indented!(w, [()], r"c_lib = ctypes.cdll.LoadLibrary(path)")?;

    w.newline()?;
    for f in i.inventory.functions() {
        let args = f.signature().params().iter().map(|x| to_ctypes_name(x.the_type(), false)).collect::<Vec<_>>();

        indented!(w, [()], r"c_lib.{}.argtypes = [{}]", f.name(), args.join(", "))?;
    }

    w.newline()?;
    for f in i.inventory.functions() {
        let rtype = to_ctypes_name(f.signature().rval(), false);
        if !rtype.is_empty() {
            indented!(w, [()], r"c_lib.{}.restype = {}", f.name(), rtype)?;
        }
    }

    w.newline()?;
    for f in i.inventory.functions() {
        if let CType::Pattern(TypePattern::FFIErrorEnum(e)) = f.signature().rval() {
            let value = e.success_variant().value();
            indented!(w, [()], r"c_lib.{}.errcheck = lambda rval, _fptr, _args: _errcheck(rval, {})", f.name(), value)?;
        }
    }

    Ok(())
}
