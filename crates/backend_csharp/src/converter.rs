use crate::Interop;
use crate::interop::FunctionNameFlavor;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use interoptopus::backend::safe_name;
use interoptopus::lang::{Composite, ConstantValue, Enum, Field, FnPointer, Function, Opaque, Parameter, Primitive, PrimitiveValue, SugaredReturnType, Type};
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::callback::{AsyncCallback, NamedCallback};
use interoptopus::pattern::slice::SliceType;
use interoptopus::pattern::vec::VecType;

/// Converts a primitive (Rust) type to a native C# type name, e.g., `f32` to `float`.
pub fn primitive_to_typename(x: Primitive) -> String {
    match x {
        Primitive::Void => "void".to_string(),
        Primitive::Bool => "bool".to_string(),
        Primitive::U8 => "byte".to_string(),
        Primitive::U16 => "ushort".to_string(),
        Primitive::U32 => "uint".to_string(),
        Primitive::U64 => "ulong".to_string(),
        Primitive::I8 => "sbyte".to_string(),
        Primitive::I16 => "short".to_string(),
        Primitive::I32 => "int".to_string(),
        Primitive::I64 => "long".to_string(),
        Primitive::F32 => "float".to_string(),
        Primitive::F64 => "double".to_string(),
    }
}

/// Converts an Rust `pub fn()` to a C# delegate name such as `InteropDelegate`.
pub fn fnpointer_to_typename(x: &FnPointer) -> String {
    ["InteropDelegate".to_string(), safe_name(&x.internal_name())].join("_")
}

/// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
#[allow(clippy::only_used_in_recursion)]
pub fn to_typespecifier_in_field(x: &Type) -> String {
    match &x {
        Type::Primitive(x) => primitive_to_typename(*x),
        Type::Array(_) => "TODO".to_string(),
        Type::Enum(x) => x.rust_name().to_string(),
        Type::Opaque(x) => "IntPtr".to_string(),
        Type::Composite(x) => x.rust_name().to_string(),
        Type::ReadPointer(_) => "IntPtr".to_string(),
        Type::ReadWritePointer(_) => "IntPtr".to_string(),
        Type::FnPointer(x) => fnpointer_to_typename(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "string".to_string(),
            TypePattern::Utf8String(_) => "Utf8String".to_string(),
            TypePattern::Slice(x) => format!("Slice{}", get_slice_type_argument(x)),
            TypePattern::SliceMut(x) => format!("SliceMut{}", get_slice_type_argument(x)),
            TypePattern::Option(e) => e.the_enum().rust_name().to_string(),
            TypePattern::Result(e) => e.the_enum().rust_name().to_string(),
            TypePattern::NamedCallback(e) => e.name().to_string(),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => to_typespecifier_in_field(&x.fallback_type()),
            TypePattern::Vec(x) => x.composite_type().rust_name().to_string(),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

/// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
#[allow(clippy::only_used_in_recursion)]
pub fn to_typespecifier_in_field_unmanaged(x: &Type) -> String {
    match &x {
        Type::Primitive(x) => primitive_to_typename(*x),
        Type::Array(_) => "TODO".to_string(),
        Type::Enum(x) => format!("{}.Unmanaged", x.rust_name()),
        Type::Opaque(x) => "TODO".to_string(),
        Type::Composite(x) => format!("{}.Unmanaged", x.rust_name()),
        Type::ReadPointer(_) => "IntPtr".to_string(),
        Type::ReadWritePointer(_) => "IntPtr".to_string(),
        Type::FnPointer(x) => fnpointer_to_typename(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "TODO".to_string(),
            TypePattern::Utf8String(_) => "Utf8String.Unmanaged".to_string(),
            TypePattern::Slice(x) => format!("Slice{}.Unmanaged.", get_slice_type_argument(x)),
            TypePattern::SliceMut(x) => format!("SliceMut{}.Unmanaged", get_slice_type_argument(x)),
            TypePattern::Option(e) => format!("{}.Unmanaged", e.the_enum().rust_name()),
            TypePattern::Result(e) => format!("{}.Unmanaged", e.the_enum().rust_name()),
            TypePattern::NamedCallback(e) => format!("{}.Unmanaged", e.name()),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => to_typespecifier_in_field(&x.fallback_type()),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

/// Converts the `u32` part in a Rust paramter `x: u32` to a C# equivalent. Might convert pointers to `out X` or `ref X`.
pub fn to_typespecifier_in_param(x: &Type) -> String {
    match &x {
        Type::Primitive(x) => match x {
            Primitive::Bool => "[MarshalAs(UnmanagedType.U1)] bool".to_string(),
            _ => primitive_to_typename(*x),
        },
        Type::Array(_) => todo!(),
        Type::Enum(x) => x.rust_name().to_string(),
        Type::Opaque(x) => "IntPtr".to_string(),
        Type::Composite(x) => x.rust_name().to_string(),
        Type::ReadPointer(z) => match &**z {
            Type::Opaque(_) => "IntPtr".to_string(),
            Type::Primitive(Primitive::Void) => "IntPtr".to_string(),
            Type::ReadPointer(_) => "ref IntPtr".to_string(),
            Type::ReadWritePointer(_) => "ref IntPtr".to_string(),
            Type::Pattern(TypePattern::CChar) => "IntPtr".to_string(),
            Type::Pattern(TypePattern::Slice(y)) => format!("ref {}", y.composite_type().rust_name()),
            Type::Pattern(TypePattern::SliceMut(y)) => format!("ref {}", y.composite_type().rust_name()),
            _ => format!("ref {}", to_typespecifier_in_param(z)),
        },
        Type::ReadWritePointer(z) => match &**z {
            Type::Opaque(_) => "IntPtr".to_string(),
            Type::Primitive(Primitive::Void) => "IntPtr".to_string(),
            Type::ReadPointer(_) => "ref IntPtr".to_string(),
            Type::ReadWritePointer(_) => "ref IntPtr".to_string(),
            Type::Pattern(TypePattern::CChar) => "IntPtr".to_string(),
            Type::Pattern(TypePattern::Slice(s)) => format!("ref {}", s.composite_type().rust_name()),
            Type::Pattern(TypePattern::SliceMut(s)) => format!("ref {}", s.composite_type().rust_name()),
            _ => format!("ref {}", to_typespecifier_in_param(z)),
        },
        Type::FnPointer(x) => fnpointer_to_typename(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "[MarshalAs(UnmanagedType.LPStr)] string".to_string(),
            TypePattern::Utf8String(x) => x.rust_name().to_string(),
            TypePattern::Slice(x) => x.composite_type().rust_name().to_string(),
            TypePattern::SliceMut(x) => x.composite_type().rust_name().to_string(),
            TypePattern::Option(x) => x.the_enum().rust_name().to_string(),
            TypePattern::Result(x) => x.the_enum().rust_name().to_string(),
            TypePattern::Vec(x) => x.composite_type().rust_name().to_string(),
            TypePattern::NamedCallback(x) => x.name().to_string(),
            TypePattern::AsyncCallback(x) => "AsyncCallbackCommonNative".to_string(),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => to_typespecifier_in_param(&x.fallback_type()),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

/// Converts the `u32` part in a Rust rval `-> u32` to a C# equivalent for synchronous calls.
pub fn to_typespecifier_in_sync_fn_rval(x: &Type) -> String {
    match &x {
        Type::Primitive(x) => primitive_to_typename(*x),
        Type::Array(_) => todo!(),
        Type::Enum(x) => x.rust_name().to_string(),
        Type::Opaque(x) => "IntPtr".to_string(),
        Type::Composite(x) => x.rust_name().to_string(),
        Type::ReadPointer(_) => "IntPtr".to_string(),
        Type::ReadWritePointer(_) => "IntPtr".to_string(),
        Type::FnPointer(x) => fnpointer_to_typename(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "IntPtr".to_string(),
            TypePattern::Utf8String(x) => x.rust_name().to_string(),
            TypePattern::Result(x) => x.the_enum().rust_name().to_string(),
            TypePattern::Slice(x) => x.composite_type().rust_name().to_string(),
            TypePattern::SliceMut(x) => x.composite_type().rust_name().to_string(),
            TypePattern::Option(x) => x.the_enum().rust_name().to_string(),
            TypePattern::NamedCallback(x) => x.name().to_string(),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => to_typespecifier_in_sync_fn_rval(&x.fallback_type()),
            TypePattern::Vec(x) => x.composite_type().rust_name().to_string(),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

/// Converts the `u32` part in a Rust rval `-> u32` to a C# equivalent for async calls, such as to `Task<int>`.
pub fn to_typespecifier_in_async_fn_rval(x: &SugaredReturnType) -> String {
    match x {
        SugaredReturnType::Async(Type::Pattern(TypePattern::Utf8String(_))) => "Task<Utf8String>".to_string(),
        SugaredReturnType::Async(Type::Pattern(TypePattern::Result(x))) if x.t().is_void() => "Task".to_string(),
        SugaredReturnType::Async(Type::Pattern(TypePattern::Result(x))) => match x.t() {
            Type::Pattern(TypePattern::Utf8String(_)) => "Task<Utf8String>".to_string(),
            x => format!("Task<{}>", to_typespecifier_in_sync_fn_rval(x)),
        },
        SugaredReturnType::Async(x) => format!("Task<{}>", to_typespecifier_in_sync_fn_rval(x)),
        SugaredReturnType::Sync(x) => to_typespecifier_in_sync_fn_rval(x),
    }
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

/// Gets the function name in a specific flavor
pub fn function_name_to_csharp_name(function: &Function, flavor: FunctionNameFlavor) -> String {
    match flavor {
        FunctionNameFlavor::RawFFIName => function.name().to_string(),
        FunctionNameFlavor::CSharpMethodNameWithClass => function.name().to_upper_camel_case(),
        FunctionNameFlavor::CSharpMethodNameWithoutClass(class) => function.name().replace(class, "").to_upper_camel_case(),
    }
}

/// TODO: We might want to get rid of field renaming.
pub fn field_name_to_csharp_name(field: &Field, rename_symbols: bool) -> String {
    if rename_symbols { field.name().to_lower_camel_case() } else { field.name().into() }
}

/// For a `Slice<u8>`, returns the `u8` as a C# type, e.g., `byte`.
pub fn get_slice_type_argument(x: &SliceType) -> String {
    to_typespecifier_in_param(x.t())
}

/// For a `Vec<u8>`, returns the `u8` as a C# type, e.g., `byte`.
pub fn get_vec_type_argument(x: &VecType) -> String {
    to_typespecifier_in_param(x.t())
}

/// Checks whether the type can be FFI'ed as-is, or if it needs marshalling.
pub fn is_blittable(t: &Type) -> bool {
    match t {
        Type::Array(_) => false,
        Type::Composite(x) => x.fields().iter().all(|x| is_blittable(x.the_type())),
        Type::Enum(_) => false,
        Type::FnPointer(_) => true,
        Type::Opaque(_) => false,
        Type::Primitive(_) => true,
        Type::ReadPointer(_) => true,
        Type::ReadWritePointer(_) => true,
        Type::Pattern(p) => match p {
            TypePattern::CStrPointer => true,
            TypePattern::Utf8String(_) => false,
            TypePattern::APIVersion => true,
            TypePattern::Slice(_) => true,
            TypePattern::SliceMut(_) => true,
            TypePattern::Option(_) => false,
            TypePattern::Result(_) => false,
            TypePattern::Bool => true,
            TypePattern::CChar => true,
            TypePattern::NamedCallback(_) => true,
            TypePattern::AsyncCallback(_) => true,
            TypePattern::Vec(_) => false,
            _ => todo!("Not implemented yet"),
        },
    }
}

#[must_use]
pub fn pattern_to_native_in_signature(_: &Interop, param: &Parameter) -> String {
    match param.the_type() {
        x @ Type::Pattern(p) => match p {
            TypePattern::NamedCallback(_) => {
                format!("{}Delegate", to_typespecifier_in_param(x))
            }
            _ => to_typespecifier_in_param(param.the_type()),
        },

        x => to_typespecifier_in_param(x),
    }
}
