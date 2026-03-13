use interoptopus::pattern::asynk::Async;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{callback, ffi, AsyncRuntime};

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
pub fn delgt(x: SumDelegateReturn, _input: &u32, _input2: &mut u32) -> SumDelegateReturn {
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

    pub fn delgt(&mut self, x: SumDelegateReturn, _input: &u32, _input2: &u32) -> SumDelegateReturn {
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
    pub async fn sum(_this: Async<Self>, _x: i32, _y: i32, _z: SumDelegateReturn) -> ffi::Result<(), Error> {
        ffi::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use interoptopus::inventory::RustInventory;
    use interoptopus::{extra_type, function, service};
    use interoptopus_csharp::dispatch::Dispatch;
    use interoptopus_csharp::lang::meta::FileEmission;
    use interoptopus_csharp::output::FileName;
    use interoptopus_csharp::RustLibrary;

    // We just trick a unit test into producing our bindings, here for C#
    #[test]
    fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
        // In a real project this should be a freestanding `my_inventory()` function inside
        // your FFI or build crate.
        let inventory = RustInventory::new()
            .register(function!(super::my_function))
            .register(function!(super::refref))
            .register(function!(super::delgt))
            .register(extra_type!(super::Error))
            .register(extra_type!(super::SumDelegate2))
            .register(extra_type!(super::SumDelegateReturn))
            .register(service!(super::ServiceBasic))
            .register(service!(super::ServiceBasic2))
            .validate();

        RustLibrary::builder(inventory)
            .dll_name("hello_world")
            .dispatch(Dispatch::custom(|x, _| match x.emission {
                FileEmission::Common => FileName::new("Interop.Common.cs"),
                FileEmission::Default => FileName::new("Interop.cs"),
                FileEmission::CustomModule(_) => FileName::new("Interop.cs"),
            }))
            .build()
            .process()?
            .write_buffers_to("bindings/")?;

        Ok(())
    }
}
