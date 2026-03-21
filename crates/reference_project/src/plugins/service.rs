use crate::patterns::result::Error;
use crate::types::basic::Vec3f32;
use interoptopus::ffi;

interoptopus::plugin!(Service {
    impl ServiceA {
        fn create() -> Self;
        fn call(&self, x: u32) -> u32;
        // fn get_accumulator(&self) -> i32;
        // fn wire(&self, x: Wire<String>) -> Wire<String>;
        // fn wire2(&self, x: Wire<HashMap<String, String>>) -> Wire<HashMap<String, String>>;
        // // TODO: fn call(&self, x: &u32, cb: CallBack);
        // // async fn call_async(&self, x: Wire<String>);
    }
});
