use crate::functions::{complex_args_1, ref_mut_option, tupled};
use crate::patterns::success_enum::FFIError;
use crate::types::{Empty, Tupled, Vec3f32};
use interoptopus::ffi_function;
use interoptopus::lang::c::{CType, CompositeType, Field, FnPointerType, Function};
use interoptopus::lang::rust::CTypeInfo;

// TODO - How should this pattern be called?

#[repr(C)]
pub struct Pointers {
    f1: extern "C" fn(x: Option<&mut i64>) -> bool,
    f2: extern "C" fn(x: Tupled) -> Tupled,
    f3: extern "C" fn(_a: Vec3f32, _b: Option<&Empty>) -> FFIError,
}

impl CTypeInfo for Pointers {
    fn type_info() -> CType {
        let mut fields = Vec::new();

        {
            use interoptopus::lang::rust::FunctionInfo;
            use ref_mut_option as x;
            let function: Function = x::function_info();
            let t = CType::FnPointer(FnPointerType::new(function.signature().clone()));
            let field = Field::new("f1".to_string(), t);
            fields.push(field);
        }
        {
            use interoptopus::lang::rust::FunctionInfo;
            use tupled as x;
            let function: Function = x::function_info();
            let t = CType::FnPointer(FnPointerType::new(function.signature().clone()));
            let field = Field::new("f2".to_string(), t);
            fields.push(field);
        }
        {
            use complex_args_1 as x;
            use interoptopus::lang::rust::FunctionInfo;
            let function: Function = x::function_info();
            let t = CType::FnPointer(FnPointerType::new(function.signature().clone()));
            let field = Field::new("f3".to_string(), t);
            fields.push(field);
        }

        let composite = CompositeType::new("Pointers".to_string(), fields);
        CType::Composite(composite)
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn xxx_entry_points(x: &mut Pointers) -> FFIError {
    *x = Pointers {
        f1: ref_mut_option,
        f2: tupled,
        f3: complex_args_1,
    };

    FFIError::Ok
}
