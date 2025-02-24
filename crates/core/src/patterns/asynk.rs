use std::ffi::c_void;
use std::ptr::null;
use crate::lang::c::{CType, Documentation, FnPointerType, FunctionSignature, Meta, Parameter, PrimitiveType};
use crate::lang::rust::CTypeInfo;
use crate::patterns;
use crate::patterns::TypePattern;

/// TODO: Document must be thread safe
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AsyncCallback<T>(Option<extern "C" fn(&T, *const c_void) -> ()>, *const c_void);

unsafe impl<T> Send for AsyncCallback<T> {}
unsafe impl<T> Sync for AsyncCallback<T> {}

impl<T: CTypeInfo> AsyncCallback<T> {
    ///   Creates a new instance of the callback using  `extern "C" fn` 
    pub fn new(func: extern "C" fn(&T, *const c_void) -> ()) -> Self {
        Self(Some(func), null())
    }

    ///   Will call function if it exists, panic otherwise. 
    pub fn call(&self, t: &T) -> () {
        self.0.expect("Assumed function would exist but it didn't.")(t, self.1)
    }

    ///   Will call function only if it exists 
    pub fn call_if_some(&self, t: &T) -> Option<()> {
        match self.0 {
            Some(c) => Some(c(t, self.1)),
            None => None
        }
    }
}
impl<T: CTypeInfo> From<for<> extern "C" fn(&T, *const c_void) -> ()> for AsyncCallback<T> {
    fn from(x: extern "C" fn(&T, *const c_void) -> ()) -> Self {
        Self(Some(x), null())
    }
}

impl<T: CTypeInfo> From<AsyncCallback<T>> for Option<extern "C" fn(&T, *const c_void) -> ()> {
    fn from(x: AsyncCallback<T>) -> Self {
        x.0
    }
}

unsafe impl<T: CTypeInfo> CTypeInfo for AsyncCallback<T> {
    fn type_info() -> CType {

        let rval = <() as CTypeInfo>::type_info();

        let params = vec![
            Parameter::new("value_ptr".to_string(), CType::ReadPointer(Box::new(T::type_info()))),
            Parameter::new("callback_data".to_string(), CType::ReadPointer(Box::new(CType::Primitive(PrimitiveType::Void))))
        ];

        let meta = Meta::with_documentation(Documentation::new());
        let sig = FunctionSignature::new(params, rval);
        let name = format!("AsyncCallback{}", T::type_info().name_within_lib());
        let fn_pointer = FnPointerType::new_named(sig, name);
        let named_callback = patterns::AsyncCallback::with_meta(fn_pointer, meta);

        CType::Pattern(TypePattern::AsyncCallback(named_callback))
    }
}