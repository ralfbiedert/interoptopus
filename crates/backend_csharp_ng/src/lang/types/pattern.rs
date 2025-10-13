use crate::lang::types::TypeIdCs;

pub enum TypePattern {
    Utf8String,
    Slice(TypeIdCs),
    SliceMut(TypeIdCs),
    Vec(TypeIdCs),
    Option(TypeIdCs),
    Result(TypeIdCs, TypeIdCs),
}
