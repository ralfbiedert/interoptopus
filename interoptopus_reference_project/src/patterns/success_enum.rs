use interoptopus::ffi_type;

#[ffi_type(patterns(success_enum))]
#[repr(C)]
pub enum FFIError {
    Ok = 0,
    Null = 100,
    Panic = 200,
    Fail = 300,
}

impl Default for FFIError {
    fn default() -> Self {
        Self::Ok
    }
}

impl interoptopus::patterns::result::FFIError for FFIError {
    const SUCCESS: Self = Self::Ok;
    const NULL: Self = Self::Null;
    const PANIC: Self = Self::Panic;
}
