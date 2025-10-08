use crate::lang::types::Primitive;
use crate::lang::types::SerializationError;
use std::io::{Read, Write};

macro_rules! impl_primitive {
    ($t:ty, $t_raw:ty, $t_str:expr, $primitive:expr, $id:expr) => {
        impl $crate::lang::types::TypeInfo for $t {
            const WIRE_SAFE: bool = true;
            const RAW_SAFE: bool = true;
            const ASYNC_SAFE: bool = true;
            const SERVICE_SAFE: bool = false;
            const SERVICE_CTOR_SAFE: bool = false;

            fn id() -> $crate::inventory::TypeId {
                $id
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
                out.write_all(&self.get().to_le_bytes())?;
                Ok(())
            }

            fn read(input: &mut impl Read) -> Result<Self, SerializationError> {
                let mut bytes = [0; size_of::<$t>()];
                input.read_exact(&mut bytes)?;
                let val = <$t_raw>::from_le_bytes(bytes);
                Ok(<$t>::new(val).unwrap()) // TODO fix conversion logic?
            }

            fn live_size(&self) -> usize {
                size_of::<Self>()
            }
        }
    };
}

// These IDs must match the primitive ones as they collapse to the same type
impl_primitive!(std::num::NonZeroU8, u8, "u8", Primitive::U8, u8::id());
impl_primitive!(std::num::NonZeroU16, u16, "u16", Primitive::U16, u16::id());
impl_primitive!(std::num::NonZeroU32, u32, "u32", Primitive::U32, u32::id());
impl_primitive!(std::num::NonZeroU64, u64, "u64", Primitive::U64, u64::id());
impl_primitive!(std::num::NonZeroUsize, usize, "usize", Primitive::Usize, usize::id());
impl_primitive!(std::num::NonZeroI8, i8, "i8", Primitive::I8, i8::id());
impl_primitive!(std::num::NonZeroI16, i16, "i16", Primitive::I16, i16::id());
impl_primitive!(std::num::NonZeroI32, i32, "i32", Primitive::I32, i32::id());
impl_primitive!(std::num::NonZeroI64, i64, "i64", Primitive::I64, i64::id());
impl_primitive!(std::num::NonZeroIsize, isize, "isize", Primitive::Isize, isize::id());

// These IDs must match the primitive ones as they collapse to the same type
// impl_primitive!(Option<std::num::NonZeroU8>, "u8", Primitive::U8, u8::id());
// impl_primitive!(Option<std::num::NonZeroU16>, "u16", Primitive::U16, u16::id());
// impl_primitive!(Option<std::num::NonZeroU32>, "u32", Primitive::U32, u32::id());
// impl_primitive!(Option<std::num::NonZeroU64>, "u64", Primitive::U64, u64::id());
// impl_primitive!(Option<std::num::NonZeroUsize>, "usize", Primitive::Usize, usize::id());
// impl_primitive!(Option<std::num::NonZeroI8>, "i8", Primitive::I8, i8::id());
// impl_primitive!(Option<std::num::NonZeroI16>, "i16", Primitive::I16, i16::id());
// impl_primitive!(Option<std::num::NonZeroI32>, "i32", Primitive::I32, i32::id());
// impl_primitive!(Option<std::num::NonZeroI64>, "i64", Primitive::I64, i64::id());
// impl_primitive!(Option<std::num::NonZeroIsize>, "isize", Primitive::Isize, isize::id());
