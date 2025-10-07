#![allow(unused)]
use crate::proc_macros::fn_basic::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus_proc::{ffi_service, ffi_type};
use std::marker::PhantomData;

#[ffi_type]
struct Runtime {
    x: u8,
}

impl AsyncRuntime for Runtime {
    type T = ();

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}

impl AsyncRuntime for ServiceA {
    type T = ();
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}

impl AsyncRuntime for ServiceB<'_> {
    type T = ();
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}
impl AsyncRuntime for ServiceBad {
    type T = ();
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}
impl AsyncRuntime for ServiceMut {
    type T = ();
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }
}

type X = Async<ServiceA>;

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
    pub async fn handle_vec_string(_: Async<Self>, s: ffi::Vec<ffi::String>) -> ffi::Result<ffi::Vec<ffi::String>, Error> {
        ffi::Result::Ok(s)
    }
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
//
// Commented out async method - TODO: Fix async Vec<String> support
// pub async fn handle_vec_string(_: Async<Self>, s: ffi::Vec<ffi::String>) -> ffi::Result<ffi::Vec<ffi::String>, Error> {
//     ffi::Result::Ok(s)
// }

#[ffi_type(service)]
pub struct ServiceBad {
    // #[runtime]
    x: u32,
}

#[ffi_service]
impl ServiceBad {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self { x: 12 })
    }

    // TODO
    // - warn on methods that look like ctors but don't return Result<Self, _>
    // - warn if Async<Self> is used in non-async methods
    // - check if warning, validation logic is condensed or all over the place
    // - re-introduce manual prefix="asdasda"
    // - check waht to do with lifetime'd services
    // - check all parameters and types are ASYNC_SAFE on async services
    // pub fn call_should_not_work(x: u32) -> ffi::Result<(), Error> {
    //     // x.x += 1;
    //     ffi::Ok(())
    // }

    // TODO: Once an `async fn` is present, methods accepting `&mut self` must not compile.
    // pub fn bad(&mut self) {}
}

// const _: () = {
//     use interoptopus::lang::types::TypeInfo;
//     assert!(<ffi::Result<(), Error>>::SERVICE_CTOR_SAFE, "The method looks like a ctor, but it does not return ffi::Result<Self, _>.");
// };

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
