//! Creates `DelegateKind::Signature` sibling types for each `DelegateKind::Class` delegate.
//!
//! For every `Delegate { kind: Class, signature }` that is fully resolved, this pass
//! creates a new type representing the bare C# delegate (`{Name}Delegate`) with
//! `DelegateKind::Signature`. This sibling is used by the body overload pass to build
//! overload signatures that accept C# delegates directly instead of the wrapper class.

use crate::lang::types::{Delegate, DelegateKind, Type, TypeKind};
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;
use std::sync::Arc;

/// Links a delegate class type to its bare delegate signature sibling.
#[derive(Debug, Clone)]
pub struct Family {
    pub class: TypeId,
    pub signature: TypeId,
}

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    /// Maps either member TypeId to its family.
    families: HashMap<TypeId, Arc<Family>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, families: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        kinds: &mut model::types::kind::Pass,
        names: &mut model::types::names::Pass,
        map: &mut model::types::map::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        // Collect Delegate::Class types
        let class_delegates: Vec<(TypeId, Delegate)> = kinds
            .iter()
            .filter_map(|(&type_id, kind)| match kind {
                TypeKind::Delegate(d) if d.kind == DelegateKind::Class => Some((type_id, d.clone())),
                _ => None,
            })
            .collect();

        for (class_id, delegate) in class_delegates {
            if self.families.contains_key(&class_id) {
                continue;
            }

            // Wait until the class type is named
            let Some(class_name) = names.name(class_id) else {
                continue;
            };

            let sig_name = format!("{class_name}Delegate");

            // Derive a new TypeId for the signature sibling
            let sig_id = TypeId::from_id(class_id.id().derive(0x_646C_6774_5F73_6962)); // "dlgt_sib"

            let sig_delegate = Delegate { kind: DelegateKind::Signature, signature: delegate.signature.clone() };

            // Register in kinds, names, and map
            kinds.set_kind(sig_id, TypeKind::Delegate(sig_delegate.clone()));
            names.set_name(sig_id, sig_name.clone());
            map.register(sig_id, Type { name: sig_name, kind: TypeKind::Delegate(sig_delegate) });

            // Build family
            let family = Arc::new(Family { class: class_id, signature: sig_id });

            self.families.insert(class_id, Arc::clone(&family));
            self.families.insert(sig_id, family);
            outcome.changed();
        }

        Ok(outcome)
    }

    /// Look up the delegate family for either member TypeId.
    pub fn family(&self, type_id: TypeId) -> Option<&Arc<Family>> {
        self.families.get(&type_id)
    }
}
