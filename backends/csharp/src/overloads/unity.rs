use crate::overloads::Helper;
use crate::{OverloadWriter, Unsafe};
use interoptopus::lang::c::Function;
use interoptopus::patterns::service::Service;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub struct Unity {}

impl OverloadWriter for Unity {
    fn write_imports(&self, w: &mut IndentWriter, h: Helper) -> Result<(), Error> {
        if h.config.use_unsafe == Unsafe::UnsafePlatformMemCpy {
            indented!(w, r#"#if UNITY_2018_1_OR_NEWER"#)?;
            indented!(w, r#"using Unity.Collections.LowLevel.Unsafe;"#)?;
            indented!(w, r#"using Unity.Collections;"#)?;
            indented!(w, r#"#endif"#)?;
        }
        Ok(())
    }

    fn write_function_overloaded(&self, w: &mut IndentWriter, h: Helper, function: &Function) -> Result<(), Error> {
        todo!()
    }

    fn write_service_method_overloaded(&self, w: &mut IndentWriter, h: Helper, class: &Service, function: &Function) -> Result<(), Error> {
        todo!()
    }
}
