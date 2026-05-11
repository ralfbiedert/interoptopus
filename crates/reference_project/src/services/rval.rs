use crate::types::basic::Vec3f32;
use interoptopus::ffi;
use interoptopus::wire::Wire;

#[ffi(service)]
pub struct ServiceRval {}

#[ffi]
impl ServiceRval {
    pub fn create() -> Self {
        Self {}
    }

    pub fn number(&self) -> u32 {
        123
    }

    pub fn vecf32(&self) -> Vec3f32 {
        Vec3f32::default()
    }

    pub fn wire(&self) -> Wire<String> {
        Wire::from("hello".to_string())
    }
}
