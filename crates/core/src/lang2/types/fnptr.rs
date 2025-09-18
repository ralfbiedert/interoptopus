use crate::inventory2::Inventory;
use crate::lang2::meta::Emission;
use crate::lang2::types::TypeId;
use crate::lang2::{Register, TypeInfo};

impl<R: TypeInfo> TypeInfo for extern "C" fn() -> R {
    fn id() -> TypeId {
        TypeId::new(0xEE8602B016C043561CA68291A8142F3B).derive_id(R::id())
    }
}

impl<R: TypeInfo + Register> Register for extern "C" fn() -> R {
    fn register(inventory: &mut Inventory) {
        R::register(inventory);

        let ty_params = [&inventory.types[&R::id()].emission];

        let emission = if ty_params.iter().all(|x| matches!(x, Emission::Builtin | Emission::Common)) {
            Emission::Common
        } else {
            Emission::Module(String::new())
        };

        // let type_ =
        //     Type { emission: Emission::Common, docs: Docs::empty(), visibility: Visibility::Public, name: $name.to_string(), kind: TypeKind::$kind(T::id()) };
        //
        // inventory.register_type(Self::id(), type_);
        todo!()
    }
}

impl<R: TypeInfo> TypeInfo for Option<extern "C" fn() -> R> {
    fn id() -> TypeId {
        TypeId::new(0xEE8602B016C043561CA68291A8142F3B).derive_id(R::id())
    }
}

impl<R, T1> TypeInfo for extern "C" fn(T1) -> R
where
    T1: TypeInfo,
    R: TypeInfo,
{
    fn id() -> TypeId {
        TypeId::new(0xEE8602B016C043561CA68291A8142F3B).derive_id(R::id()).derive_id(T1::id())
    }
}

impl<R, T1> TypeInfo for Option<extern "C" fn(T1) -> R>
where
    T1: TypeInfo,
    R: TypeInfo,
{
    fn id() -> TypeId {
        TypeId::new(0xEE8602B016C043561CA68291A8142F3B).derive_id(R::id()).derive_id(T1::id())
    }
}
