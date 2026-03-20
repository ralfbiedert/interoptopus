//! Extension point for customizing the C# code generation pipeline.

use crate::Error;
use crate::pass::{ModelResult, Outcome, OutputResult, model};
use crate::pipeline::ModelPasses;
use interoptopus::inventory::RustInventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

/// Mutable references to model passes, provided to plugins after each model cycle.
#[derive(Debug)]
pub struct PostModelPass<'a> {
    /// The Rust→C# ID mappings.
    pub id_map: &'a mut model::common::id_map::Pass,
    /// All resolved C# types.
    pub types: &'a mut model::common::types::all::Pass,
    /// All resolved C# functions.
    pub fns: &'a mut model::common::fns::all::Pass,
}

impl<'a> PostModelPass<'a> {
    pub(crate) fn from_model(m: &'a mut ModelPasses) -> Self {
        Self { id_map: &mut m.id_maps, types: &mut m.type_all, fns: &mut m.fns_all }
    }
}

/// References to output passes, provided to plugins after output rendering.
#[derive(Copy, Clone, Default)]
pub struct PostOutputPass<'a> {
    _todo: PhantomData<&'a ()>,
}

/// Trait for plugins that hook into the C# code generation pipeline.
///
/// Implement any subset of the methods to customize behavior at different stages.
pub trait RustLibraryPlugin {
    /// Called once before any passes run. Can modify the inventory.
    fn init(&mut self, inventory: &mut RustInventory) {}

    /// Called after each model pass iteration. Return [`Outcome::Changed`] to request
    /// another iteration.
    fn post_model_cycle(&mut self, inventory: &RustInventory, models: PostModelPass) -> ModelResult {
        Ok(Outcome::Unchanged)
    }

    /// Called once after the model pass loop has converged.
    fn post_model_all(&mut self, inventory: &RustInventory, models: PostModelPass) -> Result<(), Error> {
        Ok(())
    }

    /// Called after all output passes, allowing modifications to the final [`Multibuf`].
    fn post_output(&mut self, multibuf: &mut Multibuf, outputs: PostOutputPass) -> OutputResult {
        Ok(())
    }
}
