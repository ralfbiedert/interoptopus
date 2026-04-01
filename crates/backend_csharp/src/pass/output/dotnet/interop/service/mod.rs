pub mod all;
pub mod ctor;
pub mod destructor;
pub mod method;

use crate::lang::TypeId;
use crate::lang::types::kind::TypeKind;
use crate::pass::{model, output};

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

/// Returns the managed service class name if `type_id` is a double-pointer to service
/// (i.e., `*const *const Service` — the FFI form of `&Service`).
fn resolve_double_ptr_to_service_name(type_id: TypeId, types: &model::common::types::all::Pass) -> Option<String> {
    let ty = types.get(type_id)?;
    if let TypeKind::Pointer(outer) = &ty.kind {
        let inner = types.get(outer.target)?;
        if let TypeKind::Pointer(p) = &inner.kind {
            let target = types.get(p.target)?;
            if matches!(&target.kind, TypeKind::Service) {
                return Some(target.name.clone());
            }
        }
    }
    None
}

/// Like `unmanaged_args` but handles pointer-to-service params by unwrapping `GCHandle`,
/// and double-pointer-to-service params (ref params) by dereferencing first.
///
/// - Owned service params (`ServiceHandle<T>`, single pointer): uses `T.Unmanaged` arg
///   type and `.IntoManaged()` (frees `GCHandle`, ownership transfer).
/// - Ref service params (`*const ServiceHandle<T>`, double pointer): uses `IntPtr` arg
///   type and `T.Unmanaged { _handle = x }.AsManaged()` (no free, borrow).
pub(super) fn service_aware_args(
    func: &crate::lang::functions::Function,
    types: &model::common::types::all::Pass,
    unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> (String, String) {
    let args: Vec<String> = func
        .signature
        .arguments
        .iter()
        .filter_map(|arg| {
            let ty_name = unmanaged_names.name(arg.ty)?;
            Some(format!("{ty_name} {}", arg.name))
        })
        .collect();

    let forward: Vec<String> = func
        .signature
        .arguments
        .iter()
        .map(|a| {
            if let Some(svc_name) = resolve_ptr_to_service_name(a.ty, types) {
                // Owned service param — construct Unmanaged wrapper and IntoManaged (frees GCHandle).
                format!("new {svc_name}.Unmanaged {{ _handle = {} }}.IntoManaged()", a.name)
            } else if let Some(svc_name) = resolve_double_ptr_to_service_name(a.ty, types) {
                // Ref service param — dereference the pointer-to-handle, then AsManaged (no free).
                format!("new {svc_name}.Unmanaged {{ _handle = Marshal.ReadIntPtr({}) }}.AsManaged()", a.name)
            } else {
                format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty))
            }
        })
        .collect();

    (args.join(", "), forward.join(", "))
}

/// Like `service_aware_args` but excludes the last argument from `forward_str`
/// (the async callback).
pub(super) fn service_aware_args_except_last(
    func: &crate::lang::functions::Function,
    types: &model::common::types::all::Pass,
    unmanaged_names: &output::common::conversion::unmanaged_names::Pass,
    unmanaged_conversion: &output::common::conversion::unmanaged_conversion::Pass,
) -> (String, String) {
    let n = func.signature.arguments.len().saturating_sub(1);

    let args: Vec<String> = func
        .signature
        .arguments
        .iter()
        .filter_map(|arg| {
            let ty_name = unmanaged_names.name(arg.ty)?;
            Some(format!("{ty_name} {}", arg.name))
        })
        .collect();

    let forward: Vec<String> = func
        .signature
        .arguments
        .iter()
        .take(n)
        .map(|a| {
            if let Some(svc_name) = resolve_ptr_to_service_name(a.ty, types) {
                format!("new {svc_name}.Unmanaged {{ _handle = {} }}.IntoManaged()", a.name)
            } else if let Some(svc_name) = resolve_double_ptr_to_service_name(a.ty, types) {
                format!("new {svc_name}.Unmanaged {{ _handle = Marshal.ReadIntPtr({}) }}.AsManaged()", a.name)
            } else {
                format!("{}{}", a.name, unmanaged_conversion.to_managed_suffix(a.ty))
            }
        })
        .collect();

    (args.join(", "), forward.join(", "))
}
