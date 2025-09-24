use interoptopus::ffi;

#[ffi(service, module = "foo")]
struct Service;

#[ffi]
impl Service {}

fn main() {}
