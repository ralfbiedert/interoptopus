use crate::inventory::Inventory;
use crate::lang::function::{Argument, Signature};
use crate::lang::meta::{common_or_module_emission, Docs, Visibility};
use crate::lang::types::{Type, TypeId, TypeInfo, TypeKind};

macro_rules! impl_fnptr {
    // No arguments: extern "C" fn() -> R
    ($r:ident) => {
        impl<$r: TypeInfo> TypeInfo for extern "C" fn() -> $r {
            const WIRE_SAFE: bool = $r::WIRE_SAFE;
            const RAW_SAFE: bool = $r::RAW_SAFE;

            fn id() -> TypeId {
                TypeId::new(0xEE8602B016C043561CA68291A8142F3B).derive_id($r::id())
            }

            fn kind() -> TypeKind {
                TypeKind::FnPointer(Signature {
                    arguments: vec![],
                    rval: $r::id()
                })
            }

            fn ty() -> Type {
                let r_ty = $r::ty();
                let emission = r_ty.emission;
                let signature = Signature {
                    arguments: vec![],
                    rval: $r::id()
                };

                Type {
                    emission,
                    docs: Docs::empty(),
                    visibility: Visibility::Public,
                    name: format!(r#"extern "C" fn() -> {}"#, r_ty.name),
                    kind: TypeKind::FnPointer(signature),
                }
            }

            fn register(inventory: &mut Inventory) {
                $r::register(inventory);
                inventory.register_type(Self::id(), Self::ty());
            }
        }

        impl<$r: TypeInfo> TypeInfo for Option<extern "C" fn() -> $r> {
            const WIRE_SAFE: bool = $r::WIRE_SAFE;
            const RAW_SAFE: bool = $r::RAW_SAFE;

            fn id() -> TypeId {
                <extern "C" fn() -> $r as TypeInfo>::id()
            }

            fn kind() -> TypeKind {
                <extern "C" fn() -> $r as TypeInfo>::kind()
            }

            fn ty() -> Type {
                <extern "C" fn() -> $r as TypeInfo>::ty()
            }

            fn register(inventory: &mut Inventory) {
                <extern "C" fn() -> $r as TypeInfo>::register(inventory);
            }
        }
    };

    // With arguments: extern "C" fn(T1, T2, ...) -> R
    ($r:ident, $($t:ident),+) => {
        #[allow(unused_assignments)]
        impl<$r, $($t),+> TypeInfo for extern "C" fn($($t),+) -> $r
        where
            $($t: TypeInfo,)+
            $r: TypeInfo,
        {
            const WIRE_SAFE: bool = $r::WIRE_SAFE $(&& $t::WIRE_SAFE)+;
            const RAW_SAFE: bool = $r::RAW_SAFE $(&& $t::RAW_SAFE)+;

            fn id() -> TypeId {
                TypeId::new(0xEE8602B016C043561CA68291A8142F3B)
                    .derive_id($r::id())
                    $(.derive_id($t::id()))+
            }

            fn kind() -> TypeKind {
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

                TypeKind::FnPointer(Signature {
                    arguments,
                    rval: $r::id()
                })
            }

            fn ty() -> Type {
                let r_ty = $r::ty();
                $(let $t = $t::ty();)+

                let ty_params = [
                    r_ty.emission.clone(),
                    $($t.emission.clone()),+
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

                Type {
                    emission,
                    docs: Docs::empty(),
                    visibility: Visibility::Public,
                    name: format!(r#"extern "C" fn({}) -> {}"#,
                        [$($t.name.clone()),+].join(", "),
                        r_ty.name
                    ),
                    kind: TypeKind::FnPointer(signature),
                }
            }

            fn register(inventory: &mut Inventory) {
                $r::register(inventory);
                $($t::register(inventory);)+
                inventory.register_type(Self::id(), Self::ty());
            }
        }

        impl<$r, $($t),+> TypeInfo for Option<extern "C" fn($($t),+) -> $r>
        where
            $($t: TypeInfo,)+
            $r: TypeInfo,
        {
            const WIRE_SAFE: bool = $r::WIRE_SAFE $(&& $t::WIRE_SAFE)+;
            const RAW_SAFE: bool = $r::RAW_SAFE $(&& $t::RAW_SAFE)+;

            fn id() -> TypeId {
                <extern "C" fn($($t),+) -> $r as TypeInfo>::id()
            }

            fn kind() -> TypeKind {
                <extern "C" fn($($t),+) -> $r as TypeInfo>::kind()
            }

            fn ty() -> Type {
                <extern "C" fn($($t),+) -> $r as TypeInfo>::ty()
            }

            fn register(inventory: &mut Inventory) {
                <extern "C" fn($($t),+) -> $r as TypeInfo>::register(inventory);
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
