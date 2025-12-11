
use crate::pattern::asynk::AsyncRuntime;
use std::sync::Arc;

#[derive(Clone)]
pub struct Tokio {
    rt: Arc<tokio::runtime::Runtime>,
}

impl Tokio {
    pub fn new() -> Self {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        Self { rt: Arc::new(rt) }
    }
}

impl AsyncRuntime for Tokio {
    type T = ();

    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(Self::T) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.rt.spawn(f(()));
    }
}