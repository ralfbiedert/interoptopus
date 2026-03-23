//! Builds the `IPlugin` interface model from raw (non-service) trampoline entries.
//!
//! Each raw function becomes a `Method` with `MethodKind::Static` and a C#-ified
//! name and signature (async functions get `Task<T>` return types).

use crate::lang::plugin::interface::{Interface, InterfaceKind, Method, MethodKind};
use crate::lang::plugin::TrampolineKind;
use crate::pass::model::dotnet::interface::csharp_signature;
use crate::pass::Outcome::Unchanged;
use crate::pass::{model, ModelResult, PassInfo};
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
    ) -> ModelResult {
        if self.done {
            return Ok(Unchanged);
        }

        if trampoline_model.entries().is_empty() {
            return Ok(Unchanged);
        }

        let mut methods = Vec::new();

        for entry in trampoline_model.entries() {
            if !matches!(entry.kind, TrampolineKind::Raw) {
                continue;
            }

            let Some(func) = fns_all.get(entry.fn_id) else { continue };

            let pascal_name = rust_to_pascal(&func.name);
            let (csharp_sig, rval_name) = csharp_signature(&func.signature.arguments, func.signature.rval, types);

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
