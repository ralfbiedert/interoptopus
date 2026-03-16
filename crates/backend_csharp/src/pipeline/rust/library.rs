use crate::Error;
use crate::pass::meta;
use crate::pass::model;
use crate::pass::output;
use crate::pass::{OutputResult, PassMeta};
use crate::pipeline::{RustLibraryBuilder, loop_model_passes_until_done};
use crate::plugin::{PostModelPass, PostOutputPass, RustLibraryPlugin};
use interoptopus::inventory::RustInventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

#[derive(Default)]
pub struct RustLibraryConfig {
    pub meta_info: meta::info::Config,
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
    pub model_type_managed_conversion: model::types::info::managed_conversion::Config,
    pub model_type_disposable: model::types::info::disposable::Config,
    pub model_type_struct_class: model::types::info::struct_class::Config,
    pub model_type_map_struct: model::types::kind::r#struct::Config,
    pub model_type_names: model::types::names::Config,
    pub model_type_overload_pointer: model::types::overload::pointer::Config,
    pub model_type_overload_delegate: model::types::overload::delegate::Config,
    pub model_type_overload_all: model::types::overload::all::Config,
    pub model_type_all: model::types::all::Config,
    pub model_type_util: model::types::util::Config,
    pub model_fn_all: model::fns::all::Config,
    pub model_fn_originals: model::fns::originals::Config,
    pub model_fn_overload_simple: model::fns::overload::simple::Config,
    pub model_fn_overload_body: model::fns::overload::body::Config,
    pub model_type_async_types: model::types::info::async_types::Config,
    pub model_service_map: model::service::all::Config,
    pub model_service_method_names: model::service::method::names::Config,
    pub model_service_method_overload: model::service::method::overload::Config,
    pub output_master: output::master::Config,
    pub output_unmanaged_conversion: output::conversion::unmanaged_conversion::Config,
    pub output_unmanaged_names: output::conversion::unmanaged_names::Config,
    pub output_conversion_fields: output::conversion::fields::Config,
    pub output_enum_ty: output::types::enums::definition::Config,
    pub output_enum_body: output::types::enums::body::Config,
    pub output_enum_body_unmanaged_variant: output::types::enums::body_unmanaged_variant::Config,
    pub output_enum_body_unmanaged: output::types::enums::body_unmanaged::Config,
    pub output_enum_body_to_unmanaged: output::types::enums::body_to_unmanaged::Config,
    pub output_enum_body_as_unmanaged: output::types::enums::body_as_unmanaged::Config,
    pub output_enum_body_ctors: output::types::enums::body_ctors::Config,
    pub output_enum_body_exception_for_variant: output::types::enums::body_exception_for_variant::Config,
    pub output_enum_body_tostring: output::types::enums::body_tostring::Config,
    pub output_enum: output::types::enums::all::Config,
    pub output_composite_ty: output::types::composites::definition::Config,
    pub output_composite_body: output::types::composites::body::Config,
    pub output_composite_body_unmanaged: output::types::composites::body_unmanaged::Config,
    pub output_composite_body_to_unmanaged: output::types::composites::body_to_unmanaged::Config,
    pub output_composite_body_as_unmanaged: output::types::composites::body_as_unmanaged::Config,
    pub output_composite: output::types::composites::all::Config,
    pub output_delegates: output::types::delegates::all::Config,
    pub output_slices: output::types::slices::Config,
    pub output_fn_imports: output::fns::rust::Config,
    pub output_fn_overload_simple: output::fns::overload::simple::Config,
    pub output_fn_overload_body: output::fns::overload::body::Config,
    pub output_asynk: output::types::asynk::Config,
    pub output_service_body_ctors: output::service::body::ctors::Config,
    pub output_service_body_methods: output::service::body::methods::Config,
    pub output_services: output::service::all::Config,
    pub output_header: output::header::Config,
    pub output_util: output::types::util::Config,
    pub output_using: output::r#using::Config,
    pub output_final: output::all::Config,
    _hidden: PhantomData<()>,
}

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
    pub type_managed_conversion: model::types::info::managed_conversion::Pass,
    pub type_disposable: model::types::info::disposable::Pass,
    pub type_struct_class: model::types::info::struct_class::Pass,
    pub type_map_struct: model::types::kind::r#struct::Pass,
    pub type_names: model::types::names::Pass,
    pub type_overload_pointer: model::types::overload::pointer::Pass,
    pub type_overload_delegate: model::types::overload::delegate::Pass,
    pub type_overload_all: model::types::overload::all::Pass,
    pub type_all: model::types::all::Pass,
    pub type_util: model::types::util::Pass,
    pub fns_all: model::fns::all::Pass,
    pub fn_originals: model::fns::originals::Pass,
    pub fn_overload_simple: model::fns::overload::simple::Pass,
    pub fn_overload_body: model::fns::overload::body::Pass,
    pub type_async_types: model::types::info::async_types::Pass,
    pub service_all: model::service::all::Pass,
    pub service_method_names: model::service::method::names::Pass,
    pub service_method_overload: model::service::method::overload::Pass,
}

pub struct IntermediateOutputPasses {
    pub unmanaged_conversion: output::conversion::unmanaged_conversion::Pass,
    pub unmanaged_names: output::conversion::unmanaged_names::Pass,
    pub conversion_fields: output::conversion::fields::Pass,
    pub enum_ty: output::types::enums::definition::Pass,
    pub enum_body_unmanaged_variant: output::types::enums::body_unmanaged_variant::Pass,
    pub enum_body_unmanaged: output::types::enums::body_unmanaged::Pass,
    pub enum_body_to_unmanaged: output::types::enums::body_to_unmanaged::Pass,
    pub enum_body_as_unmanaged: output::types::enums::body_as_unmanaged::Pass,
    pub enum_body_ctors: output::types::enums::body_ctors::Pass,
    pub enum_body_exception_for_variant: output::types::enums::body_exception_for_variant::Pass,
    pub enum_body_tostring: output::types::enums::body_tostring::Pass,
    pub enum_body: output::types::enums::body::Pass,
    pub enums: output::types::enums::all::Pass,
    pub composite_ty: output::types::composites::definition::Pass,
    pub composite_body_unmanaged: output::types::composites::body_unmanaged::Pass,
    pub composite_body_to_unmanaged: output::types::composites::body_to_unmanaged::Pass,
    pub composite_body_as_unmanaged: output::types::composites::body_as_unmanaged::Pass,
    pub composite_body: output::types::composites::body::Pass,
    pub composites: output::types::composites::all::Pass,
    pub delegates: output::types::delegates::all::Pass,
    pub slices: output::types::slices::Pass,
    pub fns_rust: output::fns::rust::Pass,
    pub fns_overload_simple: output::fns::overload::simple::Pass,
    pub fns_overload_body: output::fns::overload::body::Pass,
    pub asynk: output::types::asynk::Pass,
    pub service_body_ctors: output::service::body::ctors::Pass,
    pub service_body_methods: output::service::body::methods::Pass,
    pub services: output::service::all::Pass,
    pub header: output::header::Pass,
    pub pattern_utf8string: output::types::patterns::utf8string::Pass,
    pub util: output::types::util::Pass,
    pub using: output::r#using::Pass,
}

pub struct RustLibrary {
    // Basic input
    inventory: RustInventory,

    // Model passes (transform and enrich data)
    meta_info: meta::info::Pass,
    model_passes: ModelPasses,

    // First output pass determining files to be produced
    output_master: output::master::Pass,

    // Most other output passes. Ideally these should have no cross-dependencies,
    // only depending on the models above. The last output stages (e.g., output_master)
    // then integrate all previous outputs to write the actual artifact (into Multibuf)
    // We put them into a separate struct so we don't have to later pass 20+ of them
    // to final.
    output_passes: IntermediateOutputPasses,

    // Last output stage(s). Writes a `.cs` file (later possibly other files w. other
    // master stages) into the Multibuf.
    output_final: output::all::Pass,

    // Output
    output: Multibuf,

    // Plugins
    plugins: Vec<Box<dyn RustLibraryPlugin>>,
}

impl RustLibrary {
    #[must_use]
    pub fn new(inventory: RustInventory) -> Self {
        Self::with_config(inventory, RustLibraryConfig::default())
    }

    #[must_use]
    pub fn builder(inventory: RustInventory) -> RustLibraryBuilder {
        RustLibraryBuilder::new(inventory)
    }

    pub(crate) fn with_config(inventory: RustInventory, config: RustLibraryConfig) -> Self {
        Self {
            inventory,
            meta_info: meta::info::Pass::new(config.meta_info),
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
                type_managed_conversion: model::types::info::managed_conversion::Pass::new(config.model_type_managed_conversion),
                type_disposable: model::types::info::disposable::Pass::new(config.model_type_disposable),
                type_struct_class: model::types::info::struct_class::Pass::new(config.model_type_struct_class),
                type_map_struct: model::types::kind::r#struct::Pass::new(config.model_type_map_struct),
                type_names: model::types::names::Pass::new(config.model_type_names),
                type_overload_pointer: model::types::overload::pointer::Pass::new(config.model_type_overload_pointer),
                type_overload_delegate: model::types::overload::delegate::Pass::new(config.model_type_overload_delegate),
                type_overload_all: model::types::overload::all::Pass::new(config.model_type_overload_all),
                type_all: model::types::all::Pass::new(config.model_type_all),
                type_util: model::types::util::Pass::new(config.model_type_util),
                fns_all: model::fns::all::Pass::new(config.model_fn_all),
                fn_originals: model::fns::originals::Pass::new(config.model_fn_originals),
                fn_overload_simple: model::fns::overload::simple::Pass::new(config.model_fn_overload_simple),
                fn_overload_body: model::fns::overload::body::Pass::new(config.model_fn_overload_body),
                type_async_types: model::types::info::async_types::Pass::new(config.model_type_async_types),
                service_all: model::service::all::Pass::new(config.model_service_map),
                service_method_names: model::service::method::names::Pass::new(config.model_service_method_names),
                service_method_overload: model::service::method::overload::Pass::new(config.model_service_method_overload),
            },
            output_master: output::master::Pass::new(config.output_master),
            output_passes: IntermediateOutputPasses {
                unmanaged_conversion: output::conversion::unmanaged_conversion::Pass::new(config.output_unmanaged_conversion),
                unmanaged_names: output::conversion::unmanaged_names::Pass::new(config.output_unmanaged_names),
                conversion_fields: output::conversion::fields::Pass::new(config.output_conversion_fields),
                enum_ty: output::types::enums::definition::Pass::new(config.output_enum_ty),
                enum_body_unmanaged_variant: output::types::enums::body_unmanaged_variant::Pass::new(config.output_enum_body_unmanaged_variant),
                enum_body_unmanaged: output::types::enums::body_unmanaged::Pass::new(config.output_enum_body_unmanaged),
                enum_body_to_unmanaged: output::types::enums::body_to_unmanaged::Pass::new(config.output_enum_body_to_unmanaged),
                enum_body_as_unmanaged: output::types::enums::body_as_unmanaged::Pass::new(config.output_enum_body_as_unmanaged),
                enum_body_ctors: output::types::enums::body_ctors::Pass::new(config.output_enum_body_ctors),
                enum_body_exception_for_variant: output::types::enums::body_exception_for_variant::Pass::new(config.output_enum_body_exception_for_variant),
                enum_body_tostring: output::types::enums::body_tostring::Pass::new(config.output_enum_body_tostring),
                enum_body: output::types::enums::body::Pass::new(config.output_enum_body),
                enums: output::types::enums::all::Pass::new(config.output_enum),
                composite_ty: output::types::composites::definition::Pass::new(config.output_composite_ty),
                composite_body_unmanaged: output::types::composites::body_unmanaged::Pass::new(config.output_composite_body_unmanaged),
                composite_body_to_unmanaged: output::types::composites::body_to_unmanaged::Pass::new(config.output_composite_body_to_unmanaged),
                composite_body_as_unmanaged: output::types::composites::body_as_unmanaged::Pass::new(config.output_composite_body_as_unmanaged),
                composite_body: output::types::composites::body::Pass::new(config.output_composite_body),
                composites: output::types::composites::all::Pass::new(config.output_composite),
                delegates: output::types::delegates::all::Pass::new(config.output_delegates),
                slices: output::types::slices::Pass::new(config.output_slices),
                fns_rust: output::fns::rust::Pass::new(config.output_fn_imports),
                fns_overload_simple: output::fns::overload::simple::Pass::new(config.output_fn_overload_simple),
                fns_overload_body: output::fns::overload::body::Pass::new(config.output_fn_overload_body),
                asynk: output::types::asynk::Pass::new(config.output_asynk),
                service_body_ctors: output::service::body::ctors::Pass::new(config.output_service_body_ctors),
                service_body_methods: output::service::body::methods::Pass::new(config.output_service_body_methods),
                services: output::service::all::Pass::new(config.output_services),
                header: output::header::Pass::new(config.output_header),
                pattern_utf8string: output::types::patterns::utf8string::Pass::new(Default::default()),
                util: output::types::util::Pass::new(config.output_util),
                using: output::r#using::Pass::new(config.output_using),
            },
            output_final: output::all::Pass::new(config.output_final),
            output: Multibuf::default(),
            plugins: vec![],
        }
    }

    #[must_use]
    pub fn register_plugin(mut self, plugin: impl RustLibraryPlugin + 'static) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    fn plugin_init_pass(&mut self) {
        for plugin in &mut self.plugins {
            plugin.init(&mut self.inventory);
        }
    }

    fn plugin_post_output_pass(&mut self) -> OutputResult {
        let post_output = PostOutputPass::default();
        for plugin in &mut self.plugins {
            plugin.post_output(&mut self.output, post_output)?;
        }
        Ok(())
    }


    #[rustfmt::skip]
    pub fn process(mut self) -> Result<Multibuf, Error> {
        self.plugin_init_pass();
        let mut pass_meta = PassMeta::default();

        let m = &mut self.model_passes;
        let o = &mut self.output_passes;

        // Model passes
        loop_model_passes_until_done(|r| {
            pass_meta.clear();
            r.run(self.meta_info.process(&mut pass_meta))?;
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
            r.run(m.type_managed_conversion.process(&mut pass_meta, &m.type_all))?;
            r.run(m.type_disposable.process(&mut pass_meta, &m.type_managed_conversion, &m.type_all))?;
            r.run(m.type_struct_class.process(&mut pass_meta, &m.type_managed_conversion, &m.type_all))?;
            r.run(m.type_map_struct.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &m.type_map_struct_fields, &self.inventory.types))?;
            r.run(m.type_names.process(&mut pass_meta, &m.id_maps, &m.type_kinds, &self.inventory.types))?;
            r.run(m.type_overload_pointer.process(&mut pass_meta, &mut m.type_kinds, &mut m.type_names, &mut m.type_all, &mut m.type_overload_all))?;
            r.run(m.type_overload_delegate.process(&mut pass_meta, &mut m.type_kinds, &mut m.type_names, &mut m.type_all, &mut m.type_overload_all))?;
            r.run(m.type_all.process(&mut pass_meta, &m.type_kinds, &m.type_names, &m.id_maps, &self.inventory.types))?;
            r.run(m.type_util.process(&mut pass_meta, &mut m.type_kinds, &mut m.type_names, &mut m.type_all))?;
            r.run(m.fn_originals.process(&mut pass_meta, &m.id_maps, &mut m.fns_all, &self.inventory.functions))?;
            r.run(m.fn_overload_simple.process(&mut pass_meta, &mut m.fns_all, &m.type_all, &m.type_overload_all))?;
            r.run(m.fn_overload_body.process(&mut pass_meta, &mut m.fns_all, &mut m.type_kinds, &mut m.type_names, &mut m.type_all, &m.type_overload_all))?;
            r.run(m.type_async_types.process(&mut pass_meta, &m.fns_all))?;
            r.run(m.service_all.process(&mut pass_meta, &m.id_maps, &self.inventory.services))?;
            r.run(m.service_method_names.process(&mut pass_meta, &m.service_all, &m.fns_all, &m.type_all))?;
            r.run(m.service_method_overload.process(&mut pass_meta, &mut m.service_all, &m.fns_all, &m.type_all))?;

            for plugin in &mut self.plugins {
                let post_model = PostModelPass::from_model(m);
                r.run(plugin.post_model_cycle(&self.inventory, post_model))?;
            }
            Ok(())
        })?;

        pass_meta.lost_found.print();

        for plugin in &mut self.plugins {
            let post_model = PostModelPass::from_model(m);
            plugin.post_model_all(&self.inventory, post_model)?;
        }

        // Output passes
        self.output_master.process(&mut pass_meta, &m.type_all, &m.fns_all)?;
        o.unmanaged_conversion.process(&mut pass_meta, &m.type_managed_conversion, &m.type_all)?;
        o.unmanaged_names.process(&mut pass_meta, &m.type_all, &m.type_managed_conversion)?;
        o.enum_ty.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_struct_class)?;
        o.enum_body_unmanaged_variant.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_names)?;
        o.enum_body_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion)?;
        o.enum_body_to_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion)?;
        o.enum_body_as_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion)?;
        o.enum_body_ctors.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.enum_body_exception_for_variant.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.enum_body_tostring.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.enum_body.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_struct_class, &m.type_disposable, &o.enum_body_unmanaged_variant, &o.enum_body_unmanaged, &o.enum_body_to_unmanaged, &o.enum_body_as_unmanaged, &o.enum_body_ctors, &o.enum_body_exception_for_variant, &o.enum_body_tostring)?;
        o.enums.process(&mut pass_meta, &self.output_master, &m.type_all, &o.enum_ty, &o.enum_body)?;
        o.conversion_fields.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.composite_ty.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_struct_class)?;
        o.composite_body_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion, &o.unmanaged_names, &o.conversion_fields)?;
        o.composite_body_to_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion, &o.conversion_fields)?;
        o.composite_body_as_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion, &o.conversion_fields)?;
        o.composite_body.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_struct_class, &m.type_disposable, &o.composite_body_unmanaged, &o.composite_body_to_unmanaged, &o.composite_body_as_unmanaged)?;
        o.composites.process(&mut pass_meta, &self.output_master, &m.type_all, &o.composite_ty, &o.composite_body)?;
        o.delegates.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_names, &o.unmanaged_conversion)?;
        o.slices.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_managed_conversion, &o.unmanaged_names)?;
        o.fns_rust.process(&mut pass_meta, &self.output_master, &m.fns_all, &m.type_all)?;
        o.fns_overload_simple.process(&mut pass_meta, &self.output_master, &m.fns_all, &m.type_all)?;
        o.fns_overload_body.process(&mut pass_meta, &self.output_master, &m.fns_all, &m.type_all, &m.type_overload_all)?;
        o.asynk.process(&mut pass_meta, &self.output_master, &m.type_async_types, &m.type_all, &m.type_managed_conversion)?;
        o.service_body_ctors.process(&mut pass_meta, &self.output_master, &m.service_all, &m.fns_all, &m.type_all, &m.service_method_names)?;
        o.service_body_methods.process(&mut pass_meta, &self.output_master, &m.service_all, &m.fns_all, &m.type_all, &m.service_method_names)?;
        o.services.process(&mut pass_meta, &self.output_master, &m.service_all, &m.fns_all, &m.type_all, &o.service_body_ctors, &o.service_body_methods)?;
        o.header.process(&mut pass_meta, &self.output_master, &self.meta_info)?;
        o.pattern_utf8string.process(&mut pass_meta, &self.output_master)?;
        o.util.process(&mut pass_meta, &self.output_master)?;
        o.using.process(&mut pass_meta, &self.output_master)?;
        self.plugin_post_output_pass()?;

        // Final output pass(es)
        self.output_final.process(&mut pass_meta, &self.meta_info, &mut self.output, &self.output_master, &self.output_passes)?;

        Ok(self.output)
    }
}
