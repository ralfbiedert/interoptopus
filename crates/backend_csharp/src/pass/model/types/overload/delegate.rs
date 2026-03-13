//! Creates `DelegateKind::Signature` sibling types for each `DelegateKind::Class` delegate.
//!
//! For every `Delegate { kind: Class, signature }` that is fully resolved, this pass
//! creates a new type representing the bare C# delegate (`{Name}Delegate`) with
//! `DelegateKind::Signature`. This sibling is used by the body overload pass to build
//! overload signatures that accept C# delegates directly instead of the wrapper class.

use crate::lang::TypeId;
use crate::lang::meta::Emission;
use crate::lang::types::kind::{Delegate, DelegateKind, TypeKind};
use crate::lang::types::{DelegateFamily, OverloadFamily, Type};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    processed: HashSet<TypeId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, processed: HashSet::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        kinds: &mut model::types::kind::Pass,
        names: &mut model::types::names::Pass,
        types: &mut model::types::all::Pass,
        overloads: &mut model::types::overload::all::Pass,
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
            if self.processed.contains(&class_id) {
                continue;
            }

            // Wait until the class type is named
            let Some(class_name) = names.get(class_id) else {
                continue;
            };

            let sig_name = format!("{class_name}Delegate");

            // Derive a new TypeId for the signature sibling
            let sig_id = TypeId::from_id(class_id.id().derive(0x_646C_6774_5F73_6962)); // "dlgt_sib"

            let sig_delegate = Delegate { kind: DelegateKind::Signature, signature: delegate.signature.clone() };

            // Register in kinds, names, and types
            kinds.set(sig_id, TypeKind::Delegate(sig_delegate.clone()));
            names.set(sig_id, sig_name.clone());
            types.set(sig_id, Type { emission: Emission::Builtin, name: sig_name, kind: TypeKind::Delegate(sig_delegate) });

            // Register family in the overload all pass
            let family = Arc::new(OverloadFamily::Delegate(DelegateFamily { class: class_id, signature: sig_id }));

            overloads.register(class_id, Arc::clone(&family));
            overloads.register(sig_id, family);

            self.processed.insert(class_id);
            outcome.changed();
        }

        Ok(outcome)
    }
}
