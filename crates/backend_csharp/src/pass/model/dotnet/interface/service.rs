//! Builds service interface models (`IFoo<TSelf>`) from service definitions.
//!
//! Pointer-to-service types in return positions are resolved to service class names.
//! Result types wrapping service handles are replaced with their managed siblings
//! (created by the `service_type_siblings` pass).

use crate::lang::plugin::interface::{Interface, InterfaceKind, Method, MethodKind};
use crate::pass::Outcome::Unchanged;
use crate::pass::model::dotnet::interface::{csharp_signature, resolve_interface_rval};
use crate::pass::{ModelResult, PassInfo, model};
use interoptopus::lang::meta::{Emission, FileEmission};
use interoptopus_backends::casing::service_method_name;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    interfaces: Vec<Interface>,
    done: bool,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, interfaces: Vec::new(), done: false }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        services: &model::common::service::all::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        siblings: &model::dotnet::service_type_siblings::Pass,
    ) -> ModelResult {
        if self.done {
            return Ok(Unchanged);
        }

        if services.iter().next().is_none() {
            return Ok(Unchanged);
        }

        let service_return_map = siblings.build_service_return_map(services, fns_all, types);

        let mut sorted_services: Vec<_> = services.iter().collect();
        sorted_services.sort_by_key(|(_, svc)| types.get(svc.ty).map_or("", |t| t.name.as_str()));

        let mut interfaces = Vec::new();

        for (_svc_id, svc) in sorted_services {
            let Some(type_info) = types.get(svc.ty) else { continue };
            let type_name = &type_info.name;
            let interface_name = format!("I{type_name}");

            let mut methods = Vec::new();

            for &fn_id in &svc.ctors {
                let Some(func) = fns_all.get(fn_id) else { continue };
                let method_name = service_method_name(type_name, &func.name);
                let Some((csharp_sig, rval_name)) = csharp_signature(&func.signature.arguments, func.signature.rval, types) else {
                    return Ok(Unchanged);
                };
                methods.push(Method { name: method_name, kind: MethodKind::Static, base: fn_id, csharp: csharp_sig, rval_name });
            }

            for &fn_id in &svc.methods {
                let Some(func) = fns_all.get(fn_id) else { continue };
                let method_name = service_method_name(type_name, &func.name);
                let Some((csharp_sig, rval_name)) = csharp_signature(&func.signature.arguments, func.signature.rval, types) else {
                    return Ok(Unchanged);
                };
                let rval_name = resolve_interface_rval(&rval_name, &method_name, siblings, &service_return_map, types);
                methods.push(Method { name: method_name, kind: MethodKind::Regular, base: fn_id, csharp: csharp_sig, rval_name });
            }

            interfaces.push(Interface { name: interface_name, emission: Emission::FileEmission(FileEmission::Default), kind: InterfaceKind::Service, methods });
        }

        self.interfaces = interfaces;
        self.done = true;
        let mut outcome = Unchanged;
        outcome.changed();
        Ok(outcome)
    }

    #[must_use]
    pub fn interfaces(&self) -> &[Interface] {
        &self.interfaces
    }
}
