//! Builds service interface models (`IFoo<TSelf>`) from service definitions.
//!
//! Each service produces an `Interface` with `InterfaceKind::Service`. Constructors
//! become `MethodKind::Static` methods returning `TSelf`, regular methods become
//! `MethodKind::Regular` with their C#-ified signatures.

use crate::lang::plugin::interface::{Interface, InterfaceKind, Method, MethodKind};
use crate::pass::Outcome::Unchanged;
use crate::pass::model::dotnet::interface::csharp_signature;
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
    ) -> ModelResult {
        if self.done {
            return Ok(Unchanged);
        }

        if services.iter().next().is_none() {
            return Ok(Unchanged);
        }

        let mut sorted_services: Vec<_> = services.iter().collect();
        sorted_services.sort_by_key(|(_, svc)| types.get(svc.ty).map_or("", |t| t.name.as_str()));

        let mut interfaces = Vec::new();

        for (_svc_id, svc) in sorted_services {
            let Some(type_info) = types.get(svc.ty) else { continue };
            let type_name = &type_info.name;
            let interface_name = format!("I{type_name}");

            let mut methods = Vec::new();

            // Constructors → static abstract TSelf
            for &fn_id in &svc.ctors {
                let Some(func) = fns_all.get(fn_id) else { continue };
                let method_name = service_method_name(type_name, &func.name);
                let Some((csharp_sig, rval_name)) = csharp_signature(&func.signature.arguments, func.signature.rval, types) else {
                    return Ok(Unchanged);
                };

                methods.push(Method { name: method_name, kind: MethodKind::Static, base: fn_id, csharp: csharp_sig, rval_name });
            }

            // Methods → regular instance methods
            for &fn_id in &svc.methods {
                let Some(func) = fns_all.get(fn_id) else { continue };
                let method_name = service_method_name(type_name, &func.name);
                let Some((csharp_sig, rval_name)) = csharp_signature(&func.signature.arguments, func.signature.rval, types) else {
                    return Ok(Unchanged);
                };

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
