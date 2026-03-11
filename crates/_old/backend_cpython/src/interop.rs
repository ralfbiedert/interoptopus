pub mod bootstrap;
pub mod callbacks;
pub mod constants;
pub mod functions;
pub mod imports;
pub mod patterns;
pub mod types;
pub mod utils;

use crate::converter::to_type_hint_in;
use crate::interop::bootstrap::write_api_load_function;
use crate::interop::callbacks::write_callback_helpers;
use crate::interop::constants::write_constants;
use crate::interop::functions::write_function_proxies;
use crate::interop::imports::write_imports;
use crate::interop::patterns::write_patterns;
use crate::interop::types::write_types;
use crate::interop::utils::write_utils;
use derive_builder::Builder;
use interoptopus::inventory::Inventory;
use interoptopus::lang::Function;
use interoptopus_backend_utils::{Error, IndentWriter, indented};
use std::fs::File;
use std::path::Path;

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
    /// Creates a new [`InteropBuilder`].
    #[must_use]
    pub fn builder() -> InteropBuilder {
        InteropBuilder::new()
    }

    fn debug(&self, w: &mut IndentWriter, marker: &str) -> Result<(), Error> {
        if !self.debug {
            return Ok(());
        }

        indented!(w, r"# Debug - {} ", marker)?;

        Ok(())
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

    /// Generates FFI binding code and writes them to the [`IndentWriter`].
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        write_imports(self, w)?;
        write_api_load_function(self, w)?;
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

    /// Convenience method to write FFI bindings to the specified file with default indentation.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn write_file<P: AsRef<Path>>(&self, file_name: P) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        let mut writer = IndentWriter::new(&mut file);

        self.write_to(&mut writer)
    }

    /// Convenience method to write FFI bindings to a string.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn to_string(&self) -> Result<String, Error> {
        let mut vec = Vec::new();
        let mut writer = IndentWriter::new(&mut vec);
        self.write_to(&mut writer)?;
        Ok(String::from_utf8(vec)?)
    }
}

impl InteropBuilder {
    /// Creates a new builder instance, **start here**.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
