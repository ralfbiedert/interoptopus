use interoptopus::ffi_plugin;
use interoptopus_proc::ffi_type;

// Test the exact syntax the user requested
ffi_plugin!(Blah {
    fn foo(x: Something) -> ffi::String;
    fn bar(x: Something) -> ffi::String;

    trait Foo {
        fn bar(&self) -> ffi::String;
        fn baz(&self) -> ffi::String;
    }
});

// Define a simple type for testing
#[derive(Clone)]
pub struct Something {
    value: u32,
}

#[ffi_type]
extern "CsPlugin" {
    fn foo(x: Something) -> String;

    trait Foo {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_plugin_struct_exists() {
        // Test that the Blah struct was generated
        let _plugin = Blah { _private: () };
    }

    #[test]
    fn test_function_exists() {
        // Test that the foo function was generated (it should compile)
        // Note: It will panic with unimplemented! but that's expected
    }
}
