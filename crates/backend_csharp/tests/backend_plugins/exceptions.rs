use crate::{define_plugin, load_plugin};
use interoptopus_csharp::dispatch::Dispatch;
use interoptopus_csharp::pattern::{Try, TryExtension};
use std::error::Error;

interoptopus::plugin!(ServiceTry {
    fn create_a(value: u32) -> Try<NestedA>;
    fn get_value() -> Try<u32>;
    async fn create_a_async(value: u32) -> Try<NestedA>;
    async fn get_value_async() -> Try<u32>;

    impl NestedA {
        fn create(value: u32) -> Try<Self>;
        fn get_value(&self) -> Try<u32>;
        async fn create_async(value: u32) -> Try<Self>;

        async fn get_value_async(&self) -> Try<u32>;
        // #[allow_untry(reason="asdasd")]
        // async fn get_value_async_3(&self) -> u32;
    }
});

#[test]
fn build_plugin() -> Result<(), Box<dyn Error>> {
    define_plugin!(ServiceTry, "exceptions.dll", super::BASE);
    Ok(())
}

#[test]
fn load_plugin() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceTry, "exceptions.dll", super::BASE);

    _ = plugin.get_value();
    //
    let _ = plugin.create_a(0).ok()?;
    let _ = plugin.get_value().ok()?;
    let _ = plugin.nested_a_create(13).ok()?;
    Ok(())
}
