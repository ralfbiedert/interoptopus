use interoptopus::ffi;
use interoptopus::pattern::surrogate::{CorrectSurrogate, Surrogate};

// Let's assume we can't implement `CTypeInfo` for this.
mod foreign {
    #[repr(C)]
    pub struct SomeForeignType {
        x: u32,
    }
}

// Instead, we create a local copy of that type with matching fields.
#[ffi]
pub struct Local {
    x: u32,
}

// This is really only a marker trait where you need to guarantee that `Local` is a valid surrogate
// for `SomeForeignType`. If you messed this up, you'd get UB.
unsafe impl CorrectSurrogate<foreign::SomeForeignType> for Local {}

// Here we create a nicer alias.
type SomeForeignType = Surrogate<foreign::SomeForeignType, Local>;

#[ffi]
pub struct Container {
    // We can then use the `Surrogate` type in our interfaces. It wil
    pub foreign: SomeForeignType,
}

#[ffi]
pub fn pattern_surrogates_1(s: SomeForeignType, c: &mut Container) {
    c.foreign = s;
}
