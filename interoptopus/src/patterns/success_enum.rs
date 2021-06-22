//! For return enums with defined `Ok` variants, translating to exceptions if not met.

use crate::lang::c::{EnumType, Variant};

pub trait Success {
    const SUCCESS: Self;
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SuccessEnum {
    the_enum: EnumType,
    success_variant: Variant,
}

impl SuccessEnum {
    pub fn new(the_enum: EnumType, success_variant: Variant) -> Self {
        Self { the_enum, success_variant }
    }

    pub fn the_enum(&self) -> &EnumType {
        &self.the_enum
    }

    pub fn success_variant(&self) -> &Variant {
        &self.success_variant
    }
}

/// Helper to transform [`Error`] types to [`Success`] enums inside `extern "C"` functions.
pub fn error_to_ffi_error<E, FE: Success>(f: impl FnOnce() -> Result<(), E>) -> FE
where
    FE: From<E>,
{
    match f() {
        Ok(_) => FE::SUCCESS,
        Err(e) => e.into(),
    }
}
