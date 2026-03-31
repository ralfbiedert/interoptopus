use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::Error;
use syn::spanned::Spanned;

use crate::types::model::{TypeData, TypeModel, VariantData};

impl TypeModel {
    #[expect(clippy::unnecessary_wraps)]
    pub fn emit_typeinfo_impl(&self) -> Result<TokenStream, Error> {
        let name = &self.name;
        let generics = &self.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let wire_safe = self.generate_wire_safe();
        let raw_safe = self.generate_raw_safe();
        let async_safe = self.generate_async_safe();
        let service_safe = self.generate_service_safe();
        let id_expr = self.generate_id();
        let kind_expr = self.generate_kind();
        let ty_expr = self.generate_ty();
        let register_expr = self.generate_register();

        Ok(quote_spanned! { name.span() =>
            #[allow(clippy::eq_op, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
            unsafe impl #impl_generics ::interoptopus::lang::types::TypeInfo for #name #ty_generics #where_clause {
                const WIRE_SAFE: bool = #wire_safe;
                const RAW_SAFE: bool = #raw_safe;
                const ASYNC_SAFE: bool = #async_safe;
                const SERVICE_SAFE: bool = #service_safe;
                const SERVICE_CTOR_SAFE: bool = false;

                fn id() -> ::interoptopus::inventory::TypeId {
                    #id_expr
                }

                fn kind() -> ::interoptopus::lang::types::TypeKind {
                    #kind_expr
                }

                fn ty() -> ::interoptopus::lang::types::Type {
                    #ty_expr
                }

                fn register(inventory: &mut impl ::interoptopus::inventory::Inventory) {
                    #register_expr
                }
            }
        })
    }

    fn generate_wire_safe(&self) -> TokenStream {
        // For opaque and service types, inner fields are not emitted, so we don't check them
        if self.args.opaque || self.args.service {
            return quote_spanned! { self.name.span() => true };
        }

        match &self.data {
            TypeData::Struct(struct_data) => {
                let field_checks = struct_data.fields.iter().filter(|f| !f.skip).map(|field| {
                    let ty = &field.ty;
                    let span = field.name.as_ref().map_or_else(|| ty.span(), syn::Ident::span);
                    quote_spanned! { span => <#ty as ::interoptopus::lang::types::TypeInfo>::WIRE_SAFE }
                });

                let checks: Vec<_> = field_checks.collect();
                if checks.is_empty() {
                    quote_spanned! { self.name.span() => true }
                } else {
                    quote_spanned! { self.name.span() => #(#checks)&&* }
                }
            }
            TypeData::Enum(enum_data) => {
                let variant_checks = enum_data.variants.iter().filter_map(|variant| match &variant.data {
                    VariantData::Unit => None,
                    VariantData::Tuple(ty) => Some({
                        quote_spanned! { variant.name.span() => <#ty as ::interoptopus::lang::types::TypeInfo>::WIRE_SAFE }
                    }),
                });

                let checks: Vec<_> = variant_checks.collect();
                if checks.is_empty() {
                    quote_spanned! { self.name.span() => true }
                } else {
                    quote_spanned! { self.name.span() => #(#checks)&&* }
                }
            }
        }
    }

    fn generate_raw_safe(&self) -> TokenStream {
        // For opaque and service types, inner fields are not emitted, so we don't check them
        if self.args.opaque || self.args.service {
            return quote_spanned! { self.name.span() => true };
        }

        match &self.data {
            TypeData::Struct(struct_data) => {
                let field_checks = struct_data.fields.iter().filter(|f| !f.skip).map(|field| {
                    let ty = &field.ty;
                    let span = field.name.as_ref().map_or_else(|| ty.span(), syn::Ident::span);
                    quote_spanned! { span => <#ty as ::interoptopus::lang::types::TypeInfo>::RAW_SAFE }
                });

                let checks: Vec<_> = field_checks.collect();
                if checks.is_empty() {
                    quote_spanned! { self.name.span() => true }
                } else {
                    quote_spanned! { self.name.span() => #(#checks)&&* }
                }
            }
            TypeData::Enum(enum_data) => {
                let variant_checks = enum_data.variants.iter().filter_map(|variant| match &variant.data {
                    VariantData::Unit => None,
                    VariantData::Tuple(ty) => Some({
                        quote_spanned! { variant.name.span() => <#ty as ::interoptopus::lang::types::TypeInfo>::RAW_SAFE }
                    }),
                });

                let checks: Vec<_> = variant_checks.collect();
                if checks.is_empty() {
                    quote_spanned! { self.name.span() => true }
                } else {
                    quote_spanned! { self.name.span() => #(#checks)&&* }
                }
            }
        }
    }

    fn generate_async_safe(&self) -> TokenStream {
        // For opaque and service types, inner fields are not emitted, so we don't check them
        if self.args.opaque || self.args.service {
            return quote_spanned! { self.name.span() => true };
        }

        match &self.data {
            TypeData::Struct(struct_data) => {
                let field_checks = struct_data.fields.iter().filter(|f| !f.skip).map(|field| {
                    let ty = &field.ty;
                    let span = field.name.as_ref().map_or_else(|| ty.span(), syn::Ident::span);
                    quote_spanned! { span => <#ty as ::interoptopus::lang::types::TypeInfo>::ASYNC_SAFE }
                });

                let checks: Vec<_> = field_checks.collect();
                if checks.is_empty() {
                    quote_spanned! { self.name.span() => true }
                } else {
                    quote_spanned! { self.name.span() => #(#checks)&&* }
                }
            }
            TypeData::Enum(enum_data) => {
                let variant_checks = enum_data.variants.iter().filter_map(|variant| match &variant.data {
                    VariantData::Unit => None,
                    VariantData::Tuple(ty) => Some({
                        quote_spanned! { variant.name.span() => <#ty as ::interoptopus::lang::types::TypeInfo>::ASYNC_SAFE }
                    }),
                });

                let checks: Vec<_> = variant_checks.collect();
                if checks.is_empty() {
                    quote_spanned! { self.name.span() => true }
                } else {
                    quote_spanned! { self.name.span() => #(#checks)&&* }
                }
            }
        }
    }

    fn generate_service_safe(&self) -> TokenStream {
        if self.args.service {
            quote_spanned! { self.name.span() => true }
        } else {
            quote_spanned! { self.name.span() => false }
        }
    }

    fn generate_id(&self) -> TokenStream {
        let name = &self.name;
        let full_type = if self.generics.params.is_empty() {
            quote_spanned! { name.span() => #name }
        } else {
            quote_spanned! { name.span() => Self }
        };

        quote_spanned! { name.span() =>
            ::interoptopus::inventory::TypeId::from_id(::interoptopus::id!(#full_type))
        }
    }

    fn generate_kind(&self) -> TokenStream {
        if self.args.service {
            return quote_spanned! { self.name.span() => ::interoptopus::lang::types::TypeKind::Service };
        }

        if self.args.opaque {
            return quote_spanned! { self.name.span() => ::interoptopus::lang::types::TypeKind::Opaque };
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
                    let span = field.name.as_ref().map_or_else(|| ty.span(), syn::Ident::span);
                    quote_spanned! { span =>
                        ::interoptopus::lang::types::Field {
                            name: #field_name.to_string(),
                            docs: ::interoptopus::lang::meta::Docs::from_line(#field_docs),
                            visibility: ::interoptopus::lang::meta::Visibility::Public,
                            ty: <#ty as ::interoptopus::lang::types::TypeInfo>::id(),
                        }
                    }
                });

                let repr = self.generate_repr();

                quote_spanned! { self.name.span() =>
                    ::interoptopus::lang::types::TypeKind::Struct(
                        ::interoptopus::lang::types::Struct {
                            fields: vec![#(#fields),*],
                            repr: #repr,
                        }
                    )
                }
            }
            TypeData::Enum(enum_data) => {
                let mut next_discriminant: isize = 0;
                let variants = enum_data.variants.iter().map(|variant| {
                    let variant_name = variant.name.to_string();
                    let variant_docs = variant.docs.join("\n");
                    let kind = match &variant.data {
                        VariantData::Unit => {
                            let disc = if let Some(expr) = &variant.discriminant {
                                quote_spanned! { variant.name.span() => {
                                    #[allow(clippy::unnecessary_cast)]
                                    { (#expr) as isize }
                                }}
                            } else {
                                let d = next_discriminant;
                                quote_spanned! { variant.name.span() => #d }
                            };
                            next_discriminant += 1;
                            quote_spanned! { variant.name.span() =>
                                ::interoptopus::lang::types::VariantKind::Unit(#disc)
                            }
                        }
                        VariantData::Tuple(ty) => {
                            next_discriminant += 1;
                            quote_spanned! { variant.name.span() =>
                                ::interoptopus::lang::types::VariantKind::Tuple(
                                    <#ty as ::interoptopus::lang::types::TypeInfo>::id()
                                )
                            }
                        }
                    };

                    quote_spanned! { variant.name.span() =>
                        ::interoptopus::lang::types::Variant {
                            name: #variant_name.to_string(),
                            docs: ::interoptopus::lang::meta::Docs::from_line(#variant_docs),
                            kind: #kind,
                        }
                    }
                });

                let repr = self.generate_repr();

                quote_spanned! { self.name.span() =>
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
            return quote_spanned! { self.name.span() =>
                ::interoptopus::lang::types::Repr {
                    layout: ::interoptopus::lang::types::Layout::Opaque,
                    alignment: ::std::option::Option::None,
                }
            };
        }

        let layout = if self.args.opaque {
            quote_spanned! { self.name.span() => ::interoptopus::lang::types::Layout::Opaque }
        } else if self.args.transparent {
            quote_spanned! { self.name.span() => ::interoptopus::lang::types::Layout::Transparent }
        } else if self.args.packed {
            quote_spanned! { self.name.span() => ::interoptopus::lang::types::Layout::Packed }
        } else {
            match &self.data {
                TypeData::Struct(_) => quote_spanned! { self.name.span() => ::interoptopus::lang::types::Layout::C },
                TypeData::Enum(enum_data) => crate::types::discriminant::layout_tokens(&enum_data.discriminant, self.name.span()),
            }
        };

        quote_spanned! { self.name.span() =>
            ::interoptopus::lang::types::Repr {
                layout: #layout,
                alignment: ::std::option::Option::None,
            }
        }
    }

    #[allow(dead_code)]
    fn emit_emission(&self) -> TokenStream {
        match &self.args.module {
            Some(crate::types::args::ModuleKind::Named(name)) => {
                quote_spanned! { self.name.span() => ::interoptopus::lang::meta::Emission::FileEmission(::interoptopus::lang::meta::FileEmission::CustomModule(::interoptopus::lang::meta::Module::from_string(#name))) }
            }
            Some(crate::types::args::ModuleKind::Common) => {
                quote_spanned! { self.name.span() => ::interoptopus::lang::meta::Emission::FileEmission(::interoptopus::lang::meta::FileEmission::Common) }
            }
            None => {
                quote_spanned! { self.name.span() => ::interoptopus::lang::meta::Emission::FileEmission(::interoptopus::lang::meta::FileEmission::Default) }
            }
        }
    }

    fn generate_ty(&self) -> TokenStream {
        let docs_content = self.docs.join("\n");

        let type_name_expr = if let Some(name) = &self.args.name {
            let name = name.clone();
            quote_spanned! { self.name.span() => #name.to_string() }
        } else if self.generics.params.is_empty() {
            let base_name = self.name.to_string();
            quote_spanned! { self.name.span() => #base_name.to_string() }
        } else {
            // For generic types, generate a meaningful name based on the concrete type.
            // We strip module paths both from the outer type and from generic parameters,
            // but must not split on `::` inside angle brackets.
            quote_spanned! { self.name.span() =>
                {
                    let type_name = std::any::type_name::<Self>();
                    ::interoptopus::proc::strip_module_paths(type_name)
                }
            }
        };

        let emission = self.emit_emission();

        quote_spanned! { self.name.span() =>
            ::interoptopus::lang::types::Type {
                name: #type_name_expr,
                visibility: ::interoptopus::lang::meta::Visibility::Public,
                docs: ::interoptopus::lang::meta::Docs::from_line(#docs_content),
                emission: #emission,
                kind: Self::kind(),
            }
        }
    }

    fn generate_register(&self) -> TokenStream {
        let type_registration = quote_spanned! { self.name.span() =>
            inventory.register_type(Self::id(), Self::ty());
        };

        // For opaque and service types, inner fields are not emitted, so we don't register them
        let field_registrations = if self.args.opaque || self.args.service {
            quote_spanned! { self.name.span() => }
        } else {
            match &self.data {
                TypeData::Struct(struct_data) => {
                    let registrations = struct_data.fields.iter().filter(|f| !f.skip).map(|field| {
                        let ty = &field.ty;
                        let span = field.name.as_ref().map_or_else(|| ty.span(), syn::Ident::span);
                        quote_spanned! { span =>
                            <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
                        }
                    });
                    quote_spanned! { self.name.span() => #(#registrations)* }
                }
                TypeData::Enum(enum_data) => {
                    let registrations = enum_data.variants.iter().filter_map(|variant| match &variant.data {
                        VariantData::Unit => None,
                        VariantData::Tuple(ty) => Some(quote_spanned! { variant.name.span() =>
                            <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
                        }),
                    });
                    quote_spanned! { self.name.span() => #(#registrations)* }
                }
            }
        };

        quote_spanned! { self.name.span() =>
            #field_registrations
            #type_registration
        }
    }
}
