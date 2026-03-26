//! Registers plugin-related emissions with the master pass so that the
//! dispatch can create the correct output files.
//!
//! Registers two kinds of items:
//! - The plugin infrastructure emission (`PLUGIN_DEFAULT_EMISSION`) for the
//!   `Trampoline` class and `Interop` forwarding methods.
//! - Interface emissions for `IPlugin` and service interfaces.

use crate::dispatch::{Item, ItemKind};
use crate::lang::plugin::PLUGIN_DEFAULT_EMISSION;
use crate::pass::{OutputResult, PassInfo, model, output};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &mut output::common::master::Pass,
        plugin_interface: &model::dotnet::interface::plugin::Pass,
        service_interfaces: &model::dotnet::interface::service::Pass,
    ) -> OutputResult {
        // Register the plugin infrastructure file (Trampoline + Interop class).
        if let Some(fe) = PLUGIN_DEFAULT_EMISSION.file_emission() {
            output_master.register_item(Item { kind: ItemKind::PluginInterface, emission: fe.clone() });
        }

        // Register interface emissions.
        if let Some(iface) = plugin_interface.interface()
            && let Some(fe) = iface.emission.file_emission()
        {
            output_master.register_item(Item { kind: ItemKind::PluginInterface, emission: fe.clone() });
        }

        for iface in service_interfaces.interfaces() {
            if let Some(fe) = iface.emission.file_emission() {
                output_master.register_item(Item { kind: ItemKind::PluginInterface, emission: fe.clone() });
            }
        }

        // Register the Plugin.cs stub file.
        output_master.register_item(Item { kind: ItemKind::PluginStub, emission: interoptopus::lang::meta::FileEmission::Default });

        Ok(())
    }
}
