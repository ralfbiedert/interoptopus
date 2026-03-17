//! FFI functions, arguments, and signatures.

use crate::inventory::{FunctionId, Inventory, TypeId};
use crate::lang::meta::{Docs, Emission, Visibility};

/// Implemented by companion types generated for `#[ffi]` functions.
///
/// You do not implement this manually — the `#[ffi]` attribute on a `fn` item
/// generates a zero-sized struct that implements this trait.
pub trait FunctionInfo {
    /// The unique identifier for this function.
    fn id() -> FunctionId;
    /// Returns the function's call signature.
    fn signature() -> Signature;
    /// Returns the full function description.
    fn function() -> Function;
    /// Registers this function (and all referenced types) with the given inventory.
    fn register(inventory: &mut impl Inventory);
}

/// A single named parameter of a function.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Argument {
    /// The parameter name.
    pub name: String,
    /// The parameter's type.
    pub ty: TypeId,
}

impl Argument {
    pub fn new(name: impl AsRef<str>, ty: TypeId) -> Self {
        Self { name: name.as_ref().to_string(), ty }
    }
}

/// The arguments and return type of a function or function pointer.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature {
    /// Ordered list of parameters.
    pub arguments: Vec<Argument>,
    /// The return type (unit `()` when the function returns nothing).
    pub rval: TypeId,
}

/// A named function exported across the FFI boundary.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function {
    /// The exported symbol name.
    pub name: String,
    /// Whether the function is public or private.
    pub visibility: Visibility,
    /// Documentation extracted from `///` comments.
    pub docs: Docs,
    /// Where the function definition should be placed.
    pub emission: Emission,
    /// The function's call signature.
    pub signature: Signature,
}
