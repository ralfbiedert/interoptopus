use crate::lang::TypeId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Pointer {
    IntPtr(TypeId),
    Ref(TypeId),
    Out(TypeId),
}
