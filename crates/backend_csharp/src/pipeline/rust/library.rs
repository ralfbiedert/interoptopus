use crate::pass::meta;
use crate::pass::model;
use crate::pass::output;
use crate::pass::{OutputResult, PassMeta};
use crate::pipeline::{loop_model_passes_until_done, RustLibraryBuilder};
use crate::plugin::{PostModelPass, PostOutputPass, RustLibraryPlugin};
use crate::Error;
use interoptopus::inventory::RustInventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

#[derive(Default)]
pub struct RustLibraryConfig {
    pub meta_info: meta::info::Config,
    pub model_id_maps: model::id::Config,
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
    pub model_type_map: model::types::map::Config,
    pub model_fn_map: model::fns::Config,
    pub model_final: model::r#final::Config,
    pub output_master: output::master::Config,
    pub output_conversion_invoke: output::conversion::managed::Config,
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
    pub output_fn_imports: output::fns::import::Config,
    pub output_header: output::header::Config,
    pub output_util: output::types::util::Config,
    pub output_using: output::r#using::Config,
    pub output_final: output::r#final::Config,
    _hidden: PhantomData<()>,
}

pub struct IntermediateOutputPasses {
    pub conversion_invoke: output::conversion::managed::Pass,
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
    pub fn_imports: output::fns::import::Pass,
    pub header: output::header::Pass,
    pub util: output::types::util::Pass,
    pub using: output::r#using::Pass,
}

pub struct RustLibrary {
    // Basic input
    inventory: RustInventory,

    // Model passes (transform and enrich data)
    meta_info: meta::info::Pass,
    model_id_maps: model::id::Pass,
    model_type_kinds: model::types::kind::Pass,
    model_type_map_primitives: model::types::kind::primitives::Pass,
    model_type_map_array: model::types::kind::array::Pass,
    model_type_map_delegate: model::types::kind::delegate::Pass,
    model_type_map_pointer: model::types::kind::pointer::Pass,
    model_type_map_service: model::types::kind::service::Pass,
    model_type_map_patterns: model::types::kind::patterns::Pass,
    model_type_fallback: model::types::fallback::Pass,
    model_type_map_enum_variants: model::types::kind::enum_variants::Pass,
    model_type_map_enum: model::types::kind::r#enum::Pass,
    model_type_map_opaque: model::types::kind::opaque::Pass,
    model_type_map_struct_fields: model::types::kind::struct_fields::Pass,
    model_type_managed_conversion: model::types::info::managed_conversion::Pass,
    model_type_disposable: model::types::info::disposable::Pass,
    model_type_struct_class: model::types::info::struct_class::Pass,
    model_type_map_struct: model::types::kind::r#struct::Pass,
    model_type_names: model::types::names::Pass,
    model_type_map: model::types::map::Pass,
    model_fn_map: model::fns::Pass,
    model_final: model::r#final::Pass,

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
    output_final: output::r#final::Pass,

    // Output
    output: Multibuf,

    // Plugins
    plugins: Vec<Box<dyn RustLibraryPlugin>>,
}

impl RustLibrary {
    pub fn new(inventory: RustInventory) -> Self {
        Self::with_config(inventory, RustLibraryConfig::default())
    }

    pub fn builder(inventory: RustInventory) -> RustLibraryBuilder {
        RustLibraryBuilder::new(inventory)
    }

    pub(crate) fn with_config(inventory: RustInventory, config: RustLibraryConfig) -> Self {
        Self {
            inventory,
            meta_info: meta::info::Pass::new(config.meta_info),
            model_id_maps: model::id::Pass::new(config.model_id_maps),
            model_type_kinds: model::types::kind::Pass::new(config.model_type_kinds),
            model_type_map_primitives: model::types::kind::primitives::Pass::new(config.model_type_map_primitives),
            model_type_map_array: model::types::kind::array::Pass::new(config.model_type_map_array),
            model_type_map_delegate: model::types::kind::delegate::Pass::new(config.model_type_map_delegate),
            model_type_map_pointer: model::types::kind::pointer::Pass::new(config.model_type_map_pointer),
            model_type_map_service: model::types::kind::service::Pass::new(config.model_type_map_service),
            model_type_map_patterns: model::types::kind::patterns::Pass::new(config.model_type_map_patterns),
            model_type_fallback: model::types::fallback::Pass::new(config.model_type_fallback),
            model_type_map_enum_variants: model::types::kind::enum_variants::Pass::new(config.model_type_map_enum_variants),
            model_type_map_enum: model::types::kind::r#enum::Pass::new(config.model_type_map_enum),
            model_type_map_opaque: model::types::kind::opaque::Pass::new(config.model_type_map_opaque),
            model_type_map_struct_fields: model::types::kind::struct_fields::Pass::new(config.model_type_map_struct_fields),
            model_type_managed_conversion: model::types::info::managed_conversion::Pass::new(config.model_type_managed_conversion),
            model_type_disposable: model::types::info::disposable::Pass::new(config.model_type_disposable),
            model_type_struct_class: model::types::info::struct_class::Pass::new(config.model_type_struct_class),
            model_type_map_struct: model::types::kind::r#struct::Pass::new(config.model_type_map_struct),
            model_type_names: model::types::names::Pass::new(config.model_type_names),
            model_type_map: model::types::map::Pass::new(config.model_type_map),
            model_fn_map: model::fns::Pass::new(config.model_fn_map),
            model_final: model::r#final::Pass::new(config.model_final),
            output_master: output::master::Pass::new(config.output_master),
            output_passes: IntermediateOutputPasses {
                conversion_invoke: output::conversion::managed::Pass::new(config.output_conversion_invoke),
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
                fn_imports: output::fns::import::Pass::new(config.output_fn_imports),
                header: output::header::Pass::new(config.output_header),
                util: output::types::util::Pass::new(config.output_util),
                using: output::r#using::Pass::new(config.output_using),
            },
            output_final: output::r#final::Pass::new(config.output_final),
            output: Multibuf::default(),
            plugins: vec![],
        }
    }

    pub fn register_plugin(mut self, plugin: impl RustLibraryPlugin + 'static) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    fn plugin_init_pass(&mut self) {
        for plugin in self.plugins.iter_mut() {
            plugin.init(&mut self.inventory);
        }
    }

    fn plugin_post_output_pass(&mut self) -> OutputResult {
        let post_output = PostOutputPass::default();
        for plugin in self.plugins.iter_mut() {
            plugin.post_output(&mut self.output, post_output)?;
        }
        Ok(())
    }

    #[rustfmt::skip]
    pub fn process(mut self) -> Result<Multibuf, Error> {
        self.plugin_init_pass();
        let mut pass_meta = PassMeta::default();

        // Model passes
        loop_model_passes_until_done(|r| {
            pass_meta.clear();
            r.run(self.meta_info.process(&mut pass_meta))?;
            r.run(self.model_id_maps.process(&mut pass_meta, &self.inventory.types))?;
            r.run(self.model_type_kinds.process(&mut pass_meta))?;
            r.run(self.model_type_map_primitives.process(&mut pass_meta, &mut self.model_id_maps, &mut self.model_type_kinds, &self.inventory.types))?;
            r.run(self.model_type_map_array.process(&mut pass_meta, &mut self.model_id_maps, &mut self.model_type_kinds, &self.inventory.types))?;
            r.run(self.model_type_map_delegate.process(&mut pass_meta, &mut self.model_id_maps, &mut self.model_type_kinds, &self.inventory.types))?;
            r.run(self.model_type_map_pointer.process(&mut pass_meta, &mut self.model_id_maps, &mut self.model_type_kinds, &self.inventory.types))?;
            r.run(self.model_type_map_service.process(&mut pass_meta, &mut self.model_id_maps, &mut self.model_type_kinds, &self.inventory.types))?;
            r.run(self.model_type_fallback.process(&mut pass_meta, &self.model_id_maps, &self.inventory.types))?;
            r.run(self.model_type_map_patterns.process(&mut pass_meta, &mut self.model_id_maps, &mut self.model_type_kinds, &self.model_type_fallback, &self.inventory.types))?;
            r.run(self.model_type_map_enum_variants.process(&mut pass_meta, &mut self.model_id_maps, &self.inventory.types))?;
            r.run(self.model_type_map_enum.process(&mut pass_meta, &self.model_id_maps, &mut self.model_type_kinds, &self.model_type_map_enum_variants, &self.inventory.types))?;
            r.run(self.model_type_map_opaque.process(&mut pass_meta, &mut self.model_id_maps, &mut self.model_type_kinds, &self.inventory.types))?;
            r.run(self.model_type_map_struct_fields.process(&mut pass_meta, &mut self.model_id_maps, &self.inventory.types))?;
            r.run(self.model_type_managed_conversion.process(&mut pass_meta, &self.model_type_kinds))?;
            r.run(self.model_type_disposable.process(&mut pass_meta, &self.model_type_managed_conversion, &self.model_type_kinds))?;
            r.run(self.model_type_struct_class.process(&mut pass_meta, &self.model_type_managed_conversion, &self.model_type_kinds))?;
            r.run(self.model_type_map_struct.process(&mut pass_meta, &self.model_id_maps, &mut self.model_type_kinds, &self.model_type_map_struct_fields, &self.inventory.types))?;
            r.run(self.model_type_names.process(&mut pass_meta, &self.model_id_maps, &self.model_type_kinds, &self.inventory.types))?;
            r.run(self.model_type_map.process(&mut pass_meta, &self.model_type_kinds, &self.model_type_names))?;
            r.run(self.model_fn_map.process(&mut pass_meta, &mut self.model_id_maps, &self.inventory.functions))?;
            r.run(self.model_final.process(&mut pass_meta))?;

            let post_model = PostModelPass::default();
            for plugin in self.plugins.iter_mut() {
                r.run(plugin.post_model(&mut self.inventory, post_model))?;
            }
            Ok(())
        })?;

        pass_meta.lost_found.print();

        // Output passes
        self.output_master.process(&mut pass_meta)?;
        self.output_passes.conversion_invoke.process(&mut pass_meta, &self.model_type_managed_conversion, &self.model_type_kinds)?;
        self.output_passes.enum_ty.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.model_type_struct_class)?;
        self.output_passes.enum_body_unmanaged_variant.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.model_type_struct_class)?;
        self.output_passes.enum_body_unmanaged.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.output_passes.conversion_invoke)?;
        self.output_passes.enum_body_to_unmanaged.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.output_passes.conversion_invoke)?;
        self.output_passes.enum_body_as_unmanaged.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.output_passes.conversion_invoke)?;
        self.output_passes.enum_body_ctors.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names)?;
        self.output_passes.enum_body_exception_for_variant.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names)?;
        self.output_passes.enum_body_tostring.process(&mut pass_meta, &self.output_master, &self.model_type_kinds)?;
        self.output_passes.enum_body.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.model_type_struct_class, &self.model_type_disposable, &self.output_passes.enum_body_unmanaged_variant, &self.output_passes.enum_body_unmanaged, &self.output_passes.enum_body_to_unmanaged, &self.output_passes.enum_body_as_unmanaged, &self.output_passes.enum_body_ctors, &self.output_passes.enum_body_exception_for_variant, &self.output_passes.enum_body_tostring)?;
        self.output_passes.enums.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.output_passes.enum_ty, &self.output_passes.enum_body)?;
        self.output_passes.conversion_fields.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names)?;
        self.output_passes.composite_ty.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.model_type_struct_class)?;
        self.output_passes.composite_body_unmanaged.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.output_passes.conversion_invoke, &self.output_passes.conversion_fields)?;
        self.output_passes.composite_body_to_unmanaged.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.output_passes.conversion_invoke, &self.output_passes.conversion_fields)?;
        self.output_passes.composite_body_as_unmanaged.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.output_passes.conversion_invoke, &self.output_passes.conversion_fields)?;
        self.output_passes.composite_body.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.model_type_struct_class, &self.model_type_disposable, &self.output_passes.composite_body_unmanaged, &self.output_passes.composite_body_to_unmanaged, &self.output_passes.composite_body_as_unmanaged)?;
        self.output_passes.composites.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.output_passes.composite_ty, &self.output_passes.composite_body)?;
        self.output_passes.delegates.process(&mut pass_meta, &self.output_master, &self.model_type_kinds, &self.model_type_names, &self.model_type_managed_conversion, &self.output_passes.conversion_invoke)?;
        self.output_passes.fn_imports.process(&mut pass_meta, &self.output_master, &self.model_fn_map, &self.model_type_names)?;
        self.output_passes.header.process(&mut pass_meta, &self.output_master, &self.meta_info)?;
        self.output_passes.util.process(&mut pass_meta, &self.output_master)?;
        self.output_passes.using.process(&mut pass_meta, &self.output_master)?;
        self.plugin_post_output_pass()?;

        // Final output pass(es)
        self.output_final.process(&mut pass_meta, &self.meta_info, &mut self.output, &self.output_master, &self.output_passes)?;

        Ok(self.output)
    }
}
