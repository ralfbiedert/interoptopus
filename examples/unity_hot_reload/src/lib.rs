use interoptopus::ffi_function;
use interoptopus::lang::c::{CType, CompositeType, Field, FnPointerType, Function};
use interoptopus::lang::rust::CTypeInfo;

#[ffi_function]
#[no_mangle]
extern "C" fn do_math(x: u32) -> u32 {
    x + 2
}

#[repr(C)]
pub struct Pointers {
    do_math: extern "C" fn(x: u32) -> u32,
}

impl CTypeInfo for Pointers {
    fn type_info() -> CType {
        let mut fields = Vec::new();

        {
            use do_math as x;
            use interoptopus::lang::rust::FunctionInfo;
            let function: Function = x::function_info();
            let t = CType::FnPointer(FnPointerType::new(function.signature().clone()));
            let field = Field::new("do_math".to_string(), t);
            fields.push(field);
        }

        let composite = CompositeType::new("Pointers".to_string(), fields);
        CType::Composite(composite)
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn xxx_entry_points(x: &mut Pointers) {
    *x = Pointers { do_math };
}

// This will create a function `ffi_inventory` which can produce
// an abstract FFI representation (called `Library`) of this crate.
interoptopus::inventory_function!(ffi_inventory, [], [do_math, xxx_entry_points], []);
