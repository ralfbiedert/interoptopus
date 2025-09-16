#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(u128);

impl Id {
    pub const fn new(id: u128) -> Self {
        Self(id)
    }
}

#[macro_export]
macro_rules! new_id {
    ($t:ident) => {
        #[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $t($crate::inventory2::Id);

        impl $t {
            pub const fn new(id: u128) -> Self {
                Self($crate::inventory2::Id::new(id))
            }
        }
    };
}
