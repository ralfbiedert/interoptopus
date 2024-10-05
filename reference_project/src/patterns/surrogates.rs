use interoptopus::patterns::surrogates::{CorrectSurrogate, Surrogate};
use interoptopus::{ffi_function, ffi_type};

// Let's assume we can't implement `CTypeInfo` for this.
#[repr(C)]
pub struct SomeForeignType {
    x: u32,
}

// Instead, we create a local copy of that type with matching fields.
#[ffi_type]
#[repr(C)]
pub struct Local {
    x: u32,
}

// This is really only a marker trait where you need to guarantee that `Local` is a valid surrogate
// for `SomeForeignType`. If you messed this up, you'd get UB.
unsafe impl CorrectSurrogate<SomeForeignType> for Local {}

#[ffi_type]
#[repr(C)]
pub struct Container {
    // We can then use the `Surrogate` type in our interfaces. It wil
    pub foreign: Surrogate<SomeForeignType, Local>,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_surrogates_1(s: Surrogate<SomeForeignType, Local>, c: &mut Container) {
    c.foreign = s;
}
