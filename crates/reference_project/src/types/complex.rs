use crate::types::basic::Vec3f32;
use crate::types::enums::EnumPayload;
use interoptopus::lang::types::TypeInfo;
use interoptopus::{ffi, ffi_type};
// Some nested object hierarchy using enums and generics.

#[ffi_type]
pub enum Layer3<T: TypeInfo> {
    A(Layer1<T>),
    B(Layer2<T>),
}

#[ffi_type]
pub struct Layer2<T: TypeInfo> {
    pub layer_1: Layer1<T>,
    pub vec: Vec3f32,
    pub the_enum: EnumPayload,
    pub strings: ffi::Vec<ffi::String>,
}

#[ffi_type]
pub struct Layer1<T: TypeInfo> {
    pub maybe_1: ffi::Option<T>,
    pub maybe_2: ffi::Vec<T>,
    pub maybe_3: T,
}
