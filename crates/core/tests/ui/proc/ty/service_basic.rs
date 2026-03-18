use interoptopus::ffi;

#[ffi(service)]
struct Service1;

#[ffi(service)]
struct Service2 {}

#[ffi(service)]
struct Service3(());

fn main() {}
