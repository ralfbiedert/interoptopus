use crate::{define_plugin, load_plugin};
use interoptopus::lang::plugin::PluginInfo;
use interoptopus_csharp::dispatch::Dispatch;
use interoptopus_csharp::rt::Try;
use interoptopus_csharp::DotnetLibrary;
use reference_project::types::arrays::{Array, NestedArray};
use reference_project::types::basic::Vec3f32;
use reference_project::types::enums::{EnumPayload, EnumRenamedXYZ};
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
    }
});

#[test]
fn build_plugin() -> Result<(), Box<dyn std::error::Error>> {
    define_plugin!(ServiceTry, "exceptions.dll", super::BASE);
    Ok(())
}

#[test]
fn load_plugin() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceTry, "exceptions.dll", super::BASE);

    let a = plugin.create_a(0).unwrap();
    let b = plugin.get_value().unwrap();
    // let c = plugin.ne
    let x = plugin.nesteda_create(13);
    Ok(())
}
