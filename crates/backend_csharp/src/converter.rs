use crate::Interop;
use crate::interop::FunctionNameFlavor;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use interoptopus::backend::safe_name;
use interoptopus::indented;
use interoptopus::lang::{
    Composite, ConstantValue, Enum, Field, FnPointer, Function, Opaque, Parameter, Primitive, PrimitiveValue, SugaredReturnType, Type, VariantKind,
};
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::callback::{AsyncCallback, NamedCallback};
use interoptopus::pattern::slice::SliceType;
use interoptopus::pattern::vec::VecType;

/// Converts a primitive (Rust) type to a native C# type name, e.g., `f32` to `float`.
pub fn primitive_to_type(x: Primitive) -> String {
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
pub fn fnpointer_to_type(x: &FnPointer) -> String {
    ["InteropDelegate".to_string(), safe_name(&x.internal_name())].join("_")
}

/// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
#[allow(clippy::only_used_in_recursion)]
pub fn field_to_type(x: &Type) -> String {
    match &x {
        Type::Primitive(Primitive::Bool) => "bool".to_string(),
        Type::Primitive(x) => primitive_to_type(*x),
        Type::Array(a) => format!("{}[]", field_to_type(a.the_type())),
        Type::Enum(x) => x.rust_name().to_string(),
        Type::Opaque(x) => "IntPtr".to_string(),
        Type::Composite(x) => x.rust_name().to_string(),
        Type::ReadPointer(_) => "IntPtr".to_string(),
        Type::ReadWritePointer(_) => "IntPtr".to_string(),
        Type::FnPointer(x) => fnpointer_to_type(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "string".to_string(),
            TypePattern::Utf8String(_) => "Utf8String".to_string(),
            TypePattern::Slice(x) => format!("Slice{}", slice_t(x)),
            TypePattern::SliceMut(x) => format!("SliceMut{}", slice_t(x)),
            TypePattern::Option(e) => e.the_enum().rust_name().to_string(),
            TypePattern::Result(e) => e.the_enum().rust_name().to_string(),
            TypePattern::NamedCallback(e) => e.name().to_string(),
            TypePattern::Bool => "Bool".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => field_to_type(&x.fallback_type()),
            TypePattern::Vec(x) => x.composite_type().rust_name().to_string(),
            TypePattern::AsyncCallback(_) => todo!("Async callbacks not supported in fields"),
        },
    }
}

/// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
#[allow(clippy::only_used_in_recursion)]
pub fn field_to_type_unmanaged(x: &Type) -> String {
    match x {
        Type::Primitive(Primitive::Bool) => "byte".to_string(),
        Type::Primitive(x) => primitive_to_type(*x),
        Type::Array(x) => field_to_type(x.the_type()),
        Type::Enum(x) => format!("{}.Unmanaged", x.rust_name()),
        Type::Opaque(x) => "TODO".to_string(),
        Type::Composite(x) => format!("{}.Unmanaged", x.rust_name()),
        Type::ReadPointer(_) => "IntPtr".to_string(),
        Type::ReadWritePointer(_) => "IntPtr".to_string(),
        Type::FnPointer(x) => fnpointer_to_type(x),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "IntPtr".to_string(),
            TypePattern::Utf8String(_) => "Utf8String.Unmanaged".to_string(),
            TypePattern::Slice(x) => format!("Slice{}.Unmanaged", slice_t(x)),
            TypePattern::SliceMut(x) => format!("SliceMut{}.Unmanaged", slice_t(x)),
            TypePattern::Option(e) => format!("{}.Unmanaged", e.the_enum().rust_name()),
            TypePattern::Result(e) => format!("{}.Unmanaged", e.the_enum().rust_name()),
            TypePattern::NamedCallback(e) => format!("{}.Unmanaged", e.name()),
            TypePattern::Bool => "byte".to_string(),
            TypePattern::CChar => "sbyte".to_string(),
            TypePattern::APIVersion => field_to_type(&x.fallback_type()),
            TypePattern::AsyncCallback(_) => todo!("Async callbacks not supported in fields"),
            TypePattern::Vec(x) => format!("{}.Unmanaged", x.composite_type().rust_name()),
        },
    }
}

/// Converts the `u32` part in a Rust field `x: u32` to a C# equivalent. Might convert pointers to `IntPtr`.
#[allow(clippy::only_used_in_recursion)]
pub fn field_to_type_declaration_unmanaged(x: &Field) -> String {
    let name = match x.the_type() {
        Type::Array(a) => format!("{}[{}]", x.name(), a.len()),
        _ => x.name().to_string(),
    };

    let ty = match x.the_type() {
        Type::Array(x) => format!("fixed {}", field_to_type(x.the_type())),
        _ => field_to_type_unmanaged(x.the_type()),
    };

    format!("{ty} {name}")
}

/// Converts the `u32` part in a Rust paramter `x: u32` to a C# equivalent. Might convert pointers to `out X` or `ref X`.
pub fn param_to_type(x: &Type) -> String {
    match &x {
        Type::Primitive(x) => match x {
            Primitive::Bool => "[MarshalAs(UnmanagedType.U1)] bool".to_string(),
            _ => primitive_to_type(*x),
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
            _ => format!("ref {}", param_to_type(z)),
        },
        Type::ReadWritePointer(z) => match &**z {
            Type::Opaque(_) => "IntPtr".to_string(),
            Type::Primitive(Primitive::Void) => "IntPtr".to_string(),
            Type::ReadPointer(_) => "ref IntPtr".to_string(),
            Type::ReadWritePointer(_) => "ref IntPtr".to_string(),
            Type::Pattern(TypePattern::CChar) => "IntPtr".to_string(),
            Type::Pattern(TypePattern::Slice(s)) => format!("ref {}", s.composite_type().rust_name()),
            Type::Pattern(TypePattern::SliceMut(s)) => format!("ref {}", s.composite_type().rust_name()),
            _ => format!("ref {}", param_to_type(z)),
        },
        Type::FnPointer(x) => fnpointer_to_type(x),
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
            TypePattern::APIVersion => param_to_type(&x.fallback_type()),
        },
    }
}

/// Converts the `u32` part in a Rust paramter `x: u32` to a C# equivalent in overloaded functions.
pub fn param_to_type_overloaded(x: &Type) -> String {
    match x {
        Type::Pattern(p) => match p {
            TypePattern::NamedCallback(_) => {
                format!("{}Delegate", param_to_type(x))
            }
            _ => param_to_type(x),
        },
        x => param_to_type(x),
    }
}

pub fn param_to_managed(x: &Parameter) -> String {
    match x.the_type() {
        Type::Primitive(_) => x.name().to_string(),
        Type::ReadPointer(_) => x.name().to_string(),
        Type::ReadWritePointer(_) => x.name().to_string(),
        _ if is_blittable(x.the_type()) => format!("{}.ToManaged()", x.name()),
        _ => format!("{}.IntoManaged()", x.name()),
    }
}

pub fn field_to_managed(x: &Field) -> String {
    match x.the_type() {
        Type::Primitive(Primitive::Bool) => format!("{} == 1", x.name()),
        Type::Primitive(_) => x.name().to_string(),
        Type::ReadPointer(_) => x.name().to_string(),
        Type::ReadWritePointer(_) => x.name().to_string(),
        Type::Pattern(TypePattern::CStrPointer) => "string.Empty".to_string(),
        _ if is_blittable(x.the_type()) => format!("{}.ToManaged()", x.name()),
        _ => format!("{}.IntoManaged()", x.name()),
    }
}

pub fn field_to_unmanaged(x: &Field) -> String {
    let name = x.name();
    match x.the_type() {
        Type::Primitive(Primitive::Bool) => format!("(byte) ({name} ? 1 : 0)"),
        Type::Primitive(_) => x.name().to_string(),
        Type::ReadPointer(_) => x.name().to_string(),
        Type::ReadWritePointer(_) => x.name().to_string(),
        Type::Pattern(TypePattern::CStrPointer) => "IntPtr.Zero".to_string(),
        Type::Pattern(TypePattern::NamedCallback(_)) => format!("{name}?.ToUnmanaged() ?? default"),
        _ if is_blittable(x.the_type()) => format!("{name}.ToUnmanaged()"),
        _ => format!("{name}.IntoUnmanaged()"),
    }
}

/// Converts the `u32` part in a Rust rval `-> u32` to a C# equivalent for synchronous calls.
pub fn rval_to_type_sync(x: &Type) -> String {
    match &x {
        Type::Primitive(x) => primitive_to_type(*x),
        Type::Array(_) => todo!(),
        Type::Enum(x) => x.rust_name().to_string(),
        Type::Opaque(x) => "IntPtr".to_string(),
        Type::Composite(x) => x.rust_name().to_string(),
        Type::ReadPointer(_) => "IntPtr".to_string(),
        Type::ReadWritePointer(_) => "IntPtr".to_string(),
        Type::FnPointer(x) => fnpointer_to_type(x),
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
            TypePattern::APIVersion => rval_to_type_sync(&x.fallback_type()),
            TypePattern::Vec(x) => x.composite_type().rust_name().to_string(),
            TypePattern::AsyncCallback(_) => panic!("AsyncCallback not supported in rvals"),
        },
    }
}

pub fn rval_to_type_unmanaged(x: &Type) -> String {
    match &x {
        Type::Primitive(_) => rval_to_type_sync(x),
        _ => format!("{}.Unmanaged", rval_to_type_sync(x)),
    }
}

/// Converts the `u32` part in a Rust rval `-> u32` to a C# equivalent for async calls, such as to `Task<int>`.
pub fn rval_to_type_async(x: &SugaredReturnType) -> String {
    match x {
        SugaredReturnType::Async(Type::Pattern(TypePattern::Utf8String(_))) => "Task<Utf8String>".to_string(),
        SugaredReturnType::Async(Type::Pattern(TypePattern::Result(x))) if x.t().is_void() => "Task".to_string(),
        SugaredReturnType::Async(Type::Pattern(TypePattern::Result(x))) => match x.t() {
            Type::Pattern(TypePattern::Utf8String(_)) => "Task<Utf8String>".to_string(),
            x => format!("Task<{}>", rval_to_type_sync(x)),
        },
        SugaredReturnType::Async(x) => format!("Task<{}>", rval_to_type_sync(x)),
        SugaredReturnType::Sync(x) => rval_to_type_sync(x),
    }
}

pub fn const_value(value: &ConstantValue) -> String {
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
pub fn function_name(function: &Function, flavor: FunctionNameFlavor) -> String {
    match flavor {
        FunctionNameFlavor::RawFFIName => function.name().to_string(),
        FunctionNameFlavor::CSharpMethodNameWithClass => function.name().to_upper_camel_case(),
        FunctionNameFlavor::CSharpMethodNameWithoutClass(class) => function.name().replace(class, "").to_upper_camel_case(),
    }
}

/// TODO: We might want to get rid of field renaming.
pub fn field_name(field: &Field, rename_symbols: bool) -> String {
    if rename_symbols { field.name().to_lower_camel_case() } else { field.name().into() }
}

/// For a `Slice<u8>`, returns the `u8` as a C# type, e.g., `byte`.
pub fn slice_t(x: &SliceType) -> String {
    param_to_type(x.t())
}

/// For a `Vec<u8>`, returns the `u8` as a C# type, e.g., `byte`.
pub fn vec_t(x: &VecType) -> String {
    param_to_type(x.t())
}

/// Checks whether the type can be FFI'ed as-is, or if it needs marshalling.
pub fn is_blittable(t: &Type) -> bool {
    match t {
        Type::Array(_) => false,
        Type::Composite(x) => x.fields().iter().all(|x| is_blittable(x.the_type())),
        Type::Enum(e) => {
            for v in e.variants() {
                let blittable = match v.kind() {
                    VariantKind::Unit(_) => true,
                    VariantKind::Typed(_, t) => is_blittable(t),
                };

                if !blittable {
                    return false;
                }
            }
            true
        }
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
            TypePattern::Option(x) => is_blittable(&x.the_enum().to_type()),
            TypePattern::Result(x) => is_blittable(&x.the_enum().to_type()),
            TypePattern::Bool => true,
            TypePattern::CChar => true,
            TypePattern::NamedCallback(_) => true,
            TypePattern::AsyncCallback(_) => true,
            TypePattern::Vec(_) => false,
            _ => todo!("Not implemented yet"),
        },
    }
}
