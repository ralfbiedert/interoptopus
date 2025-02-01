use crate::generator::ToNamingStyle;
use crate::Interop;
use interoptopus::lang::c::{CType, CompositeType, Constant, ConstantValue, EnumType, FnPointerType, Function, OpaqueType, PrimitiveType, PrimitiveValue, Variant};
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::TypePattern;
use interoptopus::util::safe_name;

/// Implements [`CTypeConverter`].
#[derive(Debug, Clone, Default)]
pub(crate) struct Converter {}

/// Converts Interoptopus types to C types.
impl Converter {
    pub fn primitive_to_typename(&self, x: PrimitiveType) -> String {
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

    pub fn enum_to_typename(&self, g: &Interop, x: &EnumType) -> String {
        format!("{}{}", g.prefix, x.rust_name()).to_naming_style(&g.type_naming)
    }

    pub fn enum_variant_to_name(&self, g: &Interop, the_enum: &EnumType, x: &Variant) -> String {
        format!("{}{}_{}", g.prefix, the_enum.rust_name().to_naming_style(&g.type_naming), x.name()).to_naming_style(&g.enum_variant_naming)
    }

    pub fn opaque_to_typename(&self, g: &Interop, x: &OpaqueType) -> String {
        format!("{}{}", g.prefix, x.rust_name()).to_naming_style(&g.type_naming)
    }

    pub fn composite_to_typename(&self, g: &Interop, x: &CompositeType) -> String {
        format!("{}{}", g.prefix, x.rust_name()).to_naming_style(&g.type_naming)
    }

    pub fn fnpointer_to_typename(&self, g: &Interop, x: &FnPointerType) -> String {
        let prefixed = format!("{}fptr", g.prefix);
        [prefixed, safe_name(&x.internal_name())].join("_")
    }

    pub fn named_callback_to_typename(&self, g: &Interop, x: &NamedCallback) -> String {
        format!("{}{}", g.prefix, x.name().to_naming_style(&g.type_naming))
    }

    pub fn to_type_specifier(&self, g: &Interop, x: &CType) -> String {
        match x {
            CType::Primitive(x) => self.primitive_to_typename(*x),
            CType::Enum(x) => self.enum_to_typename(g, x),
            CType::Opaque(x) => self.opaque_to_typename(g, x),
            CType::Composite(x) => self.composite_to_typename(g, x),
            CType::ReadPointer(x) => format!("const {}*", self.to_type_specifier(g, x)),
            CType::ReadWritePointer(x) => format!("{}*", self.to_type_specifier(g, x)),
            CType::FnPointer(x) => self.fnpointer_to_typename(g, x),
            CType::Pattern(TypePattern::CChar) => "char".to_string(),
            CType::Pattern(TypePattern::NamedCallback(x)) => self.named_callback_to_typename(g, x),
            CType::Pattern(x) => self.to_type_specifier(g, &x.fallback_type()),
            // TODO: This should be handled in nicer way so that arrays-of-arrays and other thing work properly
            CType::Array(_) => panic!("Arrays need special handling in the writer."),
        }
    }

    pub fn const_name_to_name(&self, g: &Interop, x: &Constant) -> String {
        format!("{}{}", g.prefix, x.name()).to_naming_style(&g.const_naming)
    }

    pub fn constant_value_to_value(&self, value: &ConstantValue) -> String {
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

    pub fn function_name_to_c_name(&self, function: &Function) -> String {
        function.name().to_string()
    }
}
