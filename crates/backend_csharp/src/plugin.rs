use crate::pass::{model, ModelResult, Outcome, OutputResult};
use crate::pipeline::ModelPasses;
use crate::Error;
use interoptopus::inventory::RustInventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct PostModelPass<'a> {
    pub id_map: &'a mut model::id_map::Pass,
    pub types: &'a mut model::types::all::Pass,
    pub fns: &'a mut model::fns::all::Pass,
}

impl<'a> PostModelPass<'a> {
    pub(crate) fn from_model(m: &'a mut ModelPasses) -> Self {
        Self { id_map: &mut m.id_maps, types: &mut m.type_all, fns: &mut m.fn_all }
    }
}

#[derive(Copy, Clone, Default)]
pub struct PostOutputPass<'a> {
    // TODO: will contain & to various outputs?!
    _todo: PhantomData<&'a ()>,
}

pub trait RustLibraryPlugin {
    fn init(&mut self, inventory: &mut RustInventory) {}

    fn post_model_cycle(&mut self, inventory: &RustInventory, models: PostModelPass) -> ModelResult {
        Ok(Outcome::Unchanged)
    }

    fn post_model_all(&mut self, inventory: &RustInventory, models: PostModelPass) -> Result<(), Error> {
        Ok(())
    }

    fn post_output(&mut self, multibuf: &mut Multibuf, outputs: PostOutputPass) -> OutputResult {
        Ok(())
    }
}
