use proc_macro2::TokenStream;
use quote::quote;
use syn::{Type, Visibility};

use crate::types::input::{ParsedInput, InputData};

pub struct CodeGenerator<'a> {
    input: &'a ParsedInput,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(input: &'a ParsedInput) -> Self {
        Self { input }
    }

    pub fn generate(&self) -> TokenStream {
        let transformed_item = self.generate_transformed_item();
        let type_info_impl = self.generate_type_info_impl();

        quote! {
            #transformed_item
            #type_info_impl
        }
    }

    fn generate_transformed_item(&self) -> TokenStream {
        let original_input = &self.input.original_input;
        let repr_attr = self.generate_repr_attr();

        let vis = &original_input.vis;
        let ident = &original_input.ident;

        // Create generics without where clause to avoid duplication
        let mut generics_without_where = original_input.generics.clone();
        generics_without_where.where_clause = None;

        match &original_input.data {
            syn::Data::Struct(data_struct) => {
                let fields = &data_struct.fields;
                quote! {
                    #repr_attr
                    #vis struct #ident #generics_without_where #fields
                }
            }
            syn::Data::Enum(data_enum) => {
                // For enums, we need to iterate through variants properly
                let variants = data_enum.variants.iter();
                quote! {
                    #repr_attr
                    #vis enum #ident #generics_without_where {
                        #(#variants),*
                    }
                }
            }
            syn::Data::Union(_) => {
                // This should have been caught earlier
                quote! { #original_input }
            }
        }
    }

    fn generate_repr_attr(&self) -> TokenStream {
        // Temporarily disable to debug
        quote! {}
    }


    fn generate_type_info_impl(&self) -> TokenStream {
        if self.input.args.service {
            return self.generate_service_type_info();
        }

        let ident = &self.input.ident;
        let generics = &self.input.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let wire_safe = self.generate_wire_safe_expr();
        let raw_safe = self.generate_raw_safe_expr();
        let id_expr = self.generate_id_expr();
        let kind_expr = self.generate_kind_expr();
        let ty_expr = self.generate_ty_expr();
        let register_expr = self.generate_register_expr();

        let type_info_bounds = self.generate_type_info_bounds();

        quote! {
            impl #impl_generics ::interoptopus::lang::types::TypeInfo for #ident #ty_generics
            #type_info_bounds
            #where_clause
            {
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

    fn generate_service_type_info(&self) -> TokenStream {
        let ident = &self.input.ident;
        let generics = &self.input.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let id_expr = self.generate_id_expr();
        let ty_expr = self.generate_service_ty_expr();

        let type_info_bounds = self.generate_type_info_bounds();

        quote! {
            impl #impl_generics ::interoptopus::lang::types::TypeInfo for #ident #ty_generics
            #type_info_bounds
            #where_clause
            {
                const WIRE_SAFE: bool = false;
                const RAW_SAFE: bool = false;

                fn id() -> ::interoptopus::inventory::TypeId {
                    #id_expr
                }

                fn kind() -> ::interoptopus::lang::types::TypeKind {
                    ::interoptopus::lang::types::TypeKind::Service
                }

                fn ty() -> ::interoptopus::lang::types::Type {
                    #ty_expr
                }

                fn register(inventory: &mut ::interoptopus::inventory::Inventory) {
                    inventory.register_type(Self::id(), Self::ty());
                }
            }
        }
    }

    fn generate_type_info_bounds(&self) -> TokenStream {
        if self.input.generics.params.is_empty() {
            return quote! {};
        }

        let bounds: Vec<TokenStream> = self.input.generics.type_params()
            .map(|param| {
                let ident = &param.ident;
                quote! { #ident: ::interoptopus::lang::types::TypeInfo }
            })
            .collect();

        if bounds.is_empty() {
            quote! {}
        } else {
            quote! { where #(#bounds),* }
        }
    }

    fn generate_wire_safe_expr(&self) -> TokenStream {
        let field_types = self.collect_field_types();

        if field_types.is_empty() {
            quote! { true }
        } else {
            let safety_checks: Vec<TokenStream> = field_types.iter()
                .map(|ty| quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::WIRE_SAFE })
                .collect();

            quote! { #(#safety_checks)&&* }
        }
    }

    fn generate_raw_safe_expr(&self) -> TokenStream {
        let field_types = self.collect_field_types();

        if field_types.is_empty() {
            quote! { true }
        } else {
            let safety_checks: Vec<TokenStream> = field_types.iter()
                .map(|ty| quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::RAW_SAFE })
                .collect();

            quote! { #(#safety_checks)&&* }
        }
    }

    fn generate_id_expr(&self) -> TokenStream {
        let ident = &self.input.ident;
        let generics = &self.input.generics;
        let (_, ty_generics, _) = generics.split_for_impl();

        quote! {
            ::interoptopus::type_id!(#ident #ty_generics)
        }
    }

    fn generate_kind_expr(&self) -> TokenStream {
        match &self.input.data {
            InputData::Struct(_) => {
                let fields_expr = self.generate_struct_fields_expr();
                let repr_expr = self.generate_repr_expr();

                if self.input.args.opaque {
                    quote! { ::interoptopus::lang::types::TypeKind::Opaque }
                } else {
                    quote! {
                        ::interoptopus::lang::types::TypeKind::Struct(
                            ::interoptopus::lang::types::Struct {
                                fields: #fields_expr,
                                repr: #repr_expr,
                            }
                        )
                    }
                }
            }
            InputData::Enum(_) => {
                let variants_expr = self.generate_enum_variants_expr();
                let repr_expr = self.generate_repr_expr();

                quote! {
                    ::interoptopus::lang::types::TypeKind::Enum(
                        ::interoptopus::lang::types::Enum {
                            variants: #variants_expr,
                            repr: #repr_expr,
                        }
                    )
                }
            }
        }
    }

    fn generate_struct_fields_expr(&self) -> TokenStream {
        if let InputData::Struct(struct_data) = &self.input.data {
            let field_exprs: Vec<TokenStream> = struct_data.fields
                .iter()
                .filter(|field| !field.skip)
                .enumerate()
                .map(|(index, field)| {
                    let field_name = if let Some(ident) = &field.ident {
                        ident.to_string()
                    } else {
                        index.to_string()
                    };

                    let ty = &field.ty;
                    let docs_expr = self.generate_docs_expr(&field.docs);
                    let vis_expr = self.generate_visibility_expr(&field.vis);

                    let field_name_str = field_name.clone();
                    quote! {
                        ::interoptopus::lang::types::Field {
                            name: #field_name_str.to_string(),
                            docs: #docs_expr,
                            visibility: #vis_expr,
                            ty: <#ty as ::interoptopus::lang::types::TypeInfo>::id(),
                        }
                    }
                })
                .collect();

            quote! { vec![#(#field_exprs),*] }
        } else {
            quote! { vec![] }
        }
    }

    fn generate_enum_variants_expr(&self) -> TokenStream {
        if let InputData::Enum(enum_data) = &self.input.data {
            let variant_exprs: Vec<TokenStream> = enum_data.variants
                .iter()
                .enumerate()
                .map(|(index, variant)| {
                    let variant_name = variant.ident.to_string();
                    let docs_expr = self.generate_docs_expr(&variant.docs);

                    let kind_expr = if variant.fields.is_empty() {
                        let discriminant_value = if let Some(discriminant) = &variant.discriminant {
                            // Try to evaluate the discriminant as a literal
                            if let syn::Expr::Lit(expr_lit) = discriminant {
                                if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                                    lit_int.base10_parse::<usize>().unwrap_or(index)
                                } else {
                                    index
                                }
                            } else {
                                index
                            }
                        } else {
                            index
                        };

                        quote! {
                            ::interoptopus::lang::types::VariantKind::Unit(#discriminant_value)
                        }
                    } else if variant.fields.len() == 1 && variant.fields[0].ident.is_none() {
                        let field_ty = &variant.fields[0].ty;
                        quote! {
                            ::interoptopus::lang::types::VariantKind::Tuple(
                                <#field_ty as ::interoptopus::lang::types::TypeInfo>::id()
                            )
                        }
                    } else {
                        // Fallback to unit
                        quote! {
                            ::interoptopus::lang::types::VariantKind::Unit(#index)
                        }
                    };

                    let variant_name_str = variant_name.clone();
                    quote! {
                        ::interoptopus::lang::types::Variant {
                            name: #variant_name_str.to_string(),
                            docs: #docs_expr,
                            kind: #kind_expr,
                        }
                    }
                })
                .collect();

            quote! { vec![#(#variant_exprs),*] }
        } else {
            quote! { vec![] }
        }
    }

    fn generate_repr_expr(&self) -> TokenStream {
        if self.input.args.transparent {
            quote! { ::interoptopus::lang::types::Repr::c() }
        } else if self.input.args.packed {
            quote! { ::interoptopus::lang::types::Repr::c() }
        } else if self.input.args.opaque {
            quote! { ::interoptopus::lang::types::Repr::c() }
        } else {
            match &self.input.data {
                InputData::Enum(_) => {
                    quote! { ::interoptopus::lang::types::Repr::u32() }
                }
                InputData::Struct(_) => {
                    quote! { ::interoptopus::lang::types::Repr::c() }
                }
            }
        }
    }

    fn generate_ty_expr(&self) -> TokenStream {
        let name = self.generate_type_name();
        let docs_expr = self.generate_docs_expr(&self.input.docs);
        let vis_expr = self.generate_visibility_expr(&self.input.vis);
        let emission_expr = self.generate_emission_expr();
        let kind_expr = self.generate_kind_expr();

        quote! {
            ::interoptopus::lang::types::Type {
                name: #name,
                visibility: #vis_expr,
                docs: #docs_expr,
                emission: #emission_expr,
                kind: #kind_expr,
            }
        }
    }

    fn generate_service_ty_expr(&self) -> TokenStream {
        let name = self.generate_type_name();
        let docs_expr = self.generate_docs_expr(&self.input.docs);
        let vis_expr = self.generate_visibility_expr(&self.input.vis);

        quote! {
            ::interoptopus::lang::types::Type {
                name: #name,
                visibility: #vis_expr,
                docs: #docs_expr,
                emission: ::interoptopus::lang::meta::Emission::Common,
                kind: ::interoptopus::lang::types::TypeKind::Service,
            }
        }
    }

    fn generate_type_name(&self) -> TokenStream {
        let name = if let Some(name) = &self.input.args.name {
            name.clone()
        } else {
            let base_name = self.input.ident.to_string();
            if self.input.generics.params.is_empty() {
                base_name
            } else {
                let param_names: Vec<String> = self.input.generics.type_params()
                    .map(|param| param.ident.to_string())
                    .collect();
                format!("{}{{{}}}", base_name, param_names.join("}{"))
            }
        };
        quote! { #name.to_string() }
    }

    fn generate_docs_expr(&self, docs: &[String]) -> TokenStream {
        if docs.is_empty() {
            quote! { ::interoptopus::lang::meta::Docs::empty() }
        } else {
            let lines: Vec<String> = docs.iter().cloned().collect();
            quote! {
                ::interoptopus::lang::meta::Docs::from_lines(vec![#(#lines.to_string()),*])
            }
        }
    }

    fn generate_visibility_expr(&self, vis: &Visibility) -> TokenStream {
        match vis {
            Visibility::Public(_) => quote! { ::interoptopus::lang::meta::Visibility::Public },
            _ => quote! { ::interoptopus::lang::meta::Visibility::Private },
        }
    }

    fn generate_emission_expr(&self) -> TokenStream {
        if let Some(module) = &self.input.args.module {
            let module_name = module.clone();
            quote! { ::interoptopus::lang::meta::Emission::Module(#module_name.to_string()) }
        } else {
            quote! { ::interoptopus::lang::meta::Emission::Common }
        }
    }

    fn generate_register_expr(&self) -> TokenStream {
        let field_types = self.collect_field_types();

        let register_calls: Vec<TokenStream> = field_types.iter()
            .map(|ty| quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory); })
            .collect();

        quote! {
            #(#register_calls)*
            inventory.register_type(Self::id(), Self::ty());
        }
    }

    fn collect_field_types(&self) -> Vec<&Type> {
        let mut types = Vec::new();

        match &self.input.data {
            InputData::Struct(struct_data) => {
                for field in &struct_data.fields {
                    if !field.skip {
                        types.push(&field.ty);
                    }
                }
            }
            InputData::Enum(enum_data) => {
                for variant in &enum_data.variants {
                    for field in &variant.fields {
                        if !field.skip {
                            types.push(&field.ty);
                        }
                    }
                }
            }
        }

        types
    }
}