use crate::lang::meta::{Docs, Emission, Visibility};
use crate::lang::types::{Type, TypeId, TypeInfo, TypeKind};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Array {
    pub ty: TypeId,
    pub len: usize,
}

impl<T, const N: usize> TypeInfo for [T; N]
where
    T: TypeInfo,
{
    const WIRE_SAFE: bool = T::WIRE_SAFE;
    const RAW_SAFE: bool = T::RAW_SAFE;

    fn id() -> TypeId {
        // Reliably derive an ID for an array of type T and length N.
        T::id().derive(0x06A3676E231857123975EA87924CA277).derive(N as u128)
    }

    fn kind() -> TypeKind {
        TypeKind::Array(Array { ty: T::id(), len: N })
    }

    fn ty() -> Type {
        Type { emission: Emission::Builtin, docs: Docs::empty(), visibility: Visibility::Public, name: format!("[{}; {N}]", T::ty().name), kind: Self::kind() }
    }

    fn register(inventory: &mut crate::inventory::Inventory) {
        // Ensure base type is registered.
        T::register(inventory);

        inventory.register_type(Self::id(), Self::ty());
    }
}
