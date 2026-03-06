//! Determines whether types use copy or move semantics at the FFI boundary.
//!
//! A type is `Copy` (reusable) if, after converting a managed C# value to its
//! unmanaged representation, the original managed value remains valid.
//! - Primitives, pointers, delegates, arrays are copy
//! - `CStrPointer`, `Bool`, `CChar`, `CVoid`, `ApiVersion` are copy
//! - `NamedCallback`, `AsyncCallback`, `Slice`, `SliceMut` are copy (borrowing)
//! - `Utf8String`, `Vec` are move (ownership transfer)
//! - Structs are copy if all fields are copy
//! - Enums are copy if all variant data types are copy

use crate::lang::types::{Reuse, TypeKind};
use crate::model::TypeId;
use crate::pass::Outcome::{Changed, Unchanged};
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    reuse: HashMap<TypeId, Reuse>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, reuse: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, kinds: &model::types::kind::Pass) -> ModelResult {
        let mut outcome = Unchanged;

        for (cs_id, type_kind) in kinds.iter() {
            if self.reuse.contains_key(cs_id) {
                continue;
            }

            let reuse = match type_kind {
                TypeKind::Primitive(_) => Reuse::Copy,
                TypeKind::Pointer(_) => Reuse::Copy,
                TypeKind::Delegate(_) => Reuse::Copy,
                TypeKind::Service => Reuse::Move,
                TypeKind::Opaque => Reuse::Move,

                TypeKind::Array(arr) => match self.reuse.get(&arr.ty) {
                    Some(r) => *r,
                    None => continue,
                },

                TypeKind::TypePattern(pattern) => {
                    use crate::lang::types::TypePattern;
                    match pattern {
                        TypePattern::Bool | TypePattern::CChar | TypePattern::CVoid => Reuse::Copy,
                        TypePattern::CStrPointer => Reuse::Copy,
                        TypePattern::ApiVersion => Reuse::Copy,
                        TypePattern::NamedCallback(_) => Reuse::Copy,
                        TypePattern::AsyncCallback(_) => Reuse::Copy,
                        TypePattern::Slice(_) | TypePattern::SliceMut(_) => Reuse::Copy,
                        TypePattern::Utf8String => Reuse::Move,
                        TypePattern::Vec(_) => Reuse::Move,
                        TypePattern::Option(_) | TypePattern::Result(_, _) => Reuse::Move,
                    }
                }

                TypeKind::DataEnum(data_enum) => {
                    let mut all_copy = true;
                    let mut pending = false;
                    for variant in &data_enum.variants {
                        if let Some(variant_ty) = variant.ty {
                            match self.reuse.get(&variant_ty) {
                                Some(Reuse::Copy) => continue,
                                Some(Reuse::Move) => {
                                    all_copy = false;
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

                    if all_copy { Reuse::Copy } else { Reuse::Move }
                }

                TypeKind::Composite(composite) => {
                    let mut all_copy = true;
                    let mut pending = false;
                    for field in &composite.fields {
                        match self.reuse.get(&field.ty) {
                            Some(Reuse::Copy) => continue,
                            Some(Reuse::Move) => {
                                all_copy = false;
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

                    if all_copy { Reuse::Copy } else { Reuse::Move }
                }

                TypeKind::AsyncHelper(_) | TypeKind::WireHelper(_) => Reuse::Move,
            };

            self.reuse.insert(*cs_id, reuse);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn reuse(&self, ty: TypeId) -> Option<Reuse> {
        self.reuse.get(&ty).copied()
    }
}
