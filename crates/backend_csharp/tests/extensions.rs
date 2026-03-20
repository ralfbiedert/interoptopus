use interoptopus::inventory::RustInventory;
use interoptopus_backends::output::Multibuf;
use interoptopus_csharp::RustLibrary;
use interoptopus_csharp::pass::Outcome::Unchanged;
use interoptopus_csharp::pass::{ModelResult, OutputResult};
use interoptopus_csharp::extensions::{PostModelPass, PostOutputPass, RustCodegenExtension};
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
#[allow(clippy::struct_field_names)]
pub struct MyExtension {
    init_called: Arc<AtomicBool>,
    post_model_called: Arc<AtomicBool>,
    post_output_called: Arc<AtomicBool>,
}

impl RustCodegenExtension for MyExtension {
    fn init(&mut self, _: &mut RustInventory) {
        self.init_called.store(true, Ordering::Relaxed);
    }

    fn post_model_cycle(&mut self, _: &RustInventory, _: PostModelPass) -> ModelResult {
        self.post_model_called.store(true, Ordering::Relaxed);
        Ok(Unchanged)
    }

    fn post_model_all(&mut self, _: &RustInventory, _: PostModelPass) -> Result<(), interoptopus_csharp::Error> {
        Ok(())
    }

    fn post_output(&mut self, _: &mut Multibuf, _: PostOutputPass) -> OutputResult {
        self.post_output_called.store(true, Ordering::Relaxed);
        Ok(())
    }
}

#[test]
fn can_register() -> Result<(), Box<dyn Error>> {
    let inventory = RustInventory::new();
    let init_called = Arc::new(AtomicBool::new(false));
    let post_model_called = Arc::new(AtomicBool::new(false));
    let post_output_called = Arc::new(AtomicBool::new(false));
    let ext = MyExtension { init_called: init_called.clone(), post_model_called: post_model_called.clone(), post_output_called: post_output_called.clone() };

    let _ = RustLibrary::new(inventory).register_extension(ext).process()?;

    assert!(init_called.load(Ordering::Relaxed));
    assert!(post_model_called.load(Ordering::Relaxed));
    assert!(post_output_called.load(Ordering::Relaxed));

    Ok(())
}
