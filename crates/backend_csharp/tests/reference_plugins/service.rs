use crate::{define_plugin, load_plugin};
use reference_project::plugins::service::Service;
use std::error::Error;

#[test]
fn define_plugin() -> Result<(), Box<dyn Error>> {
    define_plugin!(Service, "service.dll");
    Ok(())
}

// Test ignored since we can't rely on a working .NET runtime being available on CI
#[ignore]
#[test]
fn load_plugin() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Service, "service.dll");

    let svc = plugin.servicea_create();
    assert_eq!(svc.call(5), 6);
    assert_eq!(svc.call(10), 11);

    Ok(())
}
