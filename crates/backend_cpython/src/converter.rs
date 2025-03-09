use interoptopus::lang::c::{CType, ConstantValue, Documentation, FnPointerType, PrimitiveType, PrimitiveValue};
use interoptopus::patterns::TypePattern;

#[must_use]
pub fn documentation(documentation: &Documentation) -> String {
    let docs: String = documentation.lines().join("\n");
    format!(r#""""{docs}""""#)
}

#[must_use]
pub fn to_type_hint(the_type: &CType, is_parameter: bool) -> String {
    match the_type {
        CType::Primitive(x) => match x {
            PrimitiveType::Void => String::new(),
            PrimitiveType::Bool => "bool".to_string(),
            PrimitiveType::U8 => "int".to_string(),
            PrimitiveType::U16 => "int".to_string(),
            PrimitiveType::U32 => "int".to_string(),
            PrimitiveType::U64 => "int".to_string(),
            PrimitiveType::I8 => "int".to_string(),
            PrimitiveType::I16 => "int".to_string(),
            PrimitiveType::I32 => "int".to_string(),
            PrimitiveType::I64 => "int".to_string(),
            PrimitiveType::F32 => "float".to_string(),
            PrimitiveType::F64 => "float".to_string(),
        },
        CType::ReadPointer(x) => match &**x {
            CType::Opaque(_) => "ctypes.c_void_p".to_string(),
            CType::Primitive(PrimitiveType::Void) => "ctypes.c_void_p".to_string(),
            _ => format!("ctypes.POINTER({})", to_ctypes_name(x, true)),
        },
        CType::ReadWritePointer(x) => match &**x {
            CType::Opaque(_) => "ctypes.c_void_p".to_string(),
            CType::Primitive(PrimitiveType::Void) => "ctypes.c_void_p".to_string(),
            _ => format!("ctypes.POINTER({})", to_ctypes_name(x, true)),
        },
        CType::Enum(_) => "ctypes.c_int".to_string(), // is this correct?
        CType::Composite(x) => x.rust_name().to_string(),
        CType::Pattern(x) => match x {
            TypePattern::CStrPointer => "bytes".to_string(),
            TypePattern::Option(c) => c.rust_name().to_string(),
            TypePattern::Slice(c) | TypePattern::SliceMut(c) => {
                let mut res = c.rust_name().to_string();
                let inner = to_ctypes_name(c.target_type(), false);
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
pub fn to_ctypes_name(the_type: &CType, with_type_annotations: bool) -> String {
    match the_type {
        CType::Primitive(x) => match x {
            PrimitiveType::Void => String::new(),
            PrimitiveType::Bool => "ctypes.c_bool".to_string(),
            PrimitiveType::U8 => "ctypes.c_uint8".to_string(),
            PrimitiveType::U16 => "ctypes.c_uint16".to_string(),
            PrimitiveType::U32 => "ctypes.c_uint32".to_string(),
            PrimitiveType::U64 => "ctypes.c_uint64".to_string(),
            PrimitiveType::I8 => "ctypes.c_int8".to_string(),
            PrimitiveType::I16 => "ctypes.c_int16".to_string(),
            PrimitiveType::I32 => "ctypes.c_int32".to_string(),
            PrimitiveType::I64 => "ctypes.c_int64".to_string(),
            PrimitiveType::F32 => "ctypes.c_float".to_string(),
            PrimitiveType::F64 => "ctypes.c_double".to_string(),
        },
        CType::Enum(_) => "ctypes.c_int".to_string(), // is this correct?
        CType::Composite(x) => x.rust_name().to_string(),
        CType::Array(x) => format!("{} * {}", to_ctypes_name(x.array_type(), with_type_annotations), x.len()),
        CType::Opaque(_) => "ERROR".to_string(),
        CType::FnPointer(x) => fnpointer_to_typename(x),
        CType::ReadPointer(x) => match &**x {
            CType::Opaque(_) => "ctypes.c_void_p".to_string(),
            CType::Primitive(PrimitiveType::Void) => "ctypes.c_void_p".to_string(),
            _ => format!("ctypes.POINTER({})", to_ctypes_name(x, with_type_annotations)),
        },
        CType::ReadWritePointer(x) => match &**x {
            CType::Opaque(_) => "ctypes.c_void_p".to_string(),
            CType::Primitive(PrimitiveType::Void) => "ctypes.c_void_p".to_string(),
            _ => format!("ctypes.POINTER({})", to_ctypes_name(x, with_type_annotations)),
        },
        CType::Pattern(pattern) => match pattern {
            TypePattern::CStrPointer => to_ctypes_name(&pattern.fallback_type(), with_type_annotations),
            TypePattern::APIVersion => "ctypes.c_uint64".to_string(),
            TypePattern::Utf8String(c) => c.rust_name().to_string(),
            TypePattern::FFIErrorEnum(_) => "ctypes.c_int".to_string(),
            TypePattern::Slice(c) => c.rust_name().to_string(),
            TypePattern::SliceMut(c) => c.rust_name().to_string(),
            TypePattern::Option(x) => x.rust_name().to_string(),
            TypePattern::Bool => "ctypes.c_uint8".to_string(),
            TypePattern::CChar => "ctypes.c_char".to_string(),
            TypePattern::NamedCallback(x) => fnpointer_to_typename(x.fnpointer()),
            TypePattern::Result(c) => c.composite().rust_name().to_string(),
            TypePattern::AsyncCallback(x) => fnpointer_to_typename(x.fnpointer()),
            _ => panic!("Pattern not explicitly handled"),
        },
    }
}

#[allow(clippy::useless_format)]
#[must_use]
pub fn to_type_hint_in(the_type: &CType, is_parameter: bool) -> String {
    let type_hint = to_type_hint(the_type, is_parameter);
    if type_hint.is_empty() { format!("") } else { format!(": {type_hint}") }
}

#[allow(clippy::useless_format)]
#[must_use]
pub fn to_type_hint_out(the_type: &CType) -> String {
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
            PrimitiveValue::I8(x) => format!("{x}"),
            PrimitiveValue::I16(x) => format!("{x}"),
            PrimitiveValue::I32(x) => format!("{x}"),
            PrimitiveValue::I64(x) => format!("{x}"),
            PrimitiveValue::F32(x) => format!("{x}"),
            PrimitiveValue::F64(x) => format!("{x}"),
        },
    }
}

#[must_use]
pub fn fnpointer_to_typename(fn_pointer: &FnPointerType) -> String {
    let rval = match fn_pointer.signature().rval() {
        CType::Primitive(PrimitiveType::Void) => "None".to_string(),
        x => to_ctypes_name(x, true),
    };

    let args = fn_pointer.signature().params().iter().map(|x| to_ctypes_name(x.the_type(), true)).collect::<Vec<_>>();

    format!("ctypes.CFUNCTYPE({}, {})", rval, args.join(", "))
}
