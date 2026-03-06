//! Determines whether types are blittable or disposable.
//!
//! A type is blittable if it can be copied directly between managed and unmanaged memory.
//! - Primitives are blittable
//! - Delegates (function pointers) are blittable
//! - Pointers are blittable
//! - Arrays are blittable if their element type is blittable
//! - Some type patterns are blittable
//! - Structs are blittable if all their fields are blittable
//! - Enums are blittable if all their variant data types are blittable

use crate::lang::types::{Ownership, TypeKind};
use crate::model::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    blittable: HashMap<TypeId, Ownership>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, blittable: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, kinds: &model::types::kind::Pass) -> ModelResult {
        let mut outcome = Unchanged;

        for (cs_id, type_kind) in kinds.iter() {
            // Skip if we've already determined blittability
            if self.blittable.contains_key(cs_id) {
                continue;
            }

            // Determine if this type is blittable
            let blittable = match type_kind {
                // Arrays are blittable if element type is blittable
                TypeKind::Array(arr) => match self.blittable.get(&arr.ty) {
                    Some(Ownership::Blittable) => true,
                    Some(Ownership::Disposable) => false,
                    None => {
                        continue;
                    }
                },

                TypeKind::Delegate(_) => true,
                TypeKind::Primitive(_) => true,
                TypeKind::Pointer(_) => true,
                TypeKind::Service => false,
                TypeKind::Opaque => false,

                // Type patterns: some are blittable, some are disposable
                TypeKind::TypePattern(pattern) => {
                    use crate::lang::types::TypePattern;
                    match pattern {
                        TypePattern::Bool | TypePattern::CChar | TypePattern::CVoid => true,
                        TypePattern::CStrPointer => true,
                        TypePattern::ApiVersion => true,
                        TypePattern::Utf8String => false,
                        TypePattern::Slice(_) | TypePattern::SliceMut(_) | TypePattern::Vec(_) => false,
                        TypePattern::Option(_) | TypePattern::Result(_, _) => false,
                        TypePattern::NamedCallback(_) => true,
                        TypePattern::AsyncCallback(_) => false, // Async callbacks are disposable
                    }
                }

                // DataEnum: blittable if all variant data types are blittable
                TypeKind::DataEnum(data_enum) => {
                    let mut all_blittable = true;
                    let mut pending = false;
                    for variant in &data_enum.variants {
                        if let Some(variant_ty) = variant.ty {
                            match self.blittable.get(&variant_ty) {
                                Some(Ownership::Blittable) => continue,
                                Some(Ownership::Disposable) => {
                                    all_blittable = false;
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

                    all_blittable
                }

                // Composite: blittable if all fields are blittable
                TypeKind::Composite(composite) => {
                    let mut all_blittable = true;
                    let mut pending = false;
                    for field in &composite.fields {
                        match self.blittable.get(&field.ty) {
                            Some(Ownership::Blittable) => continue,
                            Some(Ownership::Disposable) => {
                                all_blittable = false;
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

                    all_blittable
                }

                // Helper types: assume disposable for now
                TypeKind::AsyncHelper(_) | TypeKind::WireHelper(_) => false,
            };

            let kind = if blittable { Ownership::Blittable } else { Ownership::Disposable };
            // println!("{cs_id:?}, {kind:?}");
            self.blittable.insert(*cs_id, kind);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn blittable(&self, ty: TypeId) -> Option<Ownership> {
        self.blittable.get(&ty).copied()
    }
}
