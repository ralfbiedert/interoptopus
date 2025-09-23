use interoptopus::{ffi_service, ffi_type};

#[ffi_type(service, module = "foo")]
struct Service;

#[ffi_service]
impl Service {}

#[allow(dead_code)]
fn main() {}
