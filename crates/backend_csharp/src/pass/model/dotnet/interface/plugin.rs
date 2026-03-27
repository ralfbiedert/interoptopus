//! Builds the `IPlugin` interface model from raw (non-service) trampoline entries.

use crate::lang::plugin::interface::{Interface, InterfaceKind, Method, MethodKind};
use crate::lang::plugin::TrampolineKind;
use crate::pass::model::dotnet::interface::resolve_method_info;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
use crate::pattern::ExceptionError;
use interoptopus::lang::meta::{Emission, FileEmission};
use interoptopus::lang::types::TypeInfo;
use interoptopus_backends::casing::rust_to_pascal;

#[derive(Default)]
pub struct Config {}

pub struct Pass {
    info: PassInfo,
    interface: Option<Interface>,
}

impl Pass {
    #[must_use]
    pub fn new(_: Config) -> Self {
        Self { info: PassInfo { name: file!() }, interface: None }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        trampoline_model: &model::dotnet::trampoline::Pass,
        fns_all: &model::common::fns::all::Pass,
        types: &model::common::types::all::Pass,
        id_maps: &model::common::id_map::Pass,
    ) -> ModelResult {
        if trampoline_model.entries().is_empty() {
            return Ok(Unchanged);
        }

        let unwrap_error_id = id_maps.ty(ExceptionError::id());
        let mut methods = Vec::new();

        for entry in trampoline_model.entries() {
            if !matches!(entry.kind, TrampolineKind::Raw) {
                continue;
            }

            let Some(func) = fns_all.get(entry.fn_id) else { continue };

            let pascal_name = rust_to_pascal(&func.name);
            let (csharp_sig, rval_id, is_async, unwrapped_result_id) = resolve_method_info(&func.signature.arguments, func.signature.rval, types, unwrap_error_id);

            methods.push(Method { name: pascal_name, kind: MethodKind::Static, base: entry.fn_id, csharp: csharp_sig, rval_id, is_async, unwrapped_result_id });
        }

        let new_interface = if methods.is_empty() {
            None
        } else {
            Some(Interface { name: "IPlugin".to_string(), emission: Emission::FileEmission(FileEmission::Default), kind: InterfaceKind::Plugin, methods })
        };

        let mut outcome = Unchanged;
        if self.interface != new_interface {
            self.interface = new_interface;
            outcome.changed();
        }
        Ok(outcome)
    }

    #[must_use]
    pub fn interface(&self) -> Option<&Interface> {
        self.interface.as_ref()
    }
}
