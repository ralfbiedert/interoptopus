use interoptopus::ffi;

#[ffi]
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Error {
    Fail,
}
