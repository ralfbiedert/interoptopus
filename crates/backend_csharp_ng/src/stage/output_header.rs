//! Main output configuration.

use crate::output::{Output, OutputKind};
use crate::stage::{ProcessError, output_master};
use interoptopus::inventory::Inventory;
use interoptopus_backends::render;
use interoptopus_backends::template::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Stage {
    headers: HashMap<Output, String>,
}

impl Stage {
    pub fn new(_: Config) -> Self {
        Self { headers: Default::default() }
    }

    pub fn process(&mut self, _: &Inventory, output_director: &output_master::Stage) -> ProcessError {
        let templates = output_director.templates();
        for output in output_director.outputs_of(OutputKind::Csharp) {
            // let mut context = Context::new();
            // context.insert("foo", "bar");
            // let result = templates.render("header.cs", &context)?;
            // self.headers.insert(output.clone(), result);

            let result = render!(templates, "header.cs", ("foo", "bar"), ("other", "something"))?;
            self.headers.insert(output.clone(), result);
        }
        Ok(())
    }
}
