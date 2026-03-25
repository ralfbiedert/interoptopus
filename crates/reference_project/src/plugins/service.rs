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

    // async_basic_create(^) -> usize // knows Self is special usize service
    //

    impl AsyncBasic {
        fn create() -> Self;
        async fn call_void(&self);
        async fn add_one(&self, x: u32) -> u32;
        async fn wire_1(&self, x: Wire<HashMap<String, String>>) -> Wire<HashMap<String, String>>;
        async fn wire_2(&self, x: Wire<HashMap<String, String>>) -> ffi::Result<Wire<HashMap<String, String>>, Error>;
    }
});

interoptopus::plugin!(ServiceNested {
    // extern service AsyncBasic;

    fn create_a() -> NestedA;
    async fn create_a_async() -> NestedA;
    fn create_a_result() -> ffi::Result<NestedA, Error>;
    async fn create_a_async_result() -> ffi::Result<NestedA, Error>;

    impl NestedA {
        fn create() -> Self;
        fn create_result() -> ffi::Result<Self, Error>;
        async fn create_async() -> Self;
        async fn create_result_async() -> ffi::Result<Self, Error>;
        fn create_other(&self) -> NestedB;
        fn create_other_result(&self) -> ffi::Result<NestedB, Error>;
        async fn create_other_async(&self) -> NestedB;
        async fn create_other_result_async(&self) -> ffi::Result<NestedB, Error>;
    }

    impl NestedB {
        fn accept(&self, a: NestedA); // knows nested is other service!!!!
        async fn accept_async(&self, a: NestedA);
        fn accept_ref(&self, a: &NestedA);
        async fn accept_async_ref(&self, a: &NestedA);
    }
});
