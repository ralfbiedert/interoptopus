//! Creates managed sibling types for compound types that wrap service handles.
//!
//! When a `Result<*const Service, Error>` type is found, this pass creates a
//! managed sibling `Result<Service, Error>` type. The sibling uses the service's
//! TypeId directly instead of a pointer, making it suitable for managed C# code.
//!
//! Siblings are keyed by the **target service TypeId**, not the compound TypeId. This
//! avoids conflicts when multiple services share the same `Result<nint, Error>` type.
//!
//! The sibling types are registered in kinds, names, and types_all so output passes
//! emit them naturally.

use crate::lang::types::Type;
use crate::lang::types::kind::{TypeKind, TypePattern, Variant};
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Config {}

/// Per-service sibling type IDs for compound types wrapping that service.
#[derive(Debug, Clone)]
pub struct ServiceSiblings {
    /// `Result<ServiceName, Error>` sibling TypeId.
    pub result: Option<TypeId>,
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
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Collect candidates first to avoid borrowing issues with `types`.
        let candidates: Vec<_> = types
            .iter()
            .filter_map(|(_, ty)| {
                let (ok_ty, err_ty, data_enum) = match &ty.kind {
                    TypeKind::TypePattern(TypePattern::Result(ok, err, de)) => (*ok, *err, de.clone()),
                    _ => return None,
                };

                // Check if Ok type is pointer-to-service
                let ok_type = types.get(ok_ty)?;
                let TypeKind::Pointer(p) = &ok_type.kind else { return None };
                let target = types.get(p.target)?;
                if !matches!(&target.kind, TypeKind::Service) {
                    return None;
                }

                let service_type_id = p.target;
                let original_name = ty.name.clone();

                Some((service_type_id, original_name, err_ty, data_enum, ty.emission.clone(), ty.decorators.clone()))
            })
            .collect();

        for (service_type_id, original_name, err_ty, data_enum, emission, decorators) in candidates {
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
            let sibling_data_enum = crate::lang::types::kind::DataEnum { variants: sibling_variants };
            let sibling_kind = TypeKind::TypePattern(TypePattern::Result(service_type_id, err_ty, sibling_data_enum));

            // Use the original Result type's name to ensure consistent casing.
            let sibling_name = original_name;

            kinds.set(sibling_id, sibling_kind.clone());
            names.set(sibling_id, sibling_name.clone());
            // The sibling is used internally by interface method return types but must not
            // be emitted as its own type definition — the original pointer-based Result type
            // already provides the Unmanaged struct and conversions.
            let sibling_emission = crate::lang::meta::Emission::Builtin;
            types.set(sibling_id, Type { emission: sibling_emission, name: sibling_name, kind: sibling_kind, decorators });

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
}
