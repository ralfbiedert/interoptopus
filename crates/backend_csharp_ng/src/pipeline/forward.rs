use crate::stage::{master_output, type_id_mapping};
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

#[derive(Default)]
pub struct RustPluginConfig {
    pub type_id_mapping: type_id_mapping::Config,
    pub type_id_mapping2: type_id_mapping::Config,
    pub type_id_mapping3: type_id_mapping::Config,
    pub type_id_mapping4: type_id_mapping::Config,
    pub master_output: master_output::Config,
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
    // todo: configure multibuf output / setup somehow via config objects
    // "foo.cs" -> Output::Code({ types:[0xa, 0xb, ...], fns:[0xc,...], ... })
    // "bar.cs" -> Output::Code(...)
    // "helper.csproj" -> Output::BuildDef(...)
    //
    // todo ... now add regular stages, but should in theory be
    // runnable in any order at this stage w.r.t. data model
    // (only dependency needed should be for assembling files)
    // For example
    // - write import definitions in each multibuf section
    //

    // Output
    output: Multibuf,
}

impl RustPlugin {
    pub fn new(inventory: Inventory) -> Self {
        Self::with_config(inventory, RustPluginConfig::default())
    }

    pub fn with_config(inventory: Inventory, config: RustPluginConfig) -> Self {
        Self {
            inventory,
            type_id_mappings: type_id_mapping::Stage::new(config.type_id_mapping),
            type_id_mappings2: type_id_mapping::Stage::new(config.type_id_mapping2),
            type_id_mappings3: type_id_mapping::Stage::new(config.type_id_mapping3),
            type_id_mappings4: type_id_mapping::Stage::new(config.type_id_mapping4),
            output: Multibuf::default(),
        }
    }

    pub fn process(mut self) -> Multibuf {
        self.type_id_mappings.process(&mut self.inventory);
        self.output
    }
}
