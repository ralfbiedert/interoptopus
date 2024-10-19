use core_library_ffi::ffi_inventory;
use interoptopus::Interop;
use interoptopus_backend_csharp::overloads::DotNet;
use interoptopus_backend_csharp::{ConfigBuilder, Generator};

// By adding the interop generation logic into a `build.rs` that depends on
// our `core_library_ffi` we ensure that upon `cargo build` both the `.dll`
// gets built, as well as the `.cs` files.
//
// Instead, if you used to unit test trick in the other examples, you will have
// to run both `cargo build` to produce the `.dll` as well as `cargo test`
// to produce the bindings (since `cargo test` does not imply `cargo build`).
fn main() {
    let inventory = ffi_inventory();
    let overload = DotNet::new();
    let config = ConfigBuilder::default().build().unwrap();

    Generator::new(config, inventory)
        .add_overload_writer(overload)
        .write_file("bindings/Interop.cs")
        .unwrap();
}
