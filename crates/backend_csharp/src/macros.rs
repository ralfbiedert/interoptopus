/// Creates an ad-hoc plugin that runs the given block in `post_model_all`.
///
/// The block has access to `inventory: &RustInventory` and `models: PostModelPass`.
///
/// ```ignore
/// debug_plugin!(|inventory, models| {
///     dbg!(&models);
/// })
/// ```
#[macro_export]
macro_rules! debug_plugin {
    (|$inventory:ident, $models:ident| $body:expr) => {{
        struct __DebugPlugin;

        impl $crate::plugin::RustLibraryPlugin for __DebugPlugin {
            fn post_model_all(
                &mut self,
                $inventory: &::interoptopus::inventory::RustInventory,
                $models: $crate::plugin::PostModelPass,
            ) -> Result<(), $crate::Error> {
                $body;
                Ok(())
            }
        }

        __DebugPlugin
    }};
}
