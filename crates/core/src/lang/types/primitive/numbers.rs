use crate::lang::types::Primitive;
use crate::lang::types::SerializationError;
use std::io::{Read, Write};

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

        impl $crate::lang::types::wire::WireIO for $t {
            fn write(&self, out: &mut impl Write) -> Result<(), SerializationError> {
                out.write_all(&self.to_le_bytes())?;
                Ok(())
            }

            fn read(input: &mut impl Read) -> Result<Self, SerializationError> {
                let mut bytes = [0; size_of::<$t>()];
                input.read_exact(&mut bytes)?;
                Ok(<$t>::from_le_bytes(bytes))
            }

            fn live_size(&self) -> usize {
                size_of::<Self>()
            }
        }
    };
}

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
