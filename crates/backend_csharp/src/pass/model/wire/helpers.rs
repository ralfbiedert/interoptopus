//! Discovers the `interoptopus_wire_create` and `interoptopus_wire_destroy` helper
//! functions emitted by `builtins_wire!` for the `WireBuffer` pattern type.
//!
//! This pass scans the Rust inventory for those functions and stores their
//! `FunctionId`s so that the `wire_buffer` output pass can look up the
//! entry-point names and emit the nested `WireInterop` imports.

use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo};
use interoptopus::inventory::{FunctionId, Functions};

#[derive(Default)]
pub struct Config {}

#[derive(Clone, Debug)]
pub struct WireHelpers {
    pub create_fn: FunctionId,
    pub destroy_fn: FunctionId,
}

pub struct Pass {
    info: PassInfo,
    helpers: Option<WireHelpers>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, helpers: None }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, rs_functions: &Functions) -> ModelResult {
        if self.helpers.is_some() {
            return Ok(Unchanged);
        }

        let mut create = None;
        let mut destroy = None;

        for (fn_id, rust_fn) in rs_functions {
            if rust_fn.name.starts_with("interoptopus_wire_create") {
                create = Some(*fn_id);
            } else if rust_fn.name.starts_with("interoptopus_wire_destroy") {
                destroy = Some(*fn_id);
            }
        }

        let mut outcome = Unchanged;

        if let (Some(c), Some(d)) = (create, destroy) {
            self.helpers = Some(WireHelpers { create_fn: c, destroy_fn: d });
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn helpers(&self) -> Option<&WireHelpers> {
        self.helpers.as_ref()
    }
}
