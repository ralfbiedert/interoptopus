use interoptopus::ffi_type;

#[ffi_type(transparent)]
pub struct TransparentNum(usize);

#[ffi_type]
pub struct IVec3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[ffi_type]
pub enum EnumNum {
    A(usize),
    B,
    C(isize),
}
