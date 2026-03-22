use crate::{define_plugin, load_plugin};
use reference_project::plugins::service::{ServiceAsync, ServiceBasic};
use std::error::Error;

#[test]
fn define_plugins() -> Result<(), Box<dyn Error>> {
    define_plugin!(ServiceBasic, "service_basic.dll");
    define_plugin!(ServiceAsync, "service_async.dll");
    Ok(())
}

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[ignore]
#[test]
fn load_plugin_service_basic() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceBasic, "service_basic.dll");

    let svc = plugin.servicea_create();
    assert_eq!(svc.call(5), 6);
    assert_eq!(svc.call(10), 11);

    Ok(())
}

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[ignore]
#[tokio::test]
async fn load_plugin_service_async() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceAsync, "service_async.dll");

    plugin.call_void().await;
    let i = plugin.add_one(1).await;
    assert_eq!(i, 2);

    let svc = plugin.asyncbasic_create();
    svc.call_void().await;
    let i = svc.add_one(1).await;
    assert_eq!(i, 2);

    Ok(())
}
