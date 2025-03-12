use crate::Interop;
use interoptopus::backend::writer::IndentWriter;
use interoptopus::lang::c::Function;
use interoptopus::{Error, indented};

pub fn write_success_enum_aware_rval(i: &Interop, w: &mut IndentWriter, function: &Function, args: &str, ret: bool) -> Result<(), Error> {
    i.debug(w, "write_success_enum_aware_rval")?;

    if ret {
        indented!(w, [()], r"return c_lib.{}({})", function.name(), &args)?;
    } else {
        indented!(w, [()], r"c_lib.{}({})", function.name(), &args)?;
    }
    Ok(())
}

pub fn write_utils(_i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    // indented!(w, r#"class Slice(ctypes.Structure, typing.Generic[T]):"#)?;
    // indented!(w, [_], r#"# These fields represent the underlying C data layout"#)?;
    // indented!(w, [_], r#"_fields_ = ["#)?;
    // indented!(w, [_], r#"    ("data", ctypes.c_void_p),"#)?;
    // indented!(w, [_], r#"    ("len", ctypes.c_uint64),"#)?;
    // indented!(w, [_], r#"]"#)?;
    // w.newline()?;
    // indented!(w, [_], r#"def cast(self, x):"#)?;
    // indented!(w, [_ _], r#"return ctypes.cast(self.data, ctypes.POINTER(x))"#)?;
    // w.newline()?;
    // w.newline()?;
    //
    // indented!(w, r#"class SliceMut(ctypes.Structure, typing.Generic[T]):"#)?;
    // indented!(w, [_], r#"# These fields represent the underlying C data layout"#)?;
    // indented!(w, [_], r#"_fields_ = ["#)?;
    // indented!(w, [_], r#"    ("data", ctypes.c_void_p),"#)?;
    // indented!(w, [_], r#"    ("len", ctypes.c_uint64),"#)?;
    // indented!(w, [_], r#"]"#)?;
    // w.newline()?;
    // indented!(w, [_], r#"def cast(self, x):"#)?;
    // indented!(w, [_ _], r#"return ctypes.cast(self.data, ctypes.POINTER(x))"#)?;
    // w.newline()?;
    // w.newline()?;

    indented!(w, r"TRUE = ctypes.c_uint8(1)")?;
    indented!(w, r"FALSE = ctypes.c_uint8(0)")?;
    w.newline()?;
    w.newline()?;

    indented!(w, r"def _errcheck(returned, success):")?;
    indented!(w, [()], r#""""Checks for FFIErrors and converts them to an exception.""""#)?;
    indented!(w, [()], r"if returned == success: return")?;
    indented!(w, [()], r#"else: raise Exception(f"Function returned error: {{returned}}")"#)?;
    w.newline()?;
    w.newline()?;

    indented!(w, r"class CallbackVars(object):")?;
    indented!(w, [()], r#""""Helper to be used `lambda x: setattr(cv, "x", x)` when getting values from callbacks.""""#)?;
    indented!(w, [()], r"def __str__(self):")?;
    indented!(w, [()()], r#"rval = """#)?;
    indented!(w, [()()], r#"for var in  filter(lambda x: "__" not in x, dir(self)):"#)?;
    indented!(w, [()()()], r#"rval += f"{{var}}: {{getattr(self, var)}}""#)?;
    indented!(w, [()()], r"return rval")?;
    w.newline()?;
    w.newline()?;

    indented!(w, r"class _Iter(object):")?;
    indented!(w, [()], r#""""Helper for slice iterators.""""#)?;
    indented!(w, [()], r"def __init__(self, target):")?;
    indented!(w, [()()], r"self.i = 0")?;
    indented!(w, [()()], r"self.target = target")?;
    w.newline()?;
    indented!(w, [()], r"def __iter__(self):")?;
    indented!(w, [()()], r"self.i = 0")?;
    indented!(w, [()()], r"return self")?;
    w.newline()?;
    indented!(w, [()], r"def __next__(self):")?;
    indented!(w, [()()], r"if self.i >= self.target.len:")?;
    indented!(w, [()()()], r"raise StopIteration()")?;
    indented!(w, [()()], r"rval = self.target[self.i]")?;
    indented!(w, [()()], r"self.i += 1")?;
    indented!(w, [()()], r"return rval")?;
    w.newline()?;
    w.newline()?;

    Ok(())
}
