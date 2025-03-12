pub mod bootstrap;
pub mod callbacks;
pub mod constants;
pub mod functions;
pub mod imports;
pub mod patterns;
pub mod types;
pub mod utils;

use crate::converter::to_type_hint_in;
use crate::interop::bootstrap::write_api_load_fuction;
use crate::interop::callbacks::write_callback_helpers;
use crate::interop::constants::write_constants;
use crate::interop::functions::write_function_proxies;
use crate::interop::imports::write_imports;
use crate::interop::patterns::write_patterns;
use crate::interop::types::write_types;
use crate::interop::utils::write_utils;
use derive_builder::Builder;
use interoptopus::backend::writer::IndentWriter;
use interoptopus::lang::c::Function;
use interoptopus::{Bindings, Error, Inventory, indented};

/// Generates Python `ctypes` files, **get this with [`InteropBuilder`]**.🐙
#[derive(Clone, Debug, Default, Builder)]
#[builder(default)]
pub struct Interop {
    /// Namespace for callback helpers, e.g., `callbacks`.
    #[builder(default = "\"callbacks\".to_string()")]
    callback_namespace: String,
    debug: bool,
    pub(crate) inventory: Inventory,
}

#[allow(clippy::unused_self)]
impl Interop {
    fn debug(&self, w: &mut IndentWriter, marker: &str) -> Result<(), Error> {
        if !self.debug {
            return Ok(());
        }

        indented!(w, r"# Debug - {} ", marker)
    }

    #[must_use]
    fn function_args_to_string(&self, function: &Function, type_hints: bool, skip_first: bool) -> String {
        let skip = usize::from(skip_first);
        function
            .signature()
            .params()
            .iter()
            .skip(skip)
            .map(|x| {
                let type_hint = if type_hints { to_type_hint_in(x.the_type(), true) } else { String::new() };
                format!("{}{}", x.name(), type_hint)
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    #[must_use]
    fn get_method_args(&self, function: &Function, ctx: &str) -> String {
        let mut args = self.function_args_to_string(function, false, true);
        args.insert_str(0, &format!("{ctx}, "));
        args
    }

    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        write_imports(self, w)?;
        write_api_load_fuction(self, w)?;
        w.newline()?;
        w.newline()?;

        write_function_proxies(self, w)?;
        w.newline()?;
        w.newline()?;

        write_constants(self, w)?;
        w.newline()?;
        w.newline()?;

        write_utils(self, w)?;
        write_types(self, w)?;
        w.newline()?;
        w.newline()?;

        write_callback_helpers(self, w)?;
        w.newline()?;
        w.newline()?;

        write_patterns(self, w)?;

        Ok(())
    }
}

impl Bindings for Interop {
    fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        self.write_to(w)
    }
}

impl InteropBuilder {
    /// Creates a new builder instance, **start here**.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
