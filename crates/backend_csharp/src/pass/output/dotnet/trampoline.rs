//! Renders the `public static class Interop` with `[UnmanagedCallersOnly]` trampoline methods.
//!
//! Each trampoline entry produces one exported method. Raw functions forward to
//! `Plugin.Method(args…)`. Service ctors/methods/destructors use `GCHandle` for
//! lifetime management.
//!
//! Wire<T> parameters and return values are handled specially: the FFI signature
//! uses `WireBuffer` while the interface method uses the managed inner type.
//! The trampoline body converts between the two representations.

use crate::lang::ServiceId;
use crate::lang::plugin::TrampolineKind;
use crate::lang::service::Service;
use crate::lang::types::kind::{TypeKind, TypePattern};
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use interoptopus_backends::casing::{rust_to_pascal, service_method_name};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    trampolines: HashMap<Output, Vec<String>>,
}

/// Info about a Wire type argument for trampoline conversion.
struct WireArgInfo {
    wire_name: String,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, trampolines: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        trampoline_model: &model::dotnet::trampoline::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        services: &model::common::service::all::Pass,
    ) -> OutputResult {
        let svc_lookup: HashMap<ServiceId, &Service> = services.iter().map(|(&id, svc)| (id, svc)).collect();

        for file in output_master.outputs_of(FileType::Csharp) {
            let mut methods = Vec::new();

            for entry in trampoline_model.entries() {
                let Some(func) = fns_all.get(entry.fn_id) else { continue };

                let ffi_name = &func.name;

                // Detect Wire return type and resolve the FFI-level type name.
                let rval_wire_info = wire_info(func.signature.rval, types);
                let rval_ffi_type = if rval_wire_info.is_some() {
                    "WireBuffer"
                } else {
                    types.get(func.signature.rval).map(|t| t.name.as_str()).unwrap_or("")
                };

                // Build FFI argument list (Wire<T> → WireBuffer) and track wire args.
                let mut ffi_args: Vec<String> = Vec::new();
                let mut wire_args: HashMap<String, WireArgInfo> = HashMap::new();
                for arg in &func.signature.arguments {
                    let info = wire_info(arg.ty, types);
                    if let Some(wi) = info {
                        ffi_args.push(format!("WireBuffer {}", arg.name));
                        wire_args.insert(arg.name.clone(), wi);
                    } else {
                        let ty_name = types.get(arg.ty).map(|t| t.name.as_str()).unwrap_or("void");
                        ffi_args.push(format!("{ty_name} {}", arg.name));
                    }
                }
                let ffi_args_str = ffi_args.join(", ");

                // Build managed forwarding args (unwiring where needed).
                let managed_forward_args: Vec<String> = func
                    .signature
                    .arguments
                    .iter()
                    .map(|a| {
                        if let Some(wi) = wire_args.get(&a.name) {
                            format!("new {} {{ Buffer = {} }}.Unwire()", wi.wire_name, a.name)
                        } else {
                            a.name.clone()
                        }
                    })
                    .collect();

                let method = match &entry.kind {
                    TrampolineKind::Raw => {
                        let pascal_name = rust_to_pascal(ffi_name);
                        let forward = managed_forward_args.join(", ");

                        if rval_wire_info.is_some() {
                            let wi = rval_wire_info.as_ref().unwrap();
                            format!(
                                "    [UnmanagedCallersOnly]\n    \
                                 public static {rval_ffi_type} {ffi_name}({ffi_args_str})\n    \
                                 {{\n        \
                                     var _result = Plugin.{pascal_name}({forward});\n        \
                                     return {}.From(_result).Buffer;\n    \
                                 }}",
                                wi.wire_name
                            )
                        } else {
                            format!(
                                "    [UnmanagedCallersOnly]\n    public static {rval_ffi_type} {ffi_name}({ffi_args_str}) => Plugin.{pascal_name}({forward});"
                            )
                        }
                    }
                    TrampolineKind::ServiceCtor { service_id } => {
                        let Some(svc) = svc_lookup.get(service_id) else { continue };
                        let type_name = types.get(svc.ty).map(|t| t.name.as_str()).unwrap_or("");
                        let method_name = service_method_name(type_name, ffi_name);
                        let forward = managed_forward_args.join(", ");
                        format!(
                            "    [UnmanagedCallersOnly]\n    \
                             public static nint {ffi_name}({ffi_args_str})\n    \
                             {{\n        \
                                 var obj = {type_name}.{method_name}({forward});\n        \
                                 var handle = GCHandle.Alloc(obj);\n        \
                                 return GCHandle.ToIntPtr(handle);\n    \
                             }}"
                        )
                    }
                    TrampolineKind::ServiceMethod { service_id } => {
                        let Some(svc) = svc_lookup.get(service_id) else { continue };
                        let type_name = types.get(svc.ty).map(|t| t.name.as_str()).unwrap_or("");
                        let method_name = service_method_name(type_name, ffi_name);

                        let self_ffi_args_str = if ffi_args_str.is_empty() {
                            "nint self".to_string()
                        } else {
                            format!("nint self, {ffi_args_str}")
                        };
                        let forward = managed_forward_args.join(", ");

                        let call = format!("obj.{method_name}({forward})");
                        let body = if let Some(wi) = &rval_wire_info {
                            format!(
                                "        var _result = {call};\n        return {}.From(_result).Buffer;",
                                wi.wire_name
                            )
                        } else if rval_ffi_type == "void" {
                            format!("        {call};")
                        } else {
                            format!("        return {call};")
                        };

                        format!(
                            "    [UnmanagedCallersOnly]\n    \
                             public static {rval_ffi_type} {ffi_name}({self_ffi_args_str})\n    \
                             {{\n        \
                                 var handle = GCHandle.FromIntPtr(self);\n        \
                                 var obj = (I{type_name}<{type_name}>)handle.Target!;\n\
                                 {body}\n    \
                             }}"
                        )
                    }
                    TrampolineKind::ServiceDestructor { .. } => {
                        format!(
                            "    [UnmanagedCallersOnly]\n    \
                             public static void {ffi_name}(nint self)\n    \
                             {{\n        \
                                 var handle = GCHandle.FromIntPtr(self);\n        \
                                 handle.Free();\n    \
                             }}"
                        )
                    }
                };

                methods.push(method);
            }

            self.trampolines.insert(file.clone(), methods);
        }

        Ok(())
    }

    #[must_use]
    pub fn trampolines_for(&self, output: &Output) -> Option<&[String]> {
        self.trampolines.get(output).map(Vec::as_slice)
    }
}

/// Returns `Some(WireArgInfo)` if the given C# type is a `Wire<T>` pattern.
fn wire_info(ty_id: crate::lang::TypeId, types: &model::common::types::all::Pass) -> Option<WireArgInfo> {
    let ty = types.get(ty_id)?;
    if matches!(&ty.kind, TypeKind::TypePattern(TypePattern::Wire(_))) {
        Some(WireArgInfo { wire_name: ty.name.clone() })
    } else {
        None
    }
}
