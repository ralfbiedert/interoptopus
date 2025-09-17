use crate::lang2::function::FunctionId;
use crate::lang2::types::TypeId;
use crate::new_id;

new_id!(ServiceId);

/// Combines a receiver, constructor, destructor and multiple methods in one entity.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Service {
    ty: TypeId,
    ctors: Vec<FunctionId>,
    destructor: FunctionId,
    methods: Vec<FunctionId>,
}
