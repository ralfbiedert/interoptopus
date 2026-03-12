//! Container for all type overload families (pointers, delegates).
//!
//! Each overloaded type belongs to a family — e.g. an IntPtr type has ByRef/ByOut
//! siblings, a delegate class has a bare signature sibling. Individual overload
//! passes (pointer, delegate) register their families here; downstream passes
//! query this pass instead of the individual ones.

use crate::lang::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

/// The IntPtr/ByRef/ByOut family for a single pointer type.
#[derive(Debug, Clone)]
pub struct PointerFamily {
    pub intptr: TypeId,
    pub by_ref: TypeId,
    pub by_out: TypeId,
}

/// Links a delegate class type to its bare delegate signature sibling.
#[derive(Debug, Clone)]
pub struct DelegateFamily {
    pub class: TypeId,
    pub signature: TypeId,
}

/// Discriminated union over all overload family kinds.
#[derive(Debug, Clone)]
pub enum OverloadFamily {
    Pointer(PointerFamily),
    Delegate(DelegateFamily),
}

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    families: HashMap<TypeId, Arc<OverloadFamily>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { families: Default::default() }
    }

    pub fn register(&mut self, id: TypeId, family: Arc<OverloadFamily>) {
        self.families.insert(id, family);
    }

    pub fn get(&self, type_id: TypeId) -> Option<&OverloadFamily> {
        self.families.get(&type_id).map(|a| a.as_ref())
    }
}
