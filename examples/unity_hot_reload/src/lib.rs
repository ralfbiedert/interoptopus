use interoptopus::ffi_function;

#[ffi_function]
#[no_mangle]
extern "C" fn do_math(x: u32) -> u32 {
    // Change this line, run `cargo build` and click `Hot Reload` in Unity
    x + 1
}

interoptopus::inventory_function!(ffi_inventory, [], [do_math], []);
