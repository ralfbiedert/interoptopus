use crate::types::{ambiguous1, ambiguous2, EnumDocumented, EnumRenamedXYZ, StructDocumented, StructRenamedXYZ, Visibility1, Visibility2};
use interoptopus::ffi_function;

#[ffi_function]
pub fn meta_renamed(x: StructRenamedXYZ) -> EnumRenamedXYZ {
    x.e
}

/// This function has documentation.
#[ffi_function]
pub fn meta_documented(_x: StructDocumented) -> EnumDocumented {
    EnumDocumented::A
}

#[ffi_function]
pub fn meta_ambiguous_1(x: ambiguous1::Vec) -> ambiguous1::Vec {
    x
}

#[ffi_function]
pub fn meta_ambiguous_2(x: ambiguous2::Vec) -> ambiguous2::Vec {
    x
}

#[ffi_function]
pub fn meta_ambiguous_3(x: ambiguous1::Vec, y: ambiguous2::Vec) -> bool {
    (x.x as f64 - y.x).abs() < 0.5
}

#[ffi_function]
pub fn meta_visibility1(_x: Visibility1, _y: Visibility2) {}
