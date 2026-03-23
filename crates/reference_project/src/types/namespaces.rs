use interoptopus::ffi;

pub mod common {
    use interoptopus::ffi;

    #[ffi(module = common)]
    pub struct Vec {
        pub x: f64,
        pub z: f64,
    }
}

#[ffi(module = "other")]
pub struct VecOther {
    pub x: f64,
    pub z: f64,
}
