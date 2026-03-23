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

    #[cfg(any(feature = "unstable-plugins", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-plugins")))]
    pub fn plugin_defaults_with(name: impl Into<String>) -> Self {
        use crate::lang::plugin::PLUGIN_DEFAULT_MODULE;
        let name = name.into();
        let name_common = format!("{name}.Common");
        Self::custom(move |x, _| match x.emission {
            FileEmission::Common => Target::new("Interop.Common.cs", name_common.clone()),
            FileEmission::Default => Target::new("Interop.User.cs", name.clone()),
            FileEmission::CustomModule(ref m) if *m == PLUGIN_DEFAULT_MODULE => Target::new("Interop.Plugin.cs", "Interoptopus.API"),
            FileEmission::CustomModule(_) => Target::new("Interop.User.cs", name.clone()),
        })
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
    /// A plugin or service interface (e.g. `IPlugin`, `IFoo<TSelf>`).
    PluginInterface,
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
