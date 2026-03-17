//! Wire output passes for `Wire<T>` C# code generation.
//!
//! Split into focused submodules:
//! - [`codegen`] — Shared C# code generation logic (type mapping, serialize/deserialize/size emission)
//! - [`wire_type`] — Renders `WireOf*` structs for each `Wire<T>` pattern
//! - [`helper_classes`] — Emits managed classes for nested structs with `WireOnly` fields
//! - [`all`] — Assembles wire_type and helper_classes results per output file

pub mod all;
pub mod codegen;
pub mod helper_classes;
pub mod wire_type;
