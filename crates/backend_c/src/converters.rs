use crate::Interop;
use crate::interop::ToNamingStyle;
use interoptopus::lang::c::{CType, CompositeType, Constant, ConstantValue, EnumType, FnPointerType, Function, OpaqueType, PrimitiveType, PrimitiveValue, Variant};
use interoptopus::patterns::TypePattern;
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::util::safe_name;

pub fn primitive_to_typename(x: PrimitiveType) -> String {
    match x {
        PrimitiveType::Void => "void".to_string(),
        PrimitiveType::Bool => "bool".to_string(),
        PrimitiveType::U8 => "uint8_t".to_string(),
        PrimitiveType::U16 => "uint16_t".to_string(),
        PrimitiveType::U32 => "uint32_t".to_string(),
        PrimitiveType::U64 => "uint64_t".to_string(),
        PrimitiveType::I8 => "int8_t".to_string(),
        PrimitiveType::I16 => "int16_t".to_string(),
        PrimitiveType::I32 => "int32_t".to_string(),
        PrimitiveType::I64 => "int64_t".to_string(),
        PrimitiveType::F32 => "float".to_string(),
        PrimitiveType::F64 => "double".to_string(),
    }
}

pub fn enum_to_typename(g: &Interop, x: &EnumType) -> String {
    format!("{}{}", g.prefix, x.rust_name()).to_naming_style(&g.enum_variant_naming)
}

pub fn enum_variant_to_name(g: &Interop, the_enum: &EnumType, x: &Variant) -> String {
    format!("{}{}_{}", g.prefix, the_enum.rust_name().to_naming_style(&g.enum_variant_naming), x.name()).to_naming_style(&g.enum_variant_naming)
}

pub fn opaque_to_typename(g: &Interop, x: &OpaqueType) -> String {
    format!("{}{}", g.prefix, x.rust_name()).to_naming_style(&g.type_naming)
}

pub fn composite_to_typename(g: &Interop, x: &CompositeType) -> String {
    format!("{}{}", g.prefix, x.rust_name()).to_naming_style(&g.type_naming)
}

pub fn fnpointer_to_typename(g: &Interop, x: &FnPointerType) -> String {
    let prefixed = format!("{}fptr", g.prefix);
    [prefixed, safe_name(&x.internal_name())].join("_")
}

pub fn named_callback_to_typename(g: &Interop, x: &NamedCallback) -> String {
    format!("{}{}", g.prefix, x.name().to_naming_style(&g.type_naming))
}

pub fn to_type_specifier(g: &Interop, x: &CType) -> String {
    match x {
        CType::Primitive(x) => primitive_to_typename(*x),
        CType::Enum(x) => enum_to_typename(g, x),
        CType::Opaque(x) => opaque_to_typename(g, x),
        CType::Composite(x) => composite_to_typename(g, x),
        CType::ReadPointer(x) => format!("const {}*", to_type_specifier(g, x)),
        CType::ReadWritePointer(x) => format!("{}*", to_type_specifier(g, x)),
        CType::FnPointer(x) => fnpointer_to_typename(g, x),
        CType::Pattern(TypePattern::CChar) => "char".to_string(),
        CType::Pattern(TypePattern::NamedCallback(x)) => named_callback_to_typename(g, x),
        CType::Pattern(x) => to_type_specifier(g, &x.fallback_type()),
        // TODO: This should be handled in nicer way so that arrays-of-arrays and other thing work properly
        CType::Array(_) => panic!("Arrays need special handling in the writer."),
    }
}

pub fn const_name_to_name(g: &Interop, x: &Constant) -> String {
    format!("{}{}", g.prefix, x.name()).to_naming_style(&g.const_naming)
}

pub fn constant_value_to_value(value: &ConstantValue) -> String {
    match value {
        ConstantValue::Primitive(x) => match x {
            PrimitiveValue::Bool(x) => format!("{x}"),
            PrimitiveValue::U8(x) => format!("{x}"),
            PrimitiveValue::U16(x) => format!("{x}"),
            PrimitiveValue::U32(x) => format!("{x}"),
            PrimitiveValue::U64(x) => format!("{x}"),
            PrimitiveValue::I8(x) => format!("{x}"),
            PrimitiveValue::I16(x) => format!("{x}"),
            PrimitiveValue::I32(x) => format!("{x}"),
            PrimitiveValue::I64(x) => format!("{x}"),
            PrimitiveValue::F32(x) => format!("{x}"),
            PrimitiveValue::F64(x) => format!("{x}"),
        },
    }
}

pub fn function_name_to_c_name(function: &Function) -> String {
    function.name().to_string()
}
