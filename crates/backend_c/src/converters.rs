use crate::Interop;
use crate::interop::{EnumVariants, ToNamingStyle};
use interoptopus::backend::safe_name;
use interoptopus::lang::{Composite, Constant, ConstantValue, Enum, FnPointer, Function, Opaque, Primitive, PrimitiveValue, Type, Variant};
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::callback::NamedCallback;

pub fn primitive_to_typename(x: Primitive) -> String {
    match x {
        Primitive::Void => "void".to_string(),
        Primitive::Bool => "bool".to_string(),
        Primitive::U8 => "uint8_t".to_string(),
        Primitive::U16 => "uint16_t".to_string(),
        Primitive::U32 => "uint32_t".to_string(),
        Primitive::U64 => "uint64_t".to_string(),
        Primitive::Usize => "size_t".to_string(),
        Primitive::I8 => "int8_t".to_string(),
        Primitive::I16 => "int16_t".to_string(),
        Primitive::I32 => "int32_t".to_string(),
        Primitive::I64 => "int64_t".to_string(),
        Primitive::Isize => "ptrdiff_t".to_string(),
        Primitive::F32 => "float".to_string(),
        Primitive::F64 => "double".to_string(),
    }
}

pub fn enum_to_typename(g: &Interop, x: &Enum) -> String {
    format!("{}{}", g.prefix, x.rust_name()).to_naming_style(&g.enum_variant_naming)
}

pub fn enum_variant_to_name(g: &Interop, the_enum: &Enum, x: &Variant) -> String {
    if g.enum_variant_style == EnumVariants::WithEnumName {
        format!("{}{}_{}", g.prefix, the_enum.rust_name().to_naming_style(&g.enum_variant_naming), x.name()).to_naming_style(&g.enum_variant_naming)
    } else {
        format!("{}{}", g.prefix, x.name()).to_naming_style(&g.enum_variant_naming)
    }
}

pub fn opaque_to_typename(g: &Interop, x: &Opaque) -> String {
    format!("{}{}", g.prefix, x.rust_name()).to_naming_style(&g.type_naming)
}

pub fn composite_to_typename(g: &Interop, x: &Composite) -> String {
    format!("{}{}", g.prefix, x.rust_name()).to_naming_style(&g.type_naming)
}

pub fn fnpointer_to_typename(g: &Interop, x: &FnPointer) -> String {
    let prefixed = format!("{}fptr", g.prefix);
    [prefixed, safe_name(&x.internal_name())].join("_")
}

pub fn named_callback_to_typename(g: &Interop, x: &NamedCallback) -> String {
    format!("{}{}", g.prefix, x.name().to_naming_style(&g.type_naming))
}

pub fn to_type_specifier(g: &Interop, x: &Type) -> String {
    match x {
        Type::Primitive(x) => primitive_to_typename(*x),
        Type::Enum(x) => enum_to_typename(g, x),
        Type::Opaque(x) => opaque_to_typename(g, x),
        Type::Composite(x) => composite_to_typename(g, x),
        Type::Wired(_) => todo!(),
        Type::Domain(_) => todo!(),
        Type::ReadPointer(x) => format!("const {}*", to_type_specifier(g, x)),
        Type::ReadWritePointer(x) => format!("{}*", to_type_specifier(g, x)),
        Type::FnPointer(x) => fnpointer_to_typename(g, x),
        Type::Pattern(TypePattern::CChar) => "char".to_string(),
        Type::Pattern(TypePattern::NamedCallback(x)) => named_callback_to_typename(g, x),
        Type::Pattern(x) => to_type_specifier(g, &x.fallback_type()),
        // TODO: This should be handled in nicer way so that arrays-of-arrays and other thing work properly
        Type::Array(_) => panic!("Arrays need special handling in the writer."),
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
            PrimitiveValue::Usize(x) => format!("{x}"),
            PrimitiveValue::I8(x) => format!("{x}"),
            PrimitiveValue::I16(x) => format!("{x}"),
            PrimitiveValue::I32(x) => format!("{x}"),
            PrimitiveValue::I64(x) => format!("{x}"),
            PrimitiveValue::Isize(x) => format!("{x}"),
            PrimitiveValue::F32(x) => format!("{x}"),
            PrimitiveValue::F64(x) => format!("{x}"),
        },
    }
}

pub fn function_name_to_c_name(function: &Function) -> String {
    function.name().to_string()
}
