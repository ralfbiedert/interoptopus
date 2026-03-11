//! Assembles the final rendered method list per service.
//!
//! For each service method, collects all available renders from the sub-passes.
//! A method may have multiple C# overloads (e.g., one with original types and
//! one accepting bare delegates). The ref variant replaces the plain variant
//! since it's strictly better, but the delegate variant is additive.

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
        methods_plain: &output::service::body::methods_plain::Pass,
        methods_ref: &output::service::body::methods_ref::Pass,
        methods_delegate: &output::service::body::methods_delegate::Pass,
    ) -> OutputResult {
        for (service_id, service) in service_map.iter() {
            let mut rendered_methods = Vec::new();

            for &method_fn_id in &service.methods {
                // Ref replaces plain (strictly better signature), but both are the
                // "base" method that uses the native types.
                if let Some(s) = methods_ref.get(method_fn_id) {
                    rendered_methods.push(s.to_string());
                } else if let Some(s) = methods_plain.get(method_fn_id) {
                    rendered_methods.push(s.to_string());
                }

                // Delegate overload is an additional C# overload accepting bare delegates.
                if let Some(s) = methods_delegate.get(method_fn_id) {
                    rendered_methods.push(s.to_string());
                }
            }

            self.body_methods.insert(*service_id, rendered_methods);
        }

        Ok(())
    }

    pub fn get(&self, service_id: ServiceId) -> Option<&[String]> {
        self.body_methods.get(&service_id).map(|v| v.as_slice())
    }
}
