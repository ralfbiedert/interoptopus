use crate::types::basic::Vec3f32;
use crate::types::enums::EnumPayload;
use interoptopus::ffi;
use interoptopus::lang::types::TypeInfo;
// Some nested object hierarchy using enums and generics.

#[ffi]
pub enum Layer3<T: TypeInfo> {
    A(Layer1<T>),
    B(Layer2<T>),
}

#[ffi]
pub struct Layer2<T: TypeInfo> {
    pub layer_1: Layer1<T>,
    pub vec: Vec3f32,
    pub the_enum: EnumPayload,
    pub strings: ffi::Vec<ffi::String>,
}

#[ffi]
pub struct Layer1<T: TypeInfo> {
    pub maybe_1: ffi::Option<T>,
    pub maybe_2: ffi::Vec<T>,
    pub maybe_3: T,
}
