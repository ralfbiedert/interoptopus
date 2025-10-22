use crate::lang::function::FunctionIdCs;
use crate::lang::types::TypeIdCs;
use interoptopus::new_id;

new_id!(ServiceIdCs);

pub struct Service {
    pub ty: TypeIdCs,
    // TODO: Should overloads for service methods be stored here?
    pub ctors: Vec<FunctionIdCs>,   // These are interop functions, we might have more overloads
    pub methods: Vec<FunctionIdCs>, // These are interop functions, we might have more overloads
    pub destructor: FunctionIdCs,
}
