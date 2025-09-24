pub mod common {
    use interoptopus::ffi;

    #[ffi(module = common)]
    pub struct Vec {
        pub x: f64,
        pub z: f64,
    }
}
