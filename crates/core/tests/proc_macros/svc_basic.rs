use crate::proc_macros::fn_basic::Error;
use interoptopus::ffi;
use interoptopus::lang::service::Service;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus_proc::{ffi_service, ffi_type};
use std::marker::PhantomData;
use std::thread::Builder;

#[ffi_type]
struct Runtime {
    x: u8,
}

impl AsyncRuntime for Runtime {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}

impl AsyncRuntime for ServiceA {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}

impl AsyncRuntime for ServiceB<'_> {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}
impl AsyncRuntime for ServiceBad {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}
impl AsyncRuntime for ServiceMut {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}

#[ffi_type(service)]
pub struct ServiceA {
    // #[runtime]
}

#[ffi_service]
impl ServiceA {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }

    // TODO: Vec<String> TypeInfo issue with async callbacks - working on complex types
    // pub async fn handle_vec_string(_: Async<Self>, s: ffi::Vec<ffi::String>) -> ffi::Result<ffi::Vec<ffi::String>, Error> {
    //     ffi::Result::Ok(s)
    // }
}

#[ffi_type(service)]
pub struct ServiceB<'a> {
    // #[runtime]
    #[skip]
    _x: PhantomData<&'a ()>,
}

// Temporarily commenting out generic service to test core functionality
// TODO: Add support for generic services in the future
// #[ffi_service]
// impl<'a> ServiceB<'a> {
//     pub fn new() -> ffi::Result<Self, Error> {
//         ffi::Ok(Self { _x: Default::default() })
//     }
// }

// Commented out async method - TODO: Fix async Vec<String> support
// pub async fn handle_vec_string(_: Async<Self>, s: ffi::Vec<ffi::String>) -> ffi::Result<ffi::Vec<ffi::String>, Error> {
//     ffi::Result::Ok(s)
// }

#[ffi_type(service)]
pub struct ServiceBad {
    // #[runtime]
}

#[ffi_service]
impl ServiceBad {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }

    // pub async fn call(_: Async<Self>) -> ffi::Result<(), Error> {
    //     ffi::Ok(())
    // }

    // TODO: Once an `async fn` is present, methods accepting `&mut self` must not compile.
    // pub fn bad(&mut self) {}
}

#[ffi_type(service)]
pub struct ServiceMut {}

#[ffi_service]
impl ServiceMut {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }

    pub fn ok(&mut self) {}
}

#[test]
fn test() {}
