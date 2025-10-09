use crate::StageData;
use interoptopus::inventory::Inventory;

pub mod todo_functions;
pub mod type_id_mapping;

/// A stage that runs in our pipeline.
pub trait Stage {
    fn process(&mut self, data: &mut StageData, inventory: &Inventory);
}
