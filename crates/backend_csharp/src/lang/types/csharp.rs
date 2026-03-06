use crate::model::TypeId;

// A C# `string`, and the extra generated types `CStrPtr` and `Utf8String`
pub const STRING: TypeId = TypeId::new(0xBEA1AB15FD5682B7649431E17CB70B61);
pub const CSTR_PTR: TypeId = TypeId::new(0x23A15DA804954D6FA2092D2FA2177E7C);
pub const UTF8_STRING: TypeId = TypeId::new(0xA6E549EB2961D56F3AA8A3BDAF748FDC);
