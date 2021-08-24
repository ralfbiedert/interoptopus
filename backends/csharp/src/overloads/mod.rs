use interoptopus::lang::c::Function;
use interoptopus::patterns::service::Service;
use interoptopus::writer::IndentWriter;
use interoptopus::Error;

mod basic;
mod unity;

use crate::{CSharpTypeConverter, Config};
pub use basic::BasicCSharp;
pub use unity::Unity;

pub struct Helper<'a> {
    pub config: &'a Config,
    pub converter: &'a dyn CSharpTypeConverter,
}

pub trait OverloadWriter {
    fn write_imports(&self, w: &mut IndentWriter, h: Helper) -> Result<(), Error>;

    fn write_function_overloaded(&self, w: &mut IndentWriter, h: Helper, function: &Function) -> Result<(), Error>;

    fn write_service_method_overloaded(&self, w: &mut IndentWriter, h: Helper, class: &Service, function: &Function) -> Result<(), Error>;
}
