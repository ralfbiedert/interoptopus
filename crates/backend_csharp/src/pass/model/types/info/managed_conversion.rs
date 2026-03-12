//! Determines the managed conversion strategy for each type.
//!
//! - `AsIs`: Primitives, pointers, delegates, and other values that map directly
//!   to C# types without marshalling.
//! - `To`: Values that need marshalling but use copy semantics (original stays valid).
//! - `Into`: Values that need marshalling with move semantics (ownership transfer).
//!
//! Compounds and enums are at least `To`. If any field/variant is `Into`, the
//! compound/enum is also `Into`.

use crate::lang::types::kind::{DelegateKind, TypeKind};
use crate::lang::types::ManagedConversion;
use crate::lang::TypeId;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    managed_conversion: HashMap<TypeId, ManagedConversion>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, managed_conversion: Default::default() }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, types: &model::types::all::Pass) -> ModelResult {
        let mut outcome = Unchanged;

        for (cs_id, ty) in types.iter() {
            let type_kind = &ty.kind;
            if self.managed_conversion.contains_key(cs_id) {
                continue;
            }

            let conversion = match type_kind {
                // Direct C# value types — no marshalling needed
                TypeKind::Primitive(_) => ManagedConversion::AsIs,
                TypeKind::Pointer(_) => ManagedConversion::AsIs,
                TypeKind::Delegate(_) => ManagedConversion::AsIs,

                // Services and opaques transfer ownership
                TypeKind::Service => ManagedConversion::Into,
                TypeKind::Opaque => ManagedConversion::Into,

                // Arrays follow their element type, but are at least To
                TypeKind::Array(arr) => match self.managed_conversion.get(&arr.ty) {
                    Some(ManagedConversion::AsIs) | Some(ManagedConversion::To) => ManagedConversion::To,
                    Some(ManagedConversion::Into) => ManagedConversion::Into,
                    None => continue,
                },

                TypeKind::TypePattern(pattern) => {
                    use crate::lang::types::kind::TypePattern;
                    match pattern {
                        // Direct mappings — no marshalling
                        TypePattern::Bool | TypePattern::CChar | TypePattern::CVoid => ManagedConversion::AsIs,
                        TypePattern::CStrPointer => ManagedConversion::AsIs,
                        TypePattern::ApiVersion => ManagedConversion::AsIs,

                        // Copy semantics (borrowing, original stays valid)
                        TypePattern::Slice(_) | TypePattern::SliceMut(_) => ManagedConversion::To,

                        // Move semantics (ownership transfer)
                        TypePattern::Utf8String => ManagedConversion::Into,
                        TypePattern::Vec(_) => ManagedConversion::Into,
                        TypePattern::AsyncCallback(_) => ManagedConversion::Into,

                        // Option/Result: inspect variant payloads (same logic as DataEnum)
                        TypePattern::Option(_, e) | TypePattern::Result(_, _, e) => {
                            let mut has_into = false;
                            let mut pending = false;
                            for variant in &e.variants {
                                if let Some(variant_ty) = variant.ty {
                                    match self.managed_conversion.get(&variant_ty) {
                                        Some(ManagedConversion::Into) => {
                                            has_into = true;
                                            break;
                                        }
                                        Some(_) => continue,
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

                            if has_into { ManagedConversion::Into } else { ManagedConversion::To }
                        }
                    }
                }

                // Enums: at least To; Into if any variant data is Into
                TypeKind::DataEnum(data_enum) => {
                    let mut has_into = false;
                    let mut pending = false;
                    for variant in &data_enum.variants {
                        if let Some(variant_ty) = variant.ty {
                            match self.managed_conversion.get(&variant_ty) {
                                Some(ManagedConversion::Into) => {
                                    has_into = true;
                                    break;
                                }
                                Some(_) => continue,
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

                    if has_into { ManagedConversion::Into } else { ManagedConversion::To }
                }

                // Composites: at least To; Into if any field is Into
                TypeKind::Composite(composite) => {
                    let mut has_into = false;
                    let mut pending = false;
                    for field in &composite.fields {
                        match self.managed_conversion.get(&field.ty) {
                            Some(ManagedConversion::Into) => {
                                has_into = true;
                                break;
                            }
                            Some(_) => continue,
                            None => {
                                pending = true;
                                break;
                            }
                        }
                    }

                    if pending {
                        continue;
                    }

                    if has_into { ManagedConversion::Into } else { ManagedConversion::To }
                }

                // Helpers are always move
                TypeKind::AsyncHelper(_) | TypeKind::WireHelper(_) => ManagedConversion::Into,
            };

            self.managed_conversion.insert(*cs_id, conversion);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn managed_conversion(&self, ty: TypeId) -> Option<ManagedConversion> {
        self.managed_conversion.get(&ty).copied()
    }
}
