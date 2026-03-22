use crate::patterns::callback::{MyCallback, SumDelegate2};
use crate::patterns::result::Error;
use crate::types::basic::Vec3f32;
use interoptopus::ffi;

interoptopus::plugin!(Pattern {
    fn result(res: ffi::Result<Vec3f32, Error>) -> ffi::Result<Vec3f32, Error>;
    fn delegate_1(res: MyCallback) -> SumDelegate2;
});
