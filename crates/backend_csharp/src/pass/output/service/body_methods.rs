//! Assembles the final rendered method list per service.
//!
//! For each service method, picks the best available render from the sub-passes
//! in priority order: delegate overload > ref overload > plain.

use crate::lang::ServiceId;
use crate::pass::{model, output, OutputResult, PassInfo};
use std::collections::HashMap;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    body_methods: HashMap<ServiceId, Vec<String>>,
}

impl Pass {
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, body_methods: Default::default() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        service_map: &model::service::map::Pass,
        methods_plain: &output::service::body_methods_plain::Pass,
        methods_ref: &output::service::body_methods_ref::Pass,
        methods_delegate: &output::service::body_methods_delegate::Pass,
    ) -> OutputResult {
        for (service_id, service) in service_map.iter() {
            let mut rendered_methods = Vec::new();

            for &method_fn_id in &service.methods {
                let rendered = if let Some(s) = methods_delegate.get(method_fn_id) {
                    s.to_string()
                } else if let Some(s) = methods_ref.get(method_fn_id) {
                    s.to_string()
                } else if let Some(s) = methods_plain.get(method_fn_id) {
                    s.to_string()
                } else {
                    continue;
                };

                rendered_methods.push(rendered);
            }

            self.body_methods.insert(*service_id, rendered_methods);
        }

        Ok(())
    }

    pub fn get(&self, service_id: ServiceId) -> Option<&[String]> {
        self.body_methods.get(&service_id).map(|v| v.as_slice())
    }
}

/// Derives a C# method name from the interop function name.
///
/// E.g., `service_basic_return_default_value` → `ReturnDefaultValue`.
pub(super) fn method_name_from_interop(interop_name: &str) -> String {
    // The interop name is `{service_snake}_{method_snake}`.
    // We take the last `_`-separated segment and PascalCase it.
    // TODO: This is a simplistic heuristic; may need refinement for multi-word method names.
    let method = interop_name.rsplit('_').next().unwrap_or(interop_name);
    let mut chars = method.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => interop_name.to_string(),
    }
}
