#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Primitive {
    Void,
    Bool,
    Byte,   // U8
    UShort, // U16
    UInt,   // U32
    ULong,  // U64
    NUInt,  // Usize -> nuint
    SByte,  // I8
    Short,  // I16
    Int,    // I32
    Long,   // I64
    NInt,   // Isize -> nint
    Float,  // F32
    Double, // F64
}
