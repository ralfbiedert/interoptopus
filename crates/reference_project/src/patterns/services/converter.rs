// use crate::patterns::result::{Error, FFIError};
// use interoptopus::patterns::slice::FFISlice;
// use interoptopus::patterns::string::CStrPointer;
// use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};
// use std::sync::{Arc, RwLock};
//
// #[ffi_type(opaque)]
// pub struct ServiceConverter {}
//
// #[ffi_service(error = "FFIError")]
// impl ServiceConverter {
//     #[ffi_service_ctor]
//     pub fn new() -> Result<Self, Error> {
//         Ok(Self {})
//     }
//
//     pub fn native(&self, blah: BlahFFI) -> BlahFFI<'_> {
//         let blah = blah.from_ffi(self);
//
//         let rval = blah.to_ffi(self);
//
//         BlahFFI { data: Default::default() }
//     }
// }
//
// #[ffi_type]
// pub struct BlahFFI<'a> {
//     data: FFISlice<'a, CStrPointer<'a>>,
// }
//
// pub struct Blah {
//     data: Vec<String>,
// }
//
// trait FromFFI<T> {
//     type Target;
//
//     fn from_ffi(&self, t: &T) -> Self::Target;
// }
//
// trait ToFFI<T> {
//     type Target<'x>;
//     fn to_ffi(&self, t: &T) -> Self::Target<'a>;
// }
//
// // impl<'a> ToFFI<ServiceConverter> for Blah {
// //     type Target = BlahFFI<'a>;
// //
// //     fn to_ffi(&'a self, t: &ServiceConverter) -> Self::Target<'a> {
// //         BlahFFI { data: FFISlice::from_vec(self.data.iter().map(|x| CStrPointer::from_str(x)).collect()) }
// //     }
// // }
//
// impl<'a> FromFFI<ServiceConverter> for BlahFFI<'a> {
//     type Target = Blah;
//
//     fn from_ffi(&self, t: &ServiceConverter) -> Self::Target {
//         Blah { data: self.data.iter().map(|x| todo!()).collect() }
//     }
// }
//
// #[ffi_type(opaque)]
// pub struct FFIString {
//     data: Arc<RwLock<String>>,
// }
//
// impl FFIString {
//     pub fn from_str(s: &str) -> Self {
//         Self { data: Arc::new(RwLock::new(s.to_string())) }
//     }
//
//     pub fn append(&self, s: &str) {
//         let mut data = self.data.write().unwrap();
//         data.push_str(s);
//     }
// }
//
// #[ffi_type]
// pub struct BlahFFI<'a> {
//     data: FFIVec<FFIString>,
// }
//
// #[ffi_type(opaque)]
// pub struct ServiceConverter2 {}
//
// #[ffi_service(error = "FFIError")]
// impl ServiceConverter2 {
//     #[ffi_service_ctor]
//     pub fn new() -> Result<Self, Error> {
//         Ok(Self {})
//     }
//
//     #[ffi_service_method(on_panic = "abort")]
//     pub fn native(&self, i: FFIString2) -> FFIString2 {
//         let x = String::from("Hello ");
//         let v = Vec::from(x);
//     }
// }
//
// #[ffi_type]
// pub struct FFIString2 {
//     data: *mut u8,
//     len: u64,
//     capacity: u64,
// }
//
// impl FFIString2 {
//     pub fn from_str(s: &str) -> Self {
//         Self { data: s.to_string() }
//     }
//
//     pub fn append(&mut self, s: &str) {
//         self.data.push_str(s);
//     }
// }
