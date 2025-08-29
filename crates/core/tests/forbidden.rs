mod wire;

use interoptopus::function;
use interoptopus::inventory::Inventory;
use interoptopus_proc::ffi_function;

#[ffi_function]
fn public() {}

#[test]
#[should_panic(expected = "`public` has a forbidden name that might cause issues in other languages.")]
fn panics_on_forbidden_names() {
    _ = Inventory::builder().register(function!(public)).validate().build();
}

#[test]
fn accepts_forbidden_names() {
    _ = Inventory::builder().register(function!(public)).allow_reserved_names().validate().build();
}
