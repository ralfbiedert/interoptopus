use interoptopus::lang::{Function, SugaredReturnType};
use interoptopus::pattern::callback::AsyncCallback;

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
