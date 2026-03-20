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
//! Wire<T> parameters and return values use their managed inner type.

use crate::lang::plugin::TrampolineKind;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::output::{FileType, Output};
use crate::pass::output::rust::wire::WireCodeGen;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus::inventory::Types as RsTypes;
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
        id_maps: &model::common::id_map::Pass,
        rs_types: &RsTypes,
    ) -> OutputResult {
        let codegen = WireCodeGen { rs_types };

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut members = Vec::new();

            for entry in trampoline_model.entries() {
                if !matches!(entry.kind, TrampolineKind::Raw) {
                    continue;
                }

                let Some(func) = fns_all.get(entry.fn_id) else { continue };

                let rval_name = managed_type_name(func.signature.rval, types, id_maps, &codegen);
                let pascal_name = rust_to_pascal(&func.name);

                let args: Vec<String> = func
                    .signature
                    .arguments
                    .iter()
                    .filter_map(|arg| {
                        let ty_name = managed_type_name(arg.ty, types, id_maps, &codegen);
                        Some(format!("{ty_name} {}", arg.name))
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

/// Returns the managed C# type name for a type ID.
/// For Wire<T>, returns the inner managed type (e.g., `string`).
fn managed_type_name(
    cs_ty: crate::lang::TypeId,
    types: &model::common::types::all::Pass,
    id_maps: &model::common::id_map::Pass,
    codegen: &WireCodeGen<'_>,
) -> String {
    let Some(ty) = types.get(cs_ty) else {
        return "void".to_string();
    };

    if let TypeKind::TypePattern(TypePattern::Wire(_)) = &ty.kind {
        for (rs_id, rs_ty) in codegen.rs_types {
            if id_maps.ty(*rs_id) == Some(cs_ty) {
                if let interoptopus::lang::types::TypeKind::TypePattern(interoptopus::lang::types::TypePattern::Wire(inner_rs)) = &rs_ty.kind {
                    return codegen.cs_type_name(*inner_rs);
                }
            }
        }
    }

    ty.name.clone()
}
