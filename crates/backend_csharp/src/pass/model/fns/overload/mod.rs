use crate::lang::functions::Signature;
use crate::lang::types::kind::{Pointer, PointerKind, Primitive, TypeKind, TypePattern};
use crate::lang::{FunctionId, TypeId};
use crate::pass::model;

pub mod body;
pub mod simple;

/// Result of checking `IntPtr` eligibility for `ref` overloads.
enum IntPtrEligibility {
    /// Not an `IntPtr` at all, or definitely ineligible.
    Ineligible,
    /// Eligible for `ref` overloads.
    Eligible,
    /// Target type not yet resolved — must defer.
    Unknown,
}

/// Check whether an argument type is an `IntPtr` whose pointee is eligible for `ref` overloads.
fn intptr_eligibility(ty: TypeId, types: &model::types::all::Pass) -> IntPtrEligibility {
    let Some(TypeKind::Pointer(Pointer { kind: PointerKind::IntPtr(_), target })) = types.get(ty).map(|t| &t.kind) else {
        return IntPtrEligibility::Ineligible;
    };

    let Some(target_type) = types.get(*target) else {
        return IntPtrEligibility::Unknown;
    };

    match &target_type.kind {
        // `ref void` is not valid C#.
        TypeKind::Primitive(Primitive::Void) | TypeKind::TypePattern(TypePattern::CVoid) => IntPtrEligibility::Ineligible,
        // Opaque and service types have no C# struct definition, so `ref OpaqueType` is invalid.
        TypeKind::Opaque | TypeKind::Service => IntPtrEligibility::Ineligible,
        _ => IntPtrEligibility::Eligible,
    }
}

/// Check whether an argument type is an `IntPtr` whose pointee is eligible for `ref` overloads.
fn is_eligible_intptr(ty: TypeId, types: &model::types::all::Pass) -> bool {
    matches!(intptr_eligibility(ty, types), IntPtrEligibility::Eligible)
}

fn derive_overload_id(original_id: FunctionId, signature: &Signature) -> FunctionId {
    let mut id = FunctionId::from_id(original_id.id().derive_id(signature.rval.id()));
    for arg in &signature.arguments {
        id = FunctionId::from_id(id.id().derive_id(arg.ty.id()));
    }
    id
}
