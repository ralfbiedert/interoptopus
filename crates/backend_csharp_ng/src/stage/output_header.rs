//! Main output configuration.

use crate::stage::output_director;
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;
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

    pub fn process(&mut self, _: &Inventory, output: &mut Multibuf, output_director: &output_director::Stage) {
        let dispatch = output_director.dispatch();
    }
}
