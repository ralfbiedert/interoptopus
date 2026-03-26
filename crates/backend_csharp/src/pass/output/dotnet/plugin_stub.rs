//! Renders a `Plugin.cs` stub containing empty implementations for all
//! `IPlugin` methods and service interface methods.
//!
//! The stub is emitted with `Overwrite::Never` so that it is only written
//! when the user does not yet have a `Plugin.cs` file.
//!
//! ```csharp
//! partial class Plugin : IPlugin
//! {
//!     public static ResultFoo DoMath(long a, long b)
//!     {
//!         throw new NotImplementedException();
//!     }
//! }
//!
//! partial class MyService : IMyService<MyService>
//! {
//!     public static ResultFoo Create(uint value)
//!     {
//!         throw new NotImplementedException();
//!     }
//!
//!     public ResultBar GetValue()
//!     {
//!         throw new NotImplementedException();
//!     }
//! }
//! ```

use crate::dispatch::{Item, ItemKind};
use crate::lang::plugin::interface::{InterfaceKind, MethodKind};
use crate::output::{FileType, Output};
use crate::pass::output::dotnet::interface::{format_args, rval_display_name};
use crate::pass::{OutputResult, PassInfo, model, output};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    stubs: HashMap<Output, String>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, stubs: HashMap::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        output_master: &output::common::master::Pass,
        plugin_interface: &model::dotnet::interface::plugin::Pass,
        service_interfaces: &model::dotnet::interface::service::Pass,
        types: &model::common::types::all::Pass,
    ) -> OutputResult {
        for file in output_master.outputs_of(FileType::Csharp) {
            if !output_master.item_belongs_to(Item { kind: ItemKind::PluginStub, emission: interoptopus::lang::meta::FileEmission::Default }, file) {
                self.stubs.insert(file.clone(), String::new());
                continue;
            }

            let mut classes = Vec::new();

            // Plugin class stub
            if let Some(interface) = plugin_interface.interface() {
                let rendered = render_class_stub("Plugin", "IPlugin", None, &interface.methods, types);
                if !rendered.is_empty() {
                    classes.push(rendered);
                }
            }

            // Service class stubs
            for interface in service_interfaces.interfaces() {
                if !matches!(interface.kind, InterfaceKind::Service) {
                    continue;
                }
                // Interface name is "IFoo", service class name is "Foo"
                let class_name = interface.name.strip_prefix('I').unwrap_or(&interface.name);
                let rendered = render_class_stub(class_name, &interface.name, Some(class_name), &interface.methods, types);
                if !rendered.is_empty() {
                    classes.push(rendered);
                }
            }

            self.stubs.insert(file.clone(), classes.join("\n\n"));
        }

        Ok(())
    }

    #[must_use]
    pub fn stub_for(&self, output: &Output) -> Option<&str> {
        self.stubs.get(output).map(String::as_str)
    }
}

/// Render a partial class stub implementing the given interface.
///
/// `tself` is `Some("ClassName")` for generic service interfaces (`IFoo<TSelf>`),
/// `None` for the plain `IPlugin` interface.
fn render_class_stub(
    class_name: &str,
    interface_name: &str,
    tself: Option<&str>,
    methods: &[crate::lang::plugin::interface::Method],
    types: &model::common::types::all::Pass,
) -> String {
    let implements = match tself {
        Some(t) => format!("{interface_name}<{t}>"),
        None => interface_name.to_string(),
    };

    let mut members = Vec::new();

    for method in methods {
        let args_str = format_args(&method.csharp.arguments, types);
        let rval_name = rval_display_name(method, types);

        let prefix = match method.kind {
            MethodKind::Static => "public static",
            MethodKind::Regular => "public",
        };

        members.push(format!(
            "    {prefix} {rval_name} {}({args_str})\n    {{\n        throw new NotImplementedException();\n    }}",
            method.name
        ));
    }

    if members.is_empty() {
        return String::new();
    }

    let body = members.join("\n\n");
    format!("partial class {class_name} : {implements}\n{{\n{body}\n}}")
}
