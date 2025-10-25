use crate::id::{FunctionId, TypeId};
use interoptopus::new_id;

new_id!(ServiceIdCs);

pub struct Service {
    pub ty: TypeId,
    // TODO: Should overloads for service methods be stored here?
    pub ctors: Vec<FunctionId>,   // These are interop functions, we might have more overloads
    pub methods: Vec<FunctionId>, // These are interop functions, we might have more overloads
    pub destructor: FunctionId,
}
