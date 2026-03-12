//! Maps functions from Rust inventory to C# functions.
//!
//! This only contains original functions (the raw functions
//! defined in the original Rust bindings in their basic form,
//! not any overloads we produced afterwards)

use crate::lang::FunctionId;
use crate::lang::functions::{Argument, Function, Signature};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use crate::try_resolve;
use interoptopus::inventory::Functions;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    functions: HashMap<FunctionId, Function>,
}

impl Pass {
    #[must_use] 
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, functions: HashMap::default() }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::id_map::Pass,
        all: &mut model::fns::all::Pass,
        rs_functions: &Functions,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, rust_fn) in rs_functions {
            // Resolve C# FunctionId
            let Some(cs_id) = id_map.fns(*rust_id) else { continue };

            // Skip if we've already processed this function
            if self.functions.contains_key(&cs_id) {
                continue;
            }

            // Translate the signature's return type
            let cs_rval = try_resolve!(id_map.ty(rust_fn.signature.rval), pass_meta, self.info, crate::pass::MissingItem::RustType(rust_fn.signature.rval));

            // Translate all argument types
            let mut cs_arguments = Vec::new();
            let mut all_args_available = true;

            for rust_arg in &rust_fn.signature.arguments {
                let Some(cs_arg_type) = id_map.ty(rust_arg.ty) else {
                    pass_meta.lost_found.missing(self.info, crate::pass::MissingItem::RustType(rust_arg.ty));
                    all_args_available = false;
                    break;
                };

                cs_arguments.push(Argument { name: rust_arg.name.clone(), ty: cs_arg_type });
            }

            // Skip this function if not all argument types are available
            if !all_args_available {
                continue;
            }

            let cs_signature = Signature { arguments: cs_arguments, rval: cs_rval };
            let cs_function = Function { name: rust_fn.name.clone(), signature: cs_signature };

            all.register(cs_id, cs_function.clone());
            self.functions.insert(cs_id, cs_function);
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use] 
    pub fn get(&self, id: FunctionId) -> Option<&Function> {
        self.functions.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FunctionId, &Function)> {
        self.functions.iter()
    }
}
