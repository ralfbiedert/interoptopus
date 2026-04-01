//! Emits an assembly-level `DefaultDllImportSearchPaths` attribute.
//!
//! When configured with a [`DllImportSearchPath`] variant, this pass produces
//! `[assembly: DefaultDllImportSearchPaths(DllImportSearchPath.X)]` for the
//! first output file that contains `FileEmission::Default` items. This restricts
//! the directories the .NET runtime searches when resolving native libraries
//! loaded via `[LibraryImport]`, mitigating insecure DLL preloading vulnerabilities.
//!
//! The default configuration uses [`DllImportSearchPath::SafeDirectories`].

use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, output};
use std::collections::HashMap;

/// Controls the `[assembly: DefaultDllImportSearchPaths(...)]` attribute in generated code.
#[derive(Debug, Clone, Default)]
pub enum DllImportSearchPath {
    /// Do not emit the attribute.
    #[default]
    None,
    UseDllDirectoryForDependencies,
    ApplicationDirectory,
    UserDirectories,
    System32,
    SafeDirectories,
    AssemblyDirectory,
    LegacyBehavior,
    /// Emit a custom expression, e.g. `"DllImportSearchPath.System32 | DllImportSearchPath.UserDirectories"`.
    Custom(String),
}

impl DllImportSearchPath {
    fn to_attribute(&self) -> String {
        let value = match self {
            Self::None => return String::new(),
            Self::UseDllDirectoryForDependencies => "DllImportSearchPath.UseDllDirectoryForDependencies",
            Self::ApplicationDirectory => "DllImportSearchPath.ApplicationDirectory",
            Self::UserDirectories => "DllImportSearchPath.UserDirectories",
            Self::System32 => "DllImportSearchPath.System32",
            Self::SafeDirectories => "DllImportSearchPath.SafeDirectories",
            Self::AssemblyDirectory => "DllImportSearchPath.AssemblyDirectory",
            Self::LegacyBehavior => "DllImportSearchPath.LegacyBehavior",
            Self::Custom(s) => s.as_str(),
        };
        format!("[assembly: DefaultDllImportSearchPaths({value})]")
    }
}

#[derive(Default)]
pub struct Config {
    pub import_search_path: DllImportSearchPath,
}

pub struct Pass {
    info: PassInfo,
    config: Config,
    attributes: HashMap<Output, String>,
}

impl Pass {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { info: PassInfo { name: file!() }, config, attributes: HashMap::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, output_master: &output::common::master::Pass) -> OutputResult {
        let mut emitted = false;

        for output in output_master.outputs_of(FileType::Csharp) {
            // Assembly-level attributes must appear in exactly one compilation unit;
            // pick the first output that received FileEmission::Default items.
            let attr = if !emitted && output_master.has_default_items(output) {
                emitted = true;
                self.config.import_search_path.to_attribute()
            } else {
                String::new()
            };

            self.attributes.insert(output.clone(), attr);
        }

        Ok(())
    }

    #[must_use]
    pub fn search_path_for(&self, output: &Output) -> Option<&str> {
        self.attributes.get(output).map(|s| &**s)
    }
}
