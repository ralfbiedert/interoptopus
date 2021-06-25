use interoptopus::lang::c::{CType, CompositeType, ConstantValue, EnumType, FnPointerType, Function, OpaqueType, Parameter, PrimitiveType, PrimitiveValue};
use interoptopus::util::safe_name;

/// Implements [`CTypeConverter`].
#[derive(Copy, Clone)]
pub struct Converter {}

/// Converts Interoptopus types to C types.
pub trait CTypeConverter {
    /// Converts a primitive (Rust) type to a native C# type name, e.g., `f32` to `float`.
    fn type_primitive_to_typename(&self, x: &PrimitiveType) -> String;

    /// Converts a Rust enum name such as `Error` to a C# enum name `Error`.
    fn type_enum_to_typename(&self, x: &EnumType) -> String;

    /// TODO Converts an opaque Rust struct `Context` to a C# struct ``.
    fn type_opaque_to_typename(&self, opaque: &OpaqueType) -> String;

    /// Converts an Rust struct name `Vec2` to a C# struct name `Vec2`.
    fn type_composite_to_typename(&self, x: &CompositeType) -> String;

    /// Converts an Rust `fn()` to a C# delegate name such as `InteropDelegate`.
    fn type_fnpointer_to_typename(&self, x: &FnPointerType) -> String;

    /// Converts the `u32` part in a Rust paramter `x: u32` to a C# equivalent. Might convert pointers to `out X` or `ref X`.
    fn type_to_type_specifier(&self, x: &CType) -> String;

    fn constant_value_to_value(&self, value: &ConstantValue) -> String;

    fn function_parameter_to_csharp_typename(&self, x: &Parameter, _function: &Function) -> String;

    fn function_name_to_c_name(&self, function: &Function) -> String;
}

impl CTypeConverter for Converter {
    fn type_primitive_to_typename(&self, x: &PrimitiveType) -> String {
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

    fn type_enum_to_typename(&self, x: &EnumType) -> String {
        x.rust_name().to_string()
    }

    fn type_opaque_to_typename(&self, opaque: &OpaqueType) -> String {
        // x.name().to_string()
        opaque.rust_name().to_string()
    }

    fn type_composite_to_typename(&self, x: &CompositeType) -> String {
        x.rust_name().to_string()
    }

    fn type_fnpointer_to_typename(&self, x: &FnPointerType) -> String {
        vec!["fptr".to_string(), safe_name(&x.internal_name())].join("_")
    }

    fn type_to_type_specifier(&self, x: &CType) -> String {
        match x {
            CType::Primitive(x) => self.type_primitive_to_typename(x),
            CType::Enum(x) => self.type_enum_to_typename(x),
            CType::Opaque(x) => self.type_opaque_to_typename(x),
            CType::Composite(x) => self.type_composite_to_typename(x),
            CType::ReadPointer(x) => format!("{}*", self.type_to_type_specifier(x)),
            CType::ReadWritePointer(x) => format!("{}*", self.type_to_type_specifier(x)),
            CType::FnPointer(x) => self.type_fnpointer_to_typename(x),
            CType::Pattern(x) => self.type_to_type_specifier(&x.fallback_type()),
        }
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

    fn function_parameter_to_csharp_typename(&self, x: &Parameter, _function: &Function) -> String {
        self.type_to_type_specifier(x.the_type())
    }

    fn function_name_to_c_name(&self, function: &Function) -> String {
        function.name().to_string()
    }
}
