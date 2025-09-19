use crate::lang2::meta::{Docs, Emission, Visibility};
use crate::lang2::types::{Type, TypeId, TypeInfo, TypeKind};
use crate::lang2::Register;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Array {
    pub ty: TypeId,
    pub len: usize,
}

impl<T, const N: usize> TypeInfo for [T; N]
where
    T: TypeInfo,
{
    fn id() -> TypeId {
        // Reliably derive an ID for an array of type T and length N.
        T::id().derive(0x06A3676E231857123975EA87924CA277).derive(N as u128)
    }
}

impl<T, const N: usize> Register for [T; N]
where
    T: Register + TypeInfo,
{
    fn register(inventory: &mut crate::inventory2::Inventory) {
        // Ensure base type is registered.
        T::register(inventory);

        let t = &inventory.types[&T::id()];

        let type_ = Type {
            emission: Emission::Builtin,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: format!("[{}; {N}]", t.name),
            kind: TypeKind::Array(Array { ty: T::id(), len: N }),
        };

        inventory.register_type(Self::id(), type_);
    }
}
