use crate::lang::plugin::interface::Method;
use crate::lang::types::kind::{Primitive, TypeKind};
use crate::pass::model::common::types::all::Pass as TypesAll;

pub mod plugin;
pub mod service;

/// Format function arguments for interface declarations.
/// Resolves pointer-to-service types to the service class name.
pub(super) fn format_args(args: &[crate::lang::functions::Argument], types: &TypesAll) -> String {
    let parts: Vec<String> = args
        .iter()
        .filter_map(|arg| {
            let ty = types.get(arg.ty)?;
            // If the arg is a pointer-to-service, use the target service class name.
            let name = if let TypeKind::Pointer(p) = &ty.kind {
                if let Some(target) = types.get(p.target) {
                    if matches!(&target.kind, TypeKind::Service) { &target.name } else { &ty.name }
                } else {
                    &ty.name
                }
            } else {
                &ty.name
            };
            Some(format!("{name} {}", arg.name))
        })
        .collect();
    parts.join(", ")
}

/// Render the return type display name from a Method's `rval_id` and `is_async` flag.
pub(super) fn rval_display_name(method: &Method, types: &TypesAll) -> String {
    let Some(ty) = types.get(method.rval_id) else { return "void".to_string() };

    if method.is_async {
        if matches!(&ty.kind, TypeKind::Primitive(Primitive::Void)) {
            "Task".to_string()
        } else {
            format!("Task<{}>", ty.name)
        }
    } else {
        ty.name.clone()
    }
}
