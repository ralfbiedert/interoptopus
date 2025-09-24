use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
pub enum Primitive {
    Void,
    Bool,
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum PrimitiveValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Isize(isize),
    F32(f32),
    F64(f64),
}

impl Hash for PrimitiveValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Bool(v) => v.hash(state),
            Self::U8(v) => v.hash(state),
            Self::U16(v) => v.hash(state),
            Self::U32(v) => v.hash(state),
            Self::U64(v) => v.hash(state),
            Self::Usize(v) => v.hash(state),
            Self::I8(v) => v.hash(state),
            Self::I16(v) => v.hash(state),
            Self::I32(v) => v.hash(state),
            Self::I64(v) => v.hash(state),
            Self::Isize(v) => v.hash(state),
            Self::F32(v) => v.to_bits().hash(state),
            Self::F64(v) => v.to_bits().hash(state),
        }
    }
}

macro_rules! impl_primitive {
    ($t:ty, $t_str:expr, $primitive:expr, $id:expr) => {
        impl $crate::lang::types::TypeInfo for $t {
            const WIRE_SAFE: bool = true;
            const RAW_SAFE: bool = true;
            const ASYNC_SAFE: bool = true;
            const SERVICE_SAFE: bool = false;
            const SERVICE_CTOR_SAFE: bool = false;

            fn id() -> $crate::inventory::TypeId {
                $crate::inventory::TypeId::new($id)
            }

            fn kind() -> $crate::lang::types::TypeKind {
                $crate::lang::types::TypeKind::Primitive($primitive)
            }

            fn ty() -> $crate::lang::types::Type {
                $crate::lang::types::Type {
                    emission: $crate::lang::meta::Emission::Builtin,
                    docs: $crate::lang::meta::Docs::empty(),
                    visibility: $crate::lang::meta::Visibility::Public,
                    name: $t_str.to_string(),
                    kind: Self::kind(),
                }
            }

            fn register(inventory: &mut $crate::inventory::Inventory) {
                let type_id = Self::id();
                let type_ = Self::ty();
                _ = inventory.register_type(type_id, type_)
            }
        }
    };
}

macro_rules! impl_const_value_primitive {
    (
        $rust_type:ty,
        $x:path
    ) => {
        impl From<$rust_type> for $crate::lang::constant::Value {
            fn from(x: $rust_type) -> Self {
                Self::Primitive($x(x))
            }
        }
    };
}

impl_primitive!((), "()", Primitive::Void, 0x6D87F0180F529932F56D3B4800145193);
impl_primitive!(bool, "bool", Primitive::Bool, 0xCA37AD739D5997FE7F9E1B0B2CCBACE1);

impl_primitive!(u8, "u8", Primitive::U8, 0x71B959DF6819BA4F2D20FBC7DF5D714D);
impl_primitive!(u16, "u16", Primitive::U16, 0x230B6DE211C8701AB64CB312C739A3DF);
impl_primitive!(u32, "u32", Primitive::U32, 0xEFB9E4EA19FBA8FEA6C796DF520821DF);
impl_primitive!(u64, "u64", Primitive::U64, 0xFB257E060F594616544A2AF2AB61C963);
impl_primitive!(usize, "usize", Primitive::Usize, 0x2EC966CAEA7C18435B0323E7C2B382FB);
impl_primitive!(i8, "i8", Primitive::I8, 0x10D7EB99A0957C5C5EC0007F3BBE1B40);
impl_primitive!(i16, "i16", Primitive::I16, 0x1044CC231AE1F904F9B9F7448D4A3F12);
impl_primitive!(i32, "i32", Primitive::I32, 0xDC2383EC7347146B940073226EA63AF1);
impl_primitive!(i64, "i64", Primitive::I64, 0xE8D0CA92F0E58E054DE9861583451238);
impl_primitive!(isize, "isize", Primitive::Isize, 0xDBFDABF4E0551C776CCA5FB7D7A57006);
impl_primitive!(f32, "f32", Primitive::F32, 0xCFF64C33A5D10D6817AC52138D18F407);
impl_primitive!(f64, "f64", Primitive::F64, 0xBAF8C417793FA35FF706A32A7D61DBD1);

// These IDs must match the ones above as they collapse to the same type
impl_primitive!(std::num::NonZeroU8, "u8", Primitive::U8, 0x71B959DF6819BA4F2D20FBC7DF5D714D);
impl_primitive!(std::num::NonZeroU16, "u16", Primitive::U16, 0x230B6DE211C8701AB64CB312C739A3DF);
impl_primitive!(std::num::NonZeroU32, "u32", Primitive::U32, 0xEFB9E4EA19FBA8FEA6C796DF520821DF);
impl_primitive!(std::num::NonZeroU64, "u64", Primitive::U64, 0xFB257E060F594616544A2AF2AB61C963);
impl_primitive!(std::num::NonZeroUsize, "usize", Primitive::Usize, 0x2EC966CAEA7C18435B0323E7C2B382FB);
impl_primitive!(std::num::NonZeroI8, "i8", Primitive::I8, 0x10D7EB99A0957C5C5EC0007F3BBE1B40);
impl_primitive!(std::num::NonZeroI16, "i16", Primitive::I16, 0x1044CC231AE1F904F9B9F7448D4A3F12);
impl_primitive!(std::num::NonZeroI32, "i32", Primitive::I32, 0xDC2383EC7347146B940073226EA63AF1);
impl_primitive!(std::num::NonZeroI64, "i64", Primitive::I64, 0xE8D0CA92F0E58E054DE9861583451238);
impl_primitive!(std::num::NonZeroIsize, "isize", Primitive::Isize, 0xDBFDABF4E0551C776CCA5FB7D7A57006);

// These IDs must match the ones above as they collapse to the same type
impl_primitive!(Option<std::num::NonZeroU8>, "u8", Primitive::U8, 0x71B959DF6819BA4F2D20FBC7DF5D714D);
impl_primitive!(Option<std::num::NonZeroU16>, "u16", Primitive::U16, 0x230B6DE211C8701AB64CB312C739A3DF);
impl_primitive!(Option<std::num::NonZeroU32>, "u32", Primitive::U32, 0xEFB9E4EA19FBA8FEA6C796DF520821DF);
impl_primitive!(Option<std::num::NonZeroU64>, "u64", Primitive::U64, 0xFB257E060F594616544A2AF2AB61C963);
impl_primitive!(Option<std::num::NonZeroUsize>, "usize", Primitive::Usize, 0x2EC966CAEA7C18435B0323E7C2B382FB);
impl_primitive!(Option<std::num::NonZeroI8>, "i8", Primitive::I8, 0x10D7EB99A0957C5C5EC0007F3BBE1B40);
impl_primitive!(Option<std::num::NonZeroI16>, "i16", Primitive::I16, 0x1044CC231AE1F904F9B9F7448D4A3F12);
impl_primitive!(Option<std::num::NonZeroI32>, "i32", Primitive::I32, 0xDC2383EC7347146B940073226EA63AF1);
impl_primitive!(Option<std::num::NonZeroI64>, "i64", Primitive::I64, 0xE8D0CA92F0E58E054DE9861583451238);
impl_primitive!(Option<std::num::NonZeroIsize>, "isize", Primitive::Isize, 0xDBFDABF4E0551C776CCA5FB7D7A57006);

impl_const_value_primitive!(u8, PrimitiveValue::U8);
impl_const_value_primitive!(u16, PrimitiveValue::U16);
impl_const_value_primitive!(u32, PrimitiveValue::U32);
impl_const_value_primitive!(u64, PrimitiveValue::U64);
impl_const_value_primitive!(usize, PrimitiveValue::Usize);
impl_const_value_primitive!(i8, PrimitiveValue::I8);
impl_const_value_primitive!(i16, PrimitiveValue::I16);
impl_const_value_primitive!(i32, PrimitiveValue::I32);
impl_const_value_primitive!(i64, PrimitiveValue::I64);
impl_const_value_primitive!(isize, PrimitiveValue::Isize);
impl_const_value_primitive!(f32, PrimitiveValue::F32);
impl_const_value_primitive!(f64, PrimitiveValue::F64);
impl_const_value_primitive!(bool, PrimitiveValue::Bool);
