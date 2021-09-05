use interoptopus::lang::c::{CType, Function, PrimitiveType};
use interoptopus::patterns::service::Service;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

mod dotnet;
mod unity;

use crate::{CSharpTypeConverter, Config};
pub use dotnet::DotNet;
use interoptopus::patterns::TypePattern;
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

#[rustfmt::skip]
fn write_function_overloaded_invoke_with_error_handling(w: &mut IndentWriter, function: &Function, fn_call: &str) -> Result<(), Error> {

    match function.signature().rval() {
        CType::Pattern(TypePattern::FFIErrorEnum(e)) => {
            indented!(w, [_], r#"var rval = {};"#, fn_call)?;
            indented!(w, [_], r#"if (rval != {}.{})"#, e.the_enum().rust_name(), e.success_variant().name())?;
            indented!(w, [_], r#"{{"#)?;
            indented!(w, [_ _], r#"throw new Exception($"Something went wrong: {{rval}}");"#)?;
            indented!(w, [_], r#"}}"#)?;
        }
        CType::Primitive(PrimitiveType::Void) => {
            indented!(w, [_], r#"{};"#, fn_call)?;
        }
        _ => {
            indented!(w, [_], r#"return {};"#, fn_call)?;
        }
    }

    Ok(())
}
