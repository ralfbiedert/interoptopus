use crate::lang::types::kind::TypeKind;
use crate::pass::model;

pub mod plugin;
pub mod service;

/// Format function arguments for interface declarations.
/// Resolves pointer-to-service types to the service class name.
fn format_args(args: &[crate::lang::functions::Argument], types: &model::common::types::all::Pass) -> String {
    let parts: Vec<String> = args
        .iter()
        .filter_map(|arg| {
            let ty = types.get(arg.ty)?;
            // If the arg is a pointer-to-service, use the target service class name.
            let name = if let TypeKind::Pointer(p) = &ty.kind {
                if let Some(target) = types.get(p.target) {
                    if matches!(&target.kind, TypeKind::Service) {
                        &target.name
                    } else {
                        &ty.name
                    }
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
