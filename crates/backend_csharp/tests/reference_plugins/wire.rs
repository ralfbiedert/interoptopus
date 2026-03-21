use crate::{define_plugin, load_plugin};
use interoptopus::wire::Wire;
use reference_project::plugins::service::Service;
use reference_project::plugins::wire::Wired;
use std::collections::HashMap;
use std::error::Error;

#[test]
fn define_plugin() -> Result<(), Box<dyn Error>> {
    define_plugin!(Wired, "wire.dll");
    Ok(())
}

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[ignore]
#[test]
fn load_plugin() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Wired, "wire.dll");

    let hashmap = HashMap::from([("foo".to_string(), "bar".to_string())]);
    let result = plugin.wire_hashmap_string(Wire::from(hashmap)).unwire();

    assert_eq!(result.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(result.get("hello").map(String::as_str), Some("world"));

    Ok(())
}
