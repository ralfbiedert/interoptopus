mod forbidden;
mod id;
mod macros;

pub use id::Id;

use crate::lang2::constant::{Constant, ConstantId};
use crate::lang2::function::{Function, FunctionId};
use crate::lang2::service::{Service, ServiceId};
use crate::lang2::types::{Type, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::swap;

#[derive(Default)]
pub struct Inventory {
    pub types: HashMap<TypeId, Type>,
    pub functions: HashMap<FunctionId, Function>,
    pub constants: HashMap<ConstantId, Constant>,
    pub services: HashMap<ServiceId, Service>,
    _guard: PhantomData<()>,
}

impl Inventory {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_type(&mut self, id: TypeId, ty: Type) {
        self.types.entry(id).or_insert(ty);
    }

    pub fn register_function(&mut self, id: FunctionId, function: Function) {
        self.functions.entry(id).or_insert(function);
    }

    pub fn register_constant(&mut self, id: ConstantId, constant: Constant) {
        self.constants.entry(id).or_insert(constant);
    }

    pub fn register_service(&mut self, id: ServiceId, service: Service) {
        self.services.entry(id).or_insert(service);
    }

    #[must_use]
    pub fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self {
        f(self);
        self
    }

    #[must_use]
    pub fn validate(&mut self) -> Self {
        let mut rval = Self::new();
        swap(&mut rval, self);
        rval
    }
}

// mod testxxx {
//     use crate::inventory2::Inventory;
//     use crate::lang2::function::{Function, FunctionId, Signature};
//     use crate::lang2::meta::{Docs, Emission, Visibility};
//     use crate::lang2::{FunctionInfo, Register, TypeInfo};
//     use crate::{extra_type2, function2, item2};
//
//     fn foo() {}
//     struct foo {}
//
//     impl FunctionInfo for foo {
//         fn id() -> FunctionId {
//             FunctionId::new(0)
//         }
//     }
//
//     impl Register for foo {
//         fn register(inv: &mut Inventory) {
//             inv.register_function(
//                 Self::id(),
//                 Function {
//                     name: String::new(),
//                     visibility: Visibility::Public,
//                     docs: Docs::default(),
//                     emission: Emission::Builtin,
//                     signature: Signature { arguments: vec![], rval: <()>::id() },
//                 },
//             );
//         }
//     }

// #[macro_export]
// macro_rules! builtins_string2 {
//     () => {{
//         use $crate::lang;
//         use $crate::lang::FunctionInfo;
//
//         pub fn interoptopus_string_create(utf8: *const ::std::ffi::c_void, len: u64, rval: &mut ::std::mem::MaybeUninit<$crate::pattern::string::String>) -> i64 {
//             let slice = if utf8.is_null() {
//                 &[]
//             } else {
//                 unsafe { ::std::slice::from_raw_parts::<u8>(utf8.cast(), len as usize) }
//             };
//             let vec = slice.to_vec();
//             let string = unsafe { ::std::string::String::from_utf8_unchecked(vec) };
//             rval.write($crate::pattern::string::String::from_string(string));
//             0
//         }
//
//         pub fn interoptopus_string_destroy(utf8: $crate::pattern::string::String) -> i64 {
//             0
//         }
//
//         pub fn interoptopus_string_clone(utf8: &$crate::pattern::string::String, rval: &mut ::std::mem::MaybeUninit<$crate::pattern::string::String>) -> i64 {
//             rval.write(utf8.clone());
//             0
//         }
//
//         |_: &mut Inventory| {}
//     }};
// }
//     #[test]
//     #[rustfmt::skip]
//     fn f() {
//         let mut inventory = Inventory::new()
//             .register(item2!(foo))
//             .register(item2!(u32))
//             .register(builtins_string2!())
//             .validate();
//     }
// }
