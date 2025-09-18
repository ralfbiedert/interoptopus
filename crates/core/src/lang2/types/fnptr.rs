use crate::inventory2::Inventory;
use crate::lang2::function::{Argument, Signature};
use crate::lang2::meta::{common_or_module_emission, Docs, Visibility};
use crate::lang2::types::{Type, TypeId, TypeInfo, TypeKind};
use crate::lang2::Register;

impl<R: TypeInfo> TypeInfo for extern "C" fn() -> R {
    fn id() -> TypeId {
        TypeId::new(0xEE8602B016C043561CA68291A8142F3B).derive_id(R::id())
    }
}

impl<R: TypeInfo + Register> Register for extern "C" fn() -> R {
    fn register(inventory: &mut Inventory) {
        R::register(inventory);

        let r = &inventory.types[&R::id()];

        let ty_params = [inventory.types[&R::id()].emission.clone()];
        let emission = common_or_module_emission(&ty_params);
        let signature = Signature { arguments: vec![], rval: R::id() };

        let type_ = Type {
            emission,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: format!(r#"extern "C" fn() -> {}"#, r.name),
            kind: TypeKind::FnPointer(signature),
        };

        inventory.register_type(Self::id(), type_);
    }
}

impl<R: TypeInfo> TypeInfo for Option<extern "C" fn() -> R> {
    fn id() -> TypeId {
        <extern "C" fn() -> R as TypeInfo>::id()
    }
}

impl<R: TypeInfo + Register> Register for Option<extern "C" fn() -> R> {
    fn register(inventory: &mut Inventory) {
        <extern "C" fn() -> R as Register>::register(inventory);
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

impl<R, T1> Register for extern "C" fn(T1) -> R
where
    T1: Register + TypeInfo,
    R: Register + TypeInfo,
{
    fn register(inventory: &mut Inventory) {
        R::register(inventory);
        T1::register(inventory);

        let r = &inventory.types[&R::id()];
        let t1 = &inventory.types[&T1::id()];

        let ty_params = [inventory.types[&R::id()].emission.clone(), inventory.types[&T1::id()].emission.clone()];
        let emission = common_or_module_emission(&ty_params);
        let signature = Signature { arguments: vec![Argument { string: "x1".to_string(), ty: T1::id() }], rval: R::id() };

        let type_ = Type {
            emission,
            docs: Docs::empty(),
            visibility: Visibility::Public,
            name: format!(r#"extern "C" fn({}) -> {}"#, t1.name, r.name),
            kind: TypeKind::FnPointer(signature),
        };

        inventory.register_type(Self::id(), type_);
    }
}

impl<R, T1> TypeInfo for Option<extern "C" fn(T1) -> R>
where
    T1: TypeInfo,
    R: TypeInfo,
{
    fn id() -> TypeId {
        <extern "C" fn(T1) -> R as TypeInfo>::id()
    }
}

impl<R, T1> Register for Option<extern "C" fn(T1) -> R>
where
    T1: TypeInfo + Register,
    R: TypeInfo + Register,
{
    fn register(inventory: &mut Inventory) {
        <extern "C" fn(T1) -> R as Register>::register(inventory);
    }
}
