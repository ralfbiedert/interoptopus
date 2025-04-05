use crate::Interop;
use interoptopus::backend::IndentWriter;
use interoptopus::lang::{Function, SugaredReturnType};
use interoptopus::pattern::callback::AsyncCallback;
use interoptopus::{Error, indented};

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

pub enum MoveSemantics {
    Move,
    Copy,
}

pub fn write_common_marshaller(i: &Interop, w: &mut IndentWriter, managed: &str, mv: MoveSemantics) -> Result<(), Error> {
    let prefix = match mv {
        MoveSemantics::Move => "Into",
        MoveSemantics::Copy => "To",
    };

    indented!(w, [()], r"public ref struct Marshaller")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"private {managed} _managed;")?;
    indented!(w, [()()], r"private Unmanaged _unmanaged;")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public Marshaller({managed} managed) {{ _managed = managed; }}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public Marshaller(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public void FromManaged({managed} managed) {{ _managed = managed; }}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public void FromUnmanaged(Unmanaged unmanaged) {{ _unmanaged = unmanaged; }}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public Unmanaged ToUnmanaged() {{ return _managed.{prefix}Unmanaged(); }}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public {managed} ToManaged() {{ return _unmanaged.{prefix}Managed(); }}")?;
    w.newline()?;
    i.inline_hint(w, 2)?;
    indented!(w, [()()], r"public void Free() {{ }}")?;
    indented!(w, [()], r"}}")?; // Close ref struct Marshaller.
    w.newline()?;

    Ok(())
}
