use interoptopus::ffi;
use interoptopus::pattern::api_guard::ApiVersion;

// Adding a function returning an `APIVersion` will emit an FFI guard
// in your bindings that checks if the bindings match the DLL.
#[ffi]
pub fn pattern_api_guard() -> ApiVersion {
    crate::ffi_inventory().into()
}
