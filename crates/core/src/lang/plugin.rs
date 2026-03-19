use crate::inventory::{Inventory, PluginId};

pub trait PluginInfo {
    /// The unique identifier for this plugin.
    fn id() -> PluginId;
    /// Registers this plugin (and all referenced types) with the given inventory.
    fn register(inventory: &mut impl Inventory);
}
