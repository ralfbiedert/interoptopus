use interoptopus::ffi_type;

#[ffi_type(service)]
struct Service1;

#[ffi_type(service)]
struct Service2 {}

#[ffi_type(service)]
struct Service3(());

fn main() {}
