mod builder;

use crate::stage::{output_director, type_id_mapping};
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

use crate::pipeline::forward::builder::RustPluginBuilder;

#[derive(Default)]
pub struct RustPluginConfig {
    type_id_mapping: type_id_mapping::Config,
    type_id_mapping2: type_id_mapping::Config,
    type_id_mapping3: type_id_mapping::Config,
    type_id_mapping4: type_id_mapping::Config,
    output_director: output_director::Config,
    _hidden: PhantomData<()>,
}

pub struct RustPlugin {
    // Basic input
    inventory: Inventory,

    // Model stages (transform and enrich data)
    type_id_mappings: type_id_mapping::Stage,
    type_id_mappings2: type_id_mapping::Stage,
    type_id_mappings3: type_id_mapping::Stage,
    type_id_mappings4: type_id_mapping::Stage,

    // Output stages
    output_director: output_director::Stage,
    //
    // todo ... now add regular stages, but should in theory be
    // runnable in any order at this stage w.r.t. data model
    // (only dependency needed should be for assembling files)
    // For example
    // - write import definitions in each multibuf section

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
            type_id_mappings: type_id_mapping::Stage::new(config.type_id_mapping),
            type_id_mappings2: type_id_mapping::Stage::new(config.type_id_mapping2),
            type_id_mappings3: type_id_mapping::Stage::new(config.type_id_mapping3),
            type_id_mappings4: type_id_mapping::Stage::new(config.type_id_mapping4),
            output_director: output_director::Stage::new(config.output_director),
            output: Multibuf::default(),
        }
    }

    pub fn process(mut self) -> Multibuf {
        self.type_id_mappings.process(&mut self.inventory);
        self.output
    }
}
