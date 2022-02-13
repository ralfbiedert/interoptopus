use interoptopus::{ffi_function, function, Library, LibraryBuilder};

#[ffi_function]
#[no_mangle]
extern "C" fn do_math(x: u32) -> u32 {
    // Change this line, run `cargo build` and click `Hot Reload` in Unity
    x + 1
}

pub fn my_inventory() -> Library {
    {
        LibraryBuilder::new().register(function!(do_math)).library()
    }
}
