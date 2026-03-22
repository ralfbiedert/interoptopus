use crate::patterns::result::Error;
use interoptopus::ffi;
use interoptopus::wire::Wire;
use std::collections::HashMap;

interoptopus::plugin!(ServiceBasic {
    impl ServiceA {
        fn create() -> Self;
        fn call(&self, x: u32) -> u32;
    }
});

interoptopus::plugin!(ServiceAsync {
    async fn call_void();
    async fn add_one(x: u32) -> u32;
    async fn wire_1(x: Wire<HashMap<String, String>>) -> Wire<HashMap<String, String>>;
    async fn wire_2(x: Wire<HashMap<String, String>>) -> ffi::Result<Wire<HashMap<String, String>>, Error>;

    impl AsyncBasic {
        fn create() -> Self;
        async fn call_void(&self);
        async fn add_one(&self, x: u32) -> u32;
        async fn wire_1(&self, x: Wire<HashMap<String, String>>) -> Wire<HashMap<String, String>>;
        async fn wire_2(&self, x: Wire<HashMap<String, String>>) -> ffi::Result<Wire<HashMap<String, String>>, Error>;
    }
});
