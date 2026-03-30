#![doc(hidden)]

use crate::Error;
use crate::lang::TypeId;
use std::cmp::PartialEq;

pub mod macros;
pub mod meta;
pub mod model;
pub mod output;

/// Whether generated code targets a Rust native library or a foreign plugin.
///
/// In `Rust` mode the generated C# code uses `[LibraryImport]` to call into
/// Rust functions (e.g., `WireInterop.interoptopus_wire_create`). In `Plugin`
/// mode those functions are not available via DLL import; instead the Rust host
/// registers them at load time through `Trampoline.RegisterTrampoline`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum OperationMode {
    /// Standard mode — C# calls a Rust native library via P/Invoke.
    #[default]
    Rust,
    /// Reverse-interop mode — Rust loads a foreign plugin and registers
    /// runtime trampolines (wire alloc/free, etc.) into the plugin.
    Plugin,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    Unchanged,
    Changed,
}

impl Outcome {
    pub fn changed(&mut self) {
        *self = Self::Changed;
    }
}

pub type ModelResult = Result<Outcome, Error>;
pub type OutputResult = Result<(), Error>;

#[derive(Debug, Copy, Clone)]
pub struct PassInfo {
    pub name: &'static str,
}

#[derive(Debug, Copy, Clone)]
pub enum MissingItem {
    CsType(TypeId),
    RustType(interoptopus::inventory::TypeId),
}

#[derive(Debug, Copy, Clone)]
pub struct Missing {
    pub origin: PassInfo,
    pub item: MissingItem,
}

/// Tracks items not found by passes.
#[derive(Debug, Clone, Default)]
pub struct LostAndFound {
    entries: Vec<Missing>,
}

impl LostAndFound {
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn missing(&mut self, origin: PassInfo, item: MissingItem) {
        self.entries.push(Missing { origin, item });
    }

    pub fn print(&self) {
        for missing in &self.entries {
            println!("Missing in {:?}: {:?}", missing.origin.name, missing.item);
        }
    }
}

/// Collects common pass and debug information.
#[derive(Debug, Clone, Default)]
pub struct PassMeta {
    pub lost_found: LostAndFound,
}

impl PassMeta {
    pub fn clear(&mut self) {
        self.lost_found.entries.clear();
    }
}

/// Formats doc lines as C# `///` documentation comments.
///
/// Returns an empty string if there are no doc lines.
pub fn format_docs(lines: &[String]) -> String {
    if lines.is_empty() {
        return String::new();
    }
    lines.iter().map(|line| format!("/// {line}")).collect::<Vec<_>>().join("\n")
}
