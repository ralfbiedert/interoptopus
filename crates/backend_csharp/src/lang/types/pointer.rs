use crate::lang::TypeId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum IntPtrHint {
    Read,
    ReadWrite,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Pointer {
    IntPtr(TypeId, IntPtrHint),
    ByRef(TypeId),
    ByOut(TypeId),
}
