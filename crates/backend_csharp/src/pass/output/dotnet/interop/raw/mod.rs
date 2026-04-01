pub mod all;
pub mod async_fn;
pub mod sync_fn;

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::model;

/// Returns the managed service class name if `type_id` is a pointer-to-service.
pub(super) fn resolve_ptr_to_service_name(type_id: TypeId, types: &model::common::types::all::Pass) -> Option<String> {
    let ty = types.get(type_id)?;
    if let TypeKind::Pointer(p) = &ty.kind {
        let target = types.get(p.target)?;
        if matches!(&target.kind, TypeKind::Service) {
            return Some(target.name.clone());
        }
    }
    None
}
