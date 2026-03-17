//! Routes FFI items to output files based on customizable rules.

use crate::lang::functions::Function;
use crate::lang::meta::FileEmission;
use crate::lang::types::Type;
use crate::lang::{FunctionId, TypeId};
use crate::output::Target;

/// A dispatch function that maps an item to a file name.
type DispatchFn = Box<dyn FnMut(Item, Meta) -> Target>;

/// Determines which output file each FFI item is placed in.
pub struct Dispatch {
    dispatch: DispatchFn,
}

impl Dispatch {
    /// Creates a dispatcher with a custom routing function.
    pub fn custom(f: impl FnMut(Item, Meta) -> Target + 'static) -> Self {
        Self { dispatch: Box::new(f) }
    }

    /// Creates a dispatcher that puts everything into a single `Interop.cs` file.
    #[must_use]
    pub fn single_file() -> Self {
        Self::custom(|_, _| Target::new("Interop.cs", "My.Company"))
    }

    /// Routes the given item to an output file.
    pub fn classify(&mut self, item: Item) -> Target {
        (self.dispatch)(item, Meta {})
    }
}

impl Default for Dispatch {
    fn default() -> Self {
        Self::single_file()
    }
}

/// The kind of FFI item being dispatched.
pub enum ItemKind {
    /// A type definition.
    Type(TypeId, Type),
    /// A function declaration.
    Function(FunctionId, Function),
}

/// Reserved for future dispatch metadata.
pub struct Meta {}

/// An FFI item to be routed to an output file.
pub struct Item {
    /// What kind of item this is.
    pub kind: ItemKind,
    /// Where the item's source definition requested placement.
    pub emission: FileEmission,
}
