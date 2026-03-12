use interoptopus::{ffi, service};
use interoptopus_csharp::lang::types::kind::{TypeKind, TypePattern};
use std::collections::HashSet;

#[ffi]
pub enum Error {
    A,
}

#[ffi(service)]
pub struct ServiceA {}

#[ffi(service)]
pub struct ServiceB {}

#[ffi]
impl ServiceB {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }
}

#[ffi]
impl ServiceA {
    pub fn new() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }
}

#[test]
fn result_types_have_distinct_names() -> Result<(), Box<dyn std::error::Error>> {
    let plugin = debug_plugin!(|_inventory, models| {
        let result_names: Vec<&str> = models
            .types
            .iter()
            .filter(|(_, ty)| matches!(&ty.kind, TypeKind::TypePattern(TypePattern::Result(..))))
            .map(|(_, ty)| ty.name.as_str())
            .collect();

        let unique: HashSet<&str> = result_names.iter().copied().collect();

        assert_eq!(result_names.len(), 2, "expected 2 Result types, got {}: {:?}", result_names.len(), result_names);
        assert_eq!(result_names.len(), unique.len(), "Result type names are not unique: {:?}", result_names);
    });

    test_ffi!(plugin, [service!(ServiceA), service!(ServiceB)])?;

    Ok(())
}
