use crate::Interop;
use crate::interop::FunctionNameFlavor;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use interoptopus::backend::util::{ctypes_from_type_recursive, safe_name};
use interoptopus::lang::c::{
    CType, CompositeType, ConstantValue, EnumType, Field, FnPointerType, Function, FunctionSignature, OpaqueType, Parameter, PrimitiveType, PrimitiveValue,
    SugaredReturnType,
};
use interoptopus::patterns::TypePattern;
use interoptopus::patterns::callback::{AsyncCallback, NamedCallback};
use interoptopus::patterns::slice::SliceType;
use std::collections::HashSet;

/// Converts a primitive (Rust) type to a native C# type name, e.g., `f32` to `float`.
pub fn primitive_to_typename(x: PrimitiveType) -> String {
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
pub fn enum_to_typename(x: &EnumType) -> String {
    x.rust_name().to_string()
}

/// TODO Converts an opaque Rust struct `Context` to a C# struct.
pub fn opaque_to_typename(_: &OpaqueType) -> String {
    // x.name().to_string()
    "IntPtr".to_string()
}

pub fn has_ffi_error_rval(signature: &FunctionSignature) -> bool {
    matches!(signature.rval(), CType::Pattern(TypePattern::FFIErrorEnum(_)))
}

/// Converts an Rust struct name `Vec2` to a C# struct name `Vec2`.
pub fn composite_to_typename(x: &CompositeType) -> String {
    x.rust_name().to_string()
}

/// Checks if the type is on the C# side blittable, in particular, if it can be accessed via raw pointers and memcopied.
pub fn is_blittable(x: &CType) -> bool {
    match x {
        CType::Primitive(_) => true,
        CType::Composite(c) => c.fields().iter().all(|x| is_blittable(x.the_type())),
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

pub fn named_callback_to_typename(x: &NamedCallback) -> String {
    x.name().to_string()
}

pub fn async_callback_to_typename(_: &AsyncCallback) -> String {
    "AsyncHelper".to_string()
}

/// Converts an Rust `pub fn()` to a C# delegate name such as `InteropDelegate`.
pub fn fnpointer_to_typename(x: &FnPointerType) -> String {
    ["InteropDelegate".to_string(), safe_name(&x.internal_name())].join("_")
}

/// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
#[allow(clippy::only_used_in_recursion)]
pub fn to_typespecifier_in_field(x: &CType, field: &Field, composite: &CompositeType) -> String {
    match &x {
        CType::Primitive(x) => primitive_to_typename(*x),
        // CType::Array(_) => panic!("Needs special handling in the writer."),
        CType::Array(_) => "TODO".to_string(),
        CType::Enum(x) => enum_to_typename(x),
        CType::Opaque(x) => opaque_to_typename(x),
        CType::Composite(x) => composite_to_typename(x),
        CType::ReadPointer(_) => "IntPtr".to_string(),
        CType::ReadWritePointer(_) => "IntPtr".to_string(),
        CType::FnPointer(x) => fnpointer_to_typename(x),
        CType::Pattern(x) => match x {
            TypePattern::CStrPointer => "string".to_string(),
            TypePattern::Utf8String(_) => "string".to_string(),
            TypePattern::FFIErrorEnum(e) => format!("Result{}", e.the_enum().rust_name()),
            TypePattern::Slice(x) => format!("Slice<{}>", get_slice_type_argument(x)),
            TypePattern::SliceMut(x) => format!("SliceMut<{}>", get_slice_type_argument(x)),
            TypePattern::Option(e) => composite_to_typename(e),
            TypePattern::NamedCallback(e) => named_callback_to_typename(e),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => to_typespecifier_in_field(&x.fallback_type(), field, composite),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

/// Converts the `u32` part in a Rust paramter `x: u32` to a C# equivalent. Might convert pointers to `out X` or `ref X`.
pub fn to_typespecifier_in_param(x: &CType) -> String {
    match &x {
        CType::Primitive(x) => match x {
            PrimitiveType::Bool => "[MarshalAs(UnmanagedType.U1)] bool".to_string(),
            _ => primitive_to_typename(*x),
        },
        CType::Array(_) => todo!(),
        CType::Enum(x) => enum_to_typename(x),
        CType::Opaque(x) => opaque_to_typename(x),
        CType::Composite(x) => composite_to_typename(x),
        CType::ReadPointer(z) => match &**z {
            CType::Opaque(_) => "IntPtr".to_string(),
            CType::Primitive(PrimitiveType::Void) => "IntPtr".to_string(),
            CType::ReadPointer(_) => "ref IntPtr".to_string(),
            CType::ReadWritePointer(_) => "ref IntPtr".to_string(),
            CType::Pattern(TypePattern::CChar) => "IntPtr".to_string(),
            CType::Pattern(TypePattern::Slice(y)) => format!("ref {}", composite_to_typename(y.composite_type())),
            CType::Pattern(TypePattern::SliceMut(y)) => format!("ref {}", composite_to_typename(y.composite_type())),
            _ => format!("ref {}", to_typespecifier_in_param(z)),
        },
        CType::ReadWritePointer(z) => match &**z {
            CType::Opaque(_) => "IntPtr".to_string(),
            CType::Primitive(PrimitiveType::Void) => "IntPtr".to_string(),
            CType::ReadPointer(_) => "ref IntPtr".to_string(),
            CType::ReadWritePointer(_) => "ref IntPtr".to_string(),
            CType::Pattern(TypePattern::CChar) => "IntPtr".to_string(),
            CType::Pattern(TypePattern::Slice(y)) => format!("ref {}>", composite_to_typename(y.composite_type())),
            CType::Pattern(TypePattern::SliceMut(y)) => format!("ref {}", composite_to_typename(y.composite_type())),
            _ => format!("ref {}", to_typespecifier_in_param(z)),
        },
        CType::FnPointer(x) => fnpointer_to_typename(x),
        CType::Pattern(x) => match x {
            TypePattern::CStrPointer => "[MarshalAs(UnmanagedType.LPStr)] string".to_string(),
            TypePattern::FFIErrorEnum(e) => format!("Result{}", e.the_enum().rust_name()),
            TypePattern::Utf8String(x) => composite_to_typename(x),
            TypePattern::Slice(x) => composite_to_typename(x.composite_type()),
            TypePattern::SliceMut(x) => composite_to_typename(x.composite_type()),
            TypePattern::Option(x) => composite_to_typename(x),
            TypePattern::Result(x) => composite_to_typename(x.composite()),
            TypePattern::NamedCallback(x) => named_callback_to_typename(x),
            TypePattern::AsyncCallback(x) => async_callback_to_typename(x),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => to_typespecifier_in_param(&x.fallback_type()),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

pub fn to_typespecifier_in_sync_fn_rval(x: &CType) -> String {
    match &x {
        CType::Primitive(x) => primitive_to_typename(*x),
        CType::Array(_) => todo!(),
        CType::Enum(x) => enum_to_typename(x),
        CType::Opaque(x) => opaque_to_typename(x),
        CType::Composite(x) => composite_to_typename(x),
        CType::ReadPointer(_) => "IntPtr".to_string(),
        CType::ReadWritePointer(_) => "IntPtr".to_string(),
        CType::FnPointer(x) => fnpointer_to_typename(x),
        CType::Pattern(x) => match x {
            TypePattern::CStrPointer => "IntPtr".to_string(),
            TypePattern::Utf8String(x) => composite_to_typename(x),
            TypePattern::FFIErrorEnum(e) => format!("Result{}", e.the_enum().rust_name()),
            TypePattern::Result(x) => composite_to_typename(x.composite()),
            TypePattern::Slice(x) => composite_to_typename(x.composite_type()),
            TypePattern::SliceMut(x) => composite_to_typename(x.composite_type()),
            TypePattern::Option(x) => composite_to_typename(x),
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
        SugaredReturnType::Async(CType::Pattern(TypePattern::Utf8String(_))) => "Task<string>".to_string(),
        SugaredReturnType::Async(CType::Pattern(TypePattern::FFIErrorEnum(_))) => "Task".to_string(),
        SugaredReturnType::Async(CType::Pattern(TypePattern::Result(x))) => match x.t() {
            CType::Pattern(TypePattern::Utf8String(_)) => "Task<string>".to_string(),
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

    rval.iter().any(|x| matches!(x, CType::Pattern(TypePattern::Utf8String(_))))
}

#[must_use]
pub fn pattern_to_native_in_signature(_: &Interop, param: &Parameter) -> String {
    let slice_type_name = |mutable: bool, slice: &SliceType| -> String {
        if is_owned_slice(slice) {
            match slice.target_type() {
                CType::Pattern(TypePattern::Utf8String(_)) => "string[]".to_string(),
                _ => format!("{}[]", crate::converter::to_typespecifier_in_param(slice.target_type())),
            }
        } else if mutable {
            format!("Span<{}>", to_typespecifier_in_param(slice.target_type()))
        } else {
            format!("ReadOnlySpan<{}>", to_typespecifier_in_param(slice.target_type()))
        }
    };
    match param.the_type() {
        x @ CType::Pattern(p) => match p {
            TypePattern::Slice(p) => slice_type_name(false, p),
            TypePattern::SliceMut(p) => slice_type_name(true, p),
            TypePattern::NamedCallback(_) => {
                format!("{}Delegate", to_typespecifier_in_param(x))
            }
            TypePattern::Utf8String(_) => "string".to_string(),

            _ => to_typespecifier_in_param(param.the_type()),
        },
        CType::ReadPointer(x) | CType::ReadWritePointer(x) => match &**x {
            CType::Pattern(x) => match x {
                TypePattern::Slice(p) => slice_type_name(false, p),
                TypePattern::SliceMut(p) => slice_type_name(true, p),
                _ => to_typespecifier_in_param(param.the_type()),
            },
            _ => to_typespecifier_in_param(param.the_type()),
        },

        x => to_typespecifier_in_param(x),
    }
}
