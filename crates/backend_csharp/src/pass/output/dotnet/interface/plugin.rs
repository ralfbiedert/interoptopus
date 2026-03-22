//! Renders the `IPlugin` interface containing static abstract declarations for
//! all raw (non-service) functions.
//!
//! ```csharp
//! public interface IPlugin
//! {
//!     static abstract long DoMath(long a, long b);
//! }
//! ```
//!
use crate::lang::plugin::TrampolineKind;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::lang::types::kind::Primitive;
use crate::lang::TypeId;
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::casing::rust_to_pascal;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    interfaces: HashMap<Output, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, interfaces: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        trampoline_model: &model::dotnet::trampoline::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
    ) -> OutputResult {
        for file in output_master.outputs_of(FileType::Csharp) {
            let mut members = Vec::new();

            for entry in trampoline_model.entries() {
                if !matches!(entry.kind, TrampolineKind::Raw) {
                    continue;
                }

                let Some(func) = fns_all.get(entry.fn_id) else { continue };

                let pascal_name = rust_to_pascal(&func.name);
                let async_inner = async_callback_inner(&func.signature.arguments, types);

                let rval_name = if let Some(inner_id) = async_inner {
                    task_type_name(inner_id, types)
                } else {
                    types.get(func.signature.rval).map(|t| t.name.clone()).unwrap_or_else(|| "void".to_string())
                };

                // For async methods omit the trailing AsyncCallback parameter.
                let arg_count = if async_inner.is_some() { func.signature.arguments.len().saturating_sub(1) } else { func.signature.arguments.len() };
                let args: Vec<String> = func
                    .signature
                    .arguments
                    .iter()
                    .take(arg_count)
                    .filter_map(|arg| {
                        let ty_name = types.get(arg.ty).map(|t| &t.name)?;
                        Some(format!("{} {}", ty_name, arg.name))
                    })
                    .collect();
                let args_str = args.join(", ");

                members.push(format!("    static abstract {rval_name} {pascal_name}({args_str});"));
            }

            if members.is_empty() {
                self.interfaces.insert(file.clone(), String::new());
                continue;
            }

            let body = members.join("\n");
            let rendered = format!("public interface IPlugin\n{{\n{body}\n}}");

            self.interfaces.insert(file.clone(), rendered);
        }

        Ok(())
    }

    #[must_use]
    pub fn interface_for(&self, output: &Output) -> Option<&str> {
        self.interfaces.get(output).map(String::as_str)
    }
}

/// If the last argument is `AsyncCallback<T>`, returns the inner `TypeId`.
pub(super) fn async_callback_inner(args: &[crate::lang::functions::Argument], types: &model::common::types::all::Pass) -> Option<TypeId> {
    let last = args.last()?;
    let ty = types.get(last.ty)?;
    match &ty.kind {
        TypeKind::TypePattern(TypePattern::AsyncCallback(inner)) => Some(*inner),
        _ => None,
    }
}

/// Returns `"Task"` for void inner types or `"Task<TypeName>"` for value types.
pub(super) fn task_type_name(inner_id: TypeId, types: &model::common::types::all::Pass) -> String {
    let is_void = matches!(types.get(inner_id).map(|t| &t.kind), Some(TypeKind::Primitive(Primitive::Void)));
    if is_void {
        "Task".to_string()
    } else {
        let name = types.get(inner_id).map(|t| t.name.as_str()).unwrap_or("void");
        format!("Task<{name}>")
    }
}
