use crate::lang::types::{SerializationError, TypeId};
use std::io::{Read, Write};

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

/// Utilities for (de)serializing an instance of this type.
///
/// These must be properly implemented if TypeInfo::WIRE_SAFE is true for that
/// type. Otherwise these should panic.
pub trait WireIO {
    fn write(&self, out: &mut impl Write) -> Result<(), SerializationError>;
    fn read(input: &mut impl Read) -> Result<Self, SerializationError>
    where
        Self: Sized;
    fn live_size(&self) -> usize;
}
