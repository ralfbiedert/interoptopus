use backend_csharp_ng::RustLibrary;
use backend_csharp_ng::plugin::{PostModelPass, PostOutputPass, RustLibraryPlugin};
use interoptopus::inventory::Inventory;
use interoptopus_backends::output::Multibuf;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct MyPlugin {
    init_called: Arc<AtomicBool>,
    post_model_called: Arc<AtomicBool>,
    post_output_called: Arc<AtomicBool>,
}

impl RustLibraryPlugin for MyPlugin {
    fn init(&mut self, _: &mut Inventory) {
        self.init_called.store(true, Ordering::Relaxed);
    }

    fn post_model(&mut self, _: &Inventory, _: PostModelPass) {
        self.post_model_called.store(true, Ordering::Relaxed);
    }

    fn post_output(&mut self, _: &mut Multibuf, _: PostOutputPass) {
        self.post_output_called.store(true, Ordering::Relaxed);
    }
}

#[test]
fn can_register() -> Result<(), Box<dyn Error>> {
    let inventory = Inventory::new();
    let init_called = Arc::new(AtomicBool::new(false));
    let post_model_called = Arc::new(AtomicBool::new(false));
    let post_output_called = Arc::new(AtomicBool::new(false));
    let plugin = MyPlugin { init_called: init_called.clone(), post_model_called: post_model_called.clone(), post_output_called: post_output_called.clone() };

    let _ = RustLibrary::new(inventory).register_plugin(plugin).process()?;

    assert!(init_called.load(Ordering::Relaxed));
    assert!(post_model_called.load(Ordering::Relaxed));
    assert!(post_output_called.load(Ordering::Relaxed));

    Ok(())
}
