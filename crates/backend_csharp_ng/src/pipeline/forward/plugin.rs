use crate::Error;
use crate::pipeline::forward::RustPluginBuilder;
use crate::stage::{meta_info, output_final, output_header, output_master, type_id_mapping};
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

#[derive(Default)]
pub struct RustPluginConfig {
    pub meta_info: meta_info::Config,
    pub type_id_mapping: type_id_mapping::Config,
    pub type_id_mapping2: type_id_mapping::Config,
    pub type_id_mapping3: type_id_mapping::Config,
    pub type_id_mapping4: type_id_mapping::Config,
    pub output_master: output_master::Config,
    pub output_header: output_header::Config,
    pub output_final: output_final::Config,
    _hidden: PhantomData<()>,
}

pub struct IntermediateOutputStages {
    pub header: output_header::Stage,
}

pub struct RustPlugin {
    // Basic input
    inventory: Inventory,

    // Model stages (transform and enrich data)
    meta_info: meta_info::Stage,
    type_id_mappings: type_id_mapping::Stage,
    type_id_mappings2: type_id_mapping::Stage,
    type_id_mappings3: type_id_mapping::Stage,
    type_id_mappings4: type_id_mapping::Stage,
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
}

impl RustPlugin {
    pub fn new(inventory: Inventory) -> Self {
        Self::with_config(inventory, RustPluginConfig::default())
    }

    pub fn builder(inventory: Inventory) -> RustPluginBuilder {
        RustPluginBuilder::new(inventory)
    }

    pub(crate) fn with_config(inventory: Inventory, config: RustPluginConfig) -> Self {
        Self {
            inventory,
            meta_info: meta_info::Stage::new(config.meta_info),
            type_id_mappings: type_id_mapping::Stage::new(config.type_id_mapping),
            type_id_mappings2: type_id_mapping::Stage::new(config.type_id_mapping2),
            type_id_mappings3: type_id_mapping::Stage::new(config.type_id_mapping3),
            type_id_mappings4: type_id_mapping::Stage::new(config.type_id_mapping4),
            output_master: output_master::Stage::new(config.output_master),
            output_stages: IntermediateOutputStages { header: output_header::Stage::new(config.output_header) },
            output_final: output_final::Stage::new(config.output_final),
            output: Multibuf::default(),
        }
    }

    pub fn process(mut self) -> Result<Multibuf, Error> {
        self.meta_info.process(&mut self.inventory)?;
        self.type_id_mappings.process(&mut self.inventory)?;
        self.output_master.process(&mut self.inventory)?;
        self.output_stages.header.process(&mut self.inventory, &self.output_master, &self.meta_info)?;
        self.output_final
            .process(&mut self.inventory, &mut self.output, &self.output_master, &self.output_stages)?;

        Ok(self.output)
    }
}
