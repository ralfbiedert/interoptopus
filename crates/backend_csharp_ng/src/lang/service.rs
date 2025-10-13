use crate::lang::function::FunctionIdCs;
use crate::lang::types::TypeIdCs;
use interoptopus::new_id;

new_id!(ServiceIdCs);

pub struct Service {
    ty: TypeIdCs,
    // TODO: Should overloads for service methods be stored here?
    ctors: Vec<FunctionIdCs>,   // These are interop functions, we might have more overloads
    methods: Vec<FunctionIdCs>, // These are interop functions, we might have more overloads
    destructor: FunctionIdCs,
}
