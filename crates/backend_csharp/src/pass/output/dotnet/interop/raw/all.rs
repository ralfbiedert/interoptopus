//! Collects rendered raw trampoline methods (sync and async) into a single lookup
//! keyed by `FunctionId`.
//!
//! Reads from the sub-passes `sync_fn::Pass` and `async_fn::Pass` and merges
//! their results in trampoline entry order.

use crate::lang::FunctionId;
use crate::pass::{OutputResult, PassInfo, model, output};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    methods: HashMap<FunctionId, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, methods: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        trampoline_model: &model::dotnet::trampoline::Pass,
        sync_fn_pass: &output::dotnet::interop::raw::sync_fn::Pass,
        async_fn_pass: &output::dotnet::interop::raw::async_fn::Pass,
    ) -> OutputResult {
        for entry in trampoline_model.entries() {
            if let Some(m) = sync_fn_pass.get(entry.fn_id) {
                self.methods.insert(entry.fn_id, m.to_string());
            } else if let Some(m) = async_fn_pass.get(entry.fn_id) {
                self.methods.insert(entry.fn_id, m.to_string());
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn get(&self, fn_id: FunctionId) -> Option<&str> {
        self.methods.get(&fn_id).map(String::as_str)
    }
}
