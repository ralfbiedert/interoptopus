//! Creates managed sibling types for compound types that wrap service handles.
//!
//! When a service method returns `Result<nint, Error>` and a sibling method (by name)
//! returns a pointer-to-service directly, this pass infers that the `nint` in the Result
//! represents that service and creates a managed sibling type `Result<ServiceName, Error>`.
//!
//! Siblings are keyed by the **target service TypeId**, not the compound TypeId. This
//! avoids conflicts when multiple services share the same `Result<nint, Error>` type.
//!
//! The sibling types are registered in kinds, names, and types_all so output passes
//! emit them naturally.

use crate::lang::functions::Function;
use crate::lang::types::kind::{DataEnum, TypeKind, TypePattern, Variant};
use crate::lang::types::Type;
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use interoptopus_backends::casing::service_method_name;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Config {}

/// Per-service sibling type IDs for compound types wrapping that service.
#[derive(Debug, Clone)]
pub struct ServiceSiblings {
    /// `Result<ServiceName, Error>` sibling TypeId.
    pub result: Option<TypeId>,
    // Future: pub task: Option<TypeId>, pub option: Option<TypeId>, etc.
}

pub struct Pass {
    info: PassInfo,
    /// Keyed by the target service's TypeId.
    siblings: HashMap<TypeId, ServiceSiblings>,
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
        services: &model::common::service::all::Pass,
        fns_all: &model::common::fns::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Build map: C# method name → (service_class_name, service_type_id).
        let service_return_map = build_service_return_map(services, fns_all, types);
        if service_return_map.is_empty() {
            return Ok(outcome);
        }

        // Find a representative Result type to use as the template for siblings.
        // All Result<nint, Error> methods share the same Result TypeId and DataEnum structure.
        let all_methods = collect_all_service_and_bare_methods(services, fns_all, types);

        for (method_name, func) in &all_methods {
            let Some(rval_ty) = types.get(func.signature.rval) else { continue };

            let (_ok_ty, err_ty, data_enum) = match &rval_ty.kind {
                TypeKind::TypePattern(TypePattern::Result(ok, err, de)) => (*ok, *err, de.clone()),
                _ => continue,
            };

            let Some((service_name, service_type_id)) = find_service_for_sibling(method_name, &service_return_map) else {
                continue;
            };

            // Skip if we already created a sibling for this service.
            if self.processed.contains(&service_type_id) {
                continue;
            }

            // Derive a stable sibling TypeId from the service TypeId + a magic constant.
            let sibling_id = TypeId::from_id(service_type_id.id().derive(0x_5356_435F_5249_5355)); // "SVC_RISU"

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
            types.set(sibling_id, Type { emission: rval_ty.emission.clone(), name: sibling_name, kind: sibling_kind, decorators: rval_ty.decorators.clone() });

            self.siblings.insert(service_type_id, ServiceSiblings { result: Some(sibling_id) });
            self.processed.insert(service_type_id);
            outcome.changed();
        }

        Ok(outcome)
    }

    /// Look up sibling types for a given service TypeId.
    #[must_use]
    pub fn for_service(&self, service_type_id: TypeId) -> Option<&ServiceSiblings> {
        self.siblings.get(&service_type_id)
    }

    /// Build the service return map (method_name → (service_name, service_type_id)).
    /// Used by interface passes to match method names to services.
    pub fn build_service_return_map(
        &self,
        services: &model::common::service::all::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
    ) -> HashMap<String, (String, TypeId)> {
        build_service_return_map(services, fns_all, types)
    }
}

/// Build map: C# method name → (service_class_name, service_type_id).
fn build_service_return_map(
    services: &model::common::service::all::Pass,
    fns_all: &model::common::fns::all::Pass,
    types: &model::common::types::all::Pass,
) -> HashMap<String, (String, TypeId)> {
    use interoptopus_backends::casing::rust_to_pascal;

    let mut map = HashMap::new();

    for (_svc_id, svc) in services.iter() {
        let Some(type_info) = types.get(svc.ty) else { continue };
        let type_name = &type_info.name;

        for &fn_id in &svc.methods {
            let Some(func) = fns_all.get(fn_id) else { continue };
            if let Some((svc_name, svc_ty_id)) = resolve_ptr_to_service(func.signature.rval, types) {
                let method_name = service_method_name(type_name, &func.name);
                map.insert(method_name, (svc_name, svc_ty_id));
            }
        }
    }

    for (_, func) in fns_all.originals() {
        if let Some((svc_name, svc_ty_id)) = resolve_ptr_to_service(func.signature.rval, types) {
            let pascal_name = rust_to_pascal(&func.name);
            map.insert(pascal_name, (svc_name, svc_ty_id));
        }
    }

    map
}

fn resolve_ptr_to_service(rval: TypeId, types: &model::common::types::all::Pass) -> Option<(String, TypeId)> {
    let ty = types.get(rval)?;
    if let TypeKind::Pointer(p) = &ty.kind {
        let target = types.get(p.target)?;
        if matches!(&target.kind, TypeKind::Service) {
            return Some((target.name.clone(), p.target));
        }
    }
    None
}

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
                methods.push((service_method_name(type_name, &func.name), func));
            }
        }
    }

    for (_, func) in fns_all.originals() {
        methods.push((rust_to_pascal(&func.name), func));
    }

    methods
}

// TODO: THIS LOGIC MUST NOT INFER SIBLINGS VIA NAMES OR STRINGY LOGIC!!!
fn find_service_for_sibling(method_name: &str, service_return_map: &HashMap<String, (String, TypeId)>) -> Option<(String, TypeId)> {
    for suffix in &["ResultAsync", "AsyncResult", "Result"] {
        if let Some(base) = method_name.strip_suffix(suffix) {
            if let Some(val) = service_return_map.get(base) {
                return Some(val.clone());
            }
        }
    }
    None
}
