use crate::lang::types::TypeId;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WireOnly {
    String,
    Vec(TypeId),
    Map(TypeId, TypeId),
}

#[doc(hidden)]
#[macro_export]
macro_rules! bad_wire {
    () => {
        panic!("Called a wire method on a type that does not support wiring.")
    };
}
