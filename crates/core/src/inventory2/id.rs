#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(u128);

impl Id {
    #[must_use]
    pub const fn new(id: u128) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn derive(self, x: u128) -> Self {
        let a = self.0;
        let b = x;

        let mut result = a ^ b;
        result = result.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        result ^= result >> 64;
        result = result.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        result ^= result >> 64;
        Self(result)
    }

    #[must_use]
    pub const fn derive_id(self, x: Self) -> Self {
        self.derive(x.0)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! new_id {
    ($t:ident) => {
        #[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $t($crate::inventory2::Id);

        impl $t {
            #[must_use]
            pub const fn new(id: u128) -> Self {
                Self($crate::inventory2::Id::new(id))
            }

            #[must_use]
            pub const fn derive(self, x: u128) -> Self {
                Self(self.0.derive(x))
            }

            #[must_use]
            pub const fn derive_id(self, x: Self) -> Self {
                Self(self.0.derive_id(x.0))
            }
        }
    };
}
