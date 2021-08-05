use crate::Config;
use interoptopus::lang::c::{CType, CompositeType, Constant, ConstantValue, EnumType, FnPointerType, Function, OpaqueType, PrimitiveType, PrimitiveValue, Variant};
use interoptopus::util::safe_name;

/// Implements [`CTypeConverter`].
#[derive(Clone)]
pub struct Converter {
    pub(crate) config: Config,
}

/// Converts Interoptopus types to C types.
pub trait CTypeConverter {
    fn config(&self) -> &Config;

    /// Converts a primitive (Rust) type to a native C# type name, e.g., `f32` to `float`.
    fn primitive_to_typename(&self, x: &PrimitiveType) -> String;

    /// Converts a Rust enum name such as `Error` to a C# enum name `Error`.
    fn enum_to_typename(&self, x: &EnumType) -> String;

    fn enum_variant_to_name(&self, the_enum: &EnumType, x: &Variant) -> String;

    /// TODO Converts an opaque Rust struct `Context` to a C# struct ``.
    fn opaque_to_typename(&self, opaque: &OpaqueType) -> String;

    /// Converts an Rust struct name `Vec2` to a C# struct name `Vec2`.
    fn composite_to_typename(&self, x: &CompositeType) -> String;

    /// Converts an Rust `fn()` to a C# delegate name such as `InteropDelegate`.
    fn fnpointer_to_typename(&self, x: &FnPointerType) -> String;

    /// Converts the `u32` part in a Rust paramter `x: u32` to a C# equivalent. Might convert pointers to `out X` or `ref X`.
    fn to_type_specifier(&self, x: &CType) -> String;

    fn const_name_to_name(&self, x: &Constant) -> String;

    fn constant_value_to_value(&self, value: &ConstantValue) -> String;

    fn function_name_to_c_name(&self, function: &Function) -> String;
}

impl CTypeConverter for Converter {
    fn config(&self) -> &Config {
        &self.config
    }

    fn primitive_to_typename(&self, x: &PrimitiveType) -> String {
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

    fn enum_to_typename(&self, x: &EnumType) -> String {
        format!("{}{}", self.config().prefix, x.rust_name().to_string()).to_lowercase()
    }

    fn enum_variant_to_name(&self, the_enum: &EnumType, x: &Variant) -> String {
        format!("{}{}_{}", self.config().prefix, the_enum.rust_name(), x.name().to_string()).to_uppercase()
    }

    fn opaque_to_typename(&self, x: &OpaqueType) -> String {
        format!("{}{}", self.config().prefix, x.rust_name().to_string()).to_lowercase()
    }

    fn composite_to_typename(&self, x: &CompositeType) -> String {
        format!("{}{}", self.config().prefix, x.rust_name().to_string()).to_lowercase()
    }

    fn fnpointer_to_typename(&self, x: &FnPointerType) -> String {
        let prefixed = format!("{}fptr", self.config().prefix);
        vec![prefixed, safe_name(&x.internal_name())].join("_")
    }

    fn to_type_specifier(&self, x: &CType) -> String {
        match x {
            CType::Primitive(x) => self.primitive_to_typename(x),
            CType::Enum(x) => self.enum_to_typename(x),
            CType::Opaque(x) => self.opaque_to_typename(x),
            CType::Composite(x) => self.composite_to_typename(x),
            CType::ReadPointer(x) => format!("{}*", self.to_type_specifier(x)),
            CType::ReadWritePointer(x) => format!("{}*", self.to_type_specifier(x)),
            CType::FnPointer(x) => self.fnpointer_to_typename(x),
            CType::Pattern(x) => self.to_type_specifier(&x.fallback_type()),
            // TODO: This should be handled in nicer way so that arrays-of-arrays and other thing work properly
            CType::Array(_) => panic!("Arrays need special handling in the writer."),
        }
    }

    fn const_name_to_name(&self, x: &Constant) -> String {
        format!("{}{}", self.config().prefix, x.name().to_string()).to_uppercase()
    }

    fn constant_value_to_value(&self, value: &ConstantValue) -> String {
        match value {
            ConstantValue::Primitive(x) => match x {
                PrimitiveValue::Bool(x) => format!("{}", x),
                PrimitiveValue::U8(x) => format!("{}", x),
                PrimitiveValue::U16(x) => format!("{}", x),
                PrimitiveValue::U32(x) => format!("{}", x),
                PrimitiveValue::U64(x) => format!("{}", x),
                PrimitiveValue::I8(x) => format!("{}", x),
                PrimitiveValue::I16(x) => format!("{}", x),
                PrimitiveValue::I32(x) => format!("{}", x),
                PrimitiveValue::I64(x) => format!("{}", x),
                PrimitiveValue::F32(x) => format!("{}", x),
                PrimitiveValue::F64(x) => format!("{}", x),
            },
        }
    }

    fn function_name_to_c_name(&self, function: &Function) -> String {
        function.name().to_string()
    }
}
