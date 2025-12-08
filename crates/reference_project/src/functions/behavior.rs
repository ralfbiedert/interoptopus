use crate::patterns::result::Error;
use interoptopus::pattern::result::{panic_to_result, result_to_ffi};
use interoptopus::{ffi, ffi_function};
use std::time::Duration;

// #[ffi_function]
// #[allow(unreachable_code)]
// pub fn behavior_panics() {
//     panic!("Oh no");
// }

#[allow(unreachable_code)]
#[unsafe(no_mangle)]
#[unsafe(export_name = "behavior_panics")]
pub extern "C-unwind" fn behavior_panics() {
    panic!("Oh no");
}
#[allow(non_camel_case_types)]
#[allow(clippy::redundant_pub_crate, clippy::forget_non_drop)]
pub struct behavior_panics {}
unsafe impl ::interoptopus::lang::FunctionInfo for behavior_panics {
    type Signature = fn();
    fn function_info() -> ::interoptopus::lang::Function {
        let mut doc_lines = ::std::vec::Vec::new();
        let mut params = ::std::vec::Vec::new();
        let sig = ::interoptopus::lang::Signature::new(params, ::interoptopus::lang::Type::Primitive(interoptopus::lang::Primitive::Void));
        let docs = ::interoptopus::lang::Docs::from_lines(doc_lines);
        let meta = ::interoptopus::lang::Meta::with_module_docs("".to_string(), docs);
        let domain_types = Vec::new();
        ::interoptopus::lang::Function::new("behavior_panics".to_string(), sig, meta, domain_types)
    }
}

#[ffi_function]
#[allow(unreachable_code)]
pub fn behavior_panics_via_result() -> ffi::Result<(), Error> {
    panic_to_result(|| {
        panic!("Oh no");
        result_to_ffi(|| Ok(()))
    })
}

#[ffi_function]
pub fn behavior_sleep(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}
