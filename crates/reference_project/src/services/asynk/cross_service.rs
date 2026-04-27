use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::pattern::asynk::Async;

/// A simple service produced by a factory method on another service.
#[ffi(service)]
pub struct ServiceProduced {
    val: u32,
}

#[ffi]
impl ServiceProduced {
    pub fn get_value(&self) -> u32 {
        self.val
    }
}

/// A factory service whose async method returns a *different* service type.
///
/// This exercises the code path where `ffi::Result<OtherService, Error>` is
/// the return type of an async method — the C# trampoline must correctly
/// wrap the `IntPtr` into a managed service instance.
#[ffi(service)]
#[derive(interoptopus::AsyncRuntime)]
pub struct ServiceFactory {
    runtime: interoptopus::rt::Tokio,
}

#[ffi]
impl ServiceFactory {
    pub fn create() -> ffi::Result<Self, Error> {
        interoptopus::pattern::result::result_to_ffi(|| {
            let runtime = interoptopus::rt::Tokio::new();
            Ok(Self { runtime })
        })
    }

    /// Async factory method returning a *different* service type.
    pub async fn produce(_: Async<Self>, val: u32) -> ffi::Result<ServiceProduced, Error> {
        ffi::Ok(ServiceProduced { val })
    }
}
