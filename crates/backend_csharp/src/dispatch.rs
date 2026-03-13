use crate::lang::functions::Function;
use crate::lang::meta::FileEmission;
use crate::lang::types::Type;
use crate::lang::{FunctionId, TypeId};
use crate::output::FileName;

/// A dispatch function that maps an item to a file name.
type DispatchFn = Box<dyn FnMut(Item, Meta) -> FileName>;

pub struct Dispatch {
    dispatch: DispatchFn,
}

impl Dispatch {
    pub fn custom(f: impl FnMut(Item, Meta) -> FileName + 'static) -> Self {
        Self { dispatch: Box::new(f) }
    }

    #[must_use]
    pub fn single_file() -> Self {
        Self::custom(|_, _| FileName::new("Interop.cs"))
    }

    pub fn classify(&mut self, item: Item) -> FileName {
        (self.dispatch)(item, Meta {})
    }
}

impl Default for Dispatch {
    fn default() -> Self {
        Self::single_file()
    }
}

pub enum ItemKind {
    Type(TypeId, Type),
    Function(FunctionId, Function),
}

pub struct Meta {}

pub struct Item {
    pub kind: ItemKind,
    pub emission: FileEmission,
}
