use crate::Error;
use crate::pipeline::RustLibraryBuilder;
use crate::plugin::{PostModelPass, PostOutputPass, RustLibraryPlugin};
use crate::stage::{meta_info, model_final, model_id_maps, output_final, output_header, output_master};
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

#[derive(Default)]
pub struct RustLibraryConfig {
    pub meta_info: meta_info::Config,
    pub model_id_maps: model_id_maps::Config,
    pub model_final: model_final::Config,
    pub output_master: output_master::Config,
    pub output_header: output_header::Config,
    pub output_final: output_final::Config,
    _hidden: PhantomData<()>,
}

pub struct IntermediateOutputStages {
    pub header: output_header::Stage,
}

pub struct RustLibrary {
    // Basic input
    inventory: Inventory,

    // Model stages (transform and enrich data)
    meta_info: meta_info::Stage,
    model_id_maps: model_id_maps::Stage,
    model_final: model_final::Stage,
    // ...

    // First output stage determining files to be produced
    output_master: output_master::Stage,

    // Most other output stages. Ideally these should have no cross-dependencies,
    // only depending on the models above. The last output stages (e.g., output_master)
    // then integrate all previous outputs to write the actual artifact (into Multibuf)
    // We put them into a separate struct so we don't have to later pass 20+ of them
    // to final.
    output_stages: IntermediateOutputStages,

    // Last output stage(s). Writes a `.cs` file (later possibly other files w. other
    // master stages) into the Multibuf.
    output_final: output_final::Stage,

    // Output
    output: Multibuf,

    // Plugins
    plugins: Vec<Box<dyn RustLibraryPlugin>>,
}

impl RustLibrary {
    pub fn new(inventory: Inventory) -> Self {
        Self::with_config(inventory, RustLibraryConfig::default())
    }

    pub fn builder(inventory: Inventory) -> RustLibraryBuilder {
        RustLibraryBuilder::new(inventory)
    }

    pub(crate) fn with_config(inventory: Inventory, config: RustLibraryConfig) -> Self {
        Self {
            inventory,
            meta_info: meta_info::Stage::new(config.meta_info),
            model_id_maps: model_id_maps::Stage::new(config.model_id_maps),
            model_final: model_final::Stage::new(config.model_final),
            output_master: output_master::Stage::new(config.output_master),
            output_stages: IntermediateOutputStages { header: output_header::Stage::new(config.output_header) },
            output_final: output_final::Stage::new(config.output_final),
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

    fn plugin_post_model_pass(&mut self) {
        let post_model = PostModelPass::default();
        for plugin in self.plugins.iter_mut() {
            plugin.post_model(&mut self.inventory, post_model);
        }
    }

    fn plugin_post_output_pass(&mut self) {
        let post_output = PostOutputPass::default();
        for plugin in self.plugins.iter_mut() {
            plugin.post_output(&mut self.output, post_output);
        }
    }

    pub fn process(mut self) -> Result<Multibuf, Error> {
        self.plugin_init_pass();

        // Model stages
        self.meta_info.process(&mut self.inventory)?;
        self.model_id_maps.process(&mut self.inventory)?;
        self.model_final.process(&mut self.inventory)?;
        self.plugin_post_model_pass();

        // Output stages
        self.output_master.process(&mut self.inventory)?;
        self.output_stages.header.process(&mut self.inventory, &self.output_master, &self.meta_info)?;
        self.plugin_post_output_pass();

        // Final output stage(s)
        self.output_final
            .process(&mut self.inventory, &mut self.output, &self.output_master, &self.output_stages)?;

        Ok(self.output)
    }
}
