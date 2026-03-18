//! Discovers the `interoptopus_wire_create` and `interoptopus_wire_destroy` helper
//! functions emitted by `builtins_wire!` for the `WireBuffer` pattern type.
//!
//! This pass scans the Rust inventory for those functions and stores their entry-point
//! names so that the `wire_buffer` output pass can emit the nested `WireInterop` imports.

use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo};
use interoptopus::inventory::Functions;

#[derive(Default)]
pub struct Config {}

#[derive(Clone, Debug)]
pub struct WireHelpers {
    pub create_entry_point: String,
    pub destroy_entry_point: String,
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

        for rust_fn in rs_functions.values() {
            if rust_fn.name.starts_with("interoptopus_wire_create") {
                create = Some(rust_fn.name.clone());
            } else if rust_fn.name.starts_with("interoptopus_wire_destroy") {
                destroy = Some(rust_fn.name.clone());
            }
        }

        let mut outcome = Unchanged;

        if let (Some(c), Some(d)) = (create, destroy) {
            self.helpers = Some(WireHelpers { create_entry_point: c, destroy_entry_point: d });
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn helpers(&self) -> Option<&WireHelpers> {
        self.helpers.as_ref()
    }
}
