use crate::lang::{FunctionId, TypeId};
use interoptopus::new_id;

new_id!(ServiceIdCs);

/// Immutable original function IDs as matched by the Rust inventory, used by
/// downstream passes to discover overloads.
pub struct Sources {
    pub ctors: Vec<FunctionId>,
    pub methods: Vec<FunctionId>,
}

pub struct Service {
    pub ty: TypeId,
    /// Stable source function IDs, never filtered by anything.
    pub sources: Sources,
    /// Renderable constructor functions (filtered by visibility, etc.).
    pub ctors: Vec<FunctionId>,
    /// Renderable method functions (filtered by visibility, etc.).
    pub methods: Vec<FunctionId>,
    pub destructor: FunctionId,
}
