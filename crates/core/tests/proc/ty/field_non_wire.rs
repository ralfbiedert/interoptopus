use interoptopus::ffi;
use interoptopus::inventory::{Inventory, TypeId};
use interoptopus::lang::types::{Type, TypeInfo, TypeKind};

// This simulates the error diagnostic if a type implemented `TypeInfo` manually
// but forgot to impl WireIO.

struct Bar;

impl TypeInfo for Bar {
    const WIRE_SAFE: bool = true;
    const RAW_SAFE: bool = false;
    const ASYNC_SAFE: bool = false;
    const SERVICE_SAFE: bool = false;
    const SERVICE_CTOR_SAFE: bool = false;

    fn id() -> TypeId {
        TypeId::new(0x0)
    }
    fn kind() -> TypeKind {
        todo!()
    }
    fn ty() -> Type {
        todo!()
    }
    fn register(_: &mut Inventory) {}
}

#[ffi]
struct Service {
    x: f32,
    y: Bar,
}

fn main() {}
