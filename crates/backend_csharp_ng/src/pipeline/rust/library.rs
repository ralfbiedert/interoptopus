use crate::Error;
use crate::pass::{ModelResult, OutputResult, meta_info, model_final, model_id_maps, model_type_map, output_final, output_header, output_master};
use crate::pipeline::{RustLibraryBuilder, loop_model_passes_until_done};
use crate::plugin::{PostModelPass, PostOutputPass, RustLibraryPlugin};
use interoptopus::inventory::RustInventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

#[derive(Default)]
pub struct RustLibraryConfig {
    pub meta_info: meta_info::Config,
    pub model_id_maps: model_id_maps::Config,
    pub model_type_map: model_type_map::Config,
    pub model_final: model_final::Config,
    pub output_master: output_master::Config,
    pub output_header: output_header::Config,
    pub output_final: output_final::Config,
    _hidden: PhantomData<()>,
}

pub struct IntermediateOutputPasses {
    pub header: output_header::Pass,
}

pub struct RustLibrary {
    // Basic input
    inventory: RustInventory,

    // Model passes (transform and enrich data)
    meta_info: meta_info::Pass,
    model_id_maps: model_id_maps::Pass,
    model_type_map: model_type_map::Pass,
    model_final: model_final::Pass,
    // ...

    // First output pass determining files to be produced
    output_master: output_master::Pass,

    // Most other output passes. Ideally these should have no cross-dependencies,
    // only depending on the models above. The last output stages (e.g., output_master)
    // then integrate all previous outputs to write the actual artifact (into Multibuf)
    // We put them into a separate struct so we don't have to later pass 20+ of them
    // to final.
    output_passes: IntermediateOutputPasses,

    // Last output stage(s). Writes a `.cs` file (later possibly other files w. other
    // master stages) into the Multibuf.
    output_final: output_final::Pass,

    // Output
    output: Multibuf,

    // Plugins
    plugins: Vec<Box<dyn RustLibraryPlugin>>,
}

impl RustLibrary {
    pub fn new(inventory: RustInventory) -> Self {
        Self::with_config(inventory, RustLibraryConfig::default())
    }

    pub fn builder(inventory: RustInventory) -> RustLibraryBuilder {
        RustLibraryBuilder::new(inventory)
    }

    pub(crate) fn with_config(inventory: RustInventory, config: RustLibraryConfig) -> Self {
        Self {
            inventory,
            meta_info: meta_info::Pass::new(config.meta_info),
            model_id_maps: model_id_maps::Pass::new(config.model_id_maps),
            model_type_map: model_type_map::Pass::new(config.model_type_map),
            model_final: model_final::Pass::new(config.model_final),
            output_master: output_master::Pass::new(config.output_master),
            output_passes: IntermediateOutputPasses { header: output_header::Pass::new(config.output_header) },
            output_final: output_final::Pass::new(config.output_final),
            output: Multibuf::default(),
            plugins: vec![],
        }
    }

    pub fn register_plugin(mut self, plugin: impl RustLibraryPlugin + 'static) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    fn plugin_init_pass(&mut self) {
        for plugin in self.plugins.iter_mut() {
            plugin.init(&mut self.inventory);
        }
    }

    fn plugin_post_output_pass(&mut self) -> OutputResult {
        let post_output = PostOutputPass::default();
        for plugin in self.plugins.iter_mut() {
            plugin.post_output(&mut self.output, post_output)?;
        }
        Ok(())
    }

    pub fn process(mut self) -> Result<Multibuf, Error> {
        self.plugin_init_pass();

        // Model passes
        loop_model_passes_until_done(|mut r| {
            r.run(self.meta_info.process())?;
            r.run(self.model_id_maps.process(&self.inventory.types))?;
            r.run(self.model_type_map.process(&self.inventory.types))?;
            r.run(self.model_final.process())?;

            let post_model = PostModelPass::default();
            for plugin in self.plugins.iter_mut() {
                r.run(plugin.post_model(&mut self.inventory, post_model))?;
            }
            Ok(())
        })?;

        // Output passes
        self.output_master.process()?;
        self.output_passes.header.process(&self.output_master, &self.meta_info)?;
        self.plugin_post_output_pass()?;

        // Final output pass(es)
        self.output_final.process(&mut self.output, &self.output_master, &self.output_passes)?;

        Ok(self.output)
    }
}
