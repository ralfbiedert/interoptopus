use interoptopus::ffi;
use interoptopus::pattern::result::Result;

// This is the FFI error enum you want your users to see. You are free to name and implement this
// almost any way you want.
#[ffi]
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Error {
    Fail,
}

#[ffi]
pub fn pattern_result_1(x: Result<u32, Error>) -> Result<u32, Error> {
    x
}

#[ffi]
pub fn pattern_result_2() -> Result<(), Error> {
    Result::Ok(())
}

#[ffi]
pub fn pattern_result_3(x: Result<(), Error>) -> Result<(), Error> {
    x
}

#[ffi]
pub fn pattern_result_4(x: Result<(), ()>) -> Result<(), ()> {
    x
}
