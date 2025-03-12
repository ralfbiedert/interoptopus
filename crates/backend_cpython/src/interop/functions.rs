use crate::Interop;
use crate::converter::{to_ctypes_name, to_type_hint_out};
use crate::interop::patterns::write_library_call;
use interoptopus::backend::util::safe_name;
use interoptopus::backend::writer::{IndentWriter, WriteFor};
use interoptopus::inventory::non_service_functions;
use interoptopus::lang::c::{CType, Function};
use interoptopus::pattern::TypePattern;
use interoptopus::{Error, indented};

pub fn write_function_proxies(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for function in non_service_functions(&i.inventory) {
        write_function(i, w, function, WriteFor::Code)?;
    }

    Ok(())
}

pub fn write_function(i: &Interop, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
    let rval_sig = to_type_hint_out(function.signature().rval());
    let args = i.function_args_to_string(function, true, false);
    let documentation = function.meta().documentation().lines().join("\n");

    indented!(w, r"def {}({}){}:", function.name(), args, rval_sig)?;

    if write_for == WriteFor::Code {
        if !documentation.is_empty() {
            indented!(w, [()], r#""""{}""""#, documentation)?;
        }

        write_param_helpers(i, w, function)?;
        write_library_call(i, w, function, None)?;
        w.newline()?;
    } else {
        indented!(w, [()], r"...")?;
    }

    Ok(())
}

pub fn write_param_helpers(_i: &Interop, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
    for arg in function.signature().params() {
        match arg.the_type() {
            CType::FnPointer(x) => {
                indented!(w, [()], r#"if not hasattr({}, "__ctypes_from_outparam__"):"#, arg.name())?;
                indented!(w, [()()], r"{} = callbacks.{}({})", arg.name(), safe_name(&x.internal_name()), arg.name())?;
                w.newline()?;
            }
            CType::Pattern(pattern) => match pattern {
                TypePattern::NamedCallback(x) => {
                    let x = x.fnpointer();
                    indented!(w, [()], r#"if not hasattr({}, "__ctypes_from_outparam__"):"#, arg.name())?;
                    indented!(w, [()()], r"{} = callbacks.{}({})", arg.name(), safe_name(&x.internal_name()), arg.name())?;
                    w.newline()?;
                }
                TypePattern::CStrPointer => {
                    indented!(w, [()], r#"if not hasattr({}, "__ctypes_from_outparam__"):"#, arg.name())?;
                    indented!(w, [()()], r"{} = ctypes.cast({}, ctypes.POINTER(ctypes.c_char))", arg.name(), arg.name())?;
                }
                TypePattern::Slice(t) | TypePattern::SliceMut(t) => {
                    let inner = to_ctypes_name(t.target_type(), false);
                    indented!(w, [()], r#"if hasattr({}, "_length_") and getattr({}, "_type_", "") == {}:"#, arg.name(), arg.name(), inner)?;

                    indented!(
                        w,
                        [()()],
                        r"{} = {}(data=ctypes.cast({}, ctypes.POINTER({})), len=len({}))",
                        arg.name(),
                        arg.the_type().name_within_lib(),
                        arg.name(),
                        inner,
                        arg.name()
                    )?;
                    w.newline()?;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
