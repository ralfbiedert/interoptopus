pub mod all;
pub mod body;
pub mod body_as_unmanaged;
pub mod body_ctors;
pub mod body_exception_for_variant;
pub mod body_to_unmanaged;
pub mod body_tostring;
pub mod body_unmanaged;
pub mod body_unmanaged_variant;
pub mod definition;

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::{OperationMode, model};

/// If `ty` is a pointer to a service type, return the service TypeId so that
/// enum rendering uses the managed service class and `Service.Unmanaged` instead
/// of raw `IntPtr`. Only applicable for reverse interop (`Plugin` mode) where
/// services are managed C# objects; in forward interop (`Rust` mode) services
/// are opaque Rust pointers and should remain as `IntPtr`.
fn resolve_service_variant(ty: TypeId, types: &model::common::types::all::Pass, mode: OperationMode) -> TypeId {
    if mode == OperationMode::Rust {
        return ty;
    }
    let Some(t) = types.get(ty) else { return ty };
    let TypeKind::Pointer(p) = &t.kind else { return ty };
    let Some(target) = types.get(p.target) else { return ty };
    if matches!(&target.kind, TypeKind::Service) { p.target } else { ty }
}
