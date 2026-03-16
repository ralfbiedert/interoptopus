/// Builds a `RustLibrary`, registers inventory items and a plugin, then processes.
///
/// ```ignore
/// test_csharp!(plugin, [function!(my_function), service!(MyService)]);
/// ```
macro_rules! test_ffi {
    ($plugin:expr, [$($item:expr),* $(,)?]) => {{
        let mut inventory = ::interoptopus::inventory::RustInventory::new();
        $(let _ = inventory.register($item);)*
        let inventory = inventory.validate();
        ::interoptopus_csharp::RustLibrary::builder(inventory)
            .build()
            .register_plugin($plugin)
            .process()
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

        impl ::interoptopus_csharp::plugin::RustLibraryPlugin for __DebugPlugin {
            fn post_model_all(
                &mut self,
                $inventory: &::interoptopus::inventory::RustInventory,
                $models: ::interoptopus_csharp::plugin::PostModelPass,
            ) -> Result<(), ::interoptopus_csharp::Error> {
                $body;
                Ok(())
            }
        }

        __DebugPlugin
    }};
}
