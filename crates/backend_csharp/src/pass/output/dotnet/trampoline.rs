//! Renders the `public static class Interop` with `[UnmanagedCallersOnly]` trampoline methods.
//!
//! Each trampoline entry produces one exported method. Raw functions forward to
//! `Plugin.Method(args…)`. Service ctors/methods/destructors use `GCHandle` for
//! lifetime management.
//!
//! Wire<T> parameters and return values use the Wire type name (e.g. `WireOfString`)
//! in both the interface and the trampoline. The plugin implementor is responsible
//! for calling `.Unwire()` / `.From()` as needed.

use crate::lang::ServiceId;
use crate::lang::plugin::TrampolineKind;
use crate::lang::service::Service;
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

            // The register_trampoline entry point — always emitted so the Rust
            // host can register runtime callbacks (wire alloc/free, etc.).
            methods.push(
                "    [UnmanagedCallersOnly]\n    \
                 public static void register_trampoline(long id, IntPtr fn_ptr) => Trampolines.Register(id, fn_ptr);"
                    .to_string(),
            );

            for entry in trampoline_model.entries() {
                let Some(func) = fns_all.get(entry.fn_id) else { continue };

                let ffi_name = &func.name;
                let rval_type = types.get(func.signature.rval).map(|t| t.name.as_str()).unwrap_or("");

                // Build argument list for the [UnmanagedCallersOnly] signature
                let args: Vec<String> = func
                    .signature
                    .arguments
                    .iter()
                    .filter_map(|arg| {
                        let ty_name = types.get(arg.ty).map(|t| &t.name)?;
                        Some(format!("{} {}", ty_name, arg.name))
                    })
                    .collect();
                let args_str = args.join(", ");

                // Build forwarding argument names (no types)
                let forward_args: Vec<&str> = func.signature.arguments.iter().map(|a| a.name.as_str()).collect();

                let method = match &entry.kind {
                    TrampolineKind::Raw => {
                        let pascal_name = rust_to_pascal(ffi_name);
                        let forward = forward_args.join(", ");
                        format!(
                            "    [UnmanagedCallersOnly]\n    public static {rval_type} {ffi_name}({args_str}) => Plugin.{pascal_name}({forward});"
                        )
                    }
                    TrampolineKind::ServiceCtor { service_id } => {
                        let Some(svc) = svc_lookup.get(service_id) else { continue };
                        let type_name = types.get(svc.ty).map(|t| t.name.as_str()).unwrap_or("");
                        let method_name = service_method_name(type_name, ffi_name);
                        format!(
                            "    [UnmanagedCallersOnly]\n    \
                             public static nint {ffi_name}({args_str})\n    \
                             {{\n        \
                                 var obj = {type_name}.{method_name}({});\n        \
                                 var handle = GCHandle.Alloc(obj);\n        \
                                 return GCHandle.ToIntPtr(handle);\n    \
                             }}",
                            forward_args.join(", ")
                        )
                    }
                    TrampolineKind::ServiceMethod { service_id } => {
                        let Some(svc) = svc_lookup.get(service_id) else { continue };
                        let type_name = types.get(svc.ty).map(|t| t.name.as_str()).unwrap_or("");
                        let method_name = service_method_name(type_name, ffi_name);

                        // The inventory does not include `self` in arguments, so use all
                        // args for forwarding and prepend `nint self` to the signature.
                        let self_args_str = if args_str.is_empty() {
                            "nint self".to_string()
                        } else {
                            format!("nint self, {args_str}")
                        };
                        let forward = forward_args.join(", ");

                        let call = format!("obj.{method_name}({forward})");
                        let body = if rval_type == "void" {
                            format!("        {call};")
                        } else {
                            format!("        return {call};")
                        };

                        format!(
                            "    [UnmanagedCallersOnly]\n    \
                             public static {rval_type} {ffi_name}({self_args_str})\n    \
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
