use crate::Interop;
use crate::interop::FunctionNameFlavor;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use interoptopus::backend::{ctypes_from_type_recursive, safe_name};
use interoptopus::lang::{Composite, ConstantValue, Enum, Field, FnPointer, Function, Opaque, Parameter, Primitive, PrimitiveValue, SugaredReturnType, Type};
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::callback::{AsyncCallback, NamedCallback};
use interoptopus::pattern::slice::SliceType;
use std::collections::HashSet;

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

/// Converts a Rust enum name such as `Error` to a C# enum name `Error`.
pub fn enum_to_typename(x: &Enum) -> String {
    x.rust_name().to_string()
}

/// TODO Converts an opaque Rust struct `Context` to a C# struct.
pub fn opaque_to_typename(_: &Opaque) -> String {
    // x.name().to_string()
    "IntPtr".to_string()
}

/// Converts an Rust struct name `Vec2` to a C# struct name `Vec2`.
pub fn composite_to_typename(x: &Composite) -> String {
    x.rust_name().to_string()
}

/// Checks if the type is on the C# side blittable, in particular, if it can be accessed via raw pointers and memcopied.
pub fn is_blittable(x: &Type) -> bool {
    match x {
        Type::Primitive(_) => true,
        Type::Composite(c) => c.fields().iter().all(|x| is_blittable(x.the_type())),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => false,
            TypePattern::APIVersion => true,
            TypePattern::Slice(_) => false,
            TypePattern::SliceMut(_) => false,
            TypePattern::Option(_) => true,
            TypePattern::Bool => true,
            TypePattern::CChar => true,
            TypePattern::NamedCallback(_) => false,
            _ => panic!("Pattern not explicitly handled"),
        },
        Type::Array(_) => false, // TODO: should check inner and maybe return true
        Type::Enum(_) => true,
        Type::Opaque(_) => true,
        Type::FnPointer(_) => true,
        Type::ReadPointer(_) => true,
        Type::ReadWritePointer(_) => true,
    }
}

pub fn named_callback_to_typename(x: &NamedCallback) -> String {
    x.name().to_string()
}

pub fn async_callback_to_typename(_: &AsyncCallback) -> String {
    "AsyncHelper".to_string()
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
        // CType::Array(_) => panic!("Needs special handling in the writer."),
        Type::Array(_) => "TODO".to_string(),
        Type::Enum(x) => enum_to_typename(x),
        Type::Opaque(x) => opaque_to_typename(x),
        Type::Composite(x) => composite_to_typename(x),
        Type::ReadPointer(_) => "IntPtr".to_string(),
        Type::ReadWritePointer(_) => "IntPtr".to_string(),
        Type::FnPointer(x) => fnpointer_to_typename(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "string".to_string(),
            TypePattern::Utf8String(_) => "string".to_string(),
            TypePattern::Slice(x) => format!("Slice<{}>", get_slice_type_argument(x)),
            TypePattern::SliceMut(x) => format!("SliceMut<{}>", get_slice_type_argument(x)),
            TypePattern::Option(e) => enum_to_typename(e.the_enum()),
            TypePattern::Result(e) => enum_to_typename(e.the_enum()),
            TypePattern::NamedCallback(e) => named_callback_to_typename(e),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => to_typespecifier_in_field(&x.fallback_type()),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

/// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
#[allow(clippy::only_used_in_recursion)]
pub fn to_typespecifier_in_field_unmanaged(x: &Type) -> String {
    match &x {
        Type::Primitive(x) => primitive_to_typename(*x),
        // CType::Array(_) => panic!("Needs special handling in the writer."),
        Type::Array(_) => "TODO".to_string(),
        Type::Enum(x) => format!("{}.Unmanaged", enum_to_typename(x)),
        Type::Opaque(x) => format!("{}.Unmanaged", opaque_to_typename(x)),
        Type::Composite(x) => format!("{}.Unmanaged", composite_to_typename(x)),
        Type::ReadPointer(_) => "IntPtr".to_string(),
        Type::ReadWritePointer(_) => "IntPtr".to_string(),
        Type::FnPointer(x) => fnpointer_to_typename(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "TODO".to_string(),
            TypePattern::Utf8String(_) => "Utf8String.Unmanaged".to_string(),
            TypePattern::Slice(x) => format!("Slice{}.Unmanaged.", get_slice_type_argument(x)),
            TypePattern::SliceMut(x) => format!("SliceMut{}.Unmanaged", get_slice_type_argument(x)),
            TypePattern::Option(e) => format!("{}.Unmanaged", enum_to_typename(e.the_enum())),
            TypePattern::Result(e) => format!("{}.Unmanaged", enum_to_typename(e.the_enum())),
            TypePattern::NamedCallback(e) => format!("{}.Unmanaged", named_callback_to_typename(e)),
            TypePattern::Bool => "TODO".to_string(),
            TypePattern::CChar => "TODO".to_string(),
            TypePattern::APIVersion => "TODO".to_string(),
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
        Type::Enum(x) => enum_to_typename(x),
        Type::Opaque(x) => opaque_to_typename(x),
        Type::Composite(x) => composite_to_typename(x),
        Type::ReadPointer(z) => match &**z {
            Type::Opaque(_) => "IntPtr".to_string(),
            Type::Primitive(Primitive::Void) => "IntPtr".to_string(),
            Type::ReadPointer(_) => "ref IntPtr".to_string(),
            Type::ReadWritePointer(_) => "ref IntPtr".to_string(),
            Type::Pattern(TypePattern::CChar) => "IntPtr".to_string(),
            Type::Pattern(TypePattern::Slice(y)) => format!("ref {}", composite_to_typename(y.composite_type())),
            Type::Pattern(TypePattern::SliceMut(y)) => format!("ref {}", composite_to_typename(y.composite_type())),
            _ => format!("ref {}", to_typespecifier_in_param(z)),
        },
        Type::ReadWritePointer(z) => match &**z {
            Type::Opaque(_) => "IntPtr".to_string(),
            Type::Primitive(Primitive::Void) => "IntPtr".to_string(),
            Type::ReadPointer(_) => "ref IntPtr".to_string(),
            Type::ReadWritePointer(_) => "ref IntPtr".to_string(),
            Type::Pattern(TypePattern::CChar) => "IntPtr".to_string(),
            Type::Pattern(TypePattern::Slice(y)) => format!("ref {}>", composite_to_typename(y.composite_type())),
            Type::Pattern(TypePattern::SliceMut(y)) => format!("ref {}", composite_to_typename(y.composite_type())),
            _ => format!("ref {}", to_typespecifier_in_param(z)),
        },
        Type::FnPointer(x) => fnpointer_to_typename(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "[MarshalAs(UnmanagedType.LPStr)] string".to_string(),
            TypePattern::Utf8String(x) => composite_to_typename(x),
            TypePattern::Slice(x) => composite_to_typename(x.composite_type()),
            TypePattern::SliceMut(x) => composite_to_typename(x.composite_type()),
            TypePattern::Option(x) => enum_to_typename(x.the_enum()),
            TypePattern::Result(x) => enum_to_typename(x.the_enum()),
            TypePattern::NamedCallback(x) => named_callback_to_typename(x),
            TypePattern::AsyncCallback(x) => async_callback_to_typename(x),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => to_typespecifier_in_param(&x.fallback_type()),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

pub fn to_typespecifier_in_sync_fn_rval(x: &Type) -> String {
    match &x {
        Type::Primitive(x) => primitive_to_typename(*x),
        Type::Array(_) => todo!(),
        Type::Enum(x) => enum_to_typename(x),
        Type::Opaque(x) => opaque_to_typename(x),
        Type::Composite(x) => composite_to_typename(x),
        Type::ReadPointer(_) => "IntPtr".to_string(),
        Type::ReadWritePointer(_) => "IntPtr".to_string(),
        Type::FnPointer(x) => fnpointer_to_typename(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "IntPtr".to_string(),
            TypePattern::Utf8String(x) => composite_to_typename(x),
            TypePattern::Result(x) => enum_to_typename(x.the_enum()),
            TypePattern::Slice(x) => composite_to_typename(x.composite_type()),
            TypePattern::SliceMut(x) => composite_to_typename(x.composite_type()),
            TypePattern::Option(x) => enum_to_typename(x.the_enum()),
            TypePattern::NamedCallback(x) => named_callback_to_typename(x),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => to_typespecifier_in_sync_fn_rval(&x.fallback_type()),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

pub fn to_typespecifier_in_async_fn_rval(x: &SugaredReturnType) -> String {
    match x {
        SugaredReturnType::Async(Type::Pattern(TypePattern::Utf8String(_))) => "Task<string>".to_string(),
        SugaredReturnType::Async(Type::Pattern(TypePattern::Result(x))) if x.t().is_void() => "Task".to_string(),
        SugaredReturnType::Async(Type::Pattern(TypePattern::Result(x))) => match x.t() {
            Type::Pattern(TypePattern::Utf8String(_)) => "Task<string>".to_string(),
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

pub fn function_parameter_to_csharp_typename(x: &Parameter) -> String {
    to_typespecifier_in_param(x.the_type())
}

pub fn function_rval_to_csharp_typename(function: &Function) -> String {
    to_typespecifier_in_sync_fn_rval(function.signature().rval())
}

/// Gets the function name in a specific flavor
pub fn function_name_to_csharp_name(function: &Function, flavor: FunctionNameFlavor) -> String {
    match flavor {
        FunctionNameFlavor::RawFFIName => function.name().to_string(),
        FunctionNameFlavor::CSharpMethodNameWithClass => function.name().to_upper_camel_case(),
        FunctionNameFlavor::CSharpMethodNameWithoutClass(class) => function.name().replace(class, "").to_upper_camel_case(),
    }
}

pub fn field_name_to_csharp_name(field: &Field, rename_symbols: bool) -> String {
    if rename_symbols { field.name().to_lower_camel_case() } else { field.name().into() }
}

pub fn get_slice_type_argument(x: &SliceType) -> String {
    to_typespecifier_in_param(x.target_type())
}

pub fn is_owned_slice(slice: &SliceType) -> bool {
    let mut rval = HashSet::new();
    ctypes_from_type_recursive(slice.target_type(), &mut rval);

    rval.iter().any(|x| matches!(x, Type::Pattern(TypePattern::Utf8String(_))))
}

#[must_use]
pub fn pattern_to_native_in_signature(_: &Interop, param: &Parameter) -> String {
    let slice_type_name = |mutable: bool, slice: &SliceType| -> String {
        if is_owned_slice(slice) {
            match slice.target_type() {
                Type::Pattern(TypePattern::Utf8String(_)) => "string[]".to_string(),
                _ => format!("{}[]", crate::converter::to_typespecifier_in_param(slice.target_type())),
            }
        } else if mutable {
            format!("Span<{}>", to_typespecifier_in_param(slice.target_type()))
        } else {
            format!("ReadOnlySpan<{}>", to_typespecifier_in_param(slice.target_type()))
        }
    };
    match param.the_type() {
        x @ Type::Pattern(p) => match p {
            TypePattern::Slice(p) => slice_type_name(false, p),
            TypePattern::SliceMut(p) => slice_type_name(true, p),
            TypePattern::NamedCallback(_) => {
                format!("{}Delegate", to_typespecifier_in_param(x))
            }
            TypePattern::Utf8String(_) => "string".to_string(),

            _ => to_typespecifier_in_param(param.the_type()),
        },
        Type::ReadPointer(x) | Type::ReadWritePointer(x) => match &**x {
            Type::Pattern(x) => match x {
                TypePattern::Slice(p) => slice_type_name(false, p),
                TypePattern::SliceMut(p) => slice_type_name(true, p),
                _ => to_typespecifier_in_param(param.the_type()),
            },
            _ => to_typespecifier_in_param(param.the_type()),
        },

        x => to_typespecifier_in_param(x),
    }
}
