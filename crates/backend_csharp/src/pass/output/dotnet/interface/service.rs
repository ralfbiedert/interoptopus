//! Renders service interfaces (e.g. `IFoo<TSelf>`) with method declarations.
//!
//! Each service produces an interface with a self-referencing generic constraint:
//!
//! ```csharp
//! public interface IFoo<TSelf> where TSelf : IFoo<TSelf>
//! {
//!     static abstract TSelf Create();
//!     void Bar(int x);
//!     int GetAccumulator();
//! }
//! ```
//!
//! Wire<T> parameters and return values use their managed inner type (e.g.,
//! `string` for `Wire<String>`) rather than the FFI `WireBuffer`.

use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::output::{FileType, Output};
use crate::pass::output::rust::wire::WireCodeGen;
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus::inventory::Types as RsTypes;
use interoptopus_backends::casing::service_method_name;
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    interfaces: HashMap<Output, Vec<String>>,
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
        services: &model::common::service::all::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        id_maps: &model::common::id_map::Pass,
        rs_types: &RsTypes,
    ) -> OutputResult {
        let codegen = WireCodeGen { rs_types };

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut all_interfaces = Vec::new();

            for (_svc_id, svc) in services.iter() {
                let Some(type_info) = types.get(svc.ty) else { continue };
                let type_name = &type_info.name;
                let interface_name = format!("I{type_name}");

                let mut members = Vec::new();

                // Constructors → `static abstract TSelf MethodName(args…);`
                for &fn_id in &svc.ctors {
                    let Some(func) = fns_all.get(fn_id) else { continue };
                    let method_name = service_method_name(type_name, &func.name);

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

                    members.push(format!("    static abstract TSelf {method_name}({args_str});"));
                }

                // Methods → `ReturnType MethodName(args…);`
                for &fn_id in &svc.methods {
                    let Some(func) = fns_all.get(fn_id) else { continue };
                    let method_name = service_method_name(type_name, &func.name);
                    let rval_name = managed_type_name(func.signature.rval, types, id_maps, &codegen);

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

                    members.push(format!("    {rval_name} {method_name}({args_str});"));
                }

                let body = members.join("\n");
                let rendered = format!(
                    "public interface {interface_name}<TSelf> where TSelf : {interface_name}<TSelf>\n{{\n{body}\n}}"
                );

                all_interfaces.push(rendered);
            }

            self.interfaces.insert(file.clone(), all_interfaces);
        }

        Ok(())
    }

    #[must_use]
    pub fn interfaces_for(&self, output: &Output) -> Option<&[String]> {
        self.interfaces.get(output).map(Vec::as_slice)
    }
}

/// Returns the managed C# type name for a type ID.
///
/// For Wire<T> types, returns the inner managed type (e.g., `string` for Wire<String>).
/// For all other types, returns the regular C# name.
fn managed_type_name(
    cs_ty: crate::lang::TypeId,
    types: &model::common::types::all::Pass,
    id_maps: &model::common::id_map::Pass,
    codegen: &WireCodeGen<'_>,
) -> String {
    let Some(ty) = types.get(cs_ty) else {
        return "void".to_string();
    };

    if let TypeKind::TypePattern(TypePattern::Wire(_inner_cs_ty)) = &ty.kind {
        // Find the Rust Wire type → get its inner type → resolve managed name.
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
