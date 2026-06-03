//! Builds a per-result-type model record for every async trampoline class.
//!
//! Async overloads return `Task<T>` and dispatch through a generated
//! `AsyncTrampoline*` class. The shape of that class depends on the original
//! Rust return type (bare `T`, `*const Service`, `()`, `Result<T, E>`, etc.).
//!
//! This pass owns the **typed** model: for every result `TypeId` returned from
//! an async overload it produces a [`Trampoline`] record giving the shape (a
//! closed enum), the `Task<T>` inner `TypeId`, the `AsyncOutcomeOf<T>` payload
//! `TypeId`, the managed-conversion category and the routing-id used for
//! output-file gating. All string-shaped C# identifiers (class names, field
//! names, payload expressions, method names) are derived on the output side
//! from these typed fields — see `output::rust::types::asynk_naming`.
//!
//! It also indexes per-function (`for_function`) so the overload and service
//! body output passes can resolve their trampoline by `FunctionId` without
//! re-deriving anything.

use crate::lang::functions::FunctionKind;
use crate::lang::functions::overload::{OverloadKind, RvalTransform};
use crate::lang::types::ManagedConversion;
use crate::lang::types::kind::{Pointer, Primitive, TypeKind, TypePattern};
use crate::lang::{FunctionId, TypeId};
use crate::pass::Outcome::Unchanged;
use crate::pass::{ModelResult, PassInfo, model};
use std::collections::HashMap;

/// Closed enum of trampoline shapes. Each variant maps to exactly one
/// rendering branch in `templates/rust/types/asynk/trampoline.cs`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrampolineShape {
    /// `async fn () -> ()` — wire payload carries only the discriminant byte.
    BareVoid,
    /// Bare `T` whose C# form is directly `T` (primitives, `IntPtr`).
    BareDirect,
    /// Bare `T` whose C# wire form is `T.Unmanaged` (e.g. `Wire<String>`).
    BareUnmanaged,
    /// `Result<T, E>` where `T` is directly representable (includes `Result<(), E>`).
    ResultDirect,
    /// `Result<T, E>` where `T` (or the Result itself) has an `.Unmanaged` wire form.
    ResultUnmanaged,
}

impl TrampolineShape {
    /// Stable string tag used by the trampoline template's shape switch.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BareVoid => "BareVoid",
            Self::BareDirect => "BareDirect",
            Self::BareUnmanaged => "BareUnmanaged",
            Self::ResultDirect => "ResultDirect",
            Self::ResultUnmanaged => "ResultUnmanaged",
        }
    }

    /// True when this trampoline's wire payload is `T.Unmanaged` rather than
    /// plain `T`.
    #[must_use]
    pub fn payload_is_unmanaged(self) -> bool {
        matches!(self, Self::BareUnmanaged | Self::ResultUnmanaged)
    }
}

/// Per-trampoline model record. One per unique result `TypeId` returned from
/// async overloads. All fields are typed / enum-shaped; output passes derive
/// C# identifiers from these.
#[derive(Clone, Debug)]
pub struct Trampoline {
    /// Original Rust result type id (the `T` of `RvalTransform::AsyncTask(T)`).
    /// Drives `AsyncTrampoline{Name}` / `_trampoline{Name}` derivation.
    pub result_ty: TypeId,
    /// Shape of the trampoline. Drives template selection and the
    /// `.Unmanaged` decision in payload rendering.
    pub shape: TrampolineShape,
    /// Inner `T` of `Task<T>`. `None` ⇒ surface API returns plain `Task`
    /// (i.e. async fn returns `()` or `Result<(), _>`).
    pub task_inner: Option<TypeId>,
    /// Type whose name fills `AsyncOutcomeOf<…>`. `None` ⇒ `BareVoid` (no
    /// payload beyond the discriminant byte). For non-`Pointer` shapes this
    /// is the result type itself; for `Pointer` shapes it's the pointer type
    /// (whose C# name is the pointer's display, typically `IntPtr`).
    pub payload_inner: Option<TypeId>,
    /// Managed-conversion category of the result type. Determines whether
    /// the trampoline calls `.ToManaged()` or `.IntoManaged()` to lift the
    /// unmanaged payload.
    pub managed_conversion: ManagedConversion,
    /// Type id used for `type_belongs_to` routing decisions. For
    /// `Result<_, _>` this is the Result type itself; for `*const Service`
    /// the target service; otherwise the result type.
    pub routing_id: TypeId,
    /// True when the trampoline is stateless (no managed dependencies in the
    /// generated class) and may be emitted into every output file — bare
    /// primitives, void, and `IntPtr`-shaped service ctors. Result-shape
    /// trampolines stay scoped to the file that owns the Result.
    pub emit_in_every_output: bool,
}

impl Trampoline {
    /// True when the surface API returns plain `Task` (not `Task<T>`).
    #[must_use]
    pub fn is_task_void(&self) -> bool {
        self.task_inner.is_none()
    }
}

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    /// Keyed by result type id from `RvalTransform::AsyncTask(result_ty)`.
    trampolines: HashMap<TypeId, Trampoline>,
    /// Maps original `FunctionId` of an async overload back to the result
    /// type id (and thus to a `Trampoline`).
    fn_to_result: HashMap<FunctionId, TypeId>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, trampolines: HashMap::default(), fn_to_result: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        managed_conversion: &model::common::types::info::managed_conversion::Pass,
    ) -> ModelResult {
        let mut outcome = Unchanged;

        for (&fn_id, func) in fns_all.overloads() {
            let FunctionKind::Overload(overload) = &func.kind else { continue };
            let OverloadKind::Async(transforms) = &overload.kind else { continue };
            let RvalTransform::AsyncTask(result_ty_id) = transforms.rval else { continue };

            if self.fn_to_result.insert(overload.base, result_ty_id).is_none() {
                outcome.changed();
            }
            if self.fn_to_result.insert(fn_id, result_ty_id).is_none() {
                outcome.changed();
            }

            if self.trampolines.contains_key(&result_ty_id) {
                continue;
            }

            let Some(result_ty) = types.get(result_ty_id) else {
                continue;
            };

            let mc = managed_conversion.managed_conversion(result_ty_id);
            let Some(mc) = mc else {
                continue;
            };

            let Some((shape, task_inner, payload_inner)) = Self::classify(result_ty_id, result_ty, types, mc) else {
                continue;
            };

            let routing_id = match &result_ty.kind {
                TypeKind::Pointer(p) => p.target,
                _ => result_ty_id,
            };

            // Bare-T trampolines (non-pointer, non-result) emit into every
            // output: primitives like `uint` have no routing entry, and
            // composite types like `Vec3f32` (Interop.cs) may be referenced
            // from async fns routed to `Interop.Common.cs`. Pointer-shaped
            // trampolines route via the pointer's target service;
            // Result-shaped trampolines route via the Result type's home.
            let emit_in_every_output = matches!(shape, TrampolineShape::BareVoid | TrampolineShape::BareDirect | TrampolineShape::BareUnmanaged)
                && !matches!(&result_ty.kind, TypeKind::Pointer(_));

            self.trampolines
                .insert(result_ty_id, Trampoline { result_ty: result_ty_id, shape, task_inner, payload_inner, managed_conversion: mc, routing_id, emit_in_every_output });
            outcome.changed();
        }

        Ok(outcome)
    }

    /// Classifies a result type into a [`TrampolineShape`] plus the
    /// `task_inner` and `payload_inner` `TypeId`s (both `Option` because
    /// `BareVoid` has no payload and `Result<(), _>` / `()` have no task
    /// inner).
    ///
    /// Returns `None` when downstream type lookups (e.g. the Ok arm of a
    /// `Result`) haven't resolved yet; the pass retries next iteration.
    fn classify(
        result_ty_id: TypeId,
        result_ty: &crate::lang::types::Type,
        types: &model::common::types::all::Pass,
        mc: ManagedConversion,
    ) -> Option<(TrampolineShape, Option<TypeId>, Option<TypeId>)> {
        let has_unmanaged = matches!(mc, ManagedConversion::To | ManagedConversion::Into);

        match &result_ty.kind {
            TypeKind::Primitive(Primitive::Void) => Some((TrampolineShape::BareVoid, None, None)),
            TypeKind::Pointer(_) => {
                // Bare `*const Service` async ctor — the C# task surface is
                // `Task<{pointer_name}>` (typically `Task<IntPtr>`) and the
                // payload is `AsyncOutcomeOf<{pointer_name}>`.
                Some((TrampolineShape::BareDirect, Some(result_ty_id), Some(result_ty_id)))
            }
            TypeKind::TypePattern(TypePattern::Result(ok_ty, _, _)) => {
                let ok_kind = types.get(*ok_ty).map(|t| &t.kind);
                let ok_is_void = matches!(ok_kind, Some(TypeKind::Primitive(Primitive::Void)));

                // Touch ok type so we wait until it's mapped, even if void.
                let _ = types.get(*ok_ty)?;

                let task_inner = if ok_is_void { None } else { Some(*ok_ty) };
                let shape = if has_unmanaged {
                    TrampolineShape::ResultUnmanaged
                } else {
                    TrampolineShape::ResultDirect
                };
                Some((shape, task_inner, Some(result_ty_id)))
            }
            _ => {
                let shape = if has_unmanaged {
                    TrampolineShape::BareUnmanaged
                } else {
                    TrampolineShape::BareDirect
                };
                Some((shape, Some(result_ty_id), Some(result_ty_id)))
            }
        }
    }

    /// All trampoline records, keyed by result type id.
    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &Trampoline)> {
        self.trampolines.iter()
    }

    /// Trampoline for a specific result type, if known.
    #[must_use]
    pub fn for_result(&self, result_ty: TypeId) -> Option<&Trampoline> {
        self.trampolines.get(&result_ty)
    }

    /// Trampoline used by a given async function (overload or original id).
    #[must_use]
    pub fn for_function(&self, fn_id: FunctionId) -> Option<&Trampoline> {
        let result_ty = self.fn_to_result.get(&fn_id)?;
        self.trampolines.get(result_ty)
    }
}
