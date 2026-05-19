use crate::error::Error;
use interoptopus::pattern::asynk::Async;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{AsyncRuntime, ffi};

// As a rule of thumb, in your FFI crate you shouldn't expose "native Rust" types, as often
// their signatures and fields diverge. Instead, re-define each Rust type and method you want
// to expose.
//
// This might seem like more upfront work (it is), but it gives you much cleaner code, and the
// ability to have APIs that do exactly what they should (instead of dealing with inconsistencies
// that are unidiomatic on either the Rust or FFI side).
#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct GameEngine {
    runtime: Tokio,
    engine: tokio::sync::RwLock<core_library::engine::GameEngine>,
}

#[ffi]
impl GameEngine {
    pub fn create() -> ffi::Result<Self, Error> {
        let engine = tokio::sync::RwLock::new(core_library::engine::GameEngine::new());
        ffi::Ok(Self { runtime: Tokio::new(), engine })
    }

    pub fn place_object(&self, name: ffi::CStrPtr, position: Vec2) -> ffi::Result<(), Error> {
        result_to_ffi(|| {
            let name = name.as_str().map_err(|_| Error::Fail)?;
            let mut engine = self.engine.blocking_write();
            engine.place_object(name, position.into_native());
            Ok(())
        })
    }

    pub fn num_objects(&self) -> u32 {
        let engine = self.engine.blocking_read();
        engine.num_objects()
    }

    pub async fn update(aself: Async<Self>, delta_time_sec: f64) -> ffi::Result<f64, Error> {
        let cancel_token = aself.context().clone();
        let mut engine = aself.engine.write().await;
        match cancel_token.run_until_cancelled_owned(engine.update_async(delta_time_sec)).await
        {
            Some(elapsed) => ffi::Ok(elapsed.as_secs_f64()),
            None => ffi::Err(Error::TaskCancelled),
        }
    }
}

/// Our FFI `Vec2` type.
#[ffi]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    fn into_native(self) -> core_library::engine::Vec2 {
        core_library::engine::Vec2 { x: self.x, y: self.y }
    }
}
