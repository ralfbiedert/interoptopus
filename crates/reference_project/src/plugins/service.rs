use interoptopus::pattern::asynk::{AsyncCallback, AsyncCallbackFuture};

interoptopus::plugin!(ServiceBasic {
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

interoptopus::plugin!(ServiceAsync {
    impl AsyncBasic {
        fn create() -> Self;
        fn raw(&self, x: u32, cb: AsyncCallback<u32>);
    }
});

// impl AsyncBasic {
//     pub fn raw(&self, x: u32, cb: AsyncCallback<u32>) {
//         (self.asyncbasic_raw)(self.handle, x, cb)
//     }
//
//     pub fn raw2(&self, x: u32) -> impl Future<Output = u32> {
//         let (future, cb) = AsyncCallbackFuture::new();
//         self.raw(x, cb);
//         future
//     }
// }
