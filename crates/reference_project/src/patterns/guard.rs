use interoptopus::ffi;
use interoptopus::pattern::guard::Version;

// Adding a function returning a `Version` will emit an FFI guard
// in your bindings that checks if the bindings match the DLL.
#[ffi]
pub fn pattern_api_guard() -> Version {
    crate::inventory().into()
}
