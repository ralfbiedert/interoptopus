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
macro_rules! new_id {
    ($t:ident) => {
        #[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $t($crate::inventory::Id);

        impl $t {
            #[must_use]
            pub const fn new(id: u128) -> Self {
                Self::from_id($crate::inventory::Id::new(id))
            }

            #[must_use]
            pub const fn from_id(id: Id) -> Self {
                Self(id)
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

/// This function returns a "pseudo-random" ID for a type.
///
/// Although the ID is probably stable between compilations, this macro must only be used inside
/// function generating IDs. The reason being the macro takes into account the file and line
/// number where it was invoked from.
#[doc(hidden)]
#[macro_export]
macro_rules! id {
    ($t:ty) => {{
        use $crate::inventory::hash_str;

        let t_name = ::std::any::type_name::<$t>();
        let base = $crate::inventory::Id::new(hash_str(t_name));
        let crate_hash = hash_str(env!("CARGO_PKG_NAME"));
        let file_hash = hash_str(file!());
        let line_hash = line!() as u128;

        base.derive(crate_hash).derive(file_hash).derive(line_hash)
    }};
}

new_id!(TypeId);
new_id!(ConstantId);
new_id!(FunctionId);
new_id!(ServiceId);

#[doc(hidden)]
#[must_use]
pub const fn hash_str(s: &str) -> u128 {
    let bytes = s.as_bytes();
    let mut hash = 0xcbf29ce484222325u128;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u128;
        hash = hash.wrapping_mul(0x100000001b3u128);
        i += 1;
    }
    hash
}
