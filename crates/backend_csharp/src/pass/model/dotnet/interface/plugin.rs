//! Builds the `IPlugin` interface model from raw (non-service) trampoline entries.

use crate::lang::plugin::TrampolineKind;
use crate::lang::plugin::interface::{Interface, InterfaceKind, Method, MethodKind};
use crate::pass::Outcome::Unchanged;
use crate::pass::model::dotnet::interface::{csharp_signature, resolve_interface_rval};
use crate::pass::{ModelResult, PassInfo, model};
use interoptopus::lang::meta::{Emission, FileEmission};
use interoptopus_backends::casing::rust_to_pascal;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    interface: Option<Interface>,
    done: bool,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, interface: None, done: false }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        trampoline_model: &model::dotnet::trampoline::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        services: &model::common::service::all::Pass,
        siblings: &model::dotnet::service_type_siblings::Pass,
    ) -> ModelResult {
        if self.done {
            return Ok(Unchanged);
        }

        if trampoline_model.entries().is_empty() {
            return Ok(Unchanged);
        }

        let service_return_map = siblings.build_service_return_map(services, fns_all, types);

        let mut methods = Vec::new();

        for entry in trampoline_model.entries() {
            if !matches!(entry.kind, TrampolineKind::Raw) {
                continue;
            }

            let Some(func) = fns_all.get(entry.fn_id) else { continue };

            let pascal_name = rust_to_pascal(&func.name);
            let Some((csharp_sig, rval_name)) = csharp_signature(&func.signature.arguments, func.signature.rval, types) else {
                return Ok(Unchanged);
            };

            let rval_name = resolve_interface_rval(&rval_name, &pascal_name, siblings, &service_return_map, types);

            methods.push(Method { name: pascal_name, kind: MethodKind::Static, base: entry.fn_id, csharp: csharp_sig, rval_name });
        }

        self.interface = if methods.is_empty() {
            None
        } else {
            Some(Interface { name: "IPlugin".to_string(), emission: Emission::FileEmission(FileEmission::Default), kind: InterfaceKind::Plugin, methods })
        };

        self.done = true;
        let mut outcome = Unchanged;
        outcome.changed();
        Ok(outcome)
    }

    #[must_use]
    pub fn interface(&self) -> Option<&Interface> {
        self.interface.as_ref()
    }
}
