use proc_macro2::TokenStream;
use quote::quote;

use crate::types::model::{TypeData, TypeModel, VariantData};

impl TypeModel {
    pub fn emit_typeinfo_impl(&self) -> TokenStream {
        let name = &self.name;
        let generics = &self.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let wire_safe = self.generate_wire_safe();
        let raw_safe = self.generate_raw_safe();
        let id_expr = self.generate_id();
        let kind_expr = self.generate_kind();
        let ty_expr = self.generate_ty();
        let register_expr = self.generate_register();

        quote! {
            impl #impl_generics ::interoptopus::lang::types::TypeInfo for #name #ty_generics #where_clause {
                const WIRE_SAFE: bool = #wire_safe;
                const RAW_SAFE: bool = #raw_safe;

                fn id() -> ::interoptopus::inventory::TypeId {
                    #id_expr
                }

                fn kind() -> ::interoptopus::lang::types::TypeKind {
                    #kind_expr
                }

                fn ty() -> ::interoptopus::lang::types::Type {
                    #ty_expr
                }

                fn register(inventory: &mut ::interoptopus::inventory::Inventory) {
                    #register_expr
                }
            }
        }
    }

    fn generate_wire_safe(&self) -> TokenStream {
        match &self.data {
            TypeData::Struct(struct_data) => {
                let field_checks = struct_data.fields.iter().filter(|f| !f.skip).map(|field| {
                    let ty = &field.ty;
                    quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::WIRE_SAFE }
                });

                let checks: Vec<_> = field_checks.collect();
                if checks.is_empty() {
                    quote! { true }
                } else {
                    quote! { #(#checks)&&* }
                }
            }
            TypeData::Enum(enum_data) => {
                let variant_checks = enum_data.variants.iter().filter_map(|variant| match &variant.data {
                    VariantData::Unit => None,
                    VariantData::Tuple(ty) => Some({
                        quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::WIRE_SAFE }
                    }),
                });

                let checks: Vec<_> = variant_checks.collect();
                if checks.is_empty() {
                    quote! { true }
                } else {
                    quote! { #(#checks)&&* }
                }
            }
        }
    }

    fn generate_raw_safe(&self) -> TokenStream {
        match &self.data {
            TypeData::Struct(struct_data) => {
                let field_checks = struct_data.fields.iter().filter(|f| !f.skip).map(|field| {
                    let ty = &field.ty;
                    quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::RAW_SAFE }
                });

                let checks: Vec<_> = field_checks.collect();
                if checks.is_empty() {
                    quote! { true }
                } else {
                    quote! { #(#checks)&&* }
                }
            }
            TypeData::Enum(enum_data) => {
                let variant_checks = enum_data.variants.iter().filter_map(|variant| match &variant.data {
                    VariantData::Unit => None,
                    VariantData::Tuple(ty) => Some({
                        quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::RAW_SAFE }
                    }),
                });

                let checks: Vec<_> = variant_checks.collect();
                if checks.is_empty() {
                    quote! { true }
                } else {
                    quote! { #(#checks)&&* }
                }
            }
        }
    }

    fn generate_id(&self) -> TokenStream {
        let name = &self.name;
        let full_type = if self.generics.params.is_empty() {
            quote! { #name }
        } else {
            quote! { Self }
        };

        quote! {
            ::interoptopus::type_id!(#full_type)
        }
    }

    fn generate_kind(&self) -> TokenStream {
        if self.args.service {
            return quote! { ::interoptopus::lang::types::TypeKind::Service };
        }

        if self.args.opaque {
            return quote! { ::interoptopus::lang::types::TypeKind::Opaque };
        }

        match &self.data {
            TypeData::Struct(struct_data) => {
                let fields = struct_data.fields.iter().filter(|f| !f.skip).enumerate().map(|(index, field)| {
                    let field_name = if let Some(name) = &field.name {
                        name.to_string()
                    } else {
                        format!("field_{index}")
                    };
                    let ty = &field.ty;
                    let field_docs = field.docs.join("\n");
                    quote! {
                        ::interoptopus::lang::types::Field {
                            name: #field_name.to_string(),
                            docs: ::interoptopus::lang::meta::Docs::from_line(#field_docs),
                            visibility: ::interoptopus::lang::meta::Visibility::Public,
                            ty: <#ty as ::interoptopus::lang::types::TypeInfo>::id(),
                        }
                    }
                });

                let repr = self.generate_repr();

                quote! {
                    ::interoptopus::lang::types::TypeKind::Struct(
                        ::interoptopus::lang::types::Struct {
                            fields: vec![#(#fields),*],
                            repr: #repr,
                        }
                    )
                }
            }
            TypeData::Enum(enum_data) => {
                let variants = enum_data.variants.iter().map(|variant| {
                    let variant_name = variant.name.to_string();
                    let variant_docs = variant.docs.join("\n");
                    let kind = match &variant.data {
                        VariantData::Unit => {
                            let value = if let Some(_discriminant) = &variant.discriminant {
                                // For now, we'll use 0 as default - proper evaluation would need more work
                                quote! { 0 }
                            } else {
                                quote! { 0 }
                            };
                            quote! {
                                ::interoptopus::lang::types::VariantKind::Unit(#value)
                            }
                        }
                        VariantData::Tuple(ty) => {
                            quote! {
                                ::interoptopus::lang::types::VariantKind::Tuple(
                                    <#ty as ::interoptopus::lang::types::TypeInfo>::id()
                                )
                            }
                        }
                    };

                    quote! {
                        ::interoptopus::lang::types::Variant {
                            name: #variant_name.to_string(),
                            docs: ::interoptopus::lang::meta::Docs::from_line(#variant_docs),
                            kind: #kind,
                        }
                    }
                });

                let repr = self.generate_repr();

                quote! {
                    ::interoptopus::lang::types::TypeKind::Enum(
                        ::interoptopus::lang::types::Enum {
                            variants: vec![#(#variants),*],
                            repr: #repr,
                        }
                    )
                }
            }
        }
    }

    fn generate_repr(&self) -> TokenStream {
        if self.args.service {
            // Services don't have a meaningful layout representation
            return quote! {
                ::interoptopus::lang::types::Repr {
                    layout: ::interoptopus::lang::types::Layout::Opaque,
                    alignment: None,
                }
            };
        }

        let layout = if self.args.opaque {
            quote! { ::interoptopus::lang::types::Layout::Opaque }
        } else if self.args.transparent {
            quote! { ::interoptopus::lang::types::Layout::Transparent }
        } else if self.args.packed {
            quote! { ::interoptopus::lang::types::Layout::Packed }
        } else {
            match &self.data {
                TypeData::Struct(_) => quote! { ::interoptopus::lang::types::Layout::C },
                TypeData::Enum(_) => quote! {
                    ::interoptopus::lang::types::Layout::Primitive(
                        ::interoptopus::lang::types::Primitive::U32
                    )
                },
            }
        };

        quote! {
            ::interoptopus::lang::types::Repr {
                layout: #layout,
                alignment: None,
            }
        }
    }

    fn generate_ty(&self) -> TokenStream {
        let docs_content = self.docs.join("\n");
        let _module_name = self.args.module.as_deref().unwrap_or("");

        let type_name_expr = if let Some(name) = &self.args.name {
            let name = name.clone();
            quote! { #name.to_string() }
        } else if self.generics.params.is_empty() {
            let base_name = self.name.to_string();
            quote! { #base_name.to_string() }
        } else {
            // For generic types, generate a meaningful name based on the concrete type
            quote! {
                {
                    let type_name = std::any::type_name::<Self>();
                    // Remove the module path and keep only the type name
                    type_name.split("::").last().unwrap_or(type_name).to_string()
                }
            }
        };

        quote! {
            ::interoptopus::lang::types::Type {
                name: #type_name_expr,
                visibility: ::interoptopus::lang::meta::Visibility::Public,
                docs: ::interoptopus::lang::meta::Docs::from_line(#docs_content),
                emission: ::interoptopus::lang::meta::Emission::External,
                kind: Self::kind(),
            }
        }
    }

    fn generate_register(&self) -> TokenStream {
        let type_registration = quote! {
            inventory.register_type(Self::id(), Self::ty());
        };

        let field_registrations = match &self.data {
            TypeData::Struct(struct_data) => {
                let registrations = struct_data.fields.iter().filter(|f| !f.skip).map(|field| {
                    let ty = &field.ty;
                    quote! {
                        <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
                    }
                });
                quote! { #(#registrations)* }
            }
            TypeData::Enum(enum_data) => {
                let registrations = enum_data.variants.iter().filter_map(|variant| match &variant.data {
                    VariantData::Unit => None,
                    VariantData::Tuple(ty) => Some(quote! {
                        <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
                    }),
                });
                quote! { #(#registrations)* }
            }
        };

        quote! {
            #field_registrations
            #type_registration
        }
    }
}
