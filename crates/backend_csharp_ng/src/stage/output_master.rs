//! Main output configuration.

use crate::stage::{output_director, output_header};
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;

#[derive(Default)]
pub struct Config {}

pub struct Stage {}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(&mut self, _: &Inventory, output: &mut Multibuf, output_director: &output_director::Stage, output_header: &output_header::Stage) {
        let dispatch = output_director.dispatch();
    }
}
