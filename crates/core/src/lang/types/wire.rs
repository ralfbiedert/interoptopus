use crate::lang::types::TypeId;
use crate::wire::SerializationError;
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
    /// A wire-transferred `Vec<T>`.
    Option(TypeId),
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
///
/// # Safety
///
/// The `write`, `read`, and `live_size` methods must be mutually consistent:
/// `live_size` must return the exact number of bytes that `write` produces,
/// and `read` must be able to reconstruct the original value from those bytes.
/// Implementations that violate this contract will corrupt the wire buffer
/// used to transfer data across the FFI boundary, leading to undefined
/// behaviour in the receiving code.
pub unsafe trait WireIO {
    /// Serializes this value into the writer.
    fn write(&self, out: &mut impl Write) -> Result<(), SerializationError>;
    /// Deserializes a value from the reader.
    fn read(input: &mut impl Read) -> Result<Self, SerializationError>
    where
        Self: Sized;
    /// Returns the serialized size of this value in bytes.
    fn live_size(&self) -> usize;
}
