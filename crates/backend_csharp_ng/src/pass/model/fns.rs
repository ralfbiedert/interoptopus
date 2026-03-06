//! Maps functions from Rust to C#.

use crate::lang::function::{Argument, Function, FunctionKind, Overload, Signature};
use crate::lang::meta::Visibility;
use crate::model::FunctionId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use interoptopus::inventory::Functions;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    functions: HashMap<FunctionId, Function>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, functions: Default::default() }
    }

    pub fn process(&mut self, pass_meta: &mut crate::pass::PassMeta, id_map: &mut model::id::Pass, rs_functions: &Functions) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, rust_fn) in rs_functions {
            // Create C# FunctionId from Rust FunctionId
            let cs_id = FunctionId::from_id(rust_id.id());

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

            // Create the C# signature
            let cs_signature = Signature { arguments: cs_arguments, rval: cs_rval };

            // Map visibility
            let cs_visibility = match rust_fn.visibility {
                interoptopus::lang::meta::Visibility::Public => Visibility::Public,
                interoptopus::lang::meta::Visibility::Private => Visibility::Private,
            };

            // Create a single overload representing the original Rust function
            let overload = Overload { signature: cs_signature, kind: FunctionKind::RustFunction };

            // Create the C# function with one overload
            let cs_function = Function { name: rust_fn.name.clone(), overloads: vec![overload] };

            id_map.set_fns(*rust_id, cs_id);
            self.functions.insert(cs_id, cs_function);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FunctionId, &Function)> {
        self.functions.iter()
    }
}
