//! Builds service interface models (`IFoo<TSelf>`) from service definitions.
//!
//! Pointer-to-service types in return positions are resolved to service `TypeIds`.
//! Result types wrapping service handles are replaced with their managed siblings
//! (created by the `service_type_siblings` pass).

use crate::lang::plugin::interface::{Interface, InterfaceKind, Method, MethodKind};
use crate::pass::model::dotnet::interface::resolve_method_info;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::pattern::ExceptionError;
use interoptopus::lang::meta::{Emission, FileEmission};
use interoptopus::lang::types::TypeInfo;
use interoptopus_backends::casing::service_method_name;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    interfaces: Vec<Interface>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, interfaces: Vec::new() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        services: &model::common::service::all::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        id_maps: &model::common::id_map::Pass,
    ) -> ModelResult {
        if services.iter().next().is_none() {
            return Ok(Unchanged);
        }

        let unwrap_error_id = id_maps.ty(ExceptionError::id());
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
                let (csharp_sig, rval_id, is_async, unwrapped_result_id) = resolve_method_info(&func.signature.arguments, func.signature.rval, types, unwrap_error_id);
                methods.push(Method { name: method_name, kind: MethodKind::Static, base: fn_id, csharp: csharp_sig, rval_id, is_async, unwrapped_result_id });
            }

            for &fn_id in &svc.methods {
                let Some(func) = fns_all.get(fn_id) else { continue };
                let method_name = service_method_name(type_name, &func.name);
                let (csharp_sig, rval_id, is_async, unwrapped_result_id) = resolve_method_info(&func.signature.arguments, func.signature.rval, types, unwrap_error_id);
                methods.push(Method { name: method_name, kind: MethodKind::Regular, base: fn_id, csharp: csharp_sig, rval_id, is_async, unwrapped_result_id });
            }

            interfaces.push(Interface { name: interface_name, emission: Emission::FileEmission(FileEmission::Default), kind: InterfaceKind::Service, methods });
        }

        let mut outcome = Unchanged;
        if self.interfaces != interfaces {
            self.interfaces = interfaces;
            outcome.changed();
        }
        Ok(outcome)
    }

    #[must_use]
    pub fn interfaces(&self) -> &[Interface] {
        &self.interfaces
    }
}
