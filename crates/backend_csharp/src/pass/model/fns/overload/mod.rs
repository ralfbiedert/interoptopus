use crate::lang::functions::Signature;
use crate::lang::types::kind::{Pointer, PointerKind, Primitive, TypeKind, TypePattern};
use crate::lang::{FunctionId, TypeId};
use crate::pass::model;

pub mod body;
pub mod simple;

/// Check whether an argument type is an `IntPtr` whose pointee is eligible for `ref` overloads.
fn is_eligible_intptr(ty: TypeId, types: &model::types::all::Pass) -> bool {
    let Some(TypeKind::Pointer(Pointer { kind: PointerKind::IntPtr(_), target })) = types.get(ty).map(|t| &t.kind) else {
        return false;
    };

    // Exclude void targets — `ref void` is not valid C#.
    if let Some(t) = types.get(*target)
        && matches!(t.kind, TypeKind::Primitive(Primitive::Void) | TypeKind::TypePattern(TypePattern::CVoid))
    {
        return false;
    }

    true
}

fn derive_overload_id(original_id: FunctionId, signature: &Signature) -> FunctionId {
    let mut id = FunctionId::from_id(original_id.id().derive_id(signature.rval.id()));
    for arg in &signature.arguments {
        id = FunctionId::from_id(id.id().derive_id(arg.ty.id()));
    }
    id
}
