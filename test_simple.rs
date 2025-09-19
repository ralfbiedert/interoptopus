use interoptopus_proc::ffi_type;

#[ffi_type(debug)]
struct Simple {
    x: u32,
}

#[ffi_type(name = "Renamed", debug)]
struct WithName {
    y: u32,
}

fn main() {}