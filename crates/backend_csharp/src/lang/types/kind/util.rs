/// Backend-specific utility types that don't map to Rust inventory items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Util {
    InteropException,
    EnumException,
    Utf8String,
    AsyncCallbackCommon,
}
