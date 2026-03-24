mod csharp;
#[cfg(any(feature = "unstable-plugins", docsrs))]
mod dotnet;
mod rust;

use crate::Error;
use crate::pass::{ModelResult, Outcome};
#[cfg(any(feature = "unstable-plugins", docsrs))]
pub use dotnet::{DotnetLibrary, DotnetLibraryBuilder, DotnetLibraryConfig, IntermediateOutputPasses as DotnetOutputPasses, ModelPasses as DotnetModelPasses};
pub use rust::{IntermediateOutputPasses, ModelPasses, RustLibrary, RustLibraryBuilder, RustLibraryConfig};

pub struct PassRunner {
    outcome: Outcome,
}

impl PassRunner {
    pub fn new() -> Self {
        Self { outcome: Outcome::Unchanged }
    }

    pub fn run(&mut self, x: ModelResult) -> Result<(), Error> {
        match x {
            Ok(Outcome::Changed) => self.outcome.changed(),
            Ok(Outcome::Unchanged) => {}
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

pub fn loop_model_passes_until_done(mut f: impl FnMut(&mut PassRunner) -> Result<(), Error>) -> Result<(), Error> {
    const PASS_LIMIT: u32 = 100;
    let mut counter = 0;

    loop {
        let mut pass_runner = PassRunner::new();

        f(&mut pass_runner)?;

        if pass_runner.outcome == Outcome::Unchanged {
            break;
        }

        counter += 1;
        if counter > PASS_LIMIT {
            return Err(Error::from("Pass iteration limit reached."));
        }
    }
    Ok(())
}
