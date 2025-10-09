mod macros;
mod plugin;

pub mod lang;
pub mod stages;

pub use plugin::Plugin;

use crate::stages::{Stage, todo_functions, type_id_mapping};
use interoptopus::inventory::Inventory;

stage_setup!(
    StageData,
    StageConfig,
    init_stages,
    [
        // Run these stages, in this order:
        type_id_mapping,
        todo_functions,
    ]
);

pub struct Pipeline {
    data: StageData,
    stages: Vec<Box<dyn Stage>>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self::with_config(StageConfig::default())
    }

    pub fn with_config(stage_config: StageConfig) -> Self {
        let stages = init_stages(stage_config);
        Self { stages, data: StageData::default(), plugins: vec![] }
    }

    pub fn register_plugin(&mut self, plugin: impl Plugin + 'static) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn execute(mut self, inventory: &Inventory) {
        for stage in &mut self.stages {
            stage.process(&mut self.data, inventory);
        }

        // TODO, probably want some sort of `Run::Before/After(StageX)` system
        for _ in &mut self.plugins {
            //
        }
    }
}
