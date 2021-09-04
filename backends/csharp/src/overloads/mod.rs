use interoptopus::lang::c::Function;
use interoptopus::patterns::service::Service;
use interoptopus::writer::IndentWriter;
use interoptopus::Error;

mod common_csharp;
mod unity;

use crate::{CSharpTypeConverter, Config};
pub use common_csharp::CommonCSharp;
pub use unity::Unity;

pub struct Helper<'a> {
    pub config: &'a Config,
    pub converter: &'a dyn CSharpTypeConverter,
}

pub trait OverloadWriter {
    fn write_imports(&self, w: &mut IndentWriter, h: Helper) -> Result<(), Error>;

    fn write_function_overload(&self, w: &mut IndentWriter, h: Helper, function: &Function) -> Result<(), Error>;

    fn write_service_method_overload(&self, w: &mut IndentWriter, h: Helper, class: &Service, function: &Function) -> Result<(), Error>;

    fn write_pattern_slice_overload(&self, w: &mut IndentWriter, h: Helper, context_type_name: &str, type_string: &str) -> Result<(), Error>;

    fn write_pattern_slice_unsafe_copied_fragment(&self, w: &mut IndentWriter, h: Helper, type_string: &str) -> Result<(), Error>;
}
