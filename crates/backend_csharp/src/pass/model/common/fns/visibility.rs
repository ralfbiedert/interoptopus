//! Reflows function visibility based on signature types and service membership.
//!
//! A function's visibility is set to the most restrictive visibility among all
//! types in its signature (arguments + return type). Additionally, all raw
//! interop functions that belong to a service (constructor, method, or destructor
//! source) are forced to `internal` — users interact with the service class, not
//! the raw interop functions.

use crate::lang::FunctionId;
use crate::lang::meta::Visibility;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashSet;

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
        _pass_meta: &mut crate::pass::PassMeta,
        fns_all: &mut model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        services: &model::common::service::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Collect all function IDs that belong to any service, including their overloads.
        let mut service_fn_ids: HashSet<FunctionId> = services
            .iter()
            .flat_map(|(_, svc)| {
                svc.sources
                    .ctors
                    .iter()
                    .chain(svc.sources.methods.iter())
                    .chain(std::iter::once(&svc.destructor))
                    .copied()
            })
            .collect();

        let source_ids: Vec<_> = service_fn_ids.iter().copied().collect();
        for source_id in source_ids {
            for (overload_id, _) in fns_all.overloads_for(source_id) {
                service_fn_ids.insert(*overload_id);
            }
        }

        let fn_ids: Vec<_> = fns_all.iter().map(|(&id, _)| id).collect();

        for fn_id in fn_ids {
            let Some(func) = fns_all.get(fn_id) else { continue };

            // Service-related raw interop functions and internal plumbing are always internal.
            let mut target = if service_fn_ids.contains(&fn_id) || func.name.starts_with("interoptopus_") {
                Visibility::Internal
            } else {
                Visibility::Public
            };

            // Additionally clamp to the most restrictive signature type.
            if let Some(rval_ty) = types.get(func.signature.rval) {
                target = target.most_restrictive(&rval_ty.visibility);
            }

            for arg in &func.signature.arguments {
                if let Some(arg_ty) = types.get(arg.ty) {
                    target = target.most_restrictive(&arg_ty.visibility);
                }
            }

            if func.visibility == target {
                continue;
            }

            let func = fns_all.get_mut(fn_id).unwrap();
            func.visibility = target;
            outcome.changed();
        }

        Ok(outcome)
    }
}
