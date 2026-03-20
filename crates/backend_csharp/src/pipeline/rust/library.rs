use crate::Error;
use crate::extensions::{PostModelPass, PostOutputPass, RustCodegenExtension};
use crate::pass::meta;
use crate::pass::model;
use crate::pass::output;
use crate::pass::{OutputResult, PassMeta};
use crate::pipeline::{RustLibraryBuilder, loop_model_passes_until_done};
use interoptopus::inventory::RustInventory;
use interoptopus_backends::output::Multibuf;
use std::marker::PhantomData;

#[doc(hidden)]
#[derive(Default)]
pub struct RustLibraryConfig {
    pub meta_info: meta::rust::info::Config,
    pub model_id_maps: model::common::id_map::Config,
    pub model_type_kinds: model::common::types::kind::Config,
    pub model_type_map_primitives: model::common::types::kind::primitives::Config,
    pub model_type_map_array: model::common::types::kind::array::Config,
    pub model_type_map_delegate: model::common::types::kind::delegate::Config,
    pub model_type_map_pointer: model::common::types::kind::pointer::Config,
    pub model_type_map_service: model::common::types::kind::service::Config,
    pub model_type_map_patterns: model::common::types::kind::patterns::Config,
    pub model_type_fallback: model::common::types::fallback::Config,
    pub model_type_map_enum_variants: model::common::types::kind::enum_variants::Config,
    pub model_type_map_enum: model::common::types::kind::r#enum::Config,
    pub model_type_map_opaque: model::common::types::kind::opaque::Config,
    pub model_type_map_struct_fields: model::common::types::kind::struct_fields::Config,
    pub model_type_managed_conversion: model::common::types::info::managed_conversion::Config,
    pub model_type_disposable: model::common::types::info::disposable::Config,
    pub model_type_nullable: model::common::types::info::nullable::Config,
    pub model_type_struct_class: model::common::types::info::struct_class::Config,
    pub model_type_map_struct: model::common::types::kind::r#struct::Config,
    pub model_type_names: model::common::types::names::Config,
    pub model_type_overload_pointer: model::rust::types::overload::pointer::Config,
    pub model_type_overload_delegate: model::rust::types::overload::delegate::Config,
    pub model_type_overload_all: model::rust::types::overload::all::Config,
    pub model_type_all: model::common::types::all::Config,
    pub model_type_util: model::common::types::util::Config,
    pub model_fn_all: model::common::fns::all::Config,
    pub model_fn_originals: model::common::fns::originals::Config,
    pub model_fn_overload_simple: model::rust::fns::overload::simple::Config,
    pub model_fn_overload_body: model::rust::fns::overload::body::Config,
    pub model_type_async_types: model::rust::types::info::async_types::Config,
    pub model_service_map: model::common::service::all::Config,
    pub model_service_method_names: model::rust::service::method::names::Config,
    pub model_service_method_overload: model::rust::service::method::overload::Config,
    pub model_pattern_string: model::rust::pattern::string::Config,
    pub model_pattern_vec: model::rust::pattern::vec::Config,
    pub model_wire_helpers: model::rust::wire::helpers::Config,
    pub model_wire_nested: model::rust::wire::nested::Config,
    pub output_master: output::common::master::Config,
    pub output_unmanaged_conversion: output::common::conversion::unmanaged_conversion::Config,
    pub output_unmanaged_names: output::common::conversion::unmanaged_names::Config,
    pub output_conversion_fields: output::common::conversion::fields::Config,
    pub output_enum_ty: output::common::types::enums::definition::Config,
    pub output_enum_body: output::common::types::enums::body::Config,
    pub output_enum_body_unmanaged_variant: output::common::types::enums::body_unmanaged_variant::Config,
    pub output_enum_body_unmanaged: output::common::types::enums::body_unmanaged::Config,
    pub output_enum_body_to_unmanaged: output::common::types::enums::body_to_unmanaged::Config,
    pub output_enum_body_as_unmanaged: output::common::types::enums::body_as_unmanaged::Config,
    pub output_enum_body_ctors: output::common::types::enums::body_ctors::Config,
    pub output_enum_body_exception_for_variant: output::common::types::enums::body_exception_for_variant::Config,
    pub output_enum_body_tostring: output::common::types::enums::body_tostring::Config,
    pub output_enum: output::common::types::enums::all::Config,
    pub output_composite_ty: output::common::types::composites::definition::Config,
    pub output_composite_body: output::common::types::composites::body::Config,
    pub output_composite_body_unmanaged: output::common::types::composites::body_unmanaged::Config,
    pub output_composite_body_to_unmanaged: output::common::types::composites::body_to_unmanaged::Config,
    pub output_composite_body_as_unmanaged: output::common::types::composites::body_as_unmanaged::Config,
    pub output_composite: output::common::types::composites::all::Config,
    pub output_delegates_class: output::rust::types::delegates::class::Config,
    pub output_delegates_signature: output::rust::types::delegates::signature::Config,
    pub output_slices: output::rust::pattern::slices::Config,
    pub output_vecs: output::rust::pattern::vec::Config,
    pub output_fn_imports: output::rust::fns::rust::Config,
    pub output_fn_api_guard: output::rust::fns::api_guard::Config,
    pub output_fn_overload_simple: output::rust::fns::overload::simple::Config,
    pub output_fn_overload_body: output::rust::fns::overload::body::Config,
    pub output_asynk: output::rust::types::asynk::Config,
    pub output_service_body_ctors: output::rust::service::body::ctors::Config,
    pub output_service_body_methods: output::rust::service::body::methods::Config,
    pub output_services: output::rust::service::all::Config,
    pub output_pattern_bools: output::rust::pattern::bools::Config,
    pub output_pattern_wire_buffer: output::rust::pattern::wire_buffer::Config,
    pub output_wire_types: output::rust::wire::wire_type::Config,
    pub output_wire_helper_classes: output::rust::wire::helper_classes::Config,
    pub output_wires: output::rust::wire::all::Config,
    pub output_header: output::rust::header::Config,
    pub output_util: output::common::types::util::Config,
    pub output_using: output::rust::r#using::Config,
    pub output_final: output::rust::all::Config,
    _hidden: PhantomData<()>,
}

pub struct ModelPasses {
    pub id_maps: model::common::id_map::Pass,
    pub type_kinds: model::common::types::kind::Pass,
    pub type_map_primitives: model::common::types::kind::primitives::Pass,
    pub type_map_array: model::common::types::kind::array::Pass,
    pub type_map_delegate: model::common::types::kind::delegate::Pass,
    pub type_map_pointer: model::common::types::kind::pointer::Pass,
    pub type_map_service: model::common::types::kind::service::Pass,
    pub type_map_patterns: model::common::types::kind::patterns::Pass,
    pub type_fallback: model::common::types::fallback::Pass,
    pub type_map_enum_variants: model::common::types::kind::enum_variants::Pass,
    pub type_map_enum: model::common::types::kind::r#enum::Pass,
    pub type_map_opaque: model::common::types::kind::opaque::Pass,
    pub type_map_struct_fields: model::common::types::kind::struct_fields::Pass,
    pub type_managed_conversion: model::common::types::info::managed_conversion::Pass,
    pub type_disposable: model::common::types::info::disposable::Pass,
    pub type_nullable: model::common::types::info::nullable::Pass,
    pub type_struct_class: model::common::types::info::struct_class::Pass,
    pub type_map_struct: model::common::types::kind::r#struct::Pass,
    pub type_names: model::common::types::names::Pass,
    pub type_overload_pointer: model::rust::types::overload::pointer::Pass,
    pub type_overload_delegate: model::rust::types::overload::delegate::Pass,
    pub type_overload_all: model::rust::types::overload::all::Pass,
    pub type_all: model::common::types::all::Pass,
    pub type_util: model::common::types::util::Pass,
    pub fns_all: model::common::fns::all::Pass,
    pub fn_originals: model::common::fns::originals::Pass,
    pub fn_overload_simple: model::rust::fns::overload::simple::Pass,
    pub fn_overload_body: model::rust::fns::overload::body::Pass,
    pub type_async_types: model::rust::types::info::async_types::Pass,
    pub service_all: model::common::service::all::Pass,
    pub service_method_names: model::rust::service::method::names::Pass,
    pub service_method_overload: model::rust::service::method::overload::Pass,
    pub pattern_string: model::rust::pattern::string::Pass,
    pub pattern_vec: model::rust::pattern::vec::Pass,
    pub wire_helpers: model::rust::wire::helpers::Pass,
    pub wire_nested: model::rust::wire::nested::Pass,
}

pub struct IntermediateOutputPasses {
    pub unmanaged_conversion: output::common::conversion::unmanaged_conversion::Pass,
    pub unmanaged_names: output::common::conversion::unmanaged_names::Pass,
    pub conversion_fields: output::common::conversion::fields::Pass,
    pub enum_ty: output::common::types::enums::definition::Pass,
    pub enum_body_unmanaged_variant: output::common::types::enums::body_unmanaged_variant::Pass,
    pub enum_body_unmanaged: output::common::types::enums::body_unmanaged::Pass,
    pub enum_body_to_unmanaged: output::common::types::enums::body_to_unmanaged::Pass,
    pub enum_body_as_unmanaged: output::common::types::enums::body_as_unmanaged::Pass,
    pub enum_body_ctors: output::common::types::enums::body_ctors::Pass,
    pub enum_body_exception_for_variant: output::common::types::enums::body_exception_for_variant::Pass,
    pub enum_body_tostring: output::common::types::enums::body_tostring::Pass,
    pub enum_body: output::common::types::enums::body::Pass,
    pub enums: output::common::types::enums::all::Pass,
    pub composite_ty: output::common::types::composites::definition::Pass,
    pub composite_body_unmanaged: output::common::types::composites::body_unmanaged::Pass,
    pub composite_body_to_unmanaged: output::common::types::composites::body_to_unmanaged::Pass,
    pub composite_body_as_unmanaged: output::common::types::composites::body_as_unmanaged::Pass,
    pub composite_body: output::common::types::composites::body::Pass,
    pub composites: output::common::types::composites::all::Pass,
    pub delegates_class: output::rust::types::delegates::class::Pass,
    pub delegates_signature: output::rust::types::delegates::signature::Pass,
    pub slices: output::rust::pattern::slices::Pass,
    pub vecs: output::rust::pattern::vec::Pass,
    pub fns_rust: output::rust::fns::rust::Pass,
    pub fns_api_guard: output::rust::fns::api_guard::Pass,
    pub fns_overload_simple: output::rust::fns::overload::simple::Pass,
    pub fns_overload_body: output::rust::fns::overload::body::Pass,
    pub asynk: output::rust::types::asynk::Pass,
    pub service_body_ctors: output::rust::service::body::ctors::Pass,
    pub service_body_methods: output::rust::service::body::methods::Pass,
    pub services: output::rust::service::all::Pass,
    pub header: output::rust::header::Pass,
    pub pattern_bools: output::rust::pattern::bools::Pass,
    pub pattern_utf8string: output::rust::pattern::utf8string::Pass,
    pub pattern_wire_buffer: output::rust::pattern::wire_buffer::Pass,
    pub wire_types: output::rust::wire::wire_type::Pass,
    pub wire_helper_classes: output::rust::wire::helper_classes::Pass,
    pub wires: output::rust::wire::all::Pass,
    pub util: output::common::types::util::Pass,
    pub using: output::rust::r#using::Pass,
}

/// The main entry point for C# code generation.
///
/// Holds the full multi-pass pipeline: inventory, model passes, output passes, and
/// extensions. Call [`process`](RustLibrary::process) to run the pipeline and produce
/// a [`Multibuf`] containing the generated `.cs` files.
///
/// # Example
///
/// ```rust,no_run
/// use interoptopus_csharp::RustLibrary;
/// # use interoptopus::inventory::RustInventory;
///
/// # let inventory = RustInventory::default();
/// let output = RustLibrary::builder(inventory)
///     .dll_name("my_lib")
///     .build()
///     .process()
///     .expect("code generation failed");
///
/// output.write_buffers_to("bindings/").unwrap();
/// ```
pub struct RustLibrary {
    // Basic input
    inventory: RustInventory,

    // Model passes (transform and enrich data)
    meta_info: meta::rust::info::Pass,
    model_passes: ModelPasses,

    // First output pass determining files to be produced
    output_master: output::common::master::Pass,

    // Most other output passes. Ideally these should have no cross-dependencies,
    // only depending on the models above. The last output stages (e.g., output_master)
    // then integrate all previous outputs to write the actual artifact (into Multibuf)
    // We put them into a separate struct so we don't have to later pass 20+ of them
    // to final.
    output_passes: IntermediateOutputPasses,

    // Last output stage(s). Writes a `.cs` file (later possibly other files w. other
    // master stages) into the Multibuf.
    output_final: output::rust::all::Pass,

    // Output
    output: Multibuf,

    // Plugins
    extensions: Vec<Box<dyn RustCodegenExtension>>,
}

impl RustLibrary {
    /// Creates a new `RustLibrary` with default configuration.
    #[must_use]
    pub fn new(inventory: RustInventory) -> Self {
        Self::with_config(inventory, RustLibraryConfig::default())
    }

    /// Returns a builder for configuring the code generation pipeline.
    #[must_use]
    pub fn builder(inventory: RustInventory) -> RustLibraryBuilder {
        RustLibraryBuilder::new(inventory)
    }

    #[allow(clippy::default_trait_access)]
    pub(crate) fn with_config(inventory: RustInventory, config: RustLibraryConfig) -> Self {
        Self {
            inventory,
            meta_info: meta::rust::info::Pass::new(config.meta_info),
            model_passes: ModelPasses {
                id_maps: model::common::id_map::Pass::new(config.model_id_maps),
                type_kinds: model::common::types::kind::Pass::new(config.model_type_kinds),
                type_map_primitives: model::common::types::kind::primitives::Pass::new(config.model_type_map_primitives),
                type_map_array: model::common::types::kind::array::Pass::new(config.model_type_map_array),
                type_map_delegate: model::common::types::kind::delegate::Pass::new(config.model_type_map_delegate),
                type_map_pointer: model::common::types::kind::pointer::Pass::new(config.model_type_map_pointer),
                type_map_service: model::common::types::kind::service::Pass::new(config.model_type_map_service),
                type_map_patterns: model::common::types::kind::patterns::Pass::new(config.model_type_map_patterns),
                type_fallback: model::common::types::fallback::Pass::new(config.model_type_fallback),
                type_map_enum_variants: model::common::types::kind::enum_variants::Pass::new(config.model_type_map_enum_variants),
                type_map_enum: model::common::types::kind::r#enum::Pass::new(config.model_type_map_enum),
                type_map_opaque: model::common::types::kind::opaque::Pass::new(config.model_type_map_opaque),
                type_map_struct_fields: model::common::types::kind::struct_fields::Pass::new(config.model_type_map_struct_fields),
                type_managed_conversion: model::common::types::info::managed_conversion::Pass::new(config.model_type_managed_conversion),
                type_disposable: model::common::types::info::disposable::Pass::new(config.model_type_disposable),
                type_nullable: model::common::types::info::nullable::Pass::new(config.model_type_nullable),
                type_struct_class: model::common::types::info::struct_class::Pass::new(config.model_type_struct_class),
                type_map_struct: model::common::types::kind::r#struct::Pass::new(config.model_type_map_struct),
                type_names: model::common::types::names::Pass::new(config.model_type_names),
                type_overload_pointer: model::rust::types::overload::pointer::Pass::new(config.model_type_overload_pointer),
                type_overload_delegate: model::rust::types::overload::delegate::Pass::new(config.model_type_overload_delegate),
                type_overload_all: model::rust::types::overload::all::Pass::new(config.model_type_overload_all),
                type_all: model::common::types::all::Pass::new(config.model_type_all),
                type_util: model::common::types::util::Pass::new(config.model_type_util),
                fns_all: model::common::fns::all::Pass::new(config.model_fn_all),
                fn_originals: model::common::fns::originals::Pass::new(config.model_fn_originals),
                fn_overload_simple: model::rust::fns::overload::simple::Pass::new(config.model_fn_overload_simple),
                fn_overload_body: model::rust::fns::overload::body::Pass::new(config.model_fn_overload_body),
                type_async_types: model::rust::types::info::async_types::Pass::new(config.model_type_async_types),
                service_all: model::common::service::all::Pass::new(config.model_service_map),
                service_method_names: model::rust::service::method::names::Pass::new(config.model_service_method_names),
                service_method_overload: model::rust::service::method::overload::Pass::new(config.model_service_method_overload),
                pattern_string: model::rust::pattern::string::Pass::new(config.model_pattern_string),
                pattern_vec: model::rust::pattern::vec::Pass::new(config.model_pattern_vec),
                wire_helpers: model::rust::wire::helpers::Pass::new(config.model_wire_helpers),
                wire_nested: model::rust::wire::nested::Pass::new(config.model_wire_nested),
            },
            output_master: output::common::master::Pass::new(config.output_master),
            output_passes: IntermediateOutputPasses {
                unmanaged_conversion: output::common::conversion::unmanaged_conversion::Pass::new(config.output_unmanaged_conversion),
                unmanaged_names: output::common::conversion::unmanaged_names::Pass::new(config.output_unmanaged_names),
                conversion_fields: output::common::conversion::fields::Pass::new(config.output_conversion_fields),
                enum_ty: output::common::types::enums::definition::Pass::new(config.output_enum_ty),
                enum_body_unmanaged_variant: output::common::types::enums::body_unmanaged_variant::Pass::new(config.output_enum_body_unmanaged_variant),
                enum_body_unmanaged: output::common::types::enums::body_unmanaged::Pass::new(config.output_enum_body_unmanaged),
                enum_body_to_unmanaged: output::common::types::enums::body_to_unmanaged::Pass::new(config.output_enum_body_to_unmanaged),
                enum_body_as_unmanaged: output::common::types::enums::body_as_unmanaged::Pass::new(config.output_enum_body_as_unmanaged),
                enum_body_ctors: output::common::types::enums::body_ctors::Pass::new(config.output_enum_body_ctors),
                enum_body_exception_for_variant: output::common::types::enums::body_exception_for_variant::Pass::new(config.output_enum_body_exception_for_variant),
                enum_body_tostring: output::common::types::enums::body_tostring::Pass::new(config.output_enum_body_tostring),
                enum_body: output::common::types::enums::body::Pass::new(config.output_enum_body),
                enums: output::common::types::enums::all::Pass::new(config.output_enum),
                composite_ty: output::common::types::composites::definition::Pass::new(config.output_composite_ty),
                composite_body_unmanaged: output::common::types::composites::body_unmanaged::Pass::new(config.output_composite_body_unmanaged),
                composite_body_to_unmanaged: output::common::types::composites::body_to_unmanaged::Pass::new(config.output_composite_body_to_unmanaged),
                composite_body_as_unmanaged: output::common::types::composites::body_as_unmanaged::Pass::new(config.output_composite_body_as_unmanaged),
                composite_body: output::common::types::composites::body::Pass::new(config.output_composite_body),
                composites: output::common::types::composites::all::Pass::new(config.output_composite),
                delegates_class: output::rust::types::delegates::class::Pass::new(config.output_delegates_class),
                delegates_signature: output::rust::types::delegates::signature::Pass::new(config.output_delegates_signature),
                slices: output::rust::pattern::slices::Pass::new(config.output_slices),
                vecs: output::rust::pattern::vec::Pass::new(config.output_vecs),
                fns_rust: output::rust::fns::rust::Pass::new(config.output_fn_imports),
                fns_api_guard: output::rust::fns::api_guard::Pass::new(config.output_fn_api_guard),
                fns_overload_simple: output::rust::fns::overload::simple::Pass::new(config.output_fn_overload_simple),
                fns_overload_body: output::rust::fns::overload::body::Pass::new(config.output_fn_overload_body),
                asynk: output::rust::types::asynk::Pass::new(config.output_asynk),
                service_body_ctors: output::rust::service::body::ctors::Pass::new(config.output_service_body_ctors),
                service_body_methods: output::rust::service::body::methods::Pass::new(config.output_service_body_methods),
                services: output::rust::service::all::Pass::new(config.output_services),
                header: output::rust::header::Pass::new(config.output_header),
                pattern_bools: output::rust::pattern::bools::Pass::new(config.output_pattern_bools),
                pattern_utf8string: output::rust::pattern::utf8string::Pass::new(Default::default()),
                pattern_wire_buffer: output::rust::pattern::wire_buffer::Pass::new(config.output_pattern_wire_buffer),
                wire_types: output::rust::wire::wire_type::Pass::new(config.output_wire_types),
                wire_helper_classes: output::rust::wire::helper_classes::Pass::new(config.output_wire_helper_classes),
                wires: output::rust::wire::all::Pass::new(config.output_wires),
                util: output::common::types::util::Pass::new(config.output_util),
                using: output::rust::r#using::Pass::new(config.output_using),
            },
            output_final: output::rust::all::Pass::new(config.output_final),
            output: Multibuf::default(),
            extensions: vec![],
        }
    }

    /// Registers an extension that can hook into model and output passes.
    #[must_use]
    pub fn register_extension(mut self, extension: impl RustCodegenExtension + 'static) -> Self {
        self.extensions.push(Box::new(extension));
        self
    }

    fn extension_init_pass(&mut self) {
        for ext in &mut self.extensions {
            ext.init(&mut self.inventory);
        }
    }

    fn extension_post_output_pass(&mut self) -> OutputResult {
        let post_output = PostOutputPass::default();
        for ext in &mut self.extensions {
            ext.post_output(&mut self.output, post_output)?;
        }
        Ok(())
    }


    #[rustfmt::skip]
    /// Runs the full code generation pipeline and returns the generated output buffers.
    pub fn process(mut self) -> Result<Multibuf, Error> {
        self.extension_init_pass();
        let mut pass_meta = PassMeta::default();

        let m = &mut self.model_passes;
        let o = &mut self.output_passes;

        // Model passes
        loop_model_passes_until_done(|r| {
            pass_meta.clear();
            r.run(self.meta_info.process(&mut pass_meta, &self.inventory))?;
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
            r.run(m.type_nullable.process(&mut pass_meta, &m.type_all))?;
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
            r.run(m.pattern_string.process(&mut pass_meta, &self.inventory.functions))?;
            r.run(m.pattern_vec.process(&mut pass_meta, &m.id_maps, &self.inventory.functions, &self.inventory.types))?;
            r.run(m.wire_helpers.process(&mut pass_meta, &self.inventory.functions))?;
            r.run(m.wire_nested.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &mut m.type_names, &self.inventory.types))?;
            r.run(m.service_all.process(&mut pass_meta, &m.id_maps, &self.inventory.services))?;
            r.run(m.service_method_names.process(&mut pass_meta, &m.service_all, &m.fns_all, &m.type_all))?;
            r.run(m.service_method_overload.process(&mut pass_meta, &mut m.service_all, &m.fns_all, &m.type_all))?;

            for ext in &mut self.extensions {
                let post_model = PostModelPass::from_model(m);
                r.run(ext.post_model_cycle(&self.inventory, post_model))?;
            }
            Ok(())
        })?;

        pass_meta.lost_found.print();

        for ext in &mut self.extensions {
            let post_model = PostModelPass::from_model(m);
            ext.post_model_all(&self.inventory, post_model)?;
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
        o.enum_body.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_struct_class, &m.type_disposable, &o.enum_body_unmanaged_variant, &o.enum_body_unmanaged, &o.enum_body_to_unmanaged, &o.enum_body_as_unmanaged, &o.enum_body_ctors, &o.enum_body_exception_for_variant, &o.enum_body_tostring, &o.unmanaged_conversion)?;
        o.enums.process(&mut pass_meta, &self.output_master, &m.type_all, &o.enum_ty, &o.enum_body)?;
        o.conversion_fields.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.composite_ty.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_struct_class)?;
        o.composite_body_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion, &o.unmanaged_names, &o.conversion_fields)?;
        o.composite_body_to_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion, &o.conversion_fields, &m.type_nullable)?;
        o.composite_body_as_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion, &o.conversion_fields, &m.type_nullable)?;
        o.composite_body.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_struct_class, &m.type_disposable, &o.unmanaged_conversion, &o.composite_body_unmanaged, &o.composite_body_to_unmanaged, &o.composite_body_as_unmanaged)?;
        o.composites.process(&mut pass_meta, &self.output_master, &m.type_all, &o.composite_ty, &o.composite_body)?;
        o.delegates_class.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_names, &o.unmanaged_conversion)?;
        o.delegates_signature.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.slices.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_managed_conversion, &o.unmanaged_names)?;
        o.vecs.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_managed_conversion, &o.unmanaged_names, &m.pattern_vec)?;
        o.fns_rust.process(&mut pass_meta, &self.output_master, &m.fns_all, &m.type_all)?;
        o.fns_api_guard.process(&mut pass_meta, &self.output_master, &m.fns_all, &m.type_all, &self.meta_info)?;
        o.fns_overload_simple.process(&mut pass_meta, &self.output_master, &m.fns_all, &m.type_all)?;
        o.fns_overload_body.process(&mut pass_meta, &self.output_master, &m.fns_all, &m.type_all, &m.type_overload_all)?;
        o.asynk.process(&mut pass_meta, &self.output_master, &m.type_async_types, &m.type_all, &m.type_managed_conversion)?;
        o.service_body_ctors.process(&mut pass_meta, &self.output_master, &m.service_all, &m.fns_all, &m.type_all, &m.service_method_names)?;
        o.service_body_methods.process(&mut pass_meta, &self.output_master, &m.service_all, &m.fns_all, &m.type_all, &m.service_method_names)?;
        o.services.process(&mut pass_meta, &self.output_master, &m.service_all, &m.fns_all, &m.type_all, &o.service_body_ctors, &o.service_body_methods)?;
        o.header.process(&mut pass_meta, &self.output_master, &self.meta_info)?;
        o.pattern_bools.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.pattern_utf8string.process(&mut pass_meta, &self.output_master, &m.pattern_string)?;
        o.pattern_wire_buffer.process(&mut pass_meta, &self.output_master, &m.wire_helpers, &self.inventory.functions, &self.inventory.types)?;
        o.wire_types.process(&mut pass_meta, &self.output_master, &m.type_all, &m.id_maps, &self.inventory.types)?;
        o.wire_helper_classes.process(&mut pass_meta, &self.output_master, &m.type_all, &m.id_maps, &self.inventory.types)?;
        o.wires.process(&mut pass_meta, &self.output_master, &o.wire_types, &o.wire_helper_classes)?;
        o.util.process(&mut pass_meta, &self.output_master)?;
        o.using.process(&mut pass_meta, &self.output_master)?;
        self.extension_post_output_pass()?;

        // Final output pass(es)
        self.output_final.process(&mut pass_meta, &self.meta_info, &mut self.output, &self.output_master, &self.output_passes)?;

        Ok(self.output)
    }
}
