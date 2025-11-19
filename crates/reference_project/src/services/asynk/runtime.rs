use interoptopus::AsyncRuntime;

mod rt {
    use interoptopus::pattern::asynk::AsyncRuntime;
    use std::future::Future;

    pub struct Runtime;

    impl AsyncRuntime for Runtime {
        type T = ();

        fn spawn<Fn, F>(&self, _f: Fn)
        where
            Fn: FnOnce(()) -> F,
            F: Future<Output = ()> + Send + 'static,
        {
        }
    }
}

#[derive(AsyncRuntime)]
pub struct ServiceRuntime {
    #[runtime(forward)]
    runtime: rt::Runtime,
}
