use crate::{define_plugin, load_plugin};
use interoptopus::ffi;
use interoptopus::wire::Wire;
use reference_project::plugins::service::{ServiceAsync, ServiceBasic, ServiceNested};
use std::collections::HashMap;
use std::error::Error;

#[test]
fn define_plugins() -> Result<(), Box<dyn Error>> {
    define_plugin!(ServiceBasic, "service_basic.dll");
    define_plugin!(ServiceAsync, "service_async.dll");
    define_plugin!(ServiceNested, "service_nested.dll");
    Ok(())
}

#[test]
fn load_plugin_service_basic() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceBasic, "service_basic.dll");

    let svc = plugin.servicea_create();
    assert_eq!(svc.call(5), 6);
    assert_eq!(svc.call(10), 11);

    Ok(())
}

#[test]
fn load_plugin_service_nested() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceNested, "service_nested.dll");

    // Bare fn returning service
    let a = plugin.create_a();
    // Service ctor
    let a2 = plugin.nesteda_create();
    // Service method returning another service
    let b = a2.create_other();
    // Service accepting another service (ownership transfer)
    b.accept(a);
    // Drop services
    drop(b);
    drop(a2);

    Ok(())
}

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[tokio::test]
async fn load_plugin_service_async() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceAsync, "service_async.dll");

    plugin.call_void().await;
    let i = plugin.add_one(1).await;
    assert_eq!(i, 2);

    let hashmap = HashMap::from([("foo".to_string(), "bar".to_string())]);
    let mut result = plugin.wire_1(Wire::from(hashmap.clone())).await;
    let map = result.unwire();
    assert_eq!(map.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(map.get("hello").map(String::as_str), Some("world"));

    let mut result = plugin.wire_2(Wire::from(hashmap.clone())).await;
    let map = result.unwrap().unwire();
    assert_eq!(map.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(map.get("hello").map(String::as_str), Some("world"));

    let svc = plugin.asyncbasic_create();
    svc.call_void().await;
    let i = svc.add_one(1).await;
    assert_eq!(i, 2);

    let mut result = svc.wire_1(Wire::from(hashmap.clone())).await;
    let map = result.unwire();
    assert_eq!(map.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(map.get("hello").map(String::as_str), Some("world"));

    let mut result = svc.wire_2(Wire::from(hashmap)).await;
    let map = result.unwrap().unwire();
    assert_eq!(map.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(map.get("hello").map(String::as_str), Some("world"));

    Ok(())
}
