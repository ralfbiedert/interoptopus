use crate::{define_plugin, load_plugin};
use interoptopus::wire::Wire;
use reference_project::plugins::service::{ServiceAsync, ServiceAsyncCancel, ServiceBasic, ServiceNested};
use std::collections::HashMap;
use std::error::Error;

#[test]
fn define_plugins() -> Result<(), Box<dyn Error>> {
    define_plugin!(ServiceBasic, "service_basic.dll", super::BASE);
    define_plugin!(ServiceAsync, "service_async.dll", super::BASE);
    define_plugin!(ServiceAsyncCancel, "service_async_cancel.dll", super::BASE);
    define_plugin!(ServiceNested, "service_nested.dll", super::BASE);
    Ok(())
}

#[test]
fn load_plugin_service_basic() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceBasic, "service_basic.dll", super::BASE);

    let svc = plugin.service_a_create();
    assert_eq!(svc.call(5), 6);
    assert_eq!(svc.call(10), 11);

    Ok(())
}

#[test]
fn load_plugin_service_nested() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceNested, "service_nested.dll", super::BASE);

    // Test bare fn creating a service
    for i in 0..100u32 {
        let a = plugin.create_a(i);
        assert_eq!(a.get_value(), i, "create_a({i}) should have value {i}");
        assert_eq!(a.add(10), i + 10, "add(10) on value {i}");
    }

    // Test bare fn creating a service in a result
    for i in 0..100u32 {
        let a = plugin.create_a_result(i).unwrap();
        assert_eq!(a.get_value(), i, "create_a({i}) should have value {i}");
        assert_eq!(a.add(10), i + 10, "add(10) on value {i}");
    }

    // Test service ctor
    for i in 0..100u32 {
        let a = plugin.nested_a_create(i * 3);
        assert_eq!(a.get_value(), i * 3);
        assert_eq!(a.add(7), i * 3 + 7);
    }

    // Test nested service creation: A creates B, B inherits A's value
    for i in 0..100u32 {
        let a = plugin.nested_a_create(i);
        let b = a.create_other();
        assert_eq!(b.get_value(), i, "create_other should inherit A's value {i}");
        assert_eq!(b.add(5), i + 5);

        // create_other_with adds extra
        let b2 = a.create_other_with(100);
        assert_eq!(b2.get_value(), i + 100);
    }

    // Test B accepting A (ownership transfer) — returns sum of values
    for i in 0..50u32 {
        let a = plugin.nested_a_create(i);
        let b_val = i * 2;
        let a_for_b = plugin.nested_a_create(b_val);
        let b = a_for_b.create_other();
        assert_eq!(b.get_value(), b_val);

        let sum = b.accept(a);
        assert_eq!(sum, b_val + i, "accept should return {b_val} + {i}");
    }

    // Test B accepting A by ref — returns sum, A stays alive
    for i in 0..50u32 {
        let a = plugin.nested_a_create(i);
        let b = plugin.nested_a_create(i + 10).create_other();

        let sum = b.accept_ref(&a);
        assert_eq!(sum, (i + 10) + i);

        // a is still alive, can still be used
        assert_eq!(a.get_value(), i);
    }

    // Mixed: create many, interleave operations
    let mut services_a = Vec::new();
    for i in 0..100u32 {
        services_a.push(plugin.nested_a_create(i));
    }
    for (i, a) in services_a.iter().enumerate() {
        assert_eq!(a.get_value(), i as u32);
        assert_eq!(a.add(1), i as u32 + 1);
    }
    // Create B's from each A
    let services_b: Vec<_> = services_a.iter().map(|a| a.create_other()).collect();
    for (i, b) in services_b.iter().enumerate() {
        assert_eq!(b.get_value(), i as u32);
    }
    // Drop all
    drop(services_b);
    drop(services_a);

    Ok(())
}

// Test cancellation of a C# plugin async task by dropping the TaskHandle from Rust.
#[tokio::test]
async fn load_plugin_service_async_cancel() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceAsyncCancel, "service_async_cancel.dll", super::BASE);

    // Bare async fn: run_long loops forever, incrementing a counter.
    // Dropping the future (and thus the TaskHandle) should cancel the C# task.
    {
        let fut = plugin.run_long();
        // Let the C# side run a few iterations
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        let counter_before = plugin.get();
        assert!(counter_before > 0, "counter should have incremented");

        // Drop the future — this drops the TaskHandle, which cancels the CTS
        drop(fut);
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        let counter_after = plugin.get();

        // Counter should have stopped (or at most +1 from in-flight iteration)
        assert!(counter_after <= counter_before + 1);
    }

    // Service async method: same pattern via service instance.
    {
        let svc = plugin.async_cancellation_create().await;
        let fut = svc.run_long();
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        let counter_before = svc.get();
        assert!(counter_before > 0, "service counter should have incremented");

        drop(fut);
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        let counter_after = svc.get();

        assert!(counter_after <= counter_before + 1);
    }

    Ok(())
}

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[tokio::test]
async fn load_plugin_service_async() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(ServiceAsync, "service_async.dll", super::BASE);

    plugin.call_void().await;
    let i = plugin.add_one(1).await;
    assert_eq!(i, 2);

    let hashmap = HashMap::from([("foo".to_string(), "bar".to_string())]);
    let mut result = plugin.wire_1(Wire::from(hashmap.clone())).await;
    let map = result.unwire();
    assert_eq!(map.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(map.get("hello").map(String::as_str), Some("world"));

    let result = plugin.wire_2(Wire::from(hashmap.clone())).await;
    let map = result.unwrap().unwire();
    assert_eq!(map.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(map.get("hello").map(String::as_str), Some("world"));

    let svc = plugin.async_basic_create();
    svc.call_void().await;
    let i = svc.add_one(1).await;
    assert_eq!(i, 2);

    let mut result = svc.wire_1(Wire::from(hashmap.clone())).await;
    let map = result.unwire();
    assert_eq!(map.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(map.get("hello").map(String::as_str), Some("world"));

    let result = svc.wire_2(Wire::from(hashmap)).await;
    let map = result.unwrap().unwire();
    assert_eq!(map.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(map.get("hello").map(String::as_str), Some("world"));

    Ok(())
}
