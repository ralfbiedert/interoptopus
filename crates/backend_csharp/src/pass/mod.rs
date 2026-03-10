use crate::lang::TypeId;
use crate::Error;
use std::cmp::PartialEq;

pub mod macros;
pub mod meta;
pub mod model;
pub mod output;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Outcome {
    Unchanged,
    Changed,
}

impl Outcome {
    pub fn changed(&mut self) {
        *self = Outcome::Changed;
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
