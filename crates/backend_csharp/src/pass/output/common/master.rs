//! Top-level output configuration.
//!
//! Determines which output files to generate by running every emittable type
//! and function through the configured dispatch. Builds a routing map so
//! downstream output passes can ask "does this item belong in this file?"
//! without duplicating dispatch logic.

use crate::dispatch::{Dispatch, Item, ItemKind};
use crate::lang::meta::FileEmission;
use crate::lang::{FunctionId, TypeId};
use crate::output::{FileType, Output, Target};
use crate::pass::{OutputResult, PassInfo, model};
use crate::template::templates;
use interoptopus_backends::template::TemplateEngine;
use std::cell::RefCell;
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
    dispatch: RefCell<Dispatch>,
    templates: TemplateEngine,
    outputs: Vec<Output>,
    /// Which output file each type should be rendered into.
    type_routing: HashMap<TypeId, Target>,
    /// Which output file each function should be rendered into.
    fn_routing: HashMap<FunctionId, Target>,
    /// Targets that received at least one `FileEmission::Default` item.
    default_targets: HashSet<Target>,
}

impl Pass {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            info: PassInfo { name: file!() },
            dispatch: RefCell::new(config.dispatch),
            templates: config.templates,
            outputs: vec![],
            type_routing: HashMap::new(),
            fn_routing: HashMap::new(),
            default_targets: HashSet::new(),
        }
    }

    pub fn process(&mut self, _pass_meta: &mut crate::pass::PassMeta, types: &model::common::types::all::Pass, fns_all: &model::common::fns::all::Pass) -> OutputResult {
        let mut seen_files: HashSet<Target> = HashSet::new();
        let dispatch = self.dispatch.get_mut();

        // Classify all emittable types
        for (&type_id, ty) in types.iter() {
            let Some(file_emission) = ty.emission.file_emission() else { continue };

            let is_default = matches!(file_emission, FileEmission::Default);
            let item = Item { kind: ItemKind::Type(type_id, ty.clone()), emission: file_emission.clone() };
            let file_name = dispatch.classify(item);

            if is_default {
                self.default_targets.insert(file_name.clone());
            }
            self.type_routing.insert(type_id, file_name.clone());
            seen_files.insert(file_name);
        }

        // Classify all emittable original functions
        for (&fn_id, func) in fns_all.originals() {
            let Some(file_emission) = func.emission.file_emission() else { continue };

            let is_default = matches!(file_emission, FileEmission::Default);
            let item = Item { kind: ItemKind::Function(fn_id, func.clone()), emission: file_emission.clone() };
            let file_name = dispatch.classify(item);

            if is_default {
                self.default_targets.insert(file_name.clone());
            }
            self.fn_routing.insert(fn_id, file_name.clone());
            seen_files.insert(file_name);
        }

        // If nothing was classified (e.g., empty inventory), still produce a default file
        if seen_files.is_empty() {
            seen_files.insert(Target::new("Interop.cs", "My.Company"));
        }

        // Build sorted output list
        let mut file_names: Vec<Target> = seen_files.into_iter().collect();
        file_names.sort();

        self.outputs = file_names.into_iter().map(|name| Output { target: name, kind: FileType::Csharp }).collect();

        Ok(())
    }

    /// Registers an additional item so its emission is classified and may
    /// create a new output file. Call after `process()`.
    pub fn register_item(&mut self, item: Item) {
        let target = self.dispatch.get_mut().classify(item);
        if !self.outputs.iter().any(|o| o.target == target) {
            self.outputs.push(Output { target, kind: FileType::Csharp });
            self.outputs.sort();
        }
    }

    #[must_use]
    pub fn templates(&self) -> &TemplateEngine {
        &self.templates
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
        self.type_routing.get(&type_id).is_some_and(|f| *f == output.target)
    }

    /// Returns true if the given function should be rendered into the given output file.
    #[must_use]
    pub fn fn_belongs_to(&self, fn_id: FunctionId, output: &Output) -> bool {
        self.fn_routing.get(&fn_id).is_some_and(|f| *f == output.target)
    }

    /// Returns true if the given item should be rendered into the given output file.
    ///
    /// Classifies the item on demand via the dispatch function.
    #[must_use]
    pub fn item_belongs_to(&self, item: Item, output: &Output) -> bool {
        let target = self.dispatch.borrow_mut().classify(item);
        target == output.target
    }

    /// Returns true if the given output received at least one `FileEmission::Default` item.
    #[must_use]
    pub fn has_default_items(&self, output: &Output) -> bool {
        self.default_targets.contains(&output.target)
    }
}
