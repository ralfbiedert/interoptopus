use interoptopus::{constant, function, Library, LibraryBuilder};

mod common;
mod ffi;
mod thirdparty;

pub fn ffi_inventory() -> Library {
    {
        LibraryBuilder::new()
            .register(function!(ffi::example_api_version))
            .register(function!(ffi::example_always_fails))
            .register(function!(ffi::example_create_context))
            .register(function!(ffi::example_destroy_context))
            .register(function!(ffi::example_print_score))
            .register(function!(ffi::example_return_score))
            .register(function!(ffi::example_update_score_by_callback))
            .register(function!(ffi::example_write_foreign_type))
            .register(function!(ffi::example_double_super_complex_entity))
            .register(constant!(ffi::THE_MAGIC_CONSTANT))
            .library()
    }
}
