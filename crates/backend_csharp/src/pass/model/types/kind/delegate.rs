//! Maps Rust function pointers and named callbacks to C# delegates.
//!
//! Both `FnPointer` and `TypePattern::NamedCallback` become `TypeKind::Delegate`
//! with `DelegateKind::Class`, representing the full delegate wrapper class in C#.

use crate::lang::functions::{Argument, Signature};
use crate::lang::types::kind::{Delegate, DelegateKind, TypeKind};
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{ModelResult, PassInfo, model};
use crate::{skip_mapped, try_resolve};
use interoptopus::lang;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
}

impl Pass {
    #[must_use] 
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() } }
    }

    pub fn process(
        &mut self,
        pass_meta: &mut crate::pass::PassMeta,
        id_map: &model::id_map::Pass,
        kinds: &mut model::types::kind::Pass,
        rs_types: &interoptopus::inventory::Types,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (rust_id, ty) in rs_types {
            skip_mapped!(kinds, id_map, rust_id);

            // Extract the Rust signature from either FnPointer or NamedCallback
            let rust_signature = match &ty.kind {
                lang::types::TypeKind::FnPointer(sig) => sig,
                lang::types::TypeKind::TypePattern(lang::types::TypePattern::NamedCallback(sig)) => sig,
                _ => continue,
            };

            let cs_id = try_resolve!(id_map.ty(*rust_id), pass_meta, self.info, crate::pass::MissingItem::RustType(*rust_id));

            // Try to convert the signature's return type and all argument types
            let Some(cs_rval) = id_map.ty(rust_signature.rval) else {
                pass_meta.lost_found.missing(self.info, crate::pass::MissingItem::RustType(rust_signature.rval));
                outcome = Changed;
                continue;
            };

            let mut cs_arguments = Vec::new();
            let mut all_args_available = true;

            for rust_arg in &rust_signature.arguments {
                let Some(cs_arg_type) = id_map.ty(rust_arg.ty) else {
                    pass_meta.lost_found.missing(self.info, crate::pass::MissingItem::RustType(rust_arg.ty));
                    all_args_available = false;
                    break;
                };

                cs_arguments.push(Argument { name: rust_arg.name.clone(), ty: cs_arg_type });
            }

            if !all_args_available {
                continue;
            }

            let cs_signature = Signature { arguments: cs_arguments, rval: cs_rval };

            kinds.set(cs_id, TypeKind::Delegate(Delegate { kind: DelegateKind::Class, signature: cs_signature }));
            outcome.changed();
        }

        Ok(outcome)
    }
}
