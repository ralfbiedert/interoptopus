//! Adds overload function IDs to service ctors and methods.
//!
//! Uses the immutable `sources` to discover
//! overloads for each original function. Discovered overloads are added to
//! the renderable `ctors` / `methods` lists (which may have been filtered
//! by visibility in `service::all`).
//!
//! This pass also handles conflict detection: for non-async overloads, if stripping
//! the self-arg produces the same signature as the base method, the overload is
//! skipped (since C# would see duplicate method signatures).

use crate::lang::FunctionId;
use crate::lang::functions::FunctionKind;
use crate::lang::functions::overload::OverloadKind;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};

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
        services: &mut model::common::service::all::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        let mut method_rebuilds: Vec<(crate::lang::ServiceId, Vec<FunctionId>)> = Vec::new();

        for (&service_id, service) in services.iter() {
            if let Some(new_methods) = rebuild_with_overloads(&service.sources.methods, &service.methods, fns_all, types) {
                method_rebuilds.push((service_id, new_methods));
            }
        }

        for (service_id, new_methods) in method_rebuilds {
            if let Some(service) = services.get_mut(service_id) {
                service.methods = new_methods;
                outcome.changed();
            }
        }

        Ok(outcome)
    }
}

/// Discovers overloads for each original in `sources` and builds the renderable list.
///
/// Starts from the current `renderable` list, then for each source original that
/// isn't already represented, adds any public overloads found in `fns_all`.
fn rebuild_with_overloads(
    sources: &[FunctionId],
    renderable: &[FunctionId],
    fns_all: &model::common::fns::all::Pass,
    types: &model::common::types::all::Pass,
) -> Option<Vec<FunctionId>> {
    let mut result: Vec<FunctionId> = renderable.to_vec();
    let mut changed = false;

    for &source_id in sources {
        let Some(func) = fns_all.get(source_id) else { continue };
        if !matches!(func.kind, FunctionKind::Original) {
            continue;
        }

        for (overload_id, overload_fn) in fns_all.overloads_for(source_id) {
            if result.contains(overload_id) {
                continue;
            }

            // Skip non-async overloads that would produce a duplicate service method.
            // Check against the original AND all overloads already in the result list.
            let dominated = match &overload_fn.kind {
                FunctionKind::Overload(o) if !matches!(o.kind, OverloadKind::Async(_)) => {
                    service_args_equivalent(&func.signature.arguments, &overload_fn.signature.arguments, types)
                        || result.iter().any(|&existing_id| {
                            fns_all
                                .get(existing_id)
                                .is_some_and(|existing| service_args_equivalent(&existing.signature.arguments, &overload_fn.signature.arguments, types))
                        })
                }
                _ => false,
            };
            if dominated {
                continue;
            }

            result.push(*overload_id);
            changed = true;
        }
    }

    if changed { Some(result) } else { None }
}

/// Checks if two function signatures are equivalent after stripping the self-arg
/// (first argument). This is used to detect overloads that would produce duplicate
/// service methods in C#.
fn service_args_equivalent(
    base_args: &[crate::lang::functions::Argument],
    overload_args: &[crate::lang::functions::Argument],
    types: &model::common::types::all::Pass,
) -> bool {
    let base_rest = if base_args.len() > 1 { &base_args[1..] } else { &[] };
    let overload_rest = if overload_args.len() > 1 { &overload_args[1..] } else { &[] };

    if base_rest.len() != overload_rest.len() {
        return false;
    }

    base_rest.iter().zip(overload_rest.iter()).all(|(a, b)| {
        let a_name = types.get(a.ty).map(|t| &t.name);
        let b_name = types.get(b.ty).map(|t| &t.name);
        a.name == b.name && a_name == b_name
    })
}
