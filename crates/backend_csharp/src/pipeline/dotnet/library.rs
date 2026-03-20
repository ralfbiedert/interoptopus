use crate::pass::{model, output, PassMeta};
use crate::pipeline::loop_model_passes_until_done;
use crate::Error;
use interoptopus::inventory::ForeignInventory;
use interoptopus_backends::output::Multibuf;

/// Configuration for the .NET codegen pipeline.
pub struct DotnetLibraryConfig {
    pub plugin_name: String,
    pub namespace: String,
    pub model_id_maps: model::id_map::Config,
    pub model_type_kinds: model::types::kind::Config,
    pub model_type_map_primitives: model::types::kind::primitives::Config,
    pub model_type_map_array: model::types::kind::array::Config,
    pub model_type_map_delegate: model::types::kind::delegate::Config,
    pub model_type_map_pointer: model::types::kind::pointer::Config,
    pub model_type_map_service: model::types::kind::service::Config,
    pub model_type_map_patterns: model::types::kind::patterns::Config,
    pub model_type_fallback: model::types::fallback::Config,
    pub model_type_map_enum_variants: model::types::kind::enum_variants::Config,
    pub model_type_map_enum: model::types::kind::r#enum::Config,
    pub model_type_map_opaque: model::types::kind::opaque::Config,
    pub model_type_map_struct_fields: model::types::kind::struct_fields::Config,
    pub model_type_map_struct: model::types::kind::r#struct::Config,
    pub model_type_names: model::types::names::Config,
    pub model_type_all: model::types::all::Config,
    pub model_fn_all: model::fns::all::Config,
    pub model_fn_originals: model::fns::originals::Config,
    pub model_service_map: model::service::all::Config,
    pub output_master: output::master::Config,
    pub output_plugin_all: output::plugin::all::Config,
}

impl Default for DotnetLibraryConfig {
    fn default() -> Self {
        Self {
            plugin_name: String::new(),
            namespace: String::from("My.Company"),
            model_id_maps: Default::default(),
            model_type_kinds: Default::default(),
            model_type_map_primitives: Default::default(),
            model_type_map_array: Default::default(),
            model_type_map_delegate: Default::default(),
            model_type_map_pointer: Default::default(),
            model_type_map_service: Default::default(),
            model_type_map_patterns: Default::default(),
            model_type_fallback: Default::default(),
            model_type_map_enum_variants: Default::default(),
            model_type_map_enum: Default::default(),
            model_type_map_opaque: Default::default(),
            model_type_map_struct_fields: Default::default(),
            model_type_map_struct: Default::default(),
            model_type_names: Default::default(),
            model_type_all: Default::default(),
            model_fn_all: Default::default(),
            model_fn_originals: Default::default(),
            model_service_map: Default::default(),
            output_master: Default::default(),
            output_plugin_all: Default::default(),
        }
    }
}

/// Model passes for the dotnet pipeline.
///
/// Only includes passes needed to populate `type_all`, `fns_all`, and `service_all`.
pub struct ModelPasses {
    pub id_maps: model::id_map::Pass,
    pub type_kinds: model::types::kind::Pass,
    pub type_map_primitives: model::types::kind::primitives::Pass,
    pub type_map_array: model::types::kind::array::Pass,
    pub type_map_delegate: model::types::kind::delegate::Pass,
    pub type_map_pointer: model::types::kind::pointer::Pass,
    pub type_map_service: model::types::kind::service::Pass,
    pub type_map_patterns: model::types::kind::patterns::Pass,
    pub type_fallback: model::types::fallback::Pass,
    pub type_map_enum_variants: model::types::kind::enum_variants::Pass,
    pub type_map_enum: model::types::kind::r#enum::Pass,
    pub type_map_opaque: model::types::kind::opaque::Pass,
    pub type_map_struct_fields: model::types::kind::struct_fields::Pass,
    pub type_map_struct: model::types::kind::r#struct::Pass,
    pub type_names: model::types::names::Pass,
    pub type_all: model::types::all::Pass,
    pub fns_all: model::fns::all::Pass,
    pub fn_originals: model::fns::originals::Pass,
    pub service_all: model::service::all::Pass,
}

/// Intermediate output passes for the dotnet pipeline.
pub struct IntermediateOutputPasses {
    pub plugin_all: output::plugin::all::Pass,
}

/// Code generation pipeline for .NET plugins (reverse interop).
///
/// Analogous to [`RustLibrary`](crate::pipeline::RustLibrary) but takes a
/// [`ForeignInventory`] describing types and functions exposed *by* a .NET
/// assembly rather than *to* one.
pub struct DotnetLibrary {
    inventory: ForeignInventory,
    config: DotnetLibraryConfig,
    model_passes: ModelPasses,
    output_master: output::master::Pass,
    output_passes: IntermediateOutputPasses,
    output: Multibuf,
}

impl DotnetLibrary {
    #[must_use]
    pub fn new(inventory: ForeignInventory) -> Self {
        Self::with_config(inventory, DotnetLibraryConfig::default())
    }

    #[must_use]
    pub fn builder(inventory: ForeignInventory) -> DotnetLibraryBuilder {
        DotnetLibraryBuilder::new(inventory)
    }

    #[allow(clippy::default_trait_access)]
    pub(crate) fn with_config(inventory: ForeignInventory, config: DotnetLibraryConfig) -> Self {
        Self {
            inventory,
            model_passes: ModelPasses {
                id_maps: model::id_map::Pass::new(config.model_id_maps),
                type_kinds: model::types::kind::Pass::new(config.model_type_kinds),
                type_map_primitives: model::types::kind::primitives::Pass::new(config.model_type_map_primitives),
                type_map_array: model::types::kind::array::Pass::new(config.model_type_map_array),
                type_map_delegate: model::types::kind::delegate::Pass::new(config.model_type_map_delegate),
                type_map_pointer: model::types::kind::pointer::Pass::new(config.model_type_map_pointer),
                type_map_service: model::types::kind::service::Pass::new(config.model_type_map_service),
                type_map_patterns: model::types::kind::patterns::Pass::new(config.model_type_map_patterns),
                type_fallback: model::types::fallback::Pass::new(config.model_type_fallback),
                type_map_enum_variants: model::types::kind::enum_variants::Pass::new(config.model_type_map_enum_variants),
                type_map_enum: model::types::kind::r#enum::Pass::new(config.model_type_map_enum),
                type_map_opaque: model::types::kind::opaque::Pass::new(config.model_type_map_opaque),
                type_map_struct_fields: model::types::kind::struct_fields::Pass::new(config.model_type_map_struct_fields),
                type_map_struct: model::types::kind::r#struct::Pass::new(config.model_type_map_struct),
                type_names: model::types::names::Pass::new(config.model_type_names),
                type_all: model::types::all::Pass::new(config.model_type_all),
                fns_all: model::fns::all::Pass::new(config.model_fn_all),
                fn_originals: model::fns::originals::Pass::new(config.model_fn_originals),
                service_all: model::service::all::Pass::new(config.model_service_map),
            },
            output_master: output::master::Pass::new(config.output_master),
            output_passes: IntermediateOutputPasses { plugin_all: output::plugin::all::Pass::new(config.output_plugin_all) },
            config: DotnetLibraryConfig { plugin_name: config.plugin_name, namespace: config.namespace, ..Default::default() },
            output: Multibuf::default(),
        }
    }

    /// Runs the code generation pipeline and returns the generated output buffers.
    #[rustfmt::skip]
    pub fn process(mut self) -> Result<Multibuf, Error> {
        let mut pass_meta = PassMeta::default();
        let m = &mut self.model_passes;
        let o = &mut self.output_passes;

        // Model passes — only what's needed to populate type_all, fns_all, service_all.
        loop_model_passes_until_done(|r| {
            pass_meta.clear();

            r.run(m.id_maps.process(&mut pass_meta, &self.inventory.types, &self.inventory.functions, &self.inventory.services))?;
            r.run(m.type_kinds.process(&mut pass_meta))?;
            r.run(m.type_map_primitives.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &self.inventory.types))?;
            r.run(m.type_map_array.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &self.inventory.types))?;
            r.run(m.type_map_delegate.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &self.inventory.types))?;
            r.run(m.type_map_pointer.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &self.inventory.types))?;
            r.run(m.type_map_service.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &self.inventory.types))?;
            r.run(m.type_fallback.process(&mut pass_meta, &m.id_maps, &self.inventory.types))?;
            r.run(m.type_map_patterns.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &m.type_fallback, &self.inventory.types))?;
            r.run(m.type_map_enum_variants.process(&mut pass_meta, &m.id_maps, &self.inventory.types))?;
            r.run(m.type_map_enum.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &m.type_map_enum_variants, &self.inventory.types))?;
            r.run(m.type_map_opaque.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &self.inventory.types))?;
            r.run(m.type_map_struct_fields.process(&mut pass_meta, &m.id_maps, &self.inventory.types))?;
            r.run(m.type_map_struct.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &m.type_map_struct_fields, &self.inventory.types))?;
            r.run(m.type_names.process(&mut pass_meta, &m.id_maps, &m.type_kinds, &self.inventory.types))?;
            r.run(m.type_all.process(&mut pass_meta, &m.type_kinds, &m.type_names, &m.id_maps, &self.inventory.types))?;
            r.run(m.fn_originals.process(&mut pass_meta, &m.id_maps, &mut m.fns_all, &self.inventory.functions))?;
            r.run(m.service_all.process(&mut pass_meta, &m.id_maps, &self.inventory.services))?;

            Ok(())
        })?;

        pass_meta.lost_found.print();

        // Output passes
        self.output_master.process(&mut pass_meta, &m.type_all, &m.fns_all)?;

        o.plugin_all.process(&self.config.plugin_name, &self.config.namespace, &self.output_master, &mut self.output)?;

        Ok(self.output)
    }
}

use super::DotnetLibraryBuilder;
