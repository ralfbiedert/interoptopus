use crate::config::Config;
use interoptopus::lang::c::{
    CType, CompositeType, Constant, ConstantValue, Documentation, EnumType, Field, FnPointerType, Function, Meta, OpaqueType, Parameter, PrimitiveType, PrimitiveValue,
    Variant,
};
use interoptopus::patterns::service::Service;
use interoptopus::patterns::{LibraryPattern, TypePattern};
use interoptopus::util::{longest_common_prefix, safe_name, IdPrettifier, NamespaceMappings};
use interoptopus::writer::IndentWriter;
use interoptopus::Interop;
use interoptopus::{Error, Library};

#[derive(Copy, Clone)]
pub struct Converter {}

pub trait TypeConverter {
    /// Converts a primitive (Rust) type to a native C# type name, e.g., `f32` to `float`.
    fn type_primitive_to_typename(&self, x: &PrimitiveType) -> String {
        match x {
            PrimitiveType::Void => "void".to_string(),
            PrimitiveType::Bool => "bool".to_string(),
            PrimitiveType::U8 => "byte".to_string(),
            PrimitiveType::U16 => "ushort".to_string(),
            PrimitiveType::U32 => "uint".to_string(),
            PrimitiveType::U64 => "ulong".to_string(),
            PrimitiveType::I8 => "sbyte".to_string(),
            PrimitiveType::I16 => "short".to_string(),
            PrimitiveType::I32 => "int".to_string(),
            PrimitiveType::I64 => "long".to_string(),
            PrimitiveType::F32 => "float".to_string(),
            PrimitiveType::F64 => "double".to_string(),
        }
    }

    /// Converts a Rust enum name such as `Error` to a C# enum name `Error`.
    fn type_enum_to_typename(&self, x: &EnumType) -> String {
        x.rust_name().to_string()
    }

    /// TODO Converts an opaque Rust struct `Context` to a C# struct ``.
    fn type_opaque_to_typename(&self, _: &OpaqueType) -> String {
        // x.name().to_string()
        "IntPtr".to_string()
    }

    /// Converts an Rust struct name `Vec2` to a C# struct name `Vec2`.
    fn type_composite_to_typename(&self, x: &CompositeType) -> String {
        x.rust_name().to_string()
    }

    /// Converts an Rust `fn()` to a C# delegate name such as `InteropDelegate`.
    fn type_fnpointer_to_typename(&self, x: &FnPointerType) -> String {
        vec!["InteropDelegate".to_string(), safe_name(&x.internal_name())].join("_")
    }

    /// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
    fn type_to_typespecifier_in_field(&self, x: &CType, _field: &Field, _composite: &CompositeType) -> String {
        match &x {
            CType::Primitive(x) => self.type_primitive_to_typename(x),
            CType::Enum(x) => self.type_enum_to_typename(x),
            CType::Opaque(x) => self.type_opaque_to_typename(x),
            CType::Composite(x) => self.type_composite_to_typename(x),
            CType::ReadPointer(_) => "IntPtr".to_string(),
            CType::ReadWritePointer(_) => "IntPtr".to_string(),
            CType::FnPointer(x) => self.type_fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::SuccessEnum(e) => self.type_enum_to_typename(e.the_enum()),
                TypePattern::Slice(e) => self.type_composite_to_typename(e),
                TypePattern::Option(e) => self.type_composite_to_typename(e),
            },
        }
    }

    /// Converts the `u32` part in a Rust paramter `x: u32` to a C# equivalent. Might convert pointers to `out X` or `ref X`.
    fn type_to_typespecifier_in_param(&self, x: &CType) -> String {
        match &x {
            CType::Primitive(x) => self.type_primitive_to_typename(x),
            CType::Enum(x) => self.type_enum_to_typename(x),
            CType::Opaque(x) => self.type_opaque_to_typename(x),
            CType::Composite(x) => self.type_composite_to_typename(x),
            CType::ReadPointer(z) => match **z {
                CType::Opaque(_) => "IntPtr".to_string(),
                CType::Primitive(PrimitiveType::Void) => "IntPtr".to_string(),
                CType::ReadPointer(_) => "ref IntPtr".to_string(),
                CType::ReadWritePointer(_) => "ref IntPtr".to_string(),
                _ => format!("ref {}", self.type_to_typespecifier_in_param(z)),
            },
            CType::ReadWritePointer(z) => match **z {
                CType::Opaque(_) => "IntPtr".to_string(),
                CType::Primitive(PrimitiveType::Void) => "IntPtr".to_string(),
                CType::ReadPointer(_) => "out IntPtr".to_string(),
                CType::ReadWritePointer(_) => "out IntPtr".to_string(),
                _ => format!("out {}", self.type_to_typespecifier_in_param(z)),
            },
            CType::FnPointer(x) => self.type_fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::SuccessEnum(e) => self.type_enum_to_typename(e.the_enum()),
                TypePattern::Slice(x) => self.type_composite_to_typename(x),
                TypePattern::Option(x) => self.type_composite_to_typename(x),
            },
        }
    }

    fn type_to_typespecifier_in_rval(&self, x: &CType) -> String {
        match &x {
            CType::Primitive(x) => self.type_primitive_to_typename(x),
            CType::Enum(x) => self.type_enum_to_typename(x),
            CType::Opaque(x) => self.type_opaque_to_typename(x),
            CType::Composite(x) => self.type_composite_to_typename(x),
            CType::ReadPointer(_) => "IntPtr".to_string(),
            CType::ReadWritePointer(_) => "IntPtr".to_string(),
            CType::FnPointer(x) => self.type_fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::SuccessEnum(e) => self.type_enum_to_typename(e.the_enum()),
                TypePattern::Slice(x) => self.type_composite_to_typename(x),
                TypePattern::Option(x) => self.type_composite_to_typename(x),
            },
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
        self.type_to_typespecifier_in_param(x.the_type())
    }

    fn function_rval_to_csharp_typename(&self, function: &Function) -> String {
        self.type_to_typespecifier_in_rval(function.signature().rval())
    }

    fn function_name_to_csharp_name(&self, function: &Function) -> String {
        function.name().to_string()
    }
}

impl TypeConverter for Converter {}
