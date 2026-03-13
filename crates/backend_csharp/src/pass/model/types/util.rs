//! Registers backend-specific utility types (`InteropException`, `Utf8String`, etc.)
//! so they participate in dispatch routing like any other type.

use crate::lang::meta::{Emission, FileEmission};
use crate::lang::types::Type;
use crate::lang::types::csharp;
use crate::lang::types::kind::TypeKind;
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    done: bool,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, done: false }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        kinds: &mut model::types::kind::Pass,
        names: &mut model::types::names::Pass,
        types: &mut model::types::all::Pass,
    ) -> ModelResult {
        if self.done {
            return Ok(Unchanged);
        }

        let utils = [
            (csharp::UTIL_INTEROP_EXCEPTION, "InteropException"),
            (csharp::UTIL_ENUM_EXCEPTION, "EnumException"),
            (csharp::UTIL_UTF8STRING, "Utf8String"),
            (csharp::UTIL_ASYNC_CALLBACK_COMMON, "AsyncCallbackCommonNative"),
        ];

        for (id, name) in utils {
            kinds.set(id, TypeKind::Util);
            names.set(id, name.to_string());
            types.set(id, Type {
                emission: Emission::FileEmission(FileEmission::Common),
                name: name.to_string(),
                kind: TypeKind::Util,
            });
        }

        self.done = true;
        let mut outcome = Unchanged;
        outcome.changed();
        Ok(outcome)
    }
}
