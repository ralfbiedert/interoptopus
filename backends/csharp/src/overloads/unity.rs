use crate::overloads::Helper;
use crate::{OverloadWriter, Unsafe};
use interoptopus::lang::c::Function;
use interoptopus::patterns::service::Service;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub struct Unity {}

impl Unity {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

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

    fn write_function_overload(&self, w: &mut IndentWriter, h: Helper, function: &Function) -> Result<(), Error> {
        Ok(())
    }

    fn write_service_method_overload(&self, w: &mut IndentWriter, h: Helper, class: &Service, function: &Function) -> Result<(), Error> {
        Ok(())
    }

    fn write_pattern_slice_overload(&self, w: &mut IndentWriter, h: Helper, context_type_name: &str, type_string: &str) -> Result<(), Error> {
        if h.config.use_unsafe == Unsafe::UnsafePlatformMemCpy {
            // Ctor unity
            indented!(w, [_], r#"#if UNITY_2018_1_OR_NEWER"#)?;
            indented!(w, [_], r#"public {}(NativeArray<{}> handle)"#, context_type_name, type_string)?;
            indented!(w, [_], r#"{{"#)?;
            indented!(w, [_ _], r#"unsafe"#)?;
            indented!(w, [_ _], r#"{{"#)?;
            indented!(w, [_ _ _], r#"this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));"#)?;
            indented!(w, [_ _ _], r#"this.len = (ulong) handle.Length;"#)?;
            indented!(w, [_ _], r#"}}"#)?;
            indented!(w, [_], r#"}}"#)?;
            indented!(w, [_], r#"#endif"#)?;
        }

        Ok(())
    }

    fn write_pattern_slice_unsafe_copied_fragment(&self, w: &mut IndentWriter, h: Helper, type_string: &str) -> Result<(), Error> {
        indented!(w, [_ _ _ _ _], r#"#elif UNITY_2018_1_OR_NEWER"#)?;
        indented!(w, [_ _ _ _ _], r#"UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof({})));"#, type_string)?;
        Ok(())
    }
}
