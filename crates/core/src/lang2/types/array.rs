use crate::lang2::meta::{Emission, Visibility};
use crate::lang2::types::{Type, TypeId, TypeKind};
use crate::lang2::{Register, TypeInfo};

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

        let type_ = Type {
            emission: Emission::Builtin,
            docs: Default::default(),
            visibility: Visibility::Public,
            name: "[T; N]".to_string(),
            kind: TypeKind::Array(Array { ty: T::id(), len: N }),
        };

        inventory.register_type(Self::id(), type_);
    }
}
