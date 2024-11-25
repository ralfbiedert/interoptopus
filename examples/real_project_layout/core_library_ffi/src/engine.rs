use crate::error::{Error, FFIError};
use interoptopus::patterns::string::CStrPointer;
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};

// As a rule of thumb, in your FFI crate you shouldn't expose "native Rust" types, as often
// their signatures and fields diverge. Instead, re-define each Rust type and method you want
// to expose.
//
// This might seem like more upfront work (it is), but it gives you much cleaner code, and the
// ability to have APIs that do exactly what they should (instead of dealing with inconsistencies
// that are unidiomatic on either the Rust of FFI side).
#[ffi_type(opaque)]
pub struct GameEngine {
    engine: core_library::engine::GameEngine,
}

// FFI-compatible implementation of our service.
#[ffi_service(error = "FFIError")]
impl GameEngine {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        let engine = core_library::engine::GameEngine::new();
        Ok(Self { engine })
    }

    pub fn place_object(&mut self, name: CStrPointer, position: Vec2) -> Result<(), Error> {
        let name = name.as_str()?;
        let position = position.into_native();
        self.engine.place_object(name, position);
        Ok(())
    }

    #[ffi_service_method(on_panic = "return_default")]
    pub fn num_objects(&self) -> u32 {
        self.engine.num_objects()
    }

    #[ffi_service_method(ignore)]
    pub fn test(_test: u32) -> Result<(), Error> {
        Ok(())
    }
}

// Our FFI `Vec2` type.
#[ffi_type]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    // Helper method to deal with the conversion.
    fn into_native(self) -> core_library::engine::Vec2 {
        core_library::engine::Vec2 { x: self.x, y: self.y }
    }
}
