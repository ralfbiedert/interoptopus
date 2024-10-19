use interoptopus::ffi_function;
use interoptopus::patterns::api_guard::APIVersion;

#[ffi_function]
pub fn pattern_api_guard() -> APIVersion {
    crate::ffi_inventory().into()
}
