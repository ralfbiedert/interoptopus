use crate::wire::WireError;
use std::io::{Read, Write};

/// Implemented via the ffi wired attribute to be usable inside [`Wire`](crate::wire::Wire).
///
/// This is not zero copy!
pub trait Ser {
    /// Write self into the buffer addressed by `out`
    fn ser(&self, out: &mut impl Write) -> Result<(), WireError>;

    /// Calculate amount of storage needed for writing self
    fn storage_size(&self) -> usize;
}

/// Implemented via the ffi wired attribute to be usable inside [`Wire`](crate::wire::Wire).
///
/// This is not zero copy!
pub trait De {
    /// Read contents of type Self from the reader `input`
    fn de(input: &mut impl Read) -> Result<Self, WireError>
    where
        Self: Sized;
}
