use crate::inventory2::Inventory;
use crate::lang2::meta::{Docs, Emission, Visibility};
use crate::lang2::types::{Type, TypeId, TypeInfo, TypeKind, WireOnly};
use crate::lang2::Register;
use std::collections::HashMap;
use std::mem::MaybeUninit;

macro_rules! impl_ptr {
    ($t:ty, $name:expr, $kind:tt, $id:expr) => {
        impl<T: TypeInfo> TypeInfo for $t {
            fn id() -> TypeId {
                T::id().derive($id)
            }
        }

        impl<T: Register + TypeInfo> Register for $t {
            fn register(inventory: &mut Inventory) {
                // Ensure base type is registered.
                T::register(inventory);

                let type_ =
                    Type { emission: Emission::Builtin, docs: Docs::empty(), visibility: Visibility::Public, name: $name.to_string(), kind: TypeKind::$kind(T::id()) };

                inventory.register_type(Self::id(), type_);
            }
        }
    };
}

// All these share the same derived ID of the base type, as all of them are the same ptr type.
impl_ptr!(std::ptr::NonNull<T>, "*mut T", ReadWritePointer, 0x7EE1DB481C7FEAD63EB329E9812A2F68);
impl_ptr!(&'_ mut T, "*mut T", ReadWritePointer, 0x7EE1DB481C7FEAD63EB329E9812A2F68);
impl_ptr!(*mut T, "*mut T", ReadWritePointer, 0x7EE1DB481C7FEAD63EB329E9812A2F68);
impl_ptr!(Option<&'_ mut T>, "*mut T", ReadPointer, 0x7EE1DB481C7FEAD63EB329E9812A2F68);

// All these share the same derived ID of the base type, as all of them are the same ptr type.
impl_ptr!(&'_ T, "*const T", ReadPointer, 0x20973BD3D67EF4E0323195B99A01FD5E);
impl_ptr!(*const T, "*const T", ReadPointer, 0x20973BD3D67EF4E0323195B99A01FD5E);
impl_ptr!(Option<&'_ T>, "*const T", ReadPointer, 0x20973BD3D67EF4E0323195B99A01FD5E);

impl<T: TypeInfo> TypeInfo for MaybeUninit<T> {
    fn id() -> TypeId {
        // Same as base type
        T::id()
    }
}

impl<T: Register + TypeInfo> Register for MaybeUninit<T> {
    fn register(inventory: &mut Inventory) {
        // Same as base type
        T::register(inventory);
    }
}

impl TypeInfo for String {
    fn id() -> TypeId {
        TypeId::new(0x121F9B85DF8484C54AFC97C4C345A715)
    }
}

impl Register for String {
    fn register(inventory: &mut Inventory) {
        let type_ = Type {
            emission: Emission::Builtin,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: "String".to_string(),
            kind: TypeKind::WireOnly(WireOnly::String),
        };

        inventory.register_type(Self::id(), type_);
    }
}

impl<T: TypeInfo> TypeInfo for Vec<T> {
    fn id() -> TypeId {
        T::id().derive(0x3D4A1327D939CFFCC50EC62B7190BDE0)
    }
}

impl<T: Register + TypeInfo> Register for Vec<T> {
    fn register(inventory: &mut Inventory) {
        // Ensure base type is registered.
        T::register(inventory);

        let t = &inventory.types[&T::id()];

        let type_ = Type {
            emission: Emission::Builtin,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: format!("Vec<{}>", t.name),
            kind: TypeKind::WireOnly(WireOnly::Vec(T::id())),
        };

        inventory.register_type(Self::id(), type_);
    }
}

impl<K: TypeInfo, V: TypeInfo, S: ::std::hash::BuildHasher> TypeInfo for HashMap<K, V, S> {
    fn id() -> TypeId {
        TypeId::new(0xB55DC9DFF8B775E03D34267E9F1DABE5).derive_id(K::id()).derive_id(V::id())
    }
}

impl<K: Register + TypeInfo, V: Register + TypeInfo, S: ::std::hash::BuildHasher> Register for HashMap<K, V, S> {
    fn register(inventory: &mut Inventory) {
        // Ensure base types are registered.
        K::register(inventory);
        V::register(inventory);

        let k = &inventory.types[&K::id()];
        let v = &inventory.types[&V::id()];

        let type_ = Type {
            emission: Emission::Builtin,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: format!("HashMap<{}, {}>", k.name, v.name),
            kind: TypeKind::WireOnly(WireOnly::Map(K::id(), V::id())),
        };

        inventory.register_type(Self::id(), type_);
    }
}
