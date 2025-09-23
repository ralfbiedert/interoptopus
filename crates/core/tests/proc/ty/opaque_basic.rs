use interoptopus::ffi_type;

#[ffi_type(opaque)]
struct Opaque1;

#[ffi_type(opaque)]
struct Opaque2 {}

#[ffi_type(opaque)]
struct Opaque3(());

fn main() {}
