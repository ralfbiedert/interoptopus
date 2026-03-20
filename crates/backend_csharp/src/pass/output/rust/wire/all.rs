//! Assembles all wire output into a single list of rendered strings per output file.
//!
//! Combines results from the `wire_type` pass (`WireOf*` structs) and the
//! `helper_classes` pass (nested managed classes) into a unified, sorted list
//! that the final assembly pass can insert into each output file.

use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, output};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    wires: HashMap<Output, Vec<String>>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, wires: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        wire_types: &output::rust::wire::wire_type::Pass,
        helper_classes: &output::rust::wire::helper_classes::Pass,
    ) -> OutputResult {
        for file in output_master.outputs_of(FileType::Csharp) {
            let mut combined = Vec::new();

            if let Some(types) = wire_types.wire_types_for(file) {
                combined.extend(types.iter().cloned());
            }
            if let Some(classes) = helper_classes.helper_classes_for(file) {
                combined.extend(classes.iter().cloned());
            }

            combined.sort();
            self.wires.insert(file.clone(), combined);
        }

        Ok(())
    }

    #[must_use]
    pub fn wires_for(&self, output: &Output) -> Option<&[String]> {
        self.wires.get(output).map(std::vec::Vec::as_slice)
    }
}
