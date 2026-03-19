use crate::error::Error;
use interoptopus::ffi;
use interoptopus::pattern::result::result_to_ffi;

// As a rule of thumb, in your FFI crate you shouldn't expose "native Rust" types, as often
// their signatures and fields diverge. Instead, re-define each Rust type and method you want
// to expose.
//
// This might seem like more upfront work (it is), but it gives you much cleaner code, and the
// ability to have APIs that do exactly what they should (instead of dealing with inconsistencies
// that are unidiomatic on either the Rust or FFI side).
#[ffi(service)]
pub struct GameEngine {
    engine: core_library::engine::GameEngine,
}

#[ffi]
impl GameEngine {
    pub fn create() -> ffi::Result<Self, Error> {
        let engine = core_library::engine::GameEngine::new();
        ffi::Ok(Self { engine })
    }

    pub fn place_object(&mut self, name: ffi::CStrPtr, position: Vec2) -> ffi::Result<(), Error> {
        result_to_ffi(|| {
            let name = name.as_str().map_err(|_| Error::Fail)?;
            self.engine.place_object(name, position.into_native());
            Ok(())
        })
    }

    pub fn num_objects(&self) -> u32 {
        self.engine.num_objects()
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
