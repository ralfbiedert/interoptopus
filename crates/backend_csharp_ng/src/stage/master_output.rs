//! Main output configuration.

use interoptopus::inventory::Inventory;
use std::marker::PhantomData;

#[derive(Default)]
pub struct Config {
    _hidden: PhantomData<()>,
}

pub struct Stage {}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(&mut self, inventory: &Inventory) {}
}
