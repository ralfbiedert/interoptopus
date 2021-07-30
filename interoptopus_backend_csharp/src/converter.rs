use interoptopus::lang::c::{
    CType, CompositeType, ConstantValue, EnumType, Field, FnPointerType, Function, FunctionSignature, OpaqueType, Parameter, PrimitiveType, PrimitiveValue,
};
use interoptopus::patterns::callbacks::NamedCallback;
use interoptopus::patterns::TypePattern;
use interoptopus::util::safe_name;

/// Implements [`CSharpTypeConverter`].
#[derive(Copy, Clone)]
pub struct Converter {}

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

    fn has_overloadable(&self, signature: &FunctionSignature) -> bool {
        signature
            .params()
            .iter()
            .any(|x| matches!(x.the_type(), CType::Pattern(TypePattern::Slice(_) | TypePattern::SliceMut(_))))
    }

    fn pattern_to_native_in_signature(&self, param: &Parameter, _signature: &FunctionSignature) -> String {
        match param.the_type() {
            CType::Pattern(p) => match p {
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::NamedCallback(x) => x.name().to_string(),
                TypePattern::SuccessEnum(e) => self.enum_to_typename(e.the_enum()),
                TypePattern::Slice(p) => {
                    let element_type = p
                        .fields()
                        .get(0)
                        .expect("First parameter must exist")
                        .the_type()
                        .deref_pointer()
                        .expect("Must be pointer");

                    format!("{}[]", self.to_typespecifier_in_param(element_type))
                }
                TypePattern::SliceMut(p) => {
                    let element_type = p
                        .fields()
                        .get(0)
                        .expect("First parameter must exist")
                        .the_type()
                        .deref_pointer()
                        .expect("Must be pointer");
                    format!("{}[]", self.to_typespecifier_in_param(element_type))
                }

                TypePattern::Option(e) => self.composite_to_typename(e),
                TypePattern::Bool => "FFIBool".to_string(),
                TypePattern::APIVersion => self.to_typespecifier_in_param(&p.fallback_type()),
            },
            x => self.to_typespecifier_in_param(x),
        }
    }

    /// Converts an Rust struct name `Vec2` to a C# struct name `Vec2`.
    fn composite_to_typename(&self, x: &CompositeType) -> String {
        x.rust_name().to_string()
    }

    fn named_callback_to_typename(&self, x: &NamedCallback) -> String {
        x.name().to_string()
    }

    /// Converts an Rust `fn()` to a C# delegate name such as `InteropDelegate`.
    fn fnpointer_to_typename(&self, x: &FnPointerType) -> String {
        vec!["InteropDelegate".to_string(), safe_name(&x.internal_name())].join("_")
    }

    /// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
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
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::SuccessEnum(e) => self.enum_to_typename(e.the_enum()),
                TypePattern::Slice(e) => self.composite_to_typename(e),
                TypePattern::SliceMut(e) => self.composite_to_typename(e),
                TypePattern::Option(e) => self.composite_to_typename(e),
                TypePattern::NamedCallback(e) => self.named_callback_to_typename(e),
                TypePattern::Bool => "FFIBool".to_string(),
                TypePattern::APIVersion => self.to_typespecifier_in_field(&x.fallback_type(), field, composite),
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
                _ => format!("ref {}", self.to_typespecifier_in_param(z)),
            },
            CType::ReadWritePointer(z) => match **z {
                CType::Opaque(_) => "IntPtr".to_string(),
                CType::Primitive(PrimitiveType::Void) => "IntPtr".to_string(),
                CType::ReadPointer(_) => "ref IntPtr".to_string(),
                CType::ReadWritePointer(_) => "ref IntPtr".to_string(),
                _ => format!("out {}", self.to_typespecifier_in_param(z)),
            },
            CType::FnPointer(x) => self.fnpointer_to_typename(x),
            CType::Pattern(x) => match x {
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::SuccessEnum(e) => self.enum_to_typename(e.the_enum()),
                TypePattern::Slice(x) => self.composite_to_typename(x),
                TypePattern::SliceMut(x) => self.composite_to_typename(x),
                TypePattern::Option(x) => self.composite_to_typename(x),
                TypePattern::NamedCallback(x) => self.named_callback_to_typename(x),
                TypePattern::Bool => "FFIBool".to_string(),
                TypePattern::APIVersion => self.to_typespecifier_in_param(&x.fallback_type()),
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
                TypePattern::AsciiPointer => "string".to_string(),
                TypePattern::SuccessEnum(e) => self.enum_to_typename(e.the_enum()),
                TypePattern::Slice(x) => self.composite_to_typename(x),
                TypePattern::SliceMut(x) => self.composite_to_typename(x),
                TypePattern::Option(x) => self.composite_to_typename(x),
                TypePattern::NamedCallback(x) => self.named_callback_to_typename(x),
                TypePattern::Bool => "FFIBool".to_string(),
                TypePattern::APIVersion => self.to_typespecifier_in_rval(&x.fallback_type()),
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
        self.to_typespecifier_in_param(x.the_type())
    }

    fn function_rval_to_csharp_typename(&self, function: &Function) -> String {
        self.to_typespecifier_in_rval(function.signature().rval())
    }

    fn function_name_to_csharp_name(&self, function: &Function) -> String {
        function.name().to_string()
    }
}

impl CSharpTypeConverter for Converter {}
