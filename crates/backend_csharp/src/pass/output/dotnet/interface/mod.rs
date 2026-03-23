use crate::pass::model;

pub mod plugin;
pub mod service;

fn format_args(args: &[crate::lang::functions::Argument], types: &model::common::types::all::Pass) -> String {
    let parts: Vec<String> = args
        .iter()
        .filter_map(|arg| {
            let ty_name = types.get(arg.ty).map(|t| &t.name)?;
            Some(format!("{ty_name} {}", arg.name))
        })
        .collect();
    parts.join(", ")
}
