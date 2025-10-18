use crate::stage::type_id_mapping;
use interoptopus::inventory::Inventory;

#[derive(Default)]
pub struct Config {
    pub type_id_mapping: type_id_mapping::Config,
    pub type_id_mapping2: type_id_mapping::Config,
    pub type_id_mapping3: type_id_mapping::Config,
    pub type_id_mapping4: type_id_mapping::Config,
}

pub struct Interop {
    // Basic input
    inventory: Inventory,

    // Stages
    type_id_mappings: type_id_mapping::Stage,
    type_id_mappings2: type_id_mapping::Stage,
    type_id_mappings3: type_id_mapping::Stage,
    type_id_mappings4: type_id_mapping::Stage,
}

impl Interop {
    pub fn new(inventory: Inventory) -> Self {
        Self::with_config(inventory, Config::default())
    }

    pub fn with_config(inventory: Inventory, config: Config) -> Self {
        Self {
            inventory,
            type_id_mappings: type_id_mapping::Stage::new(config.type_id_mapping),
            type_id_mappings2: type_id_mapping::Stage::new(config.type_id_mapping2),
            type_id_mappings3: type_id_mapping::Stage::new(config.type_id_mapping3),
            type_id_mappings4: type_id_mapping::Stage::new(config.type_id_mapping4),
        }
    }

    pub fn process(&mut self) {
        self.type_id_mappings.process(&mut self.inventory);
    }
}
