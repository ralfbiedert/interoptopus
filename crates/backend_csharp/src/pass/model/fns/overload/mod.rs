use crate::lang::functions::Signature;
use crate::lang::types::ManagedConversion;
use crate::lang::types::kind::{Pointer, PointerKind, TypeKind};
use crate::lang::{FunctionId, TypeId};
use crate::pass::model;

pub mod all;
pub mod body;
pub mod simple;

/// Check whether an argument type is an IntPtr whose pointee is eligible for `ref` overloads.
///
/// Returns `Some(true)` if eligible, `Some(false)` if definitely not eligible,
/// and `None` if the managed conversion for the pointee isn't known yet (caller should defer).
fn is_eligible_intptr(ty: TypeId, types: &model::types::all::Pass, managed_conversion: &model::types::info::managed_conversion::Pass) -> Option<bool> {
    let Some(TypeKind::Pointer(Pointer { kind: PointerKind::IntPtr(_), target })) = types.get(ty).map(|t| &t.kind) else {
        return Some(false);
    };
    match managed_conversion.managed_conversion(*target) {
        Some(ManagedConversion::AsIs | ManagedConversion::To) => Some(true),
        Some(_) => Some(false),
        None => None,
    }
}

fn derive_overload_id(original_id: FunctionId, signature: &Signature) -> FunctionId {
    let mut id = FunctionId::from_id(original_id.id().derive_id(signature.rval.id()));
    for arg in &signature.arguments {
        id = FunctionId::from_id(id.id().derive_id(arg.ty.id()));
    }
    id
}
