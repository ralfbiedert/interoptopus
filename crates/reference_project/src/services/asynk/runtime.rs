use interoptopus::AsyncRuntime;

#[derive(AsyncRuntime)]
pub struct ServiceRuntime {
    #[runtime(forward)]
    runtime: interoptopus::rt::Tokio,
}
