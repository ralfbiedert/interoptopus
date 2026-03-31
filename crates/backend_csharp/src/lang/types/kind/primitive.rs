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

impl Primitive {
    /// Returns the C# keyword for this primitive type.
    pub fn cs_name(self) -> &'static str {
        match self {
            Self::Void => "void",
            Self::Bool => "bool",
            Self::Byte => "byte",
            Self::UShort => "ushort",
            Self::UInt => "uint",
            Self::ULong => "ulong",
            Self::NUInt => "nuint",
            Self::SByte => "sbyte",
            Self::Short => "short",
            Self::Int => "int",
            Self::Long => "long",
            Self::NInt => "nint",
            Self::Float => "float",
            Self::Double => "double",
        }
    }
}
