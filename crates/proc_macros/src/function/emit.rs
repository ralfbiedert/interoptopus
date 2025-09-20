use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, ReturnType};

use crate::function::model::FunctionModel;

impl FunctionModel {
    pub fn emit_modified_function(&self, original_fn: &ItemFn) -> TokenStream {
        let vis = &self.vis;
        let name = &self.name;
        let export_name = self.generate_export_name();
        let generics = &self.signature.generics;
        let inputs = &original_fn.sig.inputs;
        let output = &self.signature.output;
        let block = &original_fn.block;
        let unsafety = if self.is_unsafe {
            quote! { unsafe }
        } else {
            quote! {}
        };
        let where_clause = &generics.where_clause;

        quote! {
            #[unsafe(no_mangle)]
            #[unsafe(export_name = #export_name)]
            #vis #unsafety extern "C" fn #name #generics(#inputs) #output #where_clause #block
        }
    }

    pub fn emit_companion_struct(&self) -> TokenStream {
        let vis = &self.vis;
        let struct_name = &self.name;
        let generics = &self.signature.generics;
        let where_clause = &generics.where_clause;

        // If we have generic parameters, we need to use them or add PhantomData
        let phantom_data_field = if generics.params.is_empty() {
            quote! {}
        } else {
            // Create a tuple type of all the generic parameters
            let param_types = generics
                .params
                .iter()
                .map(|param| match param {
                    syn::GenericParam::Lifetime(lifetime) => {
                        let lifetime_ident = &lifetime.lifetime;
                        quote! { &#lifetime_ident () }
                    }
                    syn::GenericParam::Type(type_param) => {
                        let type_ident = &type_param.ident;
                        quote! { #type_ident }
                    }
                    syn::GenericParam::Const(const_param) => {
                        let const_ident = &const_param.ident;
                        quote! { [(); #const_ident] }
                    }
                })
                .collect::<Vec<_>>();

            if param_types.len() == 1 {
                quote! { _phantom: ::std::marker::PhantomData<#(#param_types)*>, }
            } else {
                quote! { _phantom: ::std::marker::PhantomData<(#(#param_types),*)>, }
            }
        };

        quote! {
            #vis struct #struct_name #generics #where_clause {
                #phantom_data_field
            }
        }
    }

    pub fn emit_function_info_impl(&self) -> TokenStream {
        let struct_name = &self.name;
        let export_name = self.generate_export_name();
        let generics = &self.signature.generics;
        let where_clause = &generics.where_clause;

        let arguments = self.emit_arguments();
        let parameter_registrations = self.emit_parameter_types();
        let return_type = self.emit_return_type();
        let return_type_registration = self.emit_return_type_registration();
        let emission = self.emit_emission();
        let visibility = self.emit_visibility();
        let docs_tokens = self.emit_docs();

        quote! {
            impl #generics ::interoptopus::lang::function::FunctionInfo for #struct_name #generics #where_clause {
                fn id() -> ::interoptopus::inventory::FunctionId {
                    ::interoptopus::inventory::FunctionId::from_id(::interoptopus::id!(#struct_name))
                }

                fn signature() -> ::interoptopus::lang::function::Signature {
                    ::interoptopus::lang::function::Signature {
                        arguments: vec![#(#arguments),*],
                        rval: #return_type,
                    }
                }

                fn function() -> ::interoptopus::lang::function::Function {
                    ::interoptopus::lang::function::Function {
                        name: #export_name.to_string(),
                        visibility: #visibility,
                        docs: #docs_tokens,
                        emission: #emission,
                        signature: Self::signature(),
                    }
                }

                fn register(inventory: &mut ::interoptopus::inventory::Inventory) {
                    // Register all parameter types
                    #(
                        #parameter_registrations;
                    )*

                    // Register return type
                    #return_type_registration;

                    // Register the function itself
                    inventory.register_function(Self::id(), Self::function());
                }
            }
        }
    }

    fn emit_arguments(&self) -> Vec<TokenStream> {
        self.signature
            .inputs
            .iter()
            .map(|param| {
                let name = param.name.to_string();
                let ty = &param.ty;
                quote! {
                    ::interoptopus::lang::function::Argument::new(
                        #name,
                        <#ty as ::interoptopus::lang::types::TypeInfo>::id()
                    )
                }
            })
            .collect()
    }

    fn emit_parameter_types(&self) -> Vec<TokenStream> {
        self.signature
            .inputs
            .iter()
            .map(|param| {
                let ty = &param.ty;
                quote! {
                    <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory)
                }
            })
            .collect()
    }

    fn emit_return_type(&self) -> TokenStream {
        match &self.signature.output {
            ReturnType::Default => quote! {
                <() as ::interoptopus::lang::types::TypeInfo>::id()
            },
            ReturnType::Type(_, ty) => quote! {
                <#ty as ::interoptopus::lang::types::TypeInfo>::id()
            },
        }
    }

    fn emit_return_type_registration(&self) -> TokenStream {
        match &self.signature.output {
            ReturnType::Default => quote! {
                <() as ::interoptopus::lang::types::TypeInfo>::register(inventory)
            },
            ReturnType::Type(_, ty) => quote! {
                <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory)
            },
        }
    }

    fn emit_emission(&self) -> TokenStream {
        match &self.args.module {
            Some(crate::function::args::ModuleKind::Named(name)) => {
                quote! { ::interoptopus::lang::meta::Emission::Module(#name.to_string()) }
            }
            Some(crate::function::args::ModuleKind::Common) => {
                quote! { ::interoptopus::lang::meta::Emission::Common }
            }
            None => {
                quote! { ::interoptopus::lang::meta::Emission::External }
            }
        }
    }

    fn emit_visibility(&self) -> TokenStream {
        match &self.vis {
            syn::Visibility::Public(_) => quote! { ::interoptopus::lang::meta::Visibility::Public },
            syn::Visibility::Restricted(_) => quote! { ::interoptopus::lang::meta::Visibility::Private },
            syn::Visibility::Inherited => quote! { ::interoptopus::lang::meta::Visibility::Private },
        }
    }

    fn emit_docs(&self) -> TokenStream {
        let docs = &self.docs;
        quote! {
            ::interoptopus::lang::meta::Docs::from_lines(vec![#(#docs.to_string()),*])
        }
    }
}
