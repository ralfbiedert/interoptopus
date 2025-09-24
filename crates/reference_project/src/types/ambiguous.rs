// This show how potentially colliding items can be renamed.

pub mod ambiguous1 {
    use interoptopus::ffi;

    #[ffi(name = "Vec1")]
    pub struct Vec {
        pub x: f32,
        pub y: f32,
    }

    #[ffi(name = "Status1")]
    pub enum Status {
        X = 1,
        Y = 2,
    }
}

pub mod ambiguous2 {
    use interoptopus::ffi;

    #[ffi(name = "Vec2")]
    pub struct Vec {
        pub x: f64,
        pub z: f64,
    }

    #[ffi(name = "Status2")]
    pub enum Status {
        X = 100,
        Z = 200,
    }
}
