use interoptopus::ffi;

#[ffi(transparent)]
pub struct TransparentNum(usize);

#[ffi]
pub struct IVec3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[ffi]
pub enum EnumNum {
    A(usize),
    B,
    C(isize),
}
