mod id;

use crate::lang2::constant::{Constant, ConstantId};
use crate::lang2::function::{Function, FunctionId};
use crate::lang2::libpattern::LibraryPattern;
use crate::lang2::types::{Type, TypeId};
use crate::lang2::Register;
pub use id::Id;
use std::collections::HashMap;
use std::mem::swap;

#[derive(Default)]
pub struct Inventory {
    types: HashMap<TypeId, Type>,
    functions: HashMap<FunctionId, Function>,
    constants: HashMap<ConstantId, Constant>,
    library_pattern: Vec<LibraryPattern>,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_type(&mut self, id: TypeId, ty: Type) -> &mut Self {
        self.types.entry(id).or_insert(ty);
        self
    }

    pub fn register_function(&mut self, id: FunctionId, function: Function) -> &mut Self {
        self.functions.entry(id).or_insert(function);
        self
    }

    pub fn register_constant(&mut self, id: ConstantId, constant: Constant) -> &mut Self {
        self.constants.entry(id).or_insert(constant);
        self
    }

    pub fn register(&mut self, f: impl Fn(&mut Self)) -> &mut Self {
        f(self);
        self
    }

    pub fn validate(&mut self) -> Self {
        let mut rval = Inventory::new();
        swap(&mut rval, self);
        rval
    }
}

mod testxxx {
    use crate::inventory2::Inventory;
    use crate::lang2::function::{Function, FunctionId, Signature};
    use crate::lang2::meta::{Emission, Visibility};
    use crate::lang2::{FunctionInfo, Register, TypeInfo};

    fn foo() {}
    struct foo {}

    impl FunctionInfo for foo {
        fn id() -> FunctionId {
            FunctionId::new(0)
        }
    }

    impl Register for foo {
        fn register(inv: &mut Inventory) {
            inv.register_function(
                foo::id(),
                Function {
                    name: "".to_string(),
                    visibility: Visibility::Public,
                    docs: Default::default(),
                    emission: Emission::Builtin,
                    signature: Signature { arguments: vec![], rval: <()>::id() },
                },
            );
        }
    }

    #[macro_export]
    macro_rules! builtins_string2 {
        () => {{
            use crate::lang;
            use crate::lang::FunctionInfo;

            pub fn interoptopus_string_create(utf8: *const ::std::ffi::c_void, len: u64, rval: &mut ::std::mem::MaybeUninit<$crate::pattern::string::String>) -> i64 {
                let slice = if utf8.is_null() {
                    &[]
                } else {
                    unsafe { ::std::slice::from_raw_parts::<u8>(utf8.cast(), len as usize) }
                };
                let vec = slice.to_vec();
                let string = unsafe { ::std::string::String::from_utf8_unchecked(vec) };
                rval.write($crate::pattern::string::String::from_string(string));
                0
            }

            pub fn interoptopus_string_destroy(utf8: $crate::pattern::string::String) -> i64 {
                0
            }

            pub fn interoptopus_string_clone(utf8: &$crate::pattern::string::String, rval: &mut ::std::mem::MaybeUninit<$crate::pattern::string::String>) -> i64 {
                rval.write(utf8.clone());
                0
            }

            |_: &mut Inventory| {}
        }};
    }

    #[test]
    #[rustfmt::skip]
    fn f() {
        let mut inv = Inventory::new()
            // .register::<foo>()
            // .register::<foo>()
            // .register2(|x| foo::register(x))
            .register(builtins_string2!())
            .validate();

        let mut inv2 = Inventory::new();
    inv2.validate();
        // argh, issue is to register builtins and such which need to emit a function and register it ...


        // expliti
        foo::register(&mut inv);
        u32::register(&mut inv);
    }
}
