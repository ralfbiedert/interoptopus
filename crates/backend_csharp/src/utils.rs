use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Function, SugaredReturnType};
use interoptopus::pattern::callback::AsyncCallback;
use interoptopus::{Error, render};

/// Indicates the return type of a method from user code.
///
/// Sync methods have their return type as-is, in async methods
/// this indicates the type of the async callback helper.
#[must_use]
pub fn sugared_return_type(f: &Function) -> SugaredReturnType {
    let ctype = f
        .signature()
        .params()
        .last()
        .and_then(|x| x.the_type().as_async_callback())
        .map(|async_callback: &AsyncCallback| async_callback.t());

    match ctype {
        None => SugaredReturnType::Sync(f.signature().rval().clone()),
        Some(x) => SugaredReturnType::Async(x.clone()),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveSemantics {
    Move,
    Copy,
}

pub fn write_common_marshaller(_i: &Interop, w: &mut IndentWriter, managed: &str, mv: MoveSemantics) -> Result<(), Error> {
    let prefix = match mv {
        MoveSemantics::Move => "Into",
        MoveSemantics::Copy => "To",
    };

    render!(w, "marshaller.cs", ("managed", managed), ("in_to", prefix))
}
