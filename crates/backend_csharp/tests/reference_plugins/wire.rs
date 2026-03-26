use crate::{define_plugin, load_plugin};
use interoptopus::wire::Wire;
use reference_project::plugins::wire::Wired;
use std::collections::HashMap;
use std::error::Error;

#[test]
fn define_plugin() -> Result<(), Box<dyn Error>> {
    define_plugin!(Wired, "wire.dll", super::BASE);
    Ok(())
}

#[test]
fn load_plugin() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Wired, "wire.dll", super::BASE);

    let hashmap = HashMap::from([("foo".to_string(), "bar".to_string())]);
    let result = plugin.wire_hashmap_string(Wire::from(hashmap)).unwire();
    assert_eq!(result.get("foo").map(String::as_str), Some("bar"));
    assert_eq!(result.get("hello").map(String::as_str), Some("world"));

    let result = plugin.wire_string(Wire::from("{}".to_string())).unwire();
    assert!(result.contains("hello"));
    assert!(result.contains("world"));

    Ok(())
}
