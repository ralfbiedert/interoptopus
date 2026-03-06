//! Determines whether types require disposal (cleanup) in C#.
//!
//! A type needs `Dispose()` if it holds native resources:
//! - `Utf8String`, `Slice`, `SliceMut`, `Vec` need disposal (pin/ref-count native memory)
//! - `NamedCallback`, `AsyncCallback` need disposal (pinned delegate handles)
//! - Structs need disposal if any field needs disposal
//! - Enums need disposal if any variant data type needs disposal
//! - Primitives, pointers, fn pointers, `CStrPointer`, `Bool`, `CChar` do not

use crate::lang::types::{Cleanup, TypeKind};
use crate::model::TypeId;
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    cleanup: HashMap<TypeId, Cleanup>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: "model/types/info/deleteme_cleanup" }, cleanup: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, kinds: &model::types::kind::Pass) -> ModelResult {
        let mut outcome = Unchanged;

        for (cs_id, type_kind) in kinds.iter() {
            if self.cleanup.contains_key(cs_id) {
                continue;
            }

            let cleanup = match type_kind {
                TypeKind::Primitive(_) => Cleanup::None,
                TypeKind::Pointer(_) => Cleanup::None,
                TypeKind::Delegate(_) => Cleanup::None,
                TypeKind::Service => Cleanup::None,
                TypeKind::Opaque => Cleanup::None,

                TypeKind::Array(arr) => match self.cleanup.get(&arr.ty) {
                    Some(c) => *c,
                    None => continue,
                },

                TypeKind::TypePattern(pattern) => {
                    use crate::lang::types::TypePattern;
                    match pattern {
                        TypePattern::Bool | TypePattern::CChar | TypePattern::CVoid => Cleanup::None,
                        TypePattern::CStrPointer => Cleanup::None,
                        TypePattern::ApiVersion => Cleanup::None,
                        TypePattern::Utf8String => Cleanup::Disposable,
                        TypePattern::Slice(_) | TypePattern::SliceMut(_) => Cleanup::Disposable,
                        TypePattern::Vec(_) => Cleanup::Disposable,
                        TypePattern::Option(_) | TypePattern::Result(_, _) => Cleanup::Disposable,
                        TypePattern::NamedCallback(_) => Cleanup::Disposable,
                        TypePattern::AsyncCallback(_) => Cleanup::Disposable,
                    }
                }

                TypeKind::DataEnum(data_enum) => {
                    let mut any_disposable = false;
                    let mut pending = false;
                    for variant in &data_enum.variants {
                        if let Some(variant_ty) = variant.ty {
                            match self.cleanup.get(&variant_ty) {
                                Some(Cleanup::None) => continue,
                                Some(Cleanup::Disposable) => {
                                    any_disposable = true;
                                    break;
                                }
                                None => {
                                    pending = true;
                                    break;
                                }
                            }
                        }
                    }

                    if pending {
                        continue;
                    }

                    if any_disposable { Cleanup::Disposable } else { Cleanup::None }
                }

                TypeKind::Composite(composite) => {
                    let mut any_disposable = false;
                    let mut pending = false;
                    for field in &composite.fields {
                        match self.cleanup.get(&field.ty) {
                            Some(Cleanup::None) => continue,
                            Some(Cleanup::Disposable) => {
                                any_disposable = true;
                                break;
                            }
                            None => {
                                pending = true;
                                break;
                            }
                        }
                    }

                    if pending {
                        continue;
                    }

                    if any_disposable { Cleanup::Disposable } else { Cleanup::None }
                }

                TypeKind::AsyncHelper(_) | TypeKind::WireHelper(_) => Cleanup::Disposable,
            };

            self.cleanup.insert(*cs_id, cleanup);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn cleanup(&self, ty: TypeId) -> Option<Cleanup> {
        self.cleanup.get(&ty).copied()
    }
}
