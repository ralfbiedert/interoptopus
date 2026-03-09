//! Maps Rust function pointers to C# delegates.

use crate::lang::function::Signature;
use crate::lang::types::TypeKind;
use crate::model::TypeId;
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{model, ModelResult, PassInfo};
use crate::{skip_mapped, try_extract_kind};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::id::Pass,
        kinds: &mut model::types::kind::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            skip_mapped!(kinds, rust_id);
            let rust_signature = try_extract_kind!(ty, FnPointer);
            let cs_id = TypeId::from_id(rust_id.id());

            // Try to convert the signature's return type and all argument types
            let Some(cs_rval) = id_map.ty(rust_signature.rval) else {
                // Return type not yet mapped, skip for now
                pass_meta.lost_found.missing(self.info, crate::pass::MissingItem::RustType(rust_signature.rval));
                outcome = Changed;
                continue;
            };

            let mut cs_arguments = Vec::new();
            let mut all_args_available = true;

            for rust_arg in &rust_signature.arguments {
                let Some(cs_arg_type) = id_map.ty(rust_arg.ty) else {
                    // Argument type not yet mapped, skip this delegate for now
                    pass_meta.lost_found.missing(self.info, crate::pass::MissingItem::RustType(rust_arg.ty));
                    all_args_available = false;
                    break;
                };

                cs_arguments.push(crate::lang::function::Argument { name: rust_arg.name.clone(), ty: cs_arg_type });
            }

            if !all_args_available {
                continue;
            }

            // All types available, create the delegate signature
            let cs_signature = Signature { arguments: cs_arguments, rval: cs_rval };

            kinds.set_kind(cs_id, TypeKind::Delegate(cs_signature));
            outcome.changed();
        }

        Ok(outcome)
    }
}
