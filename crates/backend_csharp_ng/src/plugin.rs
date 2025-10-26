use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

// TODO: We want to be careful what exactly we expose to plugins, and how, to not leak impl details.

#[derive(Copy, Clone, Default)]
pub struct PostModelPass<'a> {
    // TODO: will contain &mut to various model data?!
    _todo: PhantomData<&'a ()>,
}

#[derive(Copy, Clone, Default)]
pub struct PostOutputPass<'a> {
    // TODO: will contain & to various outputs?!
    _todo: PhantomData<&'a ()>,
}

pub trait RustLibraryPlugin {
    fn init(&mut self, inventory: &mut Inventory);
    fn post_model(&mut self, inventory: &Inventory, models: PostModelPass);
    fn post_output(&mut self, multibuf: &mut Multibuf, outputs: PostOutputPass);
}
