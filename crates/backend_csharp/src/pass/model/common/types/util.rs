//! Registers backend-specific utility types (`InteropException`, `Utf8String`, etc.)
//! so they participate in dispatch routing like any other type.
//!
//! After registration, propagates utility type visibilities to any pattern-mapped
//! types that share the same C# name (e.g., `AsyncCallback` pattern types get the
//! visibility of the `AsyncCallbackCommonNative` util type).

use crate::lang::meta::{Emission, FileEmission, Visibility};
use crate::lang::types::csharp;
use crate::lang::types::kind::{TypeKind, TypePattern, Util};
use crate::lang::types::{Decorators, Type};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    registered: bool,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, registered: false }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        kinds: &mut model::common::types::kind::Pass,
        names: &mut model::common::types::names::Pass,
        types: &mut model::common::types::all::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        if !self.registered {
            // Bool is always synthesized — Primitive::Bool has Emission::Builtin so the
            // inventory-based lookup would never give it FileEmission::Common.
            let bool_kind = TypeKind::TypePattern(TypePattern::Bool);
            kinds.set(csharp::BOOL, bool_kind.clone());
            names.set(csharp::BOOL, "Bool".to_string());
            types.set(
                csharp::BOOL,
                Type {
                    emission: Emission::FileEmission(FileEmission::Common),
                    name: "Bool".to_string(),
                    visibility: Visibility::Public,
                    docs: Vec::new(),
                    kind: bool_kind,
                    decorators: Decorators::default(),
                },
            );

            let utils = [
                (csharp::UTIL_INTEROP_EXCEPTION, "InteropException", Util::InteropException, Visibility::Public),
                (csharp::UTIL_ENUM_EXCEPTION, "EnumException", Util::EnumException, Visibility::Public),
                (csharp::UTIL_ASYNC_CALLBACK_COMMON, "AsyncCallbackCommonNative", Util::AsyncCallbackCommon, Visibility::Internal),
                (csharp::UTIL_WIRE_BUFFER, "WireBuffer", Util::WireBuffer, Visibility::Internal),
                (csharp::UTIL_CONST_CSTR_MARSHALLER, "ConstCStrMarshaller", Util::ConstCStrMarshaller, Visibility::Internal),
            ];

            for (id, name, variant, visibility) in utils {
                let kind = TypeKind::Util(variant);
                kinds.set(id, kind.clone());
                names.set(id, name.to_string());
                types.set(
                    id,
                    Type {
                        emission: Emission::FileEmission(FileEmission::Common),
                        name: name.to_string(),
                        visibility,
                        docs: Vec::new(),
                        kind,
                        decorators: Decorators::default(),
                    },
                );
            }

            self.registered = true;
            outcome.changed();
        }

        // Propagate util type visibilities to pattern-mapped types that share the
        // same C# name but have different TypeIds (e.g., AsyncCallback pattern types
        // should inherit the visibility of the AsyncCallbackCommonNative util type).
        let util_vis: Vec<_> = types
            .iter()
            .filter(|(_, ty)| matches!(&ty.kind, TypeKind::Util(_)))
            .map(|(_, ty)| (ty.name.clone(), ty.visibility.clone()))
            .collect();

        let to_update: Vec<_> = types
            .iter()
            .filter_map(|(&id, ty)| {
                if matches!(&ty.kind, TypeKind::Util(_)) {
                    return None;
                }
                for (util_name, util_vis) in &util_vis {
                    if ty.name == *util_name && ty.visibility != *util_vis {
                        return Some((id, util_vis.clone()));
                    }
                }
                None
            })
            .collect();

        for (id, vis) in to_update {
            if let Some(ty) = types.get_mut(id) {
                ty.visibility = vis;
                outcome.changed();
            }
        }

        Ok(outcome)
    }
}
