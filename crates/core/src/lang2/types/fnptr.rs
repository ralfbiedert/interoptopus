use crate::inventory2::Inventory;
use crate::lang2::function::{Argument, Signature};
use crate::lang2::meta::{common_or_module_emission, Docs, Visibility};
use crate::lang2::types::{Type, TypeId, TypeInfo, TypeKind};
use crate::lang2::Register;

macro_rules! impl_fnptr {
    // No arguments: extern "C" fn() -> R
    ($r:ident) => {
        impl<$r: TypeInfo> TypeInfo for extern "C" fn() -> $r {
            fn id() -> TypeId {
                TypeId::new(0xEE8602B016C043561CA68291A8142F3B).derive_id($r::id())
            }
        }

        impl<$r: TypeInfo + Register> Register for extern "C" fn() -> $r {
            fn register(inventory: &mut Inventory) {
                $r::register(inventory);

                let r = &inventory.types[&$r::id()];
                let ty_params = [inventory.types[&$r::id()].emission.clone()];
                let emission = common_or_module_emission(&ty_params);
                let signature = Signature {
                    arguments: vec![],
                    rval: $r::id()
                };

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

        impl<$r: TypeInfo> TypeInfo for Option<extern "C" fn() -> $r> {
            fn id() -> TypeId {
                <extern "C" fn() -> $r as TypeInfo>::id()
            }
        }

        impl<$r: TypeInfo + Register> Register for Option<extern "C" fn() -> $r> {
            fn register(inventory: &mut Inventory) {
                <extern "C" fn() -> $r as Register>::register(inventory);
            }
        }
    };

    // With arguments: extern "C" fn(T1, T2, ...) -> R
    ($r:ident, $($t:ident),+) => {
        impl<$r, $($t),+> TypeInfo for extern "C" fn($($t),+) -> $r
        where
            $($t: TypeInfo,)+
            $r: TypeInfo,
        {
            fn id() -> TypeId {
                TypeId::new(0xEE8602B016C043561CA68291A8142F3B)
                    .derive_id($r::id())
                    $(.derive_id($t::id()))+
            }
        }

        #[allow(non_snake_case, unused_assignments)]
        impl<$r, $($t),+> Register for extern "C" fn($($t),+) -> $r
        where
            $($t: Register + TypeInfo,)+
            $r: Register + TypeInfo,
        {
            fn register(inventory: &mut Inventory) {
                $r::register(inventory);
                $($t::register(inventory);)+

                let r = &inventory.types[&$r::id()];
                $(let $t = &inventory.types[&$t::id()];)+

                let ty_params = [
                    inventory.types[&$r::id()].emission.clone(),
                    $(inventory.types[&$t::id()].emission.clone()),+
                ];
                let emission = common_or_module_emission(&ty_params);

                let arguments = {
                    let mut args = Vec::new();
                    let mut counter = 1;
                    $(
                        args.push(Argument {
                            name: format!("x{}", counter),
                            ty: $t::id()
                        });
                        counter += 1;
                    )+
                    args
                };

                let signature = Signature {
                    arguments,
                    rval: $r::id()
                };

                let type_ = Type {
                    emission,
                    docs: Docs::empty(),
                    visibility: Visibility::Public,
                    name: format!(r#"extern "C" fn({}) -> {}"#,
                        [$($t.name.clone()),+].join(", "),
                        r.name
                    ),
                    kind: TypeKind::FnPointer(signature),
                };

                inventory.register_type(Self::id(), type_);
            }
        }

        impl<$r, $($t),+> TypeInfo for Option<extern "C" fn($($t),+) -> $r>
        where
            $($t: TypeInfo,)+
            $r: TypeInfo,
        {
            fn id() -> TypeId {
                <extern "C" fn($($t),+) -> $r as TypeInfo>::id()
            }
        }

        impl<$r, $($t),+> Register for Option<extern "C" fn($($t),+) -> $r>
        where
            $($t: TypeInfo + Register,)+
            $r: TypeInfo + Register,
        {
            fn register(inventory: &mut Inventory) {
                <extern "C" fn($($t),+) -> $r as Register>::register(inventory);
            }
        }
    };
}

// Generate implementations for function pointers with different arities
impl_fnptr!(R);
impl_fnptr!(R, T1);
impl_fnptr!(R, T1, T2);
impl_fnptr!(R, T1, T2, T3);
impl_fnptr!(R, T1, T2, T3, T4);
impl_fnptr!(R, T1, T2, T3, T4, T5);
impl_fnptr!(R, T1, T2, T3, T4, T5, T6);
impl_fnptr!(R, T1, T2, T3, T4, T5, T6, T7);
impl_fnptr!(R, T1, T2, T3, T4, T5, T6, T7, T8);
impl_fnptr!(R, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_fnptr!(R, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

pub fn fnptr_typeid(sig: &Signature) -> TypeId {
    let mut rval = TypeId::new(0xEE8602B016C043561CA68291A8142F3B).derive_id(sig.rval);

    for x in &sig.arguments {
        rval = rval.derive_id(x.ty);
    }

    rval
}
