use interoptopus::ffi;

#[ffi(opaque)]
struct Opaque1;

#[ffi(opaque)]
struct Opaque2 {}

#[ffi(opaque)]
struct Opaque3(());

fn main() {}
