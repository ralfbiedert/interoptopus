//! Creates managed sibling types for compound types that wrap service handles.
//!
//! When a service method returns `Result<nint, Error>` and a sibling method (by name)
//! returns a pointer-to-service directly, this pass infers that the `nint` in the Result
//! represents that service and creates a managed sibling type `Result<ServiceName, Error>`.
//!
//! The sibling types are registered in kinds, names, and types_all. A mapping from
//! the original TypeId to the sibling TypeId allows interface passes to use the
//! managed variant in method signatures.

use crate::lang::functions::Function;
use crate::lang::meta::Emission;
use crate::lang::types::kind::{DataEnum, TypeKind, TypePattern, Variant};
use crate::lang::types::ManagedConversion;
use crate::lang::types::Type;
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use interoptopus_backends::casing::service_method_name;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    siblings: HashMap<TypeId, TypeId>,
    processed: HashSet<TypeId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, siblings: HashMap::default(), processed: HashSet::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        kinds: &mut model::common::types::kind::Pass,
        names: &mut model::common::types::names::Pass,
        types: &mut model::common::types::all::Pass,
        managed_conversion: &mut model::common::types::info::managed_conversion::Pass,
        services: &model::common::service::all::Pass,
        fns_all: &model::common::fns::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Build map: C# method name → service class name, for methods returning ptr-to-service.
        let service_return_map = build_service_return_map(services, fns_all, types);
        if service_return_map.is_empty() {
            return Ok(outcome);
        }

        // For each service method that returns a Result type, check if a sibling
        // method (by name) returns a service directly. If so, create a managed
        // sibling Result type.
        let all_methods = collect_all_service_and_bare_methods(services, fns_all, types);

        for (method_name, func) in &all_methods {
            let rval_ty = match types.get(func.signature.rval) {
                Some(t) => t,
                None => continue,
            };

            // Only process Result types.
            let (ok_ty, err_ty, data_enum) = match &rval_ty.kind {
                TypeKind::TypePattern(TypePattern::Result(ok, err, de)) => (*ok, *err, de.clone()),
                _ => continue,
            };

            if self.processed.contains(&func.signature.rval) {
                continue;
            }

            // Try to find the base method name by stripping known suffixes.
            let service_name = find_service_for_sibling(method_name, &service_return_map);
            let Some(service_name) = service_name else { continue };

            // Find the service TypeId.
            let Some(service_type_id) = find_service_type_id(&service_name, types) else {
                continue;
            };

            // Derive a stable sibling TypeId.
            let original_id = func.signature.rval;
            let sibling_id = TypeId::from_id(original_id.id().derive(0x_5356_435F_5349_424C)); // "SVC_SIBL"

            // Build sibling DataEnum: replace Ok variant's type with the service TypeId.
            let sibling_variants: Vec<Variant> = data_enum
                .variants
                .iter()
                .map(|v| {
                    if v.name == "Ok" {
                        Variant { name: v.name.clone(), docs: v.docs.clone(), tag: v.tag, ty: Some(service_type_id) }
                    } else {
                        v.clone()
                    }
                })
                .collect();
            let sibling_data_enum = DataEnum { variants: sibling_variants };
            let sibling_kind = TypeKind::TypePattern(TypePattern::Result(service_type_id, err_ty, sibling_data_enum));

            let err_name = names.get(err_ty).cloned().unwrap_or_else(|| "Error".to_string());
            let sibling_name = format!("Result{service_name}{err_name}");

            kinds.set(sibling_id, sibling_kind.clone());
            names.set(sibling_id, sibling_name.clone());
            types.set(sibling_id, Type { emission: Emission::Builtin, name: sibling_name, kind: sibling_kind, decorators: Default::default() });
            // TODO is this needed, it should be inferred by the managed pass the next round?
            managed_conversion.set(sibling_id, ManagedConversion::To);

            self.siblings.insert(original_id, sibling_id);
            self.processed.insert(original_id);
            outcome.changed();
        }

        Ok(outcome)
    }

    #[must_use]
    pub fn sibling(&self, original: TypeId) -> Option<TypeId> {
        self.siblings.get(&original).copied()
    }
}

/// Build map: C# method name → service class name, for methods returning pointer-to-service.
fn build_service_return_map(
    services: &model::common::service::all::Pass,
    fns_all: &model::common::fns::all::Pass,
    types: &model::common::types::all::Pass,
) -> HashMap<String, String> {
    use interoptopus_backends::casing::rust_to_pascal;

    let mut map = HashMap::new();

    for (_svc_id, svc) in services.iter() {
        let Some(type_info) = types.get(svc.ty) else { continue };
        let type_name = &type_info.name;

        for &fn_id in &svc.methods {
            let Some(func) = fns_all.get(fn_id) else { continue };
            if let Some(svc_name) = resolve_ptr_to_service(func.signature.rval, types) {
                let method_name = service_method_name(type_name, &func.name);
                map.insert(method_name, svc_name);
            }
        }
    }

    for (_, func) in fns_all.originals() {
        if let Some(svc_name) = resolve_ptr_to_service(func.signature.rval, types) {
            let pascal_name = rust_to_pascal(&func.name);
            map.insert(pascal_name, svc_name);
        }
    }

    map
}

fn resolve_ptr_to_service(rval: TypeId, types: &model::common::types::all::Pass) -> Option<String> {
    let ty = types.get(rval)?;
    if let TypeKind::Pointer(p) = &ty.kind {
        let target = types.get(p.target)?;
        if matches!(&target.kind, TypeKind::Service) {
            return Some(target.name.clone());
        }
    }
    None
}

/// Collect all service methods and bare functions as (method_name, &Function).
fn collect_all_service_and_bare_methods<'a>(
    services: &model::common::service::all::Pass,
    fns_all: &'a model::common::fns::all::Pass,
    types: &model::common::types::all::Pass,
) -> Vec<(String, &'a Function)> {
    use interoptopus_backends::casing::rust_to_pascal;

    let mut methods = Vec::new();

    for (_svc_id, svc) in services.iter() {
        let Some(type_info) = types.get(svc.ty) else { continue };
        let type_name = &type_info.name;

        for &fn_id in &svc.methods {
            if let Some(func) = fns_all.get(fn_id) {
                let method_name = service_method_name(type_name, &func.name);
                methods.push((method_name, func));
            }
        }
    }

    for (_, func) in fns_all.originals() {
        let pascal_name = rust_to_pascal(&func.name);
        methods.push((pascal_name, func));
    }

    methods
}

/// Try stripping known suffixes to find the base method and look it up in the service return map.
fn find_service_for_sibling(method_name: &str, service_return_map: &HashMap<String, String>) -> Option<String> {
    for suffix in &["ResultAsync", "AsyncResult", "Result"] {
        if let Some(base) = method_name.strip_suffix(suffix) {
            if let Some(svc_name) = service_return_map.get(base) {
                return Some(svc_name.clone());
            }
        }
    }
    None
}

/// Find the TypeId for a service by its class name.
fn find_service_type_id(service_name: &str, types: &model::common::types::all::Pass) -> Option<TypeId> {
    types.iter().find_map(|(&id, ty)| {
        if matches!(&ty.kind, TypeKind::Service) && ty.name == service_name {
            Some(id)
        } else {
            None
        }
    })
}
