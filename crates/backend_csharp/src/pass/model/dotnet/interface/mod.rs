pub mod plugin;
pub mod service;

use crate::lang::TypeId;
use crate::lang::functions::{Argument, Signature};
use crate::lang::types::kind::{Primitive, TypeKind, TypePattern};
use crate::pass::model;
use crate::pass::model::common::types::all::Pass as TypesAll;
use std::collections::HashMap;

/// If the last argument is `AsyncCallback<T>`, returns the inner `TypeId`.
pub(super) fn async_callback_inner(args: &[Argument], types: &TypesAll) -> Option<TypeId> {
    let last = args.last()?;
    let ty = types.get(last.ty)?;
    match &ty.kind {
        TypeKind::TypePattern(TypePattern::AsyncCallback(inner)) => Some(*inner),
        _ => None,
    }
}

/// Returns `"Task"` for void inner types or `"Task<TypeName>"` for value types.
/// Returns `None` if the inner type is not yet resolved.
pub(super) fn task_type_name(inner_id: TypeId, types: &TypesAll) -> Option<String> {
    let is_void = matches!(types.get(inner_id).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
    if is_void {
        Some("Task".to_string())
    } else {
        let name = &types.get(inner_id)?.name;
        Some(format!("Task<{name}>"))
    }
}

/// Builds a C# `Signature` from a function's arguments and return type,
/// handling async functions (replacing `AsyncCallback<T>` return with `Task<T>`
/// and stripping the callback parameter).
///
/// Returns `None` if any referenced type is not yet resolved in the type map.
pub(super) fn csharp_signature(args: &[Argument], rval: TypeId, types: &TypesAll) -> Option<(Signature, String)> {
    let async_inner = async_callback_inner(args, types);

    let rval_name = if let Some(inner_id) = async_inner {
        task_type_name(inner_id, types)?
    } else {
        let ty = types.get(rval)?;
        ty.name.clone()
    };

    let arg_count = if async_inner.is_some() { args.len().saturating_sub(1) } else { args.len() };
    let arguments: Vec<Argument> = args.iter().take(arg_count).cloned().collect();
    let rval_type_id = if let Some(inner_id) = async_inner { inner_id } else { rval };

    Some((Signature { arguments, rval: rval_type_id }, rval_name))
}

/// Builds a map from C# method name → target service class name for service methods
/// that return a service type directly.
///
/// This map is used by both the service and plugin interface passes to upgrade
/// `nint`/`ResultNintError`/`Task<nint>` return types to proper service class names
/// for Result-wrapped and async siblings.
pub(super) fn build_service_return_map(
    services: &model::common::service::all::Pass,
    fns_all: &model::common::fns::all::Pass,
    types: &TypesAll,
) -> HashMap<String, String> {
    use interoptopus_backends::casing::{rust_to_pascal, service_method_name};

    let mut map = HashMap::new();

    for (_svc_id, svc) in services.iter() {
        let Some(type_info) = types.get(svc.ty) else { continue };
        let type_name = &type_info.name;

        for &fn_id in &svc.methods {
            let Some(func) = fns_all.get(fn_id) else { continue };
            if let Some(ty) = types.get(func.signature.rval) {
                if matches!(&ty.kind, TypeKind::Service) {
                    let method_name = service_method_name(type_name, &func.name);
                    map.insert(method_name, ty.name.clone());
                }
            }
        }
    }

    // Also check bare (non-service) functions for direct service returns.
    // Use PascalCase function name as the key (matching IPlugin method naming).
    for (_, func) in fns_all.originals() {
        if let Some(ty) = types.get(func.signature.rval) {
            if matches!(&ty.kind, TypeKind::Service) {
                let pascal_name = rust_to_pascal(&func.name);
                map.insert(pascal_name, ty.name.clone());
            }
        }
    }

    map
}

/// If a sibling method with the same base name returns a service type directly,
/// upgrade this method's return type to use the service class name instead of `nint`.
///
/// Matching rules (by method name suffix convention):
/// - `FooResult` → replace `ResultNintError` → `ResultNestedBError`
/// - `FooAsync` → replace `Task<nint>` → `Task<NestedB>`
/// - `FooResultAsync` / `FooAsyncResult` → replace accordingly
pub(super) fn upgrade_service_return(method_name: &str, rval_name: &str, service_return_map: &HashMap<String, String>) -> String {
    let suffixes = ["ResultAsync", "AsyncResult", "Result", "Async"];

    for suffix in &suffixes {
        if let Some(base) = method_name.strip_suffix(suffix) {
            if let Some(service_name) = service_return_map.get(base) {
                // Replace both "Nint" (PascalCase in composed names like ResultNintError)
                // and "nint" (in generic positions like Task<nint>)
                return rval_name.replace("Nint", service_name).replace("nint", service_name);
            }
        }
    }

    rval_name.to_string()
}
