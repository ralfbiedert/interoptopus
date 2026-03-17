//! Discovers the `interoptopus_wire_destroy` helper function for the `WireBuffer` pattern type.
//!
//! The `builtins_wire!` macro emits a single helper function with a fixed name prefix.
//! This pass scans the Rust inventory for it and stores the entry-point name
//! so that the wire_buffer output pass can emit the nested `InteropHelper` import.

use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo};
use interoptopus::inventory::Functions;

#[derive(Default)]
pub struct Config {}

#[derive(Clone, Debug)]
pub struct WireHelpers {
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

        let mut destroy = None;

        for rust_fn in rs_functions.values() {
            if rust_fn.name.starts_with("interoptopus_wire_destroy") {
                destroy = Some(rust_fn.name.clone());
            }
        }

        let mut outcome = Unchanged;

        if let Some(d) = destroy {
            self.helpers = Some(WireHelpers { destroy_entry_point: d });
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn helpers(&self) -> Option<&WireHelpers> {
        self.helpers.as_ref()
    }
}
