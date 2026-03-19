use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::plugin::model::{PluginModel, PluginService};

impl PluginModel {
    pub fn emit(&self) -> TokenStream {
        let plugin_struct = self.emit_plugin_struct();
        let plugin_impl = self.emit_plugin_impl();
        let services = self.services.iter().map(|s| self.emit_service(s));

        quote! {
            #plugin_struct
            #plugin_impl
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
