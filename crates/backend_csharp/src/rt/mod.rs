//! Runtime loaders for Interoptopus plugins.
//!
//! Provides two runtime backends:
//! - [`dynamic`] — Hosts the .NET CLR via `netcorehost` and loads managed assemblies.
//! - [`aot`] — Loads ahead-of-time compiled native libraries via `libloading`.
//!
use interoptopus::ffi;

#[cfg(feature = "rt-aot")]
pub mod aot;
#[cfg(feature = "rt-dotnet")]
pub mod dynamic;
#[cfg(feature = "rt-dotnet")]
mod error;
mod shared;

pub use error::RuntimeError;
use interoptopus::inventory::{Inventory, TypeId};
use interoptopus::lang::meta::{Docs, Emission, FileEmission, Visibility};
use interoptopus::lang::types::{Field, Repr, Struct, Type, TypeInfo, TypeKind};

// TODO
pub struct ErrorXXX {
    x: u32,
}

impl TypeInfo for ErrorXXX {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = true;
    const ASYNC_SAFE: bool = true;
    const SERVICE_SAFE: bool = true;
    const SERVICE_CTOR_SAFE: bool = true;

    fn id() -> TypeId {
        TypeId::new(0x6CC48127B46F1B58B8D4FCFC55617873)
    }

    fn kind() -> TypeKind {
        let s = Struct { fields: vec![Field { name: "x".to_string(), docs: Default::default(), visibility: Default::default(), ty: u32::id() }], repr: Repr::c() };
        TypeKind::Struct(s)
    }

    fn ty() -> Type {
        Type {
            name: "ErrorXXX".to_string(),
            visibility: Visibility::Public,
            docs: Docs::default(),
            emission: Emission::FileEmission(FileEmission::Common),
            kind: Self::kind(),
        }
    }

    fn register(inventory: &mut impl Inventory) {
        inventory.register_type(Self::id(), Self::ty());
    }
}

pub type Try<T> = ffi::Result<T, ErrorXXX>;
