use interoptopus::inventory::{Inventory, RustInventory};
use interoptopus::pattern::asynk::{Async, AsyncRuntime};
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{callback, extra_type, ffi, service, AsyncRuntime};

/// A simple type in our FFI layer.
#[ffi]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// A simple type in our FFI layer.
#[ffi]
pub enum Error {
    A,
    B(u32),
}

callback!(SumDelegate2(x: i32, y: i32) -> i32);
callback!(SumDelegateReturn(x: i32, y: i32) -> ffi::Result<(), Error>);
callback!(SumDelegateReturn2(x: i32, y: i32));

/// Function using the type.
#[ffi]
pub fn my_function(input: Vec2) -> Vec2 {
    input
}

#[ffi]
pub fn refref(input: &u32) -> &u32 {
    input
}

#[ffi]
pub fn delgt(x: SumDelegateReturn, input: &u32, input2: &mut u32) -> SumDelegateReturn {
    x
}

#[ffi(service)]
pub struct ServiceBasic {}

#[ffi]
impl ServiceBasic {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }

    pub fn sum(&self, x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn delgt(&mut self, x: SumDelegateReturn, input: &u32, input2: &u32) -> SumDelegateReturn {
        x
    }
}

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceBasic2 {
    runtime: Tokio,
}

#[ffi]
impl ServiceBasic2 {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            let runtime = Tokio::new();
            Ok(Self { runtime })
        })
    }
    pub async fn sum(_this: Async<Self>, x: i32, y: i32, z: SumDelegateReturn) -> ffi::Result<(), Error> {
        ffi::Result::Ok(())
    }
}

// We just trick a unit test into producing our bindings, here for C#
#[test]
#[rustfmt::skip]
fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
    use interoptopus::function;
    use interoptopus::inventory::Inventory;
    use interoptopus_csharp::RustLibrary;

    // In a real project this should be a freestanding `my_inventory()` function inside
    // your FFI or build crate.
    let inventory = RustInventory::new()
        .register(function!(my_function))
        .register(function!(refref))
        .register(function!(delgt))
        .register(extra_type!(Error))
        .register(extra_type!(SumDelegate2))
        .register(extra_type!(SumDelegateReturn))
        .register(service!(ServiceBasic))
        .register(service!(ServiceBasic2))
        .validate();

    RustLibrary::builder(inventory)
        .dll_name("hello_world")
        .build()
        .process()?
        .write_buffers_to("bindings/")?;

    Ok(())
}
