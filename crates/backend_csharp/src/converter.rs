use crate::config::ParamSliceType;
use crate::{Config, Unsafe};
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use interoptopus::lang::c::{
    CType, CompositeType, ConstantValue, EnumType, Field, FnPointerType, Function, FunctionSignature, OpaqueType, Parameter, PrimitiveType, PrimitiveValue,
};
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::TypePattern;
use interoptopus::util::safe_name;
use std::ops::Deref;

/// Implements [`CSharpTypeConverter`].
#[derive(Copy, Clone)]
pub struct Converter {}

/// How to convert from Rust function names to C#
pub enum FunctionNameFlavor<'a> {
    /// Takes the name as it is written in Rust
    RawFFIName,
    /// Converts the name to camel case
    CSharpMethodNameWithClass,
    /// Converts the name to camel case and removes the class name
    CSharpMethodNameWithoutClass(&'a str),
}

/// Converts Interoptopus types to C# types.
pub trait CSharpTypeConverter {
    /// Converts a primitive (Rust) type to a native C# type name, e.g., `f32` to `float`.
    fn primitive_to_typename(&self, x: &PrimitiveType) -> String {
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
    fn enum_to_typename(&self, x: &EnumType) -> String {
        x.rust_name().to_string()
    }

    /// TODO Converts an opaque Rust struct `Context` to a C# struct ``.
    fn opaque_to_typename(&self, _: &OpaqueType) -> String {
        // x.name().to_string()
        "IntPtr".to_string()
    }

    fn has_ffi_error_rval(&self, signature: &FunctionSignature) -> bool {
        matches!(signature.rval(), CType::Pattern(TypePattern::FFIErrorEnum(_)))
    }

    /// Converts an Rust struct name `Vec2` to a C# struct name `Vec2`.
    fn composite_to_typename(&self, x: &CompositeType) -> String {
        x.rust_name().to_string()
    }

    /// Checks if the type is on the C# side blittable, in particular, if it can be accessed via raw pointers and memcopied.
    fn is_blittable(&self, x: &CType) -> bool {
        match x {
            CType::Primitive(_) => true,
            CType::Composite(c) => c.fields().iter().all(|x| self.is_blittable(x.the_type())),
            CType::Pattern(x) => match x {
                TypePattern::CStrPointer => false,
                TypePattern::APIVersion => true,
                TypePattern::FFIErrorEnum(_) => true,
                TypePattern::Slice(_) => false,
                TypePattern::SliceMut(_) => false,
                TypePattern::Option(_) => true,
                TypePattern::Bool => true,
                TypePattern::CChar => true,
                TypePattern::NamedCallback(_) => false,
                _ => panic!("Pattern not explicitly handled"),
            },
            CType::Array(_) => false, // TODO: should check inner and maybe return true
            CType::Enum(_) => true,
            CType::Opaque(_) => true,
            CType::FnPointer(_) => true,
            CType::ReadPointer(_) => true,
            CType::ReadWritePointer(_) => true,
        }
    }

    fn named_callback_to_typename(&self, x: &NamedCallback) -> String {
        x.name().to_string()
    }

    /// Converts an Rust `fn()` to a C# delegate name such as `InteropDelegate`.
    fn fnpointer_to_typename(&self, x: &FnPointerType) -> String {
        ["InteropDelegate".to_string(), safe_name(&x.internal_name())].join("_")
    }

    /// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
    #[allow(clippy::only_used_in_recursion)]
    fn to_typespecifier_in_field(&self, x: &CType, field: &Field, composite: &CompositeType) -> String {
        match &x {
            CType::Primitive(x) => self.primitive_to_typename(x),
            CType::Array(_) => panic!("Needs special handling in the writer."),
            CType::Enum(x) => self.enum_to_typename(x),
            CType::Opaque(x) => self.opaque_to_typename(x),
            CType::Composite(x) => self.composite_to_typename(x),
            CType::ReadPointer(_) => "IntPtr".to_string(),
            CType::ReadWritePointer(_) => "IntPtr".to_string(),
            CType::FnPointer(x) => self.fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::CStrPointer => "string".to_string(),
                TypePattern::FFIErrorEnum(e) => self.enum_to_typename(e.the_enum()),
                TypePattern::Slice(e) => self.composite_to_typename(e),
                TypePattern::SliceMut(e) => self.composite_to_typename(e),
                TypePattern::Option(e) => self.composite_to_typename(e),
                TypePattern::NamedCallback(e) => self.named_callback_to_typename(e),
                TypePattern::Bool => "Bool".to_string(),
                TypePattern::CChar => "sbyte".to_string(),
                TypePattern::APIVersion => self.to_typespecifier_in_field(&x.fallback_type(), field, composite),
                _ => panic!("Pattern not explicitly handled"),
            },
        }
    }

    /// Converts the `u32` part in a Rust paramter `x: u32` to a C# equivalent. Might convert pointers to `out X` or `ref X`.
    fn to_typespecifier_in_param(&self, x: &CType) -> String {
        match &x {
            CType::Primitive(x) => self.primitive_to_typename(x),
            CType::Array(_) => todo!(),
            CType::Enum(x) => self.enum_to_typename(x),
            CType::Opaque(x) => self.opaque_to_typename(x),
            CType::Composite(x) => self.composite_to_typename(x),
            CType::ReadPointer(z) => match **z {
                CType::Opaque(_) => "IntPtr".to_string(),
                CType::Primitive(PrimitiveType::Void) => "IntPtr".to_string(),
                CType::ReadPointer(_) => "ref IntPtr".to_string(),
                CType::ReadWritePointer(_) => "ref IntPtr".to_string(),
                CType::Pattern(TypePattern::CChar) => "IntPtr".to_string(),
                CType::Pattern(TypePattern::Slice(_)) => format!("ref {}", self.to_typespecifier_in_param(z)),
                CType::Pattern(TypePattern::SliceMut(_)) => format!("ref {}", self.to_typespecifier_in_param(z)),
                _ => format!("ref {}", self.to_typespecifier_in_param(z)),
            },
            CType::ReadWritePointer(z) => match **z {
                CType::Opaque(_) => "IntPtr".to_string(),
                CType::Primitive(PrimitiveType::Void) => "IntPtr".to_string(),
                CType::ReadPointer(_) => "ref IntPtr".to_string(),
                CType::ReadWritePointer(_) => "ref IntPtr".to_string(),
                CType::Pattern(TypePattern::CChar) => "IntPtr".to_string(),
                CType::Pattern(TypePattern::Slice(_)) => format!("ref {}", self.to_typespecifier_in_param(z)),
                CType::Pattern(TypePattern::SliceMut(_)) => format!("ref {}", self.to_typespecifier_in_param(z)),
                _ => format!("out {}", self.to_typespecifier_in_param(z)),
            },
            CType::FnPointer(x) => self.fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::CStrPointer => "string".to_string(),
                TypePattern::FFIErrorEnum(e) => self.enum_to_typename(e.the_enum()),
                TypePattern::Slice(x) => self.composite_to_typename(x),
                TypePattern::SliceMut(x) => self.composite_to_typename(x),
                TypePattern::Option(x) => self.composite_to_typename(x),
                TypePattern::NamedCallback(x) => self.named_callback_to_typename(x),
                TypePattern::Bool => "Bool".to_string(),
                TypePattern::CChar => "sbyte".to_string(),
                TypePattern::APIVersion => self.to_typespecifier_in_param(&x.fallback_type()),
                _ => panic!("Pattern not explicitly handled"),
            },
        }
    }

    fn to_typespecifier_in_rval(&self, x: &CType) -> String {
        match &x {
            CType::Primitive(x) => self.primitive_to_typename(x),
            CType::Array(_) => todo!(),
            CType::Enum(x) => self.enum_to_typename(x),
            CType::Opaque(x) => self.opaque_to_typename(x),
            CType::Composite(x) => self.composite_to_typename(x),
            CType::ReadPointer(_) => "IntPtr".to_string(),
            CType::ReadWritePointer(_) => "IntPtr".to_string(),
            CType::FnPointer(x) => self.fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::CStrPointer => "IntPtr".to_string(),
                TypePattern::FFIErrorEnum(e) => self.enum_to_typename(e.the_enum()),
                TypePattern::Slice(x) => self.composite_to_typename(x),
                TypePattern::SliceMut(x) => self.composite_to_typename(x),
                TypePattern::Option(x) => self.composite_to_typename(x),
                TypePattern::NamedCallback(x) => self.named_callback_to_typename(x),
                TypePattern::Bool => "Bool".to_string(),
                TypePattern::CChar => "sbyte".to_string(),
                TypePattern::APIVersion => self.to_typespecifier_in_rval(&x.fallback_type()),
                _ => panic!("Pattern not explicitly handled"),
            },
        }
    }

    fn has_overloadable(&self, signature: &FunctionSignature) -> bool {
        signature.params().iter().any(|x| match x.the_type() {
            CType::ReadPointer(x) | CType::ReadWritePointer(x) => match x.deref() {
                CType::Pattern(x) => matches!(x, TypePattern::Slice(_) | TypePattern::SliceMut(_)),
                _ => false,
            },
            CType::Pattern(x) => matches!(x, TypePattern::Slice(_) | TypePattern::SliceMut(_)),
            _ => false,
        })
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

    fn function_parameter_to_csharp_typename(&self, x: &Parameter) -> String {
        self.to_typespecifier_in_param(x.the_type())
    }

    fn function_rval_to_csharp_typename(&self, function: &Function) -> String {
        self.to_typespecifier_in_rval(function.signature().rval())
    }

    /// Gets the function name in a specific flavor
    fn function_name_to_csharp_name(&self, function: &Function, flavor: FunctionNameFlavor) -> String {
        match flavor {
            FunctionNameFlavor::RawFFIName => function.name().to_string(),
            FunctionNameFlavor::CSharpMethodNameWithClass => function.name().to_upper_camel_case(),
            FunctionNameFlavor::CSharpMethodNameWithoutClass(class) => function.name().replace(class, "").to_upper_camel_case(),
        }
    }

    fn field_name_to_csharp_name(&self, field: &Field, rename_symbols: bool) -> String {
        if rename_symbols {
            field.name().to_lower_camel_case()
        } else {
            field.name().into()
        }
    }
}

impl CSharpTypeConverter for Converter {}
