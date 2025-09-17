#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(u128);

impl Id {
    pub const fn new(id: u128) -> Self {
        Self(id)
    }

    pub const fn derive(self, x: u128) -> Self {
        let a = self.0;
        let b = x;

        let mut result = a ^ b;
        result = result.wrapping_mul(0x9e3779b97f4a7c15);
        result ^= result >> 64;
        result = result.wrapping_mul(0x9e3779b97f4a7c15);
        result ^= result >> 64;
        Id(result)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! new_id {
    ($t:ident) => {
        #[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $t($crate::inventory2::Id);

        impl $t {
            pub const fn new(id: u128) -> Self {
                Self($crate::inventory2::Id::new(id))
            }

            pub const fn derive(self, x: u128) -> Self {
                Self(self.0.derive(x))
            }
        }
    };
}
