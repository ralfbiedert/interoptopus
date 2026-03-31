//! Determines the managed conversion strategy for each type.
//!
//! - `AsIs`: Primitives, pointers, delegates, and other values that map directly
//!   to C# types without marshalling.
//! - `To`: Values that need marshalling but use copy semantics (original stays valid).
//! - `Into`: Values that need marshalling with move semantics (ownership transfer).
//!
//! Compounds and enums are at least `To`. If any field/variant is `Into`, the
//! compound/enum is also `Into`.

use crate::lang::TypeId;
use crate::lang::types::ManagedConversion;
use crate::lang::types::kind::{DelegateKind, TypeKind, Util};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    managed_conversion: HashMap<TypeId, ManagedConversion>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, managed_conversion: HashMap::default() }
    }

    #[allow(clippy::too_many_lines)]
    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, types: &model::common::types::all::Pass) -> ModelResult {
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
                TypeKind::Delegate(d) => match d.kind {
                    DelegateKind::Signature => ManagedConversion::AsIs,
                    DelegateKind::Class => ManagedConversion::Into,
                },

                // Services are opaque types; they don't appear directly in FFI signatures
                // (pointer-to-service does), but mark them Into for ownership semantics.
                TypeKind::Service => ManagedConversion::Into,
                TypeKind::Opaque => ManagedConversion::Into,

                // Arrays follow their element type, but are at least To
                TypeKind::Array(arr) => match self.managed_conversion.get(&arr.ty) {
                    Some(ManagedConversion::AsIs | ManagedConversion::To) => ManagedConversion::To,
                    Some(ManagedConversion::Into) => ManagedConversion::Into,
                    None => continue,
                },

                TypeKind::TypePattern(pattern) => {
                    use crate::lang::types::kind::TypePattern;
                    match pattern {
                        // Direct mappings — no marshalling
                        TypePattern::Bool | TypePattern::CChar | TypePattern::CVoid => ManagedConversion::AsIs,
                        TypePattern::CStrPointer => ManagedConversion::AsIs,
                        TypePattern::Version => ManagedConversion::AsIs,

                        // Copy semantics (borrowing, original stays valid)
                        TypePattern::Slice(_) | TypePattern::SliceMut(_) => ManagedConversion::To,

                        // Move semantics (ownership transfer)
                        TypePattern::Utf8String => ManagedConversion::Into,
                        TypePattern::Vec(_) => ManagedConversion::Into,
                        // AsyncCallbackCommonNative is already the blittable struct — it IS the unmanaged form.
                        TypePattern::AsyncCallback(_) => ManagedConversion::AsIs,
                        // TaskHandle is a blittable struct (three IntPtrs) — no marshalling needed.
                        TypePattern::TaskHandle => ManagedConversion::AsIs,
                        TypePattern::Wire(_) => ManagedConversion::Into,

                        // Option/Result: inspect variant payloads (same logic as DataEnum).
                        // Pointer-to-service variants are resolved to the service type so
                        // that their `Into` semantics propagate to the container.
                        TypePattern::Option(_, e) | TypePattern::Result(_, _, e) => {
                            let mut has_into = false;
                            let mut pending = false;
                            for variant in &e.variants {
                                if let Some(variant_ty) = variant.ty {
                                    let resolved = resolve_ptr_to_service(variant_ty, types);
                                    match self.managed_conversion.get(&resolved) {
                                        Some(ManagedConversion::Into) => {
                                            has_into = true;
                                            break;
                                        }
                                        Some(_) => {}
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
                                Some(_) => {}
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
                            Some(_) => {}
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

                // For most of these it shouldn't matter as they don't pass FFI boundaries,
                // but we mark helpers as always move (Into) to make them most restrictive.
                //
                // TODO: Maybe these enums needs an `NotApplicable`, or this returns an
                //       `Option<ManagedConversion>` instead. Also, maybe the core TypeInfo
                //       should have a Copy / Move marker this is primarily based on for all other
                //       types.
                TypeKind::AsyncHelper(_) => ManagedConversion::AsIs,
                TypeKind::Wire(_) => ManagedConversion::Into,
                TypeKind::Task(_) => ManagedConversion::Into,
                TypeKind::Util(x) => match x {
                    Util::InteropException => ManagedConversion::Into,
                    Util::EnumException => ManagedConversion::Into,
                    Util::AsyncCallbackCommon => ManagedConversion::AsIs,
                    Util::WireBuffer => ManagedConversion::Into,
                    Util::ConstCStrMarshaller => ManagedConversion::AsIs,
                    Util::TaskHandle => ManagedConversion::AsIs,
                },
                TypeKind::WireOnly(_) => ManagedConversion::Into,
            };

            self.managed_conversion.insert(*cs_id, conversion);
            outcome.changed();
        }

        Ok(outcome)
    }

    pub fn set(&mut self, ty: TypeId, conversion: ManagedConversion) {
        self.managed_conversion.insert(ty, conversion);
    }

    #[must_use]
    pub fn managed_conversion(&self, ty: TypeId) -> Option<ManagedConversion> {
        self.managed_conversion.get(&ty).copied()
    }
}

/// If `ty` is a pointer to a service type, return the service `TypeId`.
fn resolve_ptr_to_service(ty: TypeId, types: &model::common::types::all::Pass) -> TypeId {
    let Some(t) = types.get(ty) else { return ty };
    let TypeKind::Pointer(p) = &t.kind else { return ty };
    let Some(target) = types.get(p.target) else { return ty };
    if matches!(&target.kind, TypeKind::Service) { p.target } else { ty }
}
