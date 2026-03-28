use crate::bad_wire;
use crate::inventory::{Inventory, TypeId};
use crate::lang::meta::{Docs, Emission, Visibility};
use crate::wire::SerializationError;
use crate::lang::types::{Primitive, Type, TypeInfo, TypeKind, WireIO};
use std::io::{Read, Write};

unsafe impl TypeInfo for () {
    const WIRE_SAFE: bool = false;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        TypeId::new(0x6D87F0180F529932F56D3B4800145193)
    }

    fn kind() -> TypeKind {
        TypeKind::Primitive(Primitive::Void)
    }

    fn ty() -> Type {
        Type { emission: Emission::Builtin, docs: Docs::empty(), visibility: Visibility::Public, name: "()".to_string(), kind: Self::kind() }
    }

    fn register(inventory: &mut impl Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

unsafe impl WireIO for () {
    fn write(&self, _: &mut impl Write) -> Result<(), SerializationError> {
        bad_wire!()
    }

    fn read(_: &mut impl Read) -> Result<Self, SerializationError> {
        bad_wire!()
    }

    fn live_size(&self) -> usize {
        bad_wire!()
    }
}

unsafe impl TypeInfo for bool {
    const WIRE_SAFE: Self = true;
    const RAW_SAFE: Self = true;
    const ASYNC_SAFE: Self = true;
    const SERVICE_SAFE: Self = false;
    const SERVICE_CTOR_SAFE: Self = false;

    fn id() -> TypeId {
        TypeId::new(0xCA37AD739D5997FE7F9E1B0B2CCBACE1)
    }

    fn kind() -> TypeKind {
        TypeKind::Primitive(Primitive::Bool)
    }

    fn ty() -> Type {
        Type { emission: Emission::Builtin, docs: Docs::empty(), visibility: Visibility::Public, name: "bool".to_string(), kind: Self::kind() }
    }

    fn register(inventory: &mut impl Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

unsafe impl WireIO for bool {
    fn write(&self, w: &mut impl Write) -> Result<(), SerializationError> {
        u8::from(*self).write(w)
    }

    fn read(r: &mut impl Read) -> Result<Self, SerializationError> {
        Ok(u8::read(r)? != 0)
    }

    fn live_size(&self) -> usize {
        1
    }
}
