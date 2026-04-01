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

interoptopus::plugin!(ServiceAsyncCancel {
    async fn run_long();
    async fn run_long_value(x: u32) -> u32;
    fn get() -> u32;

    impl AsyncCancellation {
        async fn create() -> Self;
        async fn run_long(&self);
        async fn run_long_value(&self, x: u32) -> u32;
        fn get(&self) -> u32;
        // async fn run_long();
        // async fn run_long_value(x: u32) -> u32;
    }
});

interoptopus::plugin!(ServiceNested {
    fn create_a(value: u32) -> NestedA;
    fn create_a_result(value: u32) -> ffi::Result<NestedA, Error>;

    impl NestedA {
        fn create(value: u32) -> Self;
        fn get_value(&self) -> u32;
        fn add(&self, x: u32) -> u32;
        fn create_other(&self) -> NestedB;
        fn create_other_with(&self, extra: u32) -> NestedB;
    }

    impl NestedB {
        fn get_value(&self) -> u32;
        fn add(&self, x: u32) -> u32;
        fn accept(&self, a: NestedA) -> u32;
        fn accept_ref(&self, a: &NestedA) -> u32;
    }
});
