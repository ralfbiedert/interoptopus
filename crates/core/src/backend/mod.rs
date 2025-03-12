//! Components used by backends.

use crate::lang::c::Function;
use crate::patterns::LibraryPattern;
use crate::{Error, Inventory};
use std::fs::File;
use std::path::Path;
use writer::IndentWriter;

pub mod testing;
pub mod util;
pub mod writer;

/// Returns all functions not belonging to a [`service`](crate::patterns::service) pattern.
///
/// Useful in backends like Python that can fully encapsulate services and should not expose their
/// raw methods in the main namespace.
#[must_use]
pub fn non_service_functions(inventory: &Inventory) -> Vec<&Function> {
    let mut service_methods = vec![];
    for pattern in inventory.patterns() {
        match pattern {
            LibraryPattern::Service(service) => {
                service_methods.extend_from_slice(service.methods());
                service_methods.extend_from_slice(service.constructors());
                service_methods.push(service.destructor().clone());
            }
            LibraryPattern::Builtins(_) => {}
        }
    }

    inventory.functions().iter().filter(|&x| !service_methods.contains(x)).collect()
}

/// Create a single [`Inventory`] from a number of individual inventories.
///
/// This function can be useful when your FFI crate exports different sets of
/// symbols (e.g., _core_ and _extension_ functions) and you want to create different
/// bindings based on some compile target or configuration
///
/// # Example
///
/// ```
/// # mod my_crate {
/// #     use interoptopus::Inventory;
/// #     pub fn inventory_core() -> Inventory { Inventory::default() }
/// #     pub fn inventory_ext() -> Inventory { Inventory::default() }
/// # }
///
/// use interoptopus::backend::merge_inventories;
///
/// let inventories = [
///     my_crate::inventory_core(),
///     my_crate::inventory_ext()
/// ];
///
/// merge_inventories(&inventories);
/// ```
#[must_use]
pub fn merge_inventories(inventories: &[Inventory]) -> Inventory {
    let mut functions = Vec::new();
    let mut constants = Vec::new();
    let mut patterns = Vec::new();
    let mut types = Vec::new();

    for inventory in inventories {
        functions.extend_from_slice(inventory.functions());
        constants.extend_from_slice(inventory.constants());
        patterns.extend_from_slice(inventory.patterns());
        types.extend_from_slice(inventory.ctypes());
    }

    Inventory::new(functions, constants, patterns, types.as_slice())
}
