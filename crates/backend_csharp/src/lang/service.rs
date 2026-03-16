use crate::lang::{FunctionId, TypeId};
use interoptopus::new_id;

new_id!(ServiceIdCs);

pub struct Service {
    pub ty: TypeId,
    pub ctors: Vec<FunctionId>,   // These are interop functions and overloads for which to emit methods for
    pub methods: Vec<FunctionId>, // These are interop functions and overloads for which to emit methods for
    pub destructor: FunctionId,
}
