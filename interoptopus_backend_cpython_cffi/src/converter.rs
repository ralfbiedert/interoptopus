use interoptopus::lang::c::{ConstantValue, Documentation, FnPointerType, PrimitiveValue};
use interoptopus_backend_c::CTypeConverter as _;

/// Implements [`PythonTypeConverter`].
pub struct Converter {
    pub c_converter: interoptopus_backend_c::Converter,
}

/// Converts Interoptopus types to Python types.
pub trait PythonTypeConverter {
    fn c_converter(&self) -> &interoptopus_backend_c::Converter;

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
