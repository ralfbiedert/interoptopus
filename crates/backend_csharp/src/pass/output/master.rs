//! Top-level output configuration.
//!
//! Determines which output files to generate by running every emittable type
//! and function through the configured dispatch. Builds a routing map so
//! downstream output passes can ask "does this item belong in this file?"
//! without duplicating dispatch logic.

use crate::dispatch::{Dispatch, Item, ItemKind};
use crate::lang::meta::Emission;
use crate::lang::{FunctionId, TypeId};
use crate::output::{FileName, FileType, Output};
use crate::pass::{OutputResult, PassInfo, model};
use crate::template::templates;
use interoptopus_backends::template::TemplateEngine;
use std::collections::{HashMap, HashSet};

pub struct Config {
    pub dispatch: Dispatch,
    pub templates: TemplateEngine,
}

impl Default for Config {
    fn default() -> Self {
        Self { dispatch: Dispatch::default(), templates: templates() }
    }
}

pub struct Pass {
    info: PassInfo,
    config: Config,
    outputs: Vec<Output>,
    /// Which output file each type should be rendered into.
    type_routing: HashMap<TypeId, FileName>,
    /// Which output file each function should be rendered into.
    fn_routing: HashMap<FunctionId, FileName>,
}

impl Pass {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { info: PassInfo { name: file!() }, config, outputs: vec![], type_routing: HashMap::new(), fn_routing: HashMap::new() }
    }

    pub fn process(
        &mut self,
        _pass_meta: &mut crate::pass::PassMeta,
        types: &model::types::all::Pass,
        fn_originals: &model::fns::originals::Pass,
    ) -> OutputResult {
        let mut seen_files: HashSet<FileName> = HashSet::new();

        // Classify all emittable types
        for (&type_id, ty) in types.iter() {
            let Some(file_emission) = ty.emission.file_emission() else { continue };

            let item = Item { kind: ItemKind::Type(type_id, ty.clone()), emission: file_emission.clone() };
            let file_name = self.config.dispatch.classify(item);

            self.type_routing.insert(type_id, file_name.clone());
            seen_files.insert(file_name);
        }

        // Classify all emittable original functions
        for (&fn_id, func) in fn_originals.iter() {
            let Some(file_emission) = func.emission.file_emission() else { continue };

            let item = Item { kind: ItemKind::Function(fn_id, func.clone()), emission: file_emission.clone() };
            let file_name = self.config.dispatch.classify(item);

            self.fn_routing.insert(fn_id, file_name.clone());
            seen_files.insert(file_name);
        }

        // If nothing was classified (e.g., empty inventory), still produce a default file
        if seen_files.is_empty() {
            seen_files.insert(FileName::new("Interop.cs"));
        }

        // Build sorted output list
        let mut file_names: Vec<FileName> = seen_files.into_iter().collect();
        file_names.sort();

        self.outputs = file_names.into_iter().map(|name| Output { name, kind: FileType::Csharp }).collect();

        Ok(())
    }

    #[must_use]
    pub fn dispatch(&self) -> &Dispatch {
        &self.config.dispatch
    }

    #[must_use]
    pub fn templates(&self) -> &TemplateEngine {
        &self.config.templates
    }

    pub fn outputs(&self) -> impl Iterator<Item = &Output> {
        self.outputs.iter()
    }

    pub fn outputs_of(&self, kind: FileType) -> impl Iterator<Item = &Output> {
        self.outputs.iter().filter(move |x| x.kind == kind)
    }

    /// Returns true if the given type should be rendered into the given output file.
    #[must_use]
    pub fn type_belongs_to(&self, type_id: TypeId, output: &Output) -> bool {
        self.type_routing.get(&type_id).is_some_and(|f| *f == output.name)
    }

    /// Returns true if the given function should be rendered into the given output file.
    #[must_use]
    pub fn fn_belongs_to(&self, fn_id: FunctionId, output: &Output) -> bool {
        self.fn_routing.get(&fn_id).is_some_and(|f| *f == output.name)
    }
}
