mod csharp;
mod rust;

use crate::Error;
use crate::pass::{ModelResult, Outcome};
pub use csharp::{CsLibrary, CsLibraryConfig};
pub use rust::{IntermediateOutputPasses, RustLibrary, RustLibraryBuilder, RustLibraryConfig};

pub struct PassRunner {
    outcome: Outcome,
}

impl PassRunner {
    pub fn new() -> Self {
        Self { outcome: Outcome::Unchanged }
    }

    pub fn run(&mut self, x: ModelResult) -> Result<(), Error> {
        match x {
            Ok(Outcome::Changed) => self.outcome = Outcome::Changed,
            Ok(Outcome::Unchanged) => {}
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

pub fn loop_model_passes_until_done(f: impl FnMut(&mut PassRunner) -> Result<(), Error>) -> Result<(), Error> {
    loop {
        let mut pass_runner = PassRunner::new();
        f(&mut pass_runner)?;

        if pass_runner.outcome == Outcome::Unchanged {
            break;
        }
    }
    Ok(())
}
