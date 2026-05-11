use crate::types::basic::Vec3f32;
use interoptopus::pattern::asynk::Async;
use interoptopus::rt::Tokio;
use interoptopus::wire::Wire;
use interoptopus::{AsyncRuntime, ffi};

#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct ServiceAsyncRval {
    runtime: Tokio,
}

#[ffi]
impl ServiceAsyncRval {
    pub fn simple() -> Self {
        let runtime = Tokio::new();
        Self { runtime }
    }

    pub async fn number(_this: Async<Self>) -> u32 {
        123
    }

    pub async fn vecf32(_this: Async<Self>) -> Vec3f32 {
        Vec3f32::default()
    }

    pub async fn wire(_this: Async<Self>) -> Wire<String> {
        Wire::from("hello".to_string())
    }
}
