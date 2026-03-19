use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::plugin::model::{PluginModel, PluginService};

impl PluginModel {
    pub fn emit(&self) -> TokenStream {
        let plugin_struct = self.emit_plugin_struct();
        let plugin_impl = self.emit_plugin_impl();
        let plugin_info_impl = self.emit_plugin_info_impl();
        let services = self.services.iter().map(|s| self.emit_service(s));

        quote! {
            #plugin_struct
            #plugin_impl
            #plugin_info_impl
            #(#services)*
        }
    }

    fn emit_plugin_struct(&self) -> TokenStream {
        let name = &self.name;
        quote! {
            struct #name {}
        }
    }

    fn emit_plugin_impl(&self) -> TokenStream {
        let name = &self.name;
        let methods = self.functions.iter().map(|f| {
            let fn_name = &f.name;
            let params = f.params.iter().map(|p| {
                let pname = &p.name;
                let pty = &p.ty;
                quote! { #pname: #pty }
            });
            let ret = match &f.ret {
                Some(ty) => quote! { -> #ty },
                None => quote! {},
            };
            quote! {
                fn #fn_name(&self, #(#params),*) #ret {
                    todo!()
                }
            }
        });

        quote! {
            impl #name {
                #(#methods)*
            }
        }
    }

    fn emit_plugin_info_impl(&self) -> TokenStream {
        let name = &self.name;
        let name_str = name.to_string();

        let fn_registrations = self.functions.iter().map(|f| {
            let fn_name = &f.name;
            let fn_name_str = fn_name.to_string();

            let type_registrations = f.params.iter().map(|p| {
                let ty = &p.ty;
                quote! {
                    <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
                }
            });

            let ret_registration = f.ret.as_ref().map(|ty| {
                quote! {
                    <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
                }
            });

            let arguments = f.params.iter().map(|p| {
                let pname_str = p.name.to_string();
                let pty = &p.ty;
                quote! {
                    ::interoptopus::lang::function::Argument::new(
                        #pname_str,
                        <#pty as ::interoptopus::lang::types::TypeInfo>::id(),
                    )
                }
            });

            let rval = match &f.ret {
                Some(ty) => quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::id() },
                None => quote! { <() as ::interoptopus::lang::types::TypeInfo>::id() },
            };

            quote! {
                {
                    #(#type_registrations)*
                    #ret_registration

                    let id = ::interoptopus::inventory::FunctionId::from_id(
                        ::interoptopus::inventory::Id::new(
                            ::interoptopus::inventory::hash_str(#fn_name_str)
                        )
                    );

                    let function = ::interoptopus::lang::function::Function {
                        name: #fn_name_str.to_string(),
                        visibility: ::interoptopus::lang::meta::Visibility::Public,
                        docs: ::interoptopus::lang::meta::Docs::default(),
                        emission: ::interoptopus::lang::meta::Emission::FileEmission(
                            ::interoptopus::lang::meta::FileEmission::Default,
                        ),
                        signature: ::interoptopus::lang::function::Signature {
                            arguments: vec![#(#arguments),*],
                            rval: #rval,
                        },
                    };

                    inventory.register_function(id, function);
                }
            }
        });

        quote! {
            impl ::interoptopus::lang::plugin::PluginInfo for #name {
                fn id() -> ::interoptopus::inventory::PluginId {
                    ::interoptopus::inventory::PluginId::from_id(
                        ::interoptopus::inventory::Id::new(
                            ::interoptopus::inventory::hash_str(#name_str)
                        )
                    )
                }

                fn register(inventory: &mut impl ::interoptopus::inventory::Inventory) {
                    #(#fn_registrations)*
                }
            }
        }
    }

    fn emit_service(&self, service: &PluginService) -> TokenStream {
        let service_name = &service.name;
        let plugin_name = &self.name;

        let state_a = format_ident!("{}Unloaded", service_name);
        let state_b = format_ident!("{}Loaded", service_name);

        let state_markers = quote! {
            struct #state_a;
            struct #state_b;
        };

        let service_struct = quote! {
            struct #service_name<T = #state_b> {
                _t: ::std::marker::PhantomData<T>,
            }
        };

        let constructor_impl = quote! {
            impl #service_name<#state_a> {
                pub fn from(_plugin: &#plugin_name) -> #service_name<#state_b> {
                    todo!()
                }
            }
        };

        let methods = service.methods.iter().filter(|m| m.has_self).map(|m| {
            let method_name = &m.name;
            let params = m.params.iter().map(|p| {
                let pname = &p.name;
                let pty = &p.ty;
                quote! { #pname: #pty }
            });
            let ret = match &m.ret {
                Some(ty) => quote! { -> #ty },
                None => quote! {},
            };
            quote! {
                pub fn #method_name(&self, #(#params),*) #ret {}
            }
        });

        let methods_impl = quote! {
            impl #service_name<#state_b> {
                #(#methods)*
            }
        };

        quote! {
            #state_markers
            #service_struct
            #constructor_impl
            #methods_impl
        }
    }
}
