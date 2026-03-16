//! Discovers `interoptopus_vec_create_*` / `interoptopus_vec_destroy_*` helper
//! functions and associates them with their corresponding `Vec<T>` pattern type.
//!
//! For each `Vec<T>` type in the inventory the `builtins_vec!` macro emits a
//! create and a destroy function. This pass scans the Rust inventory functions,
//! matches them by name prefix and signature, resolves both the Vec type and the
//! function entry-point names to C# IDs, and stores the result so that output
//! passes can emit the nested `InteropHelper` imports inside each Vec class.

use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::try_resolve;
use interoptopus::inventory::{Functions, Types};
use interoptopus::lang::types::TypeKind;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

#[derive(Clone, Debug)]
pub struct VecHelpers {
    pub create_entry_point: String,
    pub destroy_entry_point: String,
}

pub struct Pass {
    info: PassInfo,
    /// Maps a C# Vec `TypeId` to its helper function entry points.
    helpers: HashMap<TypeId, VecHelpers>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, helpers: HashMap::default() }
    }

    pub fn process(&mut self, pass_meta: &mut crate::pass::PassMeta, id_map: &model::id_map::Pass, rs_functions: &Functions, rs_types: &Types) -> ModelResult {
        let mut outcome = Unchanged;

        // Collect create entry points keyed by the Vec TypeId they produce.
        // The create function's last param is `&mut MaybeUninit<Vec<T>>` which in
        // the type model is `ReadWritePointer(vec_type_id)`. We resolve the pointer
        // to get the inner Vec TypeId.
        let mut creates: HashMap<interoptopus::inventory::TypeId, String> = HashMap::new();
        // Collect destroy entry points keyed by the Vec TypeId they consume.
        // The destroy function's first param is `Vec<T>` directly.
        let mut destroys: HashMap<interoptopus::inventory::TypeId, String> = HashMap::new();

        for rust_fn in rs_functions.values() {
            if rust_fn.name.starts_with("interoptopus_vec_create") {
                if let Some(last_arg) = rust_fn.signature.arguments.last()
                    && let Some(ty) = rs_types.get(&last_arg.ty)
                    && let TypeKind::ReadWritePointer(inner_id) = &ty.kind
                {
                    creates.insert(*inner_id, rust_fn.name.clone());
                }
            } else if rust_fn.name.starts_with("interoptopus_vec_destroy")
                && let Some(first_arg) = rust_fn.signature.arguments.first()
            {
                destroys.insert(first_arg.ty, rust_fn.name.clone());
            }
        }

        // Match create and destroy by Vec TypeId and map to C# IDs.
        for (rust_vec_id, destroy_name) in &destroys {
            let cs_vec_id = try_resolve!(id_map.ty(*rust_vec_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_vec_id));

            if self.helpers.contains_key(&cs_vec_id) {
                continue;
            }

            let Some(create_name) = creates.get(rust_vec_id) else {
                continue;
            };

            self.helpers
                .insert(cs_vec_id, VecHelpers { create_entry_point: create_name.clone(), destroy_entry_point: destroy_name.clone() });
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn helpers(&self, cs_vec_id: TypeId) -> Option<&VecHelpers> {
        self.helpers.get(&cs_vec_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &VecHelpers)> {
        self.helpers.iter()
    }
}
