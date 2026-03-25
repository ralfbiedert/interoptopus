use crate::Error;
use crate::pass::{PassMeta, model, output};
use crate::pipeline::loop_model_passes_until_done;
use interoptopus::inventory::ForeignInventory;
use interoptopus_backends::output::Multibuf;

/// Configuration for the .NET codegen pipeline.
#[derive(Default)]
pub struct DotnetLibraryConfig {
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
    pub model_type_map_struct: model::common::types::kind::r#struct::Config,
    pub model_type_names: model::common::types::names::Config,
    pub model_type_all: model::common::types::all::Config,
    pub model_type_managed_conversion: model::common::types::info::managed_conversion::Config,
    pub model_type_struct_class: model::common::types::info::struct_class::Config,
    pub model_type_disposable: model::common::types::info::disposable::Config,
    pub model_type_nullable: model::common::types::info::nullable::Config,
    pub model_fn_all: model::common::fns::all::Config,
    pub model_fn_originals: model::common::fns::originals::Config,
    pub model_service_map: model::common::service::all::Config,
    pub model_type_util: model::common::types::util::Config,
    pub model_service_type_siblings: model::dotnet::service_type_siblings::Config,
    pub model_trampoline: model::dotnet::trampoline::Config,
    pub model_plugin_interface: model::dotnet::interface::plugin::Config,
    pub model_service_interfaces: model::dotnet::interface::service::Config,
    pub model_wire_helpers: model::common::wire::helpers::Config,
    pub model_wire_nested: model::common::wire::nested::Config,
    pub output_master: output::common::master::Config,
    pub output_unmanaged_conversion: output::common::conversion::unmanaged_conversion::Config,
    pub output_unmanaged_names: output::common::conversion::unmanaged_names::Config,
    pub output_conversion_fields: output::common::conversion::fields::Config,
    pub output_register_items: output::dotnet::register_items::Config,
    pub output_interop_raw: output::dotnet::interop::raw::Config,
    pub output_interop_service: output::dotnet::interop::service::Config,
    pub output_interop: output::dotnet::interop::all::Config,
    pub output_trampoline: output::dotnet::trampoline::Config,
    pub output_pattern_bools: output::common::pattern::bools::Config,
    pub output_wire_buffer: output::common::pattern::wire_buffer::Config,
    pub output_wire_types: output::common::wire::wire_type::Config,
    pub output_wire_helper_classes: output::common::wire::helper_classes::Config,
    pub output_wires: output::common::wire::all::Config,
    pub output_plugin_interface: output::dotnet::interface::plugin::Config,
    pub output_service_interface: output::dotnet::interface::service::Config,
    pub output_delegates_class: output::common::types::delegates::class::Config,
    pub output_delegates_signature: output::common::types::delegates::signature::Config,
    pub output_composite_ty: output::common::types::composites::definition::Config,
    pub output_composite_body_unmanaged: output::common::types::composites::body_unmanaged::Config,
    pub output_composite_body_to_unmanaged: output::common::types::composites::body_to_unmanaged::Config,
    pub output_composite_body_as_unmanaged: output::common::types::composites::body_as_unmanaged::Config,
    pub output_composite_body: output::common::types::composites::body::Config,
    pub output_composite: output::common::types::composites::all::Config,
    pub output_service_types: output::common::types::services::all::Config,
    pub output_enum_ty: output::common::types::enums::definition::Config,
    pub output_enum_body_unmanaged_variant: output::common::types::enums::body_unmanaged_variant::Config,
    pub output_enum_body_unmanaged: output::common::types::enums::body_unmanaged::Config,
    pub output_enum_body_to_unmanaged: output::common::types::enums::body_to_unmanaged::Config,
    pub output_enum_body_as_unmanaged: output::common::types::enums::body_as_unmanaged::Config,
    pub output_enum_body_ctors: output::common::types::enums::body_ctors::Config,
    pub output_enum_body_exception_for_variant: output::common::types::enums::body_exception_for_variant::Config,
    pub output_enum_body_tostring: output::common::types::enums::body_tostring::Config,
    pub output_enum_body: output::common::types::enums::body::Config,
    pub output_enum: output::common::types::enums::all::Config,
    pub output_util: output::common::types::util::Config,
    pub output_using: output::dotnet::using::Config,
    pub output_final: output::dotnet::all::Config,
}

/// Model passes for the dotnet pipeline.
///
/// Only includes passes needed to populate `type_all`, `fns_all`, and `service_all`.
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
    pub type_map_struct: model::common::types::kind::r#struct::Pass,
    pub type_names: model::common::types::names::Pass,
    pub type_all: model::common::types::all::Pass,
    pub type_managed_conversion: model::common::types::info::managed_conversion::Pass,
    pub type_struct_class: model::common::types::info::struct_class::Pass,
    pub type_disposable: model::common::types::info::disposable::Pass,
    pub type_nullable: model::common::types::info::nullable::Pass,
    pub fns_all: model::common::fns::all::Pass,
    pub fn_originals: model::common::fns::originals::Pass,
    pub service_all: model::common::service::all::Pass,
    pub type_util: model::common::types::util::Pass,
    pub service_type_siblings: model::dotnet::service_type_siblings::Pass,
    pub trampoline: model::dotnet::trampoline::Pass,
    pub plugin_interface: model::dotnet::interface::plugin::Pass,
    pub service_interfaces: model::dotnet::interface::service::Pass,
    pub wire_helpers: model::common::wire::helpers::Pass,
    pub wire_nested: model::common::wire::nested::Pass,
}

/// Intermediate output passes for the dotnet pipeline.
pub struct IntermediateOutputPasses {
    pub unmanaged_conversion: output::common::conversion::unmanaged_conversion::Pass,
    pub unmanaged_names: output::common::conversion::unmanaged_names::Pass,
    pub conversion_fields: output::common::conversion::fields::Pass,
    pub register_items: output::dotnet::register_items::Pass,
    pub interop_raw: output::dotnet::interop::raw::Pass,
    pub interop_service: output::dotnet::interop::service::Pass,
    pub interop: output::dotnet::interop::all::Pass,
    pub trampoline: output::dotnet::trampoline::Pass,
    pub pattern_bools: output::common::pattern::bools::Pass,
    pub wire_buffer: output::common::pattern::wire_buffer::Pass,
    pub wire_types: output::common::wire::wire_type::Pass,
    pub wire_helper_classes: output::common::wire::helper_classes::Pass,
    pub wires: output::common::wire::all::Pass,
    pub plugin_interface: output::dotnet::interface::plugin::Pass,
    pub service_interface: output::dotnet::interface::service::Pass,
    pub delegates_class: output::common::types::delegates::class::Pass,
    pub delegates_signature: output::common::types::delegates::signature::Pass,
    pub composite_ty: output::common::types::composites::definition::Pass,
    pub composite_body_unmanaged: output::common::types::composites::body_unmanaged::Pass,
    pub composite_body_to_unmanaged: output::common::types::composites::body_to_unmanaged::Pass,
    pub composite_body_as_unmanaged: output::common::types::composites::body_as_unmanaged::Pass,
    pub composite_body: output::common::types::composites::body::Pass,
    pub composites: output::common::types::composites::all::Pass,
    pub service_types: output::common::types::services::all::Pass,
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
    pub util: output::common::types::util::Pass,
    pub using: output::dotnet::using::Pass,
}

/// Code generation pipeline for .NET plugins (reverse interop).
///
/// Analogous to [`RustLibrary`](crate::pipeline::RustLibrary) but takes a
/// [`ForeignInventory`] describing types and functions exposed *by* a .NET
/// assembly rather than *to* one.
pub struct DotnetLibrary {
    inventory: ForeignInventory,
    model_passes: ModelPasses,
    output_master: output::common::master::Pass,
    output_passes: IntermediateOutputPasses,
    output_final: output::dotnet::all::Pass,
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
                type_map_struct: model::common::types::kind::r#struct::Pass::new(config.model_type_map_struct),
                type_names: model::common::types::names::Pass::new(config.model_type_names),
                type_all: model::common::types::all::Pass::new(config.model_type_all),
                type_managed_conversion: model::common::types::info::managed_conversion::Pass::new(config.model_type_managed_conversion),
                type_struct_class: model::common::types::info::struct_class::Pass::new(config.model_type_struct_class),
                type_disposable: model::common::types::info::disposable::Pass::new(config.model_type_disposable),
                type_nullable: model::common::types::info::nullable::Pass::new(config.model_type_nullable),
                fns_all: model::common::fns::all::Pass::new(config.model_fn_all),
                fn_originals: model::common::fns::originals::Pass::new(config.model_fn_originals),
                service_all: model::common::service::all::Pass::new(config.model_service_map),
                type_util: model::common::types::util::Pass::new(config.model_type_util),
                service_type_siblings: model::dotnet::service_type_siblings::Pass::new(config.model_service_type_siblings),
                trampoline: model::dotnet::trampoline::Pass::new(config.model_trampoline),
                plugin_interface: model::dotnet::interface::plugin::Pass::new(config.model_plugin_interface),
                service_interfaces: model::dotnet::interface::service::Pass::new(config.model_service_interfaces),
                wire_helpers: model::common::wire::helpers::Pass::new(config.model_wire_helpers),
                wire_nested: model::common::wire::nested::Pass::new(config.model_wire_nested),
            },
            output_master: output::common::master::Pass::new(config.output_master),
            output_passes: IntermediateOutputPasses {
                unmanaged_conversion: output::common::conversion::unmanaged_conversion::Pass::new(config.output_unmanaged_conversion),
                unmanaged_names: output::common::conversion::unmanaged_names::Pass::new(config.output_unmanaged_names),
                conversion_fields: output::common::conversion::fields::Pass::new(config.output_conversion_fields),
                register_items: output::dotnet::register_items::Pass::new(config.output_register_items),
                interop_raw: output::dotnet::interop::raw::Pass::new(config.output_interop_raw),
                interop_service: output::dotnet::interop::service::Pass::new(config.output_interop_service),
                interop: output::dotnet::interop::all::Pass::new(config.output_interop),
                trampoline: output::dotnet::trampoline::Pass::new(config.output_trampoline),
                pattern_bools: output::common::pattern::bools::Pass::new(config.output_pattern_bools),
                wire_buffer: output::common::pattern::wire_buffer::Pass::new(config.output_wire_buffer),
                wire_types: output::common::wire::wire_type::Pass::new(config.output_wire_types),
                wire_helper_classes: output::common::wire::helper_classes::Pass::new(config.output_wire_helper_classes),
                wires: output::common::wire::all::Pass::new(config.output_wires),
                plugin_interface: output::dotnet::interface::plugin::Pass::new(config.output_plugin_interface),
                service_interface: output::dotnet::interface::service::Pass::new(config.output_service_interface),
                delegates_class: output::common::types::delegates::class::Pass::new(config.output_delegates_class),
                delegates_signature: output::common::types::delegates::signature::Pass::new(config.output_delegates_signature),
                composite_ty: output::common::types::composites::definition::Pass::new(config.output_composite_ty),
                composite_body_unmanaged: output::common::types::composites::body_unmanaged::Pass::new(config.output_composite_body_unmanaged),
                composite_body_to_unmanaged: output::common::types::composites::body_to_unmanaged::Pass::new(config.output_composite_body_to_unmanaged),
                composite_body_as_unmanaged: output::common::types::composites::body_as_unmanaged::Pass::new(config.output_composite_body_as_unmanaged),
                composite_body: output::common::types::composites::body::Pass::new(config.output_composite_body),
                composites: output::common::types::composites::all::Pass::new(config.output_composite),
                service_types: output::common::types::services::all::Pass::new(config.output_service_types),
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
                util: output::common::types::util::Pass::new(config.output_util),
                using: output::dotnet::using::Pass::new(config.output_using),
            },
            output_final: output::dotnet::all::Pass::new(config.output_final),
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
            r.run(m.type_managed_conversion.process(&mut pass_meta, &m.type_all))?;
            r.run(m.type_struct_class.process(&mut pass_meta, &m.type_managed_conversion, &m.type_all))?;
            r.run(m.type_disposable.process(&mut pass_meta, &m.type_managed_conversion, &m.type_all))?;
            r.run(m.type_nullable.process(&mut pass_meta, &m.type_all))?;
            r.run(m.type_util.process(&mut pass_meta, &mut m.type_kinds, &mut m.type_names, &mut m.type_all))?;
            r.run(m.fn_originals.process(&mut pass_meta, &m.id_maps, &mut m.fns_all, &self.inventory.functions))?;
            r.run(m.service_all.process(&mut pass_meta, &m.id_maps, &self.inventory.services))?;
            r.run(m.wire_helpers.process(&mut pass_meta, &self.inventory.functions))?;
            r.run(m.wire_nested.process(&mut pass_meta, &m.id_maps, &mut m.type_kinds, &mut m.type_names, &self.inventory.types))?;
            r.run(m.service_type_siblings.process(&mut pass_meta, &mut m.type_kinds, &mut m.type_names, &mut m.type_all))?;
            r.run(m.trampoline.process(&mut pass_meta, &m.fns_all, &m.service_all))?;
            r.run(m.plugin_interface.process(&mut pass_meta, &m.trampoline, &m.fns_all, &m.type_all, &m.service_type_siblings))?;
            r.run(m.service_interfaces.process(&mut pass_meta, &m.service_all, &m.fns_all, &m.type_all, &m.service_type_siblings))?;

            Ok(())
        })?;

        pass_meta.lost_found.print();

        // Output passes
        self.output_master.process(&mut pass_meta, &m.type_all, &m.fns_all)?;

        o.register_items.process(&mut pass_meta, &mut self.output_master, &m.plugin_interface, &m.service_interfaces)?;
        o.using.process(&mut pass_meta, &self.output_master)?;
        o.unmanaged_conversion.process(&mut pass_meta, &m.type_managed_conversion, &m.type_all)?;
        o.unmanaged_names.process(&mut pass_meta, &m.type_all, &m.type_managed_conversion)?;
        o.conversion_fields.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.composite_ty.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_struct_class)?;
        o.composite_body_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion, &o.unmanaged_names, &o.conversion_fields)?;
        o.composite_body_to_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion, &o.conversion_fields, &m.type_nullable)?;
        o.composite_body_as_unmanaged.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_conversion, &o.conversion_fields, &m.type_nullable)?;
        o.composite_body.process(&mut pass_meta, &self.output_master, &m.type_all, &m.type_struct_class, &m.type_disposable, &o.unmanaged_conversion, &o.composite_body_unmanaged, &o.composite_body_to_unmanaged, &o.composite_body_as_unmanaged)?;
        o.composites.process(&mut pass_meta, &self.output_master, &m.type_all, &o.composite_ty, &o.composite_body)?;
        o.service_types.process(&mut pass_meta, &self.output_master, &m.type_all)?;
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
        o.util.process(&mut pass_meta, &self.output_master)?;
        o.delegates_class.process(&mut pass_meta, &self.output_master, &m.type_all, &o.unmanaged_names, &o.unmanaged_conversion)?;
        o.delegates_signature.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.pattern_bools.process(&mut pass_meta, &self.output_master, &m.type_all)?;
        o.wire_buffer.process(&mut pass_meta, &self.output_master, &m.wire_helpers, &self.inventory.functions, &self.inventory.types)?;
        o.wire_types.process(&mut pass_meta, &self.output_master, &m.type_all, &m.id_maps, &self.inventory.types)?;
        o.wire_helper_classes.process(&mut pass_meta, &self.output_master, &m.type_all, &m.id_maps, &self.inventory.types)?;
        o.wires.process(&mut pass_meta, &self.output_master, &o.wire_types, &o.wire_helper_classes)?;
        o.interop_raw.process(&mut pass_meta, &self.output_master, &m.trampoline, &m.plugin_interface, &m.fns_all, &m.type_all, &o.unmanaged_names, &o.unmanaged_conversion)?;
        o.interop_service.process(&mut pass_meta, &self.output_master, &m.trampoline, &m.service_interfaces, &m.fns_all, &m.type_all, &m.service_all, &o.unmanaged_names, &o.unmanaged_conversion)?;
        o.interop.process(&mut pass_meta, &self.output_master, &m.trampoline, &o.interop_raw, &o.interop_service)?;
        o.trampoline.process(&mut pass_meta, &self.output_master, &o.interop)?;
        o.plugin_interface.process(&mut pass_meta, &self.output_master, &m.plugin_interface, &m.type_all)?;
        o.service_interface.process(&mut pass_meta, &self.output_master, &m.service_interfaces, &m.type_all)?;

        // Final output pass
        self.output_final.process(&mut pass_meta, &self.output_master, &self.output_passes, &mut self.output)?;

        Ok(self.output)
    }
}

use super::DotnetLibraryBuilder;
