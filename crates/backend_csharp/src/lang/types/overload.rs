use crate::lang::TypeId;

/// The IntPtr/ByRef/ByOut family for a single pointer type.
#[derive(Debug, Clone)]
pub struct PointerFamily {
    pub intptr: TypeId,
    pub by_ref: TypeId,
    pub by_out: TypeId,
}

/// Links a delegate class type to its bare delegate signature sibling.
#[derive(Debug, Clone)]
pub struct DelegateFamily {
    pub class: TypeId,
    pub signature: TypeId,
}

/// Discriminated union over all overload family kinds.
#[derive(Debug, Clone)]
pub enum OverloadFamily {
    Pointer(PointerFamily),
    Delegate(DelegateFamily),
}
