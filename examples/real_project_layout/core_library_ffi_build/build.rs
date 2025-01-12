use core_library_ffi::ffi_inventory;
use interoptopus::Interop;
use interoptopus_backend_csharp::{ConfigBuilder, Generator};

// By adding the interop generation logic into a `build.rs` that depends on
// our `core_library_ffi` we ensure that upon `cargo build` both the `.dll`
// gets built, as well as the `.cs` files.
//
// Instead, if you used to unit test trick in the other examples, you will have
// to run both `cargo build` to produce the `.dll` and `cargo test`
// to produce the bindings (since `cargo test` does not imply `cargo build`).
fn main() {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().build().unwrap();

    Generator::new(config, inventory)
        // You might also want to consider writing to `OUT_DIR` instead, since
        // writing to any other place from a `build.rs` is discouraged (we do
        // it here to simplify our example).
        .write_file("bindings/Interop.cs")
        .unwrap();
}
