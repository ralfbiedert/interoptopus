use crate::lang::types::{SerializationError, TypeId};
use std::io::{Read, Write};

/// Types that can only appear inside a `Wire<T>` wrapper for serialized transfer.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WireOnly {
    /// A wire-transferred string.
    String,
    /// A wire-transferred `Vec<T>`.
    Vec(TypeId),
    /// A wire-transferred `Map<K, V>`.
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
/// These must be properly implemented if `TypeInfo::WIRE_SAFE` is true for that
/// type. Otherwise these should panic.
pub trait WireIO {
    /// Serializes this value into the writer.
    fn write(&self, out: &mut impl Write) -> Result<(), SerializationError>;
    /// Deserializes a value from the reader.
    fn read(input: &mut impl Read) -> Result<Self, SerializationError>
    where
        Self: Sized;
    /// Returns the serialized size of this value in bytes.
    fn live_size(&self) -> usize;
}
