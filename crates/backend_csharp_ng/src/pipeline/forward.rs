use crate::stage::type_id_mapping;
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;

#[derive(Default)]
pub struct ForwardConfig {
    pub type_id_mapping: type_id_mapping::Config,
    pub type_id_mapping2: type_id_mapping::Config,
    pub type_id_mapping3: type_id_mapping::Config,
    pub type_id_mapping4: type_id_mapping::Config,
}

pub struct ForwardPipeline {
    // Basic input
    inventory: Inventory,

    // Stages
    type_id_mappings: type_id_mapping::Stage,
    type_id_mappings2: type_id_mapping::Stage,
    type_id_mappings3: type_id_mapping::Stage,
    type_id_mappings4: type_id_mapping::Stage,

    // Output
    output: Multibuf,
}

impl ForwardPipeline {
    pub fn new(inventory: Inventory) -> Self {
        Self::with_config(inventory, ForwardConfig::default())
    }

    pub fn with_config(inventory: Inventory, config: ForwardConfig) -> Self {
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
