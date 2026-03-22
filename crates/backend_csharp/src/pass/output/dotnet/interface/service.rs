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
use crate::output::{FileType, Output};
use crate::pass::{OutputResult, PassInfo, model, output};
use crate::pass::output::dotnet::interface::plugin::{async_callback_inner, task_type_name};
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
    ) -> OutputResult {
        for file in output_master.outputs_of(FileType::Csharp) {
            let mut all_interfaces = Vec::new();

            let mut sorted_services: Vec<_> = services.iter().collect();
            sorted_services.sort_by_key(|(_, svc)| types.get(svc.ty).map(|t| t.name.as_str()).unwrap_or(""));

            for (_svc_id, svc) in sorted_services {
                let Some(type_info) = types.get(svc.ty) else { continue };
                let type_name = &type_info.name;
                let interface_name = format!("I{type_name}");

                let mut members = Vec::new();

                // Constructors → `static abstract TSelf MethodName(args…);`
                for &fn_id in &svc.ctors {
                    let Some(func) = fns_all.get(fn_id) else { continue };
                    let method_name = service_method_name(type_name, &func.name);

                    // Ctor args exclude the return (Self), and the return type is TSelf
                    let args: Vec<String> = func
                        .signature
                        .arguments
                        .iter()
                        .filter_map(|arg| {
                            let ty = types.get(arg.ty).map(|t| &t.name)?;
                            Some(format!("{} {}", ty, arg.name))
                        })
                        .collect();
                    let args_str = args.join(", ");

                    members.push(format!("    static abstract TSelf {method_name}({args_str});"));
                }

                // Methods → `ReturnType MethodName(args…);`
                for &fn_id in &svc.methods {
                    let Some(func) = fns_all.get(fn_id) else { continue };
                    let method_name = service_method_name(type_name, &func.name);
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
                            let ty = types.get(arg.ty).map(|t| &t.name)?;
                            Some(format!("{} {}", ty, arg.name))
                        })
                        .collect();
                    let args_str = args.join(", ");

                    members.push(format!("    {rval_name} {method_name}({args_str});"));
                }

                // Destructor is not part of the interface — it's handled by the trampoline

                let body = members.join("\n");
                let rendered = format!("public interface {interface_name}<TSelf> where TSelf : {interface_name}<TSelf>\n{{\n{body}\n}}");

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
