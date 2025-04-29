use crate::Interop;
use crate::converter::to_ctypes_name;
use interoptopus::{Error, backend::IndentWriter, render};
use std::collections::HashMap;

pub fn write_api_load_function(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    let functions = i
        .inventory
        .functions()
        .iter()
        .map(|f| {
            let args = f
                .signature()
                .params()
                .iter()
                .map(|x| to_ctypes_name(x.the_type(), false))
                .collect::<Vec<_>>()
                .join(", ");

            let rtype = to_ctypes_name(f.signature().rval(), false);

            (f.name(), [("signature", args), ("restype", rtype)].into())
        })
        .collect::<HashMap<_, HashMap<_, _>>>();

    render!(w, "api_load_function.py", ("functions", &functions))
}
