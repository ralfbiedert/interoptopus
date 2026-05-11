//! Output passes that render Rust-sourced types. The shared `asynk_naming`
//! helpers below derive C# identifiers from typed trampoline records and
//! are used by the trampoline-emitting passes here as well as by the
//! fn-overload and service-method body passes.

pub mod asynk;

use crate::lang::types::ManagedConversion;
use crate::lang::types::csharp;
use crate::pass::model;
use crate::pass::model::rust::types::info::trampoline::{Trampoline, TrampolineShape};

/// Pure C#-identifier derivation for async trampoline records.
///
/// The `model::rust::types::info::trampoline` pass owns the **typed** view
/// of every async trampoline (shape + TypeIds + conversion category).
/// This module is the **string** view: given that record plus a type-name
/// lookup, it produces the C# identifiers consumed by templates. Lives
/// next to the passes that consume it because Rust pass modules don't
/// import each other directly.
pub mod asynk_naming {
    use super::{ManagedConversion, Trampoline, TrampolineShape, csharp, model};

    /// Capitalises the first character so trampoline class names look like proper
    /// C# identifiers (`uint` → `Uint`, `void` → `Void`).
    fn capitalize_first(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }

    /// Display name segment derived from a trampoline's result type, e.g.
    /// `uint` → `Uint`. Used as the suffix for both class and field names.
    #[must_use]
    pub fn display(t: &Trampoline, types: &model::common::types::all::Pass) -> String {
        types.get(t.result_ty).map(|ty| capitalize_first(&ty.name)).unwrap_or_default()
    }

    /// C# class name, e.g. `AsyncTrampolineUint`.
    #[must_use]
    pub fn class_name(t: &Trampoline, types: &model::common::types::all::Pass) -> String {
        format!("AsyncTrampoline{}", display(t, types))
    }

    /// Static field name on `Interop`, e.g. `_trampolineUint`.
    #[must_use]
    pub fn field_name(t: &Trampoline, types: &model::common::types::all::Pass) -> String {
        format!("_trampoline{}", display(t, types))
    }

    /// Inner `T` of `Task<T>` for a trampoline's surface API. Returns `"void"`
    /// when the surface returns plain `Task`.
    #[must_use]
    pub fn task_inner_name(t: &Trampoline, types: &model::common::types::all::Pass) -> String {
        match t.task_inner {
            None => "void".to_string(),
            Some(id) => types.get(id).map(|ty| ty.name.clone()).unwrap_or_default(),
        }
    }

    /// Full payload C# expression, e.g. `AsyncOutcomeOf<uint>` or
    /// `AsyncOutcomeOf<Vec3f32.Unmanaged>`. Empty string for `BareVoid`
    /// (no payload beyond the discriminant byte).
    #[must_use]
    pub fn payload_full(t: &Trampoline, types: &model::common::types::all::Pass) -> String {
        let Some(payload_id) = t.payload_inner else {
            return String::new();
        };
        let Some(payload_ty) = types.get(payload_id) else {
            return String::new();
        };
        let inner = if t.shape.payload_is_unmanaged() { format!("{}.Unmanaged", payload_ty.name) } else { payload_ty.name.clone() };
        let outcome = types.get(csharp::UTIL_ASYNC_OUTCOME_PAYLOAD).map_or_else(|| "AsyncOutcomeOf".to_string(), |ty| ty.name.clone());
        format!("{outcome}<{inner}>")
    }

    /// Method name used to convert `outcome.Value` from its `.Unmanaged` form
    /// back to the managed form. Only meaningful when the payload is unmanaged.
    #[must_use]
    pub fn managed_conversion_method(mc: ManagedConversion) -> &'static str {
        match mc {
            ManagedConversion::Into => "IntoManaged",
            _ => "ToManaged",
        }
    }

    /// Convenience: all identifiers in one shot. Computing each independently
    /// is cheap, but bundling reads more clearly at call sites that need
    /// every name (the trampoline render pass).
    pub struct Names {
        pub class_name: String,
        pub field_name: String,
        pub task_inner_name: String,
        pub payload_full: String,
        pub managed_conversion_method: &'static str,
        pub shape: TrampolineShape,
        pub is_task_void: bool,
    }

    #[must_use]
    pub fn names_for(t: &Trampoline, types: &model::common::types::all::Pass) -> Names {
        Names {
            class_name: class_name(t, types),
            field_name: field_name(t, types),
            task_inner_name: task_inner_name(t, types),
            payload_full: payload_full(t, types),
            managed_conversion_method: managed_conversion_method(t.managed_conversion),
            shape: t.shape,
            is_task_void: t.is_task_void(),
        }
    }
}
