use crate::types::enums::EnumRenamedXYZ;
use interoptopus::ffi_type;
use interoptopus::lang::types::TypeInfo;
use interoptopus::lang::TypeInfo;
use std::marker::PhantomData;

#[ffi_type(name = "StructRenamed")]
pub struct StructRenamedXYZ {
    pub e: EnumRenamedXYZ,
}

#[ffi_type]
pub struct Phantom<'a, T>
where
    T: 'static,
    T: TypeInfo,
{
    pub x: u32,
    #[skip]
    pub p: PhantomData<&'a T>,
}

#[ffi_type]
pub struct Visibility1 {
    // Be conservative with naming since some languages don't like `public` as a field.
    pblc: u8,
    pub prvt: u8,
}

#[ffi_type]
pub struct Visibility2 {
    pblc1: u8,
    pblc2: u8,
}
