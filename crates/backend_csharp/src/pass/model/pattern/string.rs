//! Discovers `interoptopus_string_create` / `interoptopus_string_destroy` /
//! `interoptopus_string_clone` helper functions for the `Utf8String` pattern type.
//!
//! The `builtins_string!` macro emits three helper functions with fixed names.
//! This pass scans the Rust inventory for them and stores the entry-point names
//! so that the utf8string output pass can emit the nested `InteropHelper` imports.

use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo};
use interoptopus::inventory::Functions;

#[derive(Default)]
pub struct Config {}

#[derive(Clone, Debug)]
pub struct StringHelpers {
    pub create_entry_point: String,
    pub destroy_entry_point: String,
    pub clone_entry_point: String,
}

pub struct Pass {
    info: PassInfo,
    helpers: Option<StringHelpers>,
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
        let mut clone = None;

        for (_fn_id, rust_fn) in rs_functions {
            if rust_fn.name.starts_with("interoptopus_string_create") {
                create = Some(rust_fn.name.clone());
            } else if rust_fn.name.starts_with("interoptopus_string_destroy") {
                destroy = Some(rust_fn.name.clone());
            } else if rust_fn.name.starts_with("interoptopus_string_clone") {
                clone = Some(rust_fn.name.clone());
            }
        }

        let mut outcome = Unchanged;

        if let (Some(c), Some(d), Some(cl)) = (create, destroy, clone) {
            self.helpers = Some(StringHelpers { create_entry_point: c, destroy_entry_point: d, clone_entry_point: cl });
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn helpers(&self) -> Option<&StringHelpers> {
        self.helpers.as_ref()
    }
}
