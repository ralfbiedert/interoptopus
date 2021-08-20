use interoptopus::lang::c::{CType, ConstantValue, Documentation, FnPointerType, PrimitiveType, PrimitiveValue};
use interoptopus_backend_c::CTypeConverter as _;

/// Implements [`PythonTypeConverter`].
pub struct Converter {
    pub c_converter: interoptopus_backend_c::Converter,
}

/// Converts Interoptopus types to Python types.
pub trait PythonTypeConverter {
    fn c_converter(&self) -> &interoptopus_backend_c::Converter;

    fn to_type_hint(&self, the_type: &CType) -> String;

    fn to_type_hint_in(&self, the_type: &CType) -> String;

    fn to_type_hint_out(&self, the_type: &CType) -> String;

    fn constant_value_to_value(&self, value: &ConstantValue) -> String;

    fn documentation(&self, documentation: &Documentation) -> String;

    fn fnpointer_to_typename(&self, fn_pointer: &FnPointerType) -> String;
}

impl PythonTypeConverter for Converter {
    fn c_converter(&self) -> &interoptopus_backend_c::Converter {
        &self.c_converter
    }

    fn documentation(&self, documentation: &Documentation) -> String {
        let docs: String = documentation.lines().join("\n");
        format!(r#""""{}""""#, docs)
    }

    fn to_type_hint(&self, the_type: &CType) -> String {
        match the_type {
            CType::Primitive(x) => match x {
                PrimitiveType::Void => "".to_string(),
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
            CType::Enum(x) => x.rust_name().to_string(),
            CType::Composite(x) => x.rust_name().to_string(),
            _ => "".to_string(),
        }
    }
    fn to_type_hint_in(&self, the_type: &CType) -> String {
        let type_hint = self.to_type_hint(the_type);
        if type_hint.is_empty() {
            format!("")
        } else {
            format!(": {}", type_hint)
        }
    }

    fn to_type_hint_out(&self, the_type: &CType) -> String {
        let type_hint = self.to_type_hint(the_type);
        if type_hint.is_empty() {
            format!("")
        } else {
            format!(" -> {}", type_hint)
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

    fn fnpointer_to_typename(&self, fn_pointer: &FnPointerType) -> String {
        let rval = self.c_converter().to_type_specifier(&fn_pointer.signature().rval());
        let params = fn_pointer
            .signature()
            .params()
            .iter()
            .map(|x| self.c_converter().to_type_specifier(x.the_type()))
            .collect::<Vec<_>>()
            .join(",");

        format!("{}({})", rval, params)
    }
}
