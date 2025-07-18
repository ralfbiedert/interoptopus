pub mod common {
    use interoptopus::ffi_type;

    #[ffi_type(namespace = "_common")]
    pub struct Vec {
        pub x: f64,
        pub z: f64,
    }
}
