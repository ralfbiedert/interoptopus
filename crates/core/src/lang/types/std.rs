use crate::inventory::Inventory;
use crate::lang::meta::{Docs, Emission, Visibility};
use crate::lang::types::{Type, TypeId, TypeInfo, TypeKind, TypePattern, WireOnly};
use crate::lang::Register;
use std::collections::HashMap;
use std::mem::MaybeUninit;

macro_rules! impl_ptr {
    ($t:ty, $name:expr, $kind:tt, $id:expr) => {
        impl<T: TypeInfo> TypeInfo for $t {
            const WIRE_SAFE: bool = T::WIRE_SAFE;
            const RAW_SAFE: bool = T::RAW_SAFE;

            fn id() -> TypeId {
                T::id().derive($id)
            }

            fn kind() -> TypeKind {
                TypeKind::$kind(T::id())
            }

            fn ty() -> Type {
                Type {
                    emission: Emission::Builtin,
                    docs: Docs::empty(),
                    visibility: Visibility::Public,
                    name: $name.to_string(),
                    kind: Self::kind()
                }
            }

            fn register(inventory: &mut Inventory) {
                // Ensure base type is registered.
                T::register(inventory);
                inventory.register_type(Self::id(), Self::ty());
            }
        }

        impl<T: TypeInfo> crate::lang::Register for $t {
            fn register(inventory: &mut Inventory) {
                <Self as TypeInfo>::register(inventory)
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

pub fn ptr_typeid(x: TypeId) -> TypeId {
    x.derive(0x20973BD3D67EF4E0323195B99A01FD5E)
}

pub fn ptr_mut_typeid(x: TypeId) -> TypeId {
    x.derive(0x7EE1DB481C7FEAD63EB329E9812A2F68)
}

impl<T: TypeInfo> TypeInfo for MaybeUninit<T> {
    const WIRE_SAFE: bool = T::WIRE_SAFE;
    const RAW_SAFE: bool = T::RAW_SAFE;

    fn id() -> TypeId {
        // Same as base type
        T::id()
    }

    fn kind() -> TypeKind {
        T::kind()
    }

    fn ty() -> Type {
        T::ty()
    }

    fn register(inventory: &mut Inventory) {
        // Same as base type
        T::register(inventory);
    }
}

impl<T: TypeInfo> crate::lang::Register for MaybeUninit<T> {
    fn register(inventory: &mut Inventory) {
        <Self as TypeInfo>::register(inventory);
    }
}

impl TypeInfo for String {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = true;

    fn id() -> TypeId {
        TypeId::new(0x121F9B85DF8484C54AFC97C4C345A715)
    }

    fn kind() -> TypeKind {
        TypeKind::WireOnly(WireOnly::String)
    }

    fn ty() -> Type {
        Type {
            emission: Emission::Common,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: "String".to_string(),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl crate::lang::Register for String {
    fn register(inventory: &mut Inventory) {
        <Self as TypeInfo>::register(inventory);
    }
}

impl<T: TypeInfo> TypeInfo for Vec<T> {
    const WIRE_SAFE: bool = T::WIRE_SAFE;
    const RAW_SAFE: bool = T::RAW_SAFE;

    fn id() -> TypeId {
        T::id().derive(0x3D4A1327D939CFFCC50EC62B7190BDE0)
    }

    fn kind() -> TypeKind {
        TypeKind::WireOnly(WireOnly::Vec(T::id()))
    }

    fn ty() -> Type {
        let t = T::ty();
        Type {
            emission: Emission::Builtin,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: format!("Vec<{}>", t.name),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut Inventory) {
        // Ensure base type is registered.
        T::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl<T: TypeInfo> crate::lang::Register for Vec<T> {
    fn register(inventory: &mut Inventory) {
        <Self as TypeInfo>::register(inventory);
    }
}

impl<K: TypeInfo, V: TypeInfo, S: ::std::hash::BuildHasher> TypeInfo for HashMap<K, V, S> {
    const WIRE_SAFE: bool = K::WIRE_SAFE && V::WIRE_SAFE;
    const RAW_SAFE: bool = K::RAW_SAFE && V::RAW_SAFE;

    fn id() -> TypeId {
        TypeId::new(0xB55DC9DFF8B775E03D34267E9F1DABE5).derive_id(K::id()).derive_id(V::id())
    }

    fn kind() -> TypeKind {
        TypeKind::WireOnly(WireOnly::Map(K::id(), V::id()))
    }

    fn ty() -> Type {
        let k = K::ty();
        let v = V::ty();
        Type {
            emission: Emission::Builtin,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: format!("HashMap<{}, {}>", k.name, v.name),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut Inventory) {
        // Ensure base types are registered.
        K::register(inventory);
        V::register(inventory);
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl<K: TypeInfo, V: TypeInfo, S: ::std::hash::BuildHasher> crate::lang::Register for HashMap<K, V, S> {
    fn register(inventory: &mut Inventory) {
        <Self as TypeInfo>::register(inventory);
    }
}

impl TypeInfo for ::std::ffi::c_void {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = true;

    fn id() -> TypeId {
        TypeId::new(0x34E7C243AFCBE5D699605695ACF663B5)
    }

    fn kind() -> TypeKind {
        TypeKind::TypePattern(TypePattern::CVoid)
    }

    fn ty() -> Type {
        Type {
            emission: Emission::Builtin,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: "c_void".to_string(),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

impl crate::lang::Register for ::std::ffi::c_void {
    fn register(inventory: &mut Inventory) {
        <Self as TypeInfo>::register(inventory);
    }
}
