use crate::types::basic::Vec3f32;
use crate::types::enums::EnumRenamedXYZ;
use interoptopus::ffi_type;
use interoptopus::lang::types::TypeInfo;

#[ffi_type]
#[derive(Debug, Copy, Clone)]
pub struct Array {
    pub data: [u8; 16],
}

#[ffi_type]
#[derive(Debug)]
pub struct NestedArray {
    pub field_enum: EnumRenamedXYZ,
    pub field_vec: Vec3f32,
    pub field_bool: bool,
    pub field_int: i32,
    pub field_array: [u16; 5],
    pub field_array_2: [u16; 5],
    pub field_struct: Array,
}

#[ffi_type]
#[derive(Copy, Clone, Debug)]
pub struct FixedString<const N: usize> {
    pub data: [u8; N],
}

#[ffi_type]
#[derive(Copy, Clone)]
pub struct CharArray {
    pub str: FixedString<32>,
    pub str_2: FixedString<32>,
}

#[ffi_type]
pub struct GenericArray<T>
where
    T: TypeInfo,
{
    pub data: [T; 16],
}
