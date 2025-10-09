use crate::{StageData, stage_without_config};
use interoptopus::inventory::Inventory;

stage_without_config!();

#[derive(Default)]
pub struct Data {}

impl Stage {
    fn process(&mut self, _: &mut StageData, _: &Inventory) {}
}
