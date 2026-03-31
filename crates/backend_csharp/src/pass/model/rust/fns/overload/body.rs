//! Produces body and async overloads for functions with delegate, `IntPtr`, or
//! `AsyncCallback` arguments.
//!
//! For each original function this pass checks two conditions:
//! - **Body**: Has delegate class args (but no async callback) → produces an overload
//!   with delegate→signature, intptr→ref transforms, registered as `OverloadKind::Body`.
//! - **Async**: Last arg is `AsyncCallback<T>` → produces an overload that drops the
//!   callback, applies the same arg transforms to remaining args, and registers as
//!   `OverloadKind::Async` with `RvalTransform::AsyncTask`.
//!
//! When both conditions are true, only the Async overload is emitted. Emitting both
//! would create two C# methods with identical parameter signatures differing only in
//! return type, which C# does not allow.
//!
//! For async overloads, this pass also creates `Task` / `Task<T>` types in the type
//! system and sets the overload function's rval to the registered Task `TypeId`.

use crate::lang::functions::overload::{ArgTransform, FnTransforms, Overload, OverloadKind, RvalTransform};
use crate::lang::functions::{Argument, Function, FunctionKind, Signature};
use crate::lang::meta::{Emission, Visibility};
use crate::lang::types::OverloadFamily;
use crate::lang::types::kind::task::Task;
use crate::lang::types::kind::{DelegateKind, Primitive, TypeKind, TypePattern};
use crate::lang::types::{Decorators, Type};
use crate::lang::{FunctionId, TypeId};
use crate::pass::Outcome::Unchanged;
use crate::pass::model::rust::fns::overload::{IntPtrEligibility, derive_overload_id, intptr_eligibility, is_eligible_intptr, service_intptr_target};
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashSet;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    processed: HashSet<FunctionId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, processed: HashSet::default() }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        fns_all: &mut model::common::fns::all::Pass,
        kinds: &mut model::common::types::kind::Pass,
        names: &mut model::common::types::names::Pass,
        types: &mut model::common::types::all::Pass,
        type_overloads: &model::rust::types::overload::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Collect originals first to avoid borrowing `fns_all` mutably while iterating.
        // Sort by ID for deterministic iteration order.
        let mut originals: Vec<_> = fns_all.originals().map(|(&id, f)| (id, f.clone())).collect();
        originals.sort_by_key(|(id, _)| *id);

        for &(original_id, ref original_fn) in &originals {
            if self.processed.contains(&original_id) {
                continue;
            }

            let args = &original_fn.signature.arguments;

            // Detect async: last arg is AsyncCallback<Result<T,E>>
            let async_result_ty = detect_async_callback(args, types);

            // Determine which args to check for body transforms (exclude callback if async)
            let transformable_args = match async_result_ty {
                Some(_) => &args[..args.len() - 1],
                None => &args[..],
            };

            let has_delegate = transformable_args.iter().any(|a| is_delegate_class(a.ty, types));
            let has_service = transformable_args.iter().any(|a| service_intptr_target(a.ty, types).is_some());

            // If any IntPtr argument's target type is not yet resolved, defer.
            let has_any_unknown = transformable_args
                .iter()
                .any(|a| matches!(intptr_eligibility(a.ty, types), IntPtrEligibility::Unknown));
            if has_any_unknown {
                continue;
            }

            // Nothing to do for this function
            if async_result_ty.is_none() && !has_delegate && !has_service {
                self.processed.insert(original_id);
                continue;
            }

            // Wait for type overload families to be available
            if !all_families_ready(transformable_args, types, type_overloads) {
                continue;
            }

            // Also verify async result type is a Result if present
            if let Some(result_ty_id) = async_result_ty {
                let Some(result_ty) = types.get(result_ty_id) else { continue };
                if !matches!(&result_ty.kind, TypeKind::TypePattern(TypePattern::Result(_, _, _))) {
                    self.processed.insert(original_id);
                    continue;
                }
            }

            // Compute arg transforms for the transformable args
            let (overload_args, arg_transforms) = build_arg_transforms(transformable_args, types, type_overloads);

            // Produce Body overload if there are delegate args but NO async callback.
            // When async is present, the Async overload already applies the same arg
            // transforms; emitting both would create two methods differing only in
            // return type, which C# rejects.
            if (has_delegate || has_service) && async_result_ty.is_none() {
                let sig = Signature { arguments: overload_args.clone(), rval: original_fn.signature.rval };
                let id = derive_overload_id(original_id, &sig);
                let transforms = FnTransforms { rval: RvalTransform::PassThrough, args: arg_transforms.clone() };
                let func = Function {
                    emission: original_fn.emission.clone(),
                    name: original_fn.name.clone(),
                    visibility: Visibility::Public,
                    docs: original_fn.docs.clone(),
                    signature: sig,
                    kind: FunctionKind::Overload(Overload { kind: OverloadKind::Body(transforms), base: original_id }),
                };
                fns_all.register(id, func);
                outcome.changed();
            }

            // Produce Async overload if last arg is AsyncCallback
            if let Some(result_ty_id) = async_result_ty {
                // Resolve the Task return type from the Result type
                let task_ty_id = resolve_or_create_task_type(result_ty_id, types, kinds, names);

                // Append a synthetic CancellationToken argument
                let mut async_args = overload_args;
                async_args.push(Argument { name: "_ct".to_string(), ty: crate::lang::types::csharp::CANCELLATION_TOKEN });
                let mut async_transforms = arg_transforms;
                async_transforms.push(ArgTransform::CancellationToken);

                let sig = Signature { arguments: async_args, rval: task_ty_id };
                let id = derive_overload_id(original_id, &sig);
                let transforms = FnTransforms { rval: RvalTransform::AsyncTask(result_ty_id), args: async_transforms };
                let func = Function {
                    emission: original_fn.emission.clone(),
                    name: original_fn.name.clone(),
                    visibility: Visibility::Public,
                    docs: original_fn.docs.clone(),
                    signature: sig,
                    kind: FunctionKind::Overload(Overload { kind: OverloadKind::Async(transforms), base: original_id }),
                };
                fns_all.register(id, func);
                outcome.changed();
            }

            self.processed.insert(original_id);
        }

        Ok(outcome)
    }
}

/// Resolves or creates a `Task` / `Task<T>` type for the given Result type.
///
/// Given a `Result<OkTy, ErrTy>`, creates:
/// - `Task` if `OkTy` is void
/// - `Task<OkName>` otherwise
///
/// Returns the `TypeId` of the Task type.
fn resolve_or_create_task_type(
    result_ty_id: TypeId,
    types: &mut model::common::types::all::Pass,
    kinds: &mut model::common::types::kind::Pass,
    names: &mut model::common::types::names::Pass,
) -> TypeId {
    let result_ty = types.get(result_ty_id);

    let (inner, task_name) = match result_ty.map(|t| &t.kind) {
        Some(TypeKind::TypePattern(TypePattern::Result(ok_ty, _, _))) => {
            let ok_is_void = matches!(types.get(*ok_ty).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
            if ok_is_void {
                (None, "Task".to_string())
            } else {
                let ok_name = types.get(*ok_ty).map_or_else(|| "void".to_string(), |t| t.name.clone());
                (Some(*ok_ty), format!("Task<{ok_name}>"))
            }
        }
        _ => (None, "Task".to_string()),
    };

    // Derive a stable TypeId from the result type
    let task_ty_id = TypeId::from_id(result_ty_id.id().derive(0x_7461_736B_5F74_7970)); // "task_typ"

    // Only register if not already present
    if types.get(task_ty_id).is_none() {
        let kind = TypeKind::Task(Task { inner });
        kinds.set(task_ty_id, kind.clone());
        names.set(task_ty_id, task_name.clone());
        types.set(
            task_ty_id,
            Type { emission: Emission::Builtin, name: task_name, visibility: Visibility::Public, docs: Vec::new(), kind, decorators: Decorators::default() },
        );
    }

    task_ty_id
}

/// If the last arg is `AsyncCallback<T>` where T is a Result, returns the `TypeId` of T.
fn detect_async_callback(args: &[Argument], types: &model::common::types::all::Pass) -> Option<TypeId> {
    let last = args.last()?;
    let ty = types.get(last.ty)?;
    match &ty.kind {
        TypeKind::TypePattern(TypePattern::AsyncCallback(t)) => Some(*t),
        _ => None,
    }
}

fn all_families_ready(args: &[Argument], types: &model::common::types::all::Pass, type_overloads: &model::rust::types::overload::all::Pass) -> bool {
    args.iter().all(|arg| {
        if is_delegate_class(arg.ty, types) {
            matches!(type_overloads.get(arg.ty), Some(OverloadFamily::Delegate(_)))
        } else if is_eligible_intptr(arg.ty, types) {
            matches!(type_overloads.get(arg.ty), Some(OverloadFamily::Pointer(_)))
        } else {
            true
        }
    })
}

fn build_arg_transforms(
    args: &[Argument],
    types: &model::common::types::all::Pass,
    type_overloads: &model::rust::types::overload::all::Pass,
) -> (Vec<Argument>, Vec<ArgTransform>) {
    let mut overload_args = Vec::new();
    let mut transforms = Vec::new();

    for arg in args {
        if let Some(OverloadFamily::Delegate(family)) = is_delegate_class(arg.ty, types).then(|| type_overloads.get(arg.ty)).flatten() {
            overload_args.push(Argument { name: arg.name.clone(), ty: family.signature });
            transforms.push(ArgTransform::WrapDelegate);
        } else if let Some(OverloadFamily::Pointer(family)) = is_eligible_intptr(arg.ty, types).then(|| type_overloads.get(arg.ty)).flatten() {
            overload_args.push(Argument { name: arg.name.clone(), ty: family.by_ref });
            transforms.push(ArgTransform::Ref);
        } else if let Some(service_ty) = service_intptr_target(arg.ty, types) {
            overload_args.push(Argument { name: arg.name.clone(), ty: service_ty });
            transforms.push(ArgTransform::Service);
        } else {
            overload_args.push(Argument { name: arg.name.clone(), ty: arg.ty });
            transforms.push(ArgTransform::PassThrough);
        }
    }

    (overload_args, transforms)
}

fn is_delegate_class(ty: TypeId, types: &model::common::types::all::Pass) -> bool {
    matches!(types.get(ty).map(|t| &t.kind), Some(TypeKind::Delegate(d)) if d.kind == DelegateKind::Class)
}
