/// Builds a `RustLibrary`, registers inventory items and a plugin, then processes.
///
/// ```ignore
/// test_csharp!(plugin, [function!(my_function), service!(MyService)]);
/// ```
macro_rules! test_model {
    ($plugin:expr, [$($item:expr),* $(,)?]) => {{
        let mut inventory = ::interoptopus::inventory::RustInventory::new();
        $(let _ = inventory.register($item);)*
        let inventory = inventory.validate();
        ::interoptopus_csharp::RustLibrary::builder(inventory)
            .build()
            .register_extension($plugin)
            .process()
    }};
}

/// Performs an 'output' test.
///
/// Parameter `$file` is the output file name to snapshot. Must match one of
///   - `"Interop.cs"` — items with `FileEmission::Default` or `FileEmission::CustomModule(_)`
///   - `"Interop.Common.cs"` — items with `FileEmission::Common`
///
/// The rest are inventory registration expressions (e.g., `function!(my_fn)`, `service!(MyService)`)
/// that populate the `RustInventory`.
///
/// # Example
///
/// ```ignore
/// test_output!("Interop.cs", [service!(MyService), function!(my_fn)]);
/// ```
macro_rules! test_output {
    ($file:expr, [$($item:expr),* $(,)?]) => {{
        use interoptopus_csharp::dispatch::Dispatch;
        use interoptopus_csharp::output::Target;
        use interoptopus::lang::meta::FileEmission;

        let mut inventory = ::interoptopus::inventory::RustInventory::new();
        $(let _ = inventory.register($item);)*
        let inventory = inventory.validate();
        let multibuf = ::interoptopus_csharp::RustLibrary::builder(inventory)
            .dispatch(Dispatch::custom(|x, _| match x.emission {
                FileEmission::Common => Target::new("Interop.Common.cs", "My.Company.Common"),
                FileEmission::Default | FileEmission::CustomModule(_) => Target::new("Interop.cs", "My.Company"),
            }))
            .build()
            .process()
            .unwrap();

        // In these tests we are only
        let output = multibuf.buffer($file).unwrap();

        insta::assert_snapshot!(output);
    }};
}

/// Creates an ad-hoc plugin that runs the given block in `post_model_all`.
///
/// The block has access to `inventory: &RustInventory` and `models: PostModelPass`.
///
/// ```ignore
/// let plugin = debug_plugin!(|inventory, models| {
///     dbg!(&models);
/// });
/// ```
macro_rules! debug_plugin {
    (|$inventory:ident, $models:ident| $body:expr) => {{
        struct __DebugPlugin;

        impl ::interoptopus_csharp::extensions::RustCodegenExtension for __DebugPlugin {
            fn post_model_all(
                &mut self,
                $inventory: &::interoptopus::inventory::RustInventory,
                $models: ::interoptopus_csharp::extensions::PostModelPass,
            ) -> Result<(), ::interoptopus_csharp::Error> {
                $body;
                Ok(())
            }
        }

        __DebugPlugin
    }};
}
