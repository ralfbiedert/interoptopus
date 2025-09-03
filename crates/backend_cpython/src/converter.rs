use interoptopus::lang::{ConstantValue, Docs, FnPointer, Primitive, PrimitiveValue, Type};
use interoptopus::pattern::TypePattern;

#[must_use]
pub fn documentation(documentation: &Docs) -> String {
    let docs: String = documentation.lines().join("\n");
    format!(r#""""{docs}""""#)
}

#[must_use]
pub fn to_type_hint(the_type: &Type, is_parameter: bool) -> String {
    match the_type {
        Type::Primitive(x) => match x {
            Primitive::Void => String::new(),
            Primitive::Bool => "bool".to_string(),
            Primitive::U8 => "int".to_string(),
            Primitive::U16 => "int".to_string(),
            Primitive::U32 => "int".to_string(),
            Primitive::U64 => "int".to_string(),
            Primitive::Usize => "int".to_string(),
            Primitive::I8 => "int".to_string(),
            Primitive::I16 => "int".to_string(),
            Primitive::I32 => "int".to_string(),
            Primitive::I64 => "int".to_string(),
            Primitive::Isize => "int".to_string(),
            Primitive::F32 => "float".to_string(),
            Primitive::F64 => "float".to_string(),
        },
        Type::ReadPointer(x) => match &**x {
            Type::Opaque(_) => "ctypes.c_void_p".to_string(),
            Type::Primitive(Primitive::Void) => "ctypes.c_void_p".to_string(),
            _ => format!("ctypes.POINTER({})", to_ctypes_name(x, true)),
        },
        Type::ReadWritePointer(x) => match &**x {
            Type::Opaque(_) => "ctypes.c_void_p".to_string(),
            Type::Primitive(Primitive::Void) => "ctypes.c_void_p".to_string(),
            _ => format!("ctypes.POINTER({})", to_ctypes_name(x, true)),
        },
        Type::Enum(_) => "TODO".to_string(), // is this correct?
        Type::Composite(x) => x.rust_name().to_string(),
        Type::Pattern(x) => match x {
            TypePattern::CStrPointer => "bytes".to_string(),
            TypePattern::Option(_) => "TODO".to_string(),
            TypePattern::Slice(c) | TypePattern::SliceMut(c) => {
                let mut res = c.rust_name().to_string();
                let inner = to_ctypes_name(c.t(), false);
                if is_parameter {
                    res = format!("{res} | ctypes.Array[{inner}]");
                }
                res
            }
            TypePattern::CChar => "ctypes.c_char".to_string(),
            _ => String::new(),
        },
        _ => String::new(),
    }
}

#[allow(clippy::only_used_in_recursion)]
#[must_use]
pub fn to_ctypes_name(the_type: &Type, with_type_annotations: bool) -> String {
    match the_type {
        Type::Primitive(x) => match x {
            Primitive::Void => String::new(),
            Primitive::Bool => "ctypes.c_bool".to_string(),
            Primitive::U8 => "ctypes.c_uint8".to_string(),
            Primitive::U16 => "ctypes.c_uint16".to_string(),
            Primitive::U32 => "ctypes.c_uint32".to_string(),
            Primitive::U64 => "ctypes.c_uint64".to_string(),
            Primitive::Usize => "ctypes.c_size_t".to_string(),
            Primitive::I8 => "ctypes.c_int8".to_string(),
            Primitive::I16 => "ctypes.c_int16".to_string(),
            Primitive::I32 => "ctypes.c_int32".to_string(),
            Primitive::I64 => "ctypes.c_int64".to_string(),
            Primitive::Isize => "ctypes.c_ssize_t".to_string(),
            Primitive::F32 => "ctypes.c_float".to_string(),
            Primitive::F64 => "ctypes.c_double".to_string(),
        },
        Type::Enum(_) => "ctypes.c_int".to_string(), // is this correct?
        Type::Composite(x) => x.rust_name().to_string(),
        Type::Wire(_) => "todo".to_string(),
        Type::WirePayload(_) => "todo".to_string(),
        Type::Array(x) => format!("{} * {}", to_ctypes_name(x.the_type(), with_type_annotations), x.len()),
        Type::Opaque(_) => "ERROR".to_string(),
        Type::ExternType(_) => "ERROR".to_string(), // extern types are not supported in this backend
        Type::FnPointer(x) => fnpointer_to_typename(x),
        Type::ReadPointer(x) => match &**x {
            Type::Opaque(_) => "ctypes.c_void_p".to_string(),
            Type::Primitive(Primitive::Void) => "ctypes.c_void_p".to_string(),
            _ => format!("ctypes.POINTER({})", to_ctypes_name(x, with_type_annotations)),
        },
        Type::ReadWritePointer(x) => match &**x {
            Type::Opaque(_) => "ctypes.c_void_p".to_string(),
            Type::Primitive(Primitive::Void) => "ctypes.c_void_p".to_string(),
            _ => format!("ctypes.POINTER({})", to_ctypes_name(x, with_type_annotations)),
        },
        Type::Pattern(pattern) => match pattern {
            TypePattern::CStrPointer => to_ctypes_name(&pattern.fallback_type(), with_type_annotations),
            TypePattern::APIVersion => "ctypes.c_uint64".to_string(),
            TypePattern::Utf8String(c) => c.rust_name().to_string(),
            TypePattern::Slice(c) => c.rust_name().to_string(),
            TypePattern::SliceMut(c) => c.rust_name().to_string(),
            TypePattern::Option(_) => "TODO".to_string(),
            TypePattern::Bool => "ctypes.c_uint8".to_string(),
            TypePattern::CChar => "ctypes.c_char".to_string(),
            TypePattern::NamedCallback(x) => fnpointer_to_typename(x.fnpointer()),
            TypePattern::Result(c) => c.the_enum().rust_name().to_string(),
            TypePattern::AsyncCallback(x) => fnpointer_to_typename(x.fnpointer()),
            TypePattern::Vec(c) => c.rust_name().to_string(),
        },
    }
}

#[allow(clippy::useless_format)]
#[must_use]
pub fn to_type_hint_in(the_type: &Type, is_parameter: bool) -> String {
    let type_hint = to_type_hint(the_type, is_parameter);
    if type_hint.is_empty() { format!("") } else { format!(": {type_hint}") }
}

#[allow(clippy::useless_format)]
#[must_use]
pub fn to_type_hint_out(the_type: &Type) -> String {
    let type_hint = to_type_hint(the_type, false);
    if type_hint.is_empty() { format!("") } else { format!(" -> {type_hint}") }
}

#[must_use]
pub fn constant_value_to_value(value: &ConstantValue) -> String {
    match value {
        ConstantValue::Primitive(x) => match x {
            PrimitiveValue::Bool(x) => format!("{x}"),
            PrimitiveValue::U8(x) => format!("{x}"),
            PrimitiveValue::U16(x) => format!("{x}"),
            PrimitiveValue::U32(x) => format!("{x}"),
            PrimitiveValue::U64(x) => format!("{x}"),
            PrimitiveValue::Usize(x) => format!("{x}"),
            PrimitiveValue::I8(x) => format!("{x}"),
            PrimitiveValue::I16(x) => format!("{x}"),
            PrimitiveValue::I32(x) => format!("{x}"),
            PrimitiveValue::I64(x) => format!("{x}"),
            PrimitiveValue::Isize(x) => format!("{x}"),
            PrimitiveValue::F32(x) => format!("{x}"),
            PrimitiveValue::F64(x) => format!("{x}"),
        },
    }
}

#[must_use]
pub fn fnpointer_to_typename(fn_pointer: &FnPointer) -> String {
    let rval = match fn_pointer.signature().rval() {
        Type::Primitive(Primitive::Void) => "None".to_string(),
        x => to_ctypes_name(x, true),
    };

    let args = fn_pointer.signature().params().iter().map(|x| to_ctypes_name(x.the_type(), true)).collect::<Vec<_>>();

    format!("ctypes.CFUNCTYPE({}, {})", rval, args.join(", "))
}
