use interoptopus::pattern::api_guard::ApiVersion;
use interoptopus::{callback, ffi};

fn foo2() {}

pub struct ApiTable {
    foo: foo2,
}

#[ffi]
pub fn foo() -> ApiTable {
    ApiTable {}
}
