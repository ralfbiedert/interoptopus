//! Last output step where a buffer is fully materialized.

use crate::stage::{ProcessError, output_header, output_master};
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;

#[derive(Default)]
pub struct Config {}

pub struct Stage {}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self {}
    }

    pub fn process(&mut self, _: &Inventory, output: &mut Multibuf, output_master: &output_master::Stage, output_header: &output_header::Stage) -> ProcessError {
        let dispatch = output_master.dispatch();
        Ok(())
    }
}
