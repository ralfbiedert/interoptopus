use interoptopus::ffi_function;
use interoptopus::pattern::api_guard::ApiVersion;

// Adding a function returning an `APIVersion` will emit an FFI guard
// in your bindings that checks if the bindings match the DLL.
#[ffi_function]
pub fn pattern_api_guard() -> ApiVersion {
    crate::ffi_inventory().into()
}
