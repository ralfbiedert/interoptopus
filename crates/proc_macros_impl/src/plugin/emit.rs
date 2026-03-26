use std::collections::HashSet;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Type;

use crate::plugin::model::{
    PluginModel, PluginParam, ServiceBlock, direct_service_name, is_self_return, ref_service_name, replace_self, service_in_type,
    transitive_returned_services,
};

impl PluginModel {
    pub fn emit(&self) -> TokenStream {
        let svc_names = self.service_names();
        let plugin_struct = self.emit_plugin_struct(&svc_names);
        let plugin_impl = self.emit_plugin_impl(&svc_names);
        let plugin_trait = self.emit_plugin_trait(&svc_names);
        let plugin_info = self.emit_plugin_info(&svc_names);
        let service_type_infos: Vec<_> = self.services.iter().map(emit_service_type_info).collect();
        let service_structs: Vec<_> = self.services.iter().map(|s| emit_service_struct(s, &self.services, &svc_names)).collect();
        let service_impls: Vec<_> = self.services.iter().map(|s| emit_service_impl(s, &self.services, &svc_names)).collect();
        let service_drops: Vec<_> = self.services.iter().map(emit_service_drop).collect();
        let service_send_syncs: Vec<_> = self.services.iter().map(emit_service_send_sync).collect();
        let service_traits: Vec<_> = self.services.iter().map(emit_service_trait).collect();

        quote! {
            #plugin_struct
            #plugin_impl
            #plugin_trait
            #plugin_info
            #(#service_type_infos)*
            #(#service_structs)*
            #(#service_send_syncs)*
            #(#service_impls)*
            #(#service_drops)*
            #(#service_traits)*
        }
    }

    // -----------------------------------------------------------------------
    // Plugin struct — holds all fn pointers (bare fns + service fns)
    // -----------------------------------------------------------------------

    fn emit_plugin_struct(&self, svc_names: &HashSet<String>) -> TokenStream {
        let name = &self.name;

        let bare_fields = self.functions.iter().map(|f| {
            let field_name = &f.name;
            let ffi_ptys: Vec<_> = f.params.iter().map(|p| ffi_param_ty(&p.ty, svc_names)).collect();
            if f.is_async {
                let cb_ret = ffi_ret_or_unit(f.ret.as_ref(), svc_names);
                quote! { #field_name: extern "C" fn(#(#ffi_ptys,)* ::interoptopus::pattern::asynk::AsyncCallback<#cb_ret>) }
            } else {
                let ret = ffi_ret_arrow(f.ret.as_ref(), svc_names);
                quote! { #field_name: extern "C" fn(#(#ffi_ptys),*) #ret }
            }
        });

        let service_fields = self.services.iter().flat_map(|s| {
            let prefix = s.prefix();
            let svc_ident = &s.name;
            let mut fields = Vec::new();

            for c in s.ctors() {
                let field = prefixed_ident(&prefix, &c.name);
                let ffi_ptys: Vec<_> = c.params.iter().map(|p| ffi_param_ty(&p.ty, svc_names)).collect();
                let fn_ty = ctor_ffi_fn_ty(&ffi_ptys, c, svc_ident);
                fields.push(quote! { #field: #fn_ty });
            }

            for m in s.instance_methods() {
                let field = prefixed_ident(&prefix, &m.name);
                let ffi_ptys: Vec<_> = m.params.iter().map(|p| ffi_param_ty(&p.ty, svc_names)).collect();
                let fn_ty = method_ffi_fn_ty(&ffi_ptys, m.ret.as_ref(), m.is_async, svc_ident, svc_names);
                fields.push(quote! { #field: #fn_ty });
            }

            let drop_field = format_ident!("{}_drop", prefix);
            fields.push(quote! { #drop_field: extern "C" fn(::interoptopus::ffi::ServiceHandle<#svc_ident>) });

            fields
        });

        let register_trampoline_field = quote! {
            register_trampoline: ::interoptopus::lang::plugin::RegisterTrampolineFn
        };

        quote! { pub struct #name { #(#bare_fields,)* #(#service_fields,)* #register_trampoline_field, } }
    }

    // -----------------------------------------------------------------------
    // Plugin impl — bare fn delegates + service ctor methods
    // -----------------------------------------------------------------------

    fn emit_plugin_impl(&self, svc_names: &HashSet<String>) -> TokenStream {
        let name = &self.name;

        let bare_methods = self.functions.iter().map(|f| {
            emit_bare_method(f, &self.services, svc_names)
        });

        let ctor_methods = self.services.iter().flat_map(|s| {
            let prefix = s.prefix();
            let svc_name = &s.name;
            s.ctors()
                .into_iter()
                .map(move |c| emit_ctor_method(&prefix, svc_name, c, s, &self.services, svc_names))
                .collect::<Vec<_>>()
        });

        quote! {
            impl #name {
                pub fn new(loader: &impl ::interoptopus::lang::plugin::Loader) -> Result<Self, ::interoptopus::lang::plugin::PluginLoadError> {
                    let plugin: Self = loader.load_plugin()?;
                    plugin.register_trampolines();
                    Ok(plugin)
                }

                fn register_trampolines(&self) {
                    let register = self.register_trampoline;
                    ::interoptopus::register_wire_trampolines!(|id, ptr| {
                        (register)(id, ptr);
                    });
                }

                #(#bare_methods)*
                #(#ctor_methods)*
            }
        }
    }

    // -----------------------------------------------------------------------
    // Plugin trait impl — loads all symbols
    // -----------------------------------------------------------------------

    fn emit_plugin_trait(&self, svc_names: &HashSet<String>) -> TokenStream {
        let name = &self.name;

        let bare_loads = self.functions.iter().map(|f| {
            let field_name = &f.name;
            let symbol = field_name.to_string();
            let ffi_ptys: Vec<_> = f.params.iter().map(|p| ffi_param_ty(&p.ty, svc_names)).collect();
            let fn_ty = if f.is_async {
                let cb_ret = ffi_ret_or_unit(f.ret.as_ref(), svc_names);
                quote! { extern "C" fn(#(#ffi_ptys,)* ::interoptopus::pattern::asynk::AsyncCallback<#cb_ret>) }
            } else {
                let ret = ffi_ret_arrow(f.ret.as_ref(), svc_names);
                quote! { extern "C" fn(#(#ffi_ptys),*) #ret }
            };
            emit_load_field(field_name, &symbol, fn_ty)
        });

        let service_loads = self.services.iter().flat_map(|s| {
            let prefix = s.prefix();
            let svc_ident = &s.name;
            let mut loads = Vec::new();

            for c in s.ctors() {
                let field = prefixed_ident(&prefix, &c.name);
                let symbol = format!("{}_{}", prefix, c.name);
                let ffi_ptys: Vec<_> = c.params.iter().map(|p| ffi_param_ty(&p.ty, svc_names)).collect();
                let fn_ty = ctor_ffi_fn_ty(&ffi_ptys, c, svc_ident);
                loads.push(emit_load_field(&field, &symbol, fn_ty));
            }

            for m in s.instance_methods() {
                let field = prefixed_ident(&prefix, &m.name);
                let symbol = format!("{}_{}", prefix, m.name);
                let ffi_ptys: Vec<_> = m.params.iter().map(|p| ffi_param_ty(&p.ty, svc_names)).collect();
                let fn_ty = method_ffi_fn_ty(&ffi_ptys, m.ret.as_ref(), m.is_async, svc_ident, svc_names);
                loads.push(emit_load_field(&field, &symbol, fn_ty));
            }

            let drop_field = format_ident!("{}_drop", prefix);
            let drop_symbol = format!("{prefix}_drop");
            loads.push(emit_load_field(&drop_field, &drop_symbol, quote! { extern "C" fn(::interoptopus::ffi::ServiceHandle<#svc_ident>) }));

            loads
        });

        let register_trampoline_field = format_ident!("register_trampoline");
        let register_trampoline_load = emit_load_field(&register_trampoline_field, "register_trampoline", quote! { ::interoptopus::lang::plugin::RegisterTrampolineFn });

        quote! {
            impl ::interoptopus::lang::plugin::Plugin for #name {
                fn load_from(loader: impl Fn(&str) -> *const u8) -> Result<Self, ::interoptopus::lang::plugin::PluginLoadError> {
                    Ok(Self {
                        #(#bare_loads,)*
                        #(#service_loads,)*
                        #register_trampoline_load,
                    })
                }

                fn register_trampoline_fn(&self) -> ::interoptopus::lang::plugin::RegisterTrampolineFn {
                    self.register_trampoline
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // PluginInfo — registers types, functions, and services with the inventory
    // -----------------------------------------------------------------------

    fn emit_plugin_info(&self, svc_names: &HashSet<String>) -> TokenStream {
        let name = &self.name;
        let name_str = name.to_string();

        let bare_registrations = self.functions.iter().map(|f| {
            let ffi_ret = if f.is_async {
                None
            } else {
                f.ret.as_ref().map(|ty| ffi_reg_ret(ty, svc_names))
            };
            let cb_ty = if f.is_async {
                let cb_inner = ffi_ret_or_unit(f.ret.as_ref(), svc_names);
                Some(quote! { ::interoptopus::pattern::asynk::AsyncCallback<#cb_inner> })
            } else {
                None
            };
            emit_function_registration(&f.name.to_string(), &f.params, ffi_ret.as_ref(), cb_ty, svc_names)
        });

        let service_registrations: Vec<_> = self.services.iter().map(|s| emit_service_registration(s, svc_names)).collect();

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
                    #(#bare_registrations)*
                    #(#service_registrations)*
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Bare function methods on the plugin impl
// ---------------------------------------------------------------------------

/// Emit a public method on the plugin struct for a bare (non-service) function.
///
/// Uses `ServiceHandleMap::map_service_handle` for any return type containing a service.
fn emit_bare_method(f: &crate::plugin::model::PluginMethod, all_services: &[ServiceBlock], svc_names: &HashSet<String>) -> TokenStream {
    let fn_name = &f.name;
    let params = typed_params(&f.params);
    let ffi_args = ffi_call_args(&f.params, svc_names);
    let forget_stmts = forget_owned_services(&f.params, svc_names);

    let ret_svc_name = f.ret.as_ref().and_then(|ty| service_in_type(ty, svc_names));

    if let Some(ref svc_name) = ret_svc_name {
        let svc_block = find_service(all_services, svc_name);
        let ret_ty = &f.ret;
        let ffi_ret_ty = ffi_ret_or_unit(f.ret.as_ref(), svc_names);
        let svc_ident = format_ident!("{}", svc_name);

        if f.is_async {
            let field_src_lets = svc_field_src_lets(svc_block, all_services, svc_names, &quote! { self });
            let construct = svc_construct_expr(svc_block, all_services, svc_names);
            quote! {
                pub fn #fn_name(&self, #(#params),*) -> impl ::std::future::Future<Output = #ret_ty> + 'static {
                    #(#forget_stmts)*
                    let (future, cb) = ::interoptopus::pattern::asynk::AsyncCallbackFuture::<#ffi_ret_ty>::new();
                    (self.#fn_name)(#(#ffi_args,)* cb);
                    #(#field_src_lets)*
                    async move {
                        let raw = future.await;
                        ::interoptopus::ffi::ServiceHandleMap::map_service_handle(raw, #construct)
                    }
                }
            }
        } else {
            let field_copies = svc_field_copies(svc_block, all_services, svc_names, &quote! { self });
            quote! {
                pub fn #fn_name(&self, #(#params),*) -> #ret_ty {
                    #(#forget_stmts)*
                    let raw = (self.#fn_name)(#(#ffi_args),*);
                    ::interoptopus::ffi::ServiceHandleMap::map_service_handle(raw, |handle| #svc_ident { handle, #(#field_copies,)* })
                }
            }
        }
    } else if f.is_async {
        // async fn with no service return
        let ret_ty = ffi_ret_or_unit(f.ret.as_ref(), svc_names);
        quote! {
            pub fn #fn_name(&self, #(#params),*) -> impl ::std::future::Future<Output = #ret_ty> + 'static {
                #(#forget_stmts)*
                let (future, cb) = ::interoptopus::pattern::asynk::AsyncCallbackFuture::<#ret_ty>::new();
                (self.#fn_name)(#(#ffi_args,)* cb);
                future
            }
        }
    } else {
        // sync fn with no service involvement
        let ret = ret_tokens(f.ret.as_ref());
        quote! {
            pub fn #fn_name(&self, #(#params),*) #ret {
                #(#forget_stmts)*
                (self.#fn_name)(#(#ffi_args),*)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Service ctor methods on the plugin impl
// ---------------------------------------------------------------------------

fn emit_ctor_method(
    prefix: &str,
    svc_name: &Ident,
    c: &crate::plugin::model::PluginMethod,
    svc_block: &ServiceBlock,
    all_services: &[ServiceBlock],
    svc_names: &HashSet<String>,
) -> TokenStream {
    let method_name = prefixed_ident(prefix, &c.name);
    let ctor_field = prefixed_ident(prefix, &c.name);
    let params = typed_params(&c.params);
    let ffi_args = ffi_call_args(&c.params, svc_names);
    let forget_stmts = forget_owned_services(&c.params, svc_names);

    // Determine the FFI return type and user-facing return type
    let (ffi_ret_ty, user_ret_ty) = if is_self_return(c.ret.as_ref()) {
        (quote! { ::interoptopus::ffi::ServiceHandle<#svc_name> }, quote! { #svc_name })
    } else {
        // Wrapped Self (e.g., Result<Self, E>, Try<Self>, Option<Self>)
        let user_ty = replace_self(c.ret.as_ref().unwrap(), svc_name);
        (
            quote! { <#user_ty as ::interoptopus::ffi::ServiceAs<#svc_name>>::FFI },
            quote! { #user_ty },
        )
    };

    if c.is_async {
        let field_src_lets = svc_field_src_lets(svc_block, all_services, svc_names, &quote! { self });
        let construct = svc_construct_expr(svc_block, all_services, svc_names);
        quote! {
            pub fn #method_name(&self, #(#params),*) -> impl ::std::future::Future<Output = #user_ret_ty> + 'static {
                #(#forget_stmts)*
                let (future, cb) = ::interoptopus::pattern::asynk::AsyncCallbackFuture::<#ffi_ret_ty>::new();
                (self.#ctor_field)(#(#ffi_args,)* cb);
                #(#field_src_lets)*
                async move {
                    let raw = future.await;
                    ::interoptopus::ffi::ServiceHandleMap::map_service_handle(raw, #construct)
                }
            }
        }
    } else {
        let field_copies = svc_field_copies(svc_block, all_services, svc_names, &quote! { self });
        quote! {
            pub fn #method_name(&self, #(#params),*) -> #user_ret_ty {
                #(#forget_stmts)*
                let raw: #ffi_ret_ty = (self.#ctor_field)(#(#ffi_args),*);
                ::interoptopus::ffi::ServiceHandleMap::map_service_handle(raw, |handle| #svc_name { handle, #(#field_copies,)* })
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Service struct + impl + drop + trait
// ---------------------------------------------------------------------------

fn emit_service_struct(s: &ServiceBlock, all_services: &[ServiceBlock], svc_names: &HashSet<String>) -> TokenStream {
    let name = &s.name;
    let prefix = s.prefix();

    let inst_methods = s.instance_methods();
    let own_method_fields = inst_methods.iter().map(|m| {
        let field = prefixed_ident(&prefix, &m.name);
        let ffi_ptys: Vec<_> = m.params.iter().map(|p| ffi_param_ty(&p.ty, svc_names)).collect();
        let field_ty = method_ffi_fn_ty(&ffi_ptys, m.ret.as_ref(), m.is_async, name, svc_names);
        quote! { #field: #field_ty }
    });

    let own_drop_field = format_ident!("{}_drop", prefix);

    let returned = transitive_returned_services(s, all_services, svc_names);
    let extra_fields: Vec<_> = returned
        .iter()
        .flat_map(|svc_name| {
            let other = find_service(all_services, svc_name);
            let other_prefix = other.prefix();
            let other_ident = &other.name;
            let mut fields = Vec::new();
            for m in other.instance_methods() {
                let field = prefixed_ident(&other_prefix, &m.name);
                let ffi_ptys: Vec<_> = m.params.iter().map(|p| ffi_param_ty(&p.ty, svc_names)).collect();
                let field_ty = method_ffi_fn_ty(&ffi_ptys, m.ret.as_ref(), m.is_async, other_ident, svc_names);
                fields.push(quote! { #field: #field_ty });
            }
            let drop_field = format_ident!("{}_drop", other_prefix);
            fields.push(quote! { #drop_field: extern "C" fn(::interoptopus::ffi::ServiceHandle<#other_ident>) });
            fields
        })
        .collect();

    quote! {
        pub struct #name {
            handle: ::interoptopus::ffi::ServiceHandle<#name>,
            #(#own_method_fields,)*
            #own_drop_field: extern "C" fn(::interoptopus::ffi::ServiceHandle<#name>),
            #(#extra_fields,)*
        }
    }
}

fn emit_service_impl(s: &ServiceBlock, all_services: &[ServiceBlock], svc_names: &HashSet<String>) -> TokenStream {
    let name = &s.name;
    let prefix = s.prefix();

    let inst_methods = s.instance_methods();
    let methods = inst_methods.iter().map(|m| {
        emit_instance_method(&prefix, m, all_services, svc_names)
    });

    quote! {
        impl #name {
            #(#methods)*
        }
    }
}

fn emit_instance_method(
    prefix: &str,
    m: &crate::plugin::model::PluginMethod,
    all_services: &[ServiceBlock],
    svc_names: &HashSet<String>,
) -> TokenStream {
    let method_name = &m.name;
    let field = prefixed_ident(prefix, &m.name);
    let params = typed_params(&m.params);
    let ffi_args = ffi_call_args(&m.params, svc_names);
    let forget_stmts = forget_owned_services(&m.params, svc_names);

    let ret_svc_name = m.ret.as_ref().and_then(|ty| service_in_type(ty, svc_names));

    if let Some(ref svc_name) = ret_svc_name {
        let svc_block = find_service(all_services, svc_name);
        let ret_ty = &m.ret;
        let ffi_ret_ty = ffi_ret_or_unit(m.ret.as_ref(), svc_names);
        let ret_svc_ident = format_ident!("{}", svc_name);

        if m.is_async {
            let field_src_lets = svc_field_src_lets(svc_block, all_services, svc_names, &quote! { self });
            let construct = svc_construct_expr(svc_block, all_services, svc_names);
            quote! {
                pub fn #method_name(&self, #(#params),*) -> impl ::std::future::Future<Output = #ret_ty> + 'static {
                    #(#forget_stmts)*
                    let (future, cb) = ::interoptopus::pattern::asynk::AsyncCallbackFuture::<#ffi_ret_ty>::new();
                    (self.#field)(self.handle, #(#ffi_args,)* cb);
                    #(#field_src_lets)*
                    async move {
                        let raw = future.await;
                        ::interoptopus::ffi::ServiceHandleMap::map_service_handle(raw, #construct)
                    }
                }
            }
        } else {
            let field_copies = svc_field_copies(svc_block, all_services, svc_names, &quote! { self });
            quote! {
                pub fn #method_name(&self, #(#params),*) -> #ret_ty {
                    #(#forget_stmts)*
                    let raw: #ffi_ret_ty = (self.#field)(self.handle, #(#ffi_args),*);
                    ::interoptopus::ffi::ServiceHandleMap::map_service_handle(raw, |handle| #ret_svc_ident { handle, #(#field_copies,)* })
                }
            }
        }
    } else if m.is_async {
        let ret_ty = ffi_ret_or_unit(m.ret.as_ref(), svc_names);
        quote! {
            pub fn #method_name(&self, #(#params),*) -> impl ::std::future::Future<Output = #ret_ty> + 'static {
                #(#forget_stmts)*
                let (future, cb) = ::interoptopus::pattern::asynk::AsyncCallbackFuture::<#ret_ty>::new();
                (self.#field)(self.handle, #(#ffi_args,)* cb);
                future
            }
        }
    } else {
        let ret = ret_tokens(m.ret.as_ref());
        quote! {
            pub fn #method_name(&self, #(#params),*) #ret {
                #(#forget_stmts)*
                (self.#field)(self.handle, #(#ffi_args),*)
            }
        }
    }
}

fn emit_service_drop(s: &ServiceBlock) -> TokenStream {
    let name = &s.name;
    let drop_field = format_ident!("{}_drop", s.prefix());
    quote! {
        impl Drop for #name {
            fn drop(&mut self) {
                (self.#drop_field)(self.handle)
            }
        }
    }
}

/// Generate `impl TypeInfo` for each service struct, declaring it as `TypeKind::Service`.
fn emit_service_type_info(s: &ServiceBlock) -> TokenStream {
    let name = &s.name;
    let name_str = name.to_string();
    quote! {
        impl ::interoptopus::lang::types::TypeInfo for #name {
            const WIRE_SAFE: bool = false;
            const RAW_SAFE: bool = false;
            const ASYNC_SAFE: bool = false;
            const SERVICE_SAFE: bool = false;
            const SERVICE_CTOR_SAFE: bool = false;

            fn id() -> ::interoptopus::inventory::TypeId {
                ::interoptopus::inventory::TypeId::from_id(
                    ::interoptopus::inventory::Id::new(
                        ::interoptopus::inventory::hash_str(#name_str)
                    )
                )
            }

            fn kind() -> ::interoptopus::lang::types::TypeKind {
                ::interoptopus::lang::types::TypeKind::Service
            }

            fn ty() -> ::interoptopus::lang::types::Type {
                ::interoptopus::lang::types::Type {
                    name: #name_str.to_string(),
                    visibility: ::interoptopus::lang::meta::Visibility::Public,
                    docs: ::interoptopus::lang::meta::Docs::default(),
                    emission: ::interoptopus::lang::meta::Emission::FileEmission(
                        ::interoptopus::lang::meta::FileEmission::Default,
                    ),
                    kind: <Self as ::interoptopus::lang::types::TypeInfo>::kind(),
                }
            }

            fn register(inventory: &mut impl ::interoptopus::inventory::Inventory) {
                inventory.register_type(<Self as ::interoptopus::lang::types::TypeInfo>::id(), <Self as ::interoptopus::lang::types::TypeInfo>::ty());
            }
        }
    }
}

/// Generate `Send + Sync` impls for service structs.
///
/// The handle is an opaque pointer that is never dereferenced on the Rust side,
/// so it is safe to send across threads.
fn emit_service_send_sync(s: &ServiceBlock) -> TokenStream {
    let name = &s.name;
    quote! {
        unsafe impl Send for #name {}
        unsafe impl Sync for #name {}
    }
}

/// Generate `PluginService` trait impl for each service struct.
fn emit_service_trait(s: &ServiceBlock) -> TokenStream {
    let name = &s.name;
    quote! {
        impl ::interoptopus::ffi::PluginService for #name {
            fn service_handle(&self) -> ::interoptopus::ffi::ServiceHandle<Self> { self.handle }
            fn into_service_handle(self) -> ::interoptopus::ffi::ServiceHandle<Self> {
                let handle = self.handle;
                ::std::mem::forget(self);
                handle
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Service registration
// ---------------------------------------------------------------------------

fn emit_service_registration(s: &ServiceBlock, svc_names: &HashSet<String>) -> TokenStream {
    let svc_name = &s.name;
    let svc_name_str = svc_name.to_string();
    let prefix = s.prefix();

    // Register service type and ServiceHandle<Service> via TypeInfo.
    let register_types = quote! {
        <#svc_name as ::interoptopus::lang::types::TypeInfo>::register(inventory);
        <::interoptopus::ffi::ServiceHandle<#svc_name> as ::interoptopus::lang::types::TypeInfo>::register(inventory);
    };

    let ctors = s.ctors();
    let methods = s.instance_methods();

    let ctor_fn_names: Vec<String> = ctors.iter().map(|c| format!("{}_{}", prefix, c.name)).collect();
    let method_fn_names: Vec<String> = methods.iter().map(|m| format!("{}_{}", prefix, m.name)).collect();
    let destructor_fn_name = format!("{prefix}_drop");

    let ctor_registrations = ctors.iter().zip(ctor_fn_names.iter()).map(|(ctor, fn_name)| {
        let (ffi_ret, cb_ty) = if ctor.is_async {
            let cb_inner = ffi_ctor_cb_ret(ctor, svc_name);
            let cb = Some(quote! { ::interoptopus::pattern::asynk::AsyncCallback<#cb_inner> });
            (None, cb)
        } else if is_self_return(ctor.ret.as_ref()) {
            let ret_ty: Type = syn::parse_quote! { ::interoptopus::ffi::ServiceHandle<#svc_name> };
            (Some(ret_ty), None)
        } else {
            let user_ty = replace_self(ctor.ret.as_ref().unwrap(), svc_name);
            let ret_ty: Type = syn::parse_quote! { <#user_ty as ::interoptopus::ffi::ServiceAs<#svc_name>>::FFI };
            (Some(ret_ty), None)
        };
        emit_function_registration(fn_name, &ctor.params, ffi_ret.as_ref(), cb_ty, svc_names)
    });

    let method_registrations = methods.iter().zip(method_fn_names.iter()).map(|(method, fn_name)| {
        let (ffi_ret, cb_ty) = if method.is_async {
            let cb_inner = ffi_ret_or_unit(method.ret.as_ref(), svc_names);
            let cb = Some(quote! { ::interoptopus::pattern::asynk::AsyncCallback<#cb_inner> });
            (None, cb)
        } else {
            let ffi_ret = method.ret.as_ref().map(|ty| ffi_reg_ret(ty, svc_names));
            (ffi_ret, None)
        };
        emit_function_registration(fn_name, &method.params, ffi_ret.as_ref(), cb_ty, svc_names)
    });

    // Destructor takes ServiceHandle<Service>.
    let destructor_params = [PluginParam {
        name: format_ident!("handle"),
        ty: syn::parse_quote! { ::interoptopus::ffi::ServiceHandle<#svc_name> },
    }];
    let destructor_registration = emit_function_registration(&destructor_fn_name, &destructor_params, None, None, svc_names);

    let ctor_id_exprs = ctor_fn_names.iter().map(|n| function_id_expr(n));
    let method_id_exprs = method_fn_names.iter().map(|n| function_id_expr(n));
    let destructor_id_expr = function_id_expr(&destructor_fn_name);

    let service_id_expr = quote! {
        ::interoptopus::inventory::ServiceId::from_id(
            ::interoptopus::inventory::Id::new(
                ::interoptopus::inventory::hash_str(#svc_name_str)
            )
        )
    };

    let type_id_expr = quote! { <#svc_name as ::interoptopus::lang::types::TypeInfo>::id() };

    quote! {
        #register_types
        #(#ctor_registrations)*
        #(#method_registrations)*
        #destructor_registration
        {
            let service = ::interoptopus::lang::service::Service::new(
                #type_id_expr,
                vec![#(#ctor_id_exprs),*],
                #destructor_id_expr,
                vec![#(#method_id_exprs),*],
            );
            inventory.register_service(#service_id_expr, service);
        }
    }
}

// ---------------------------------------------------------------------------
// Function registration
// ---------------------------------------------------------------------------

fn emit_function_registration(
    fn_name_str: &str,
    params: &[PluginParam],
    ret: Option<&Type>,
    async_callback_ty: Option<TokenStream>,
    svc_names: &HashSet<String>,
) -> TokenStream {
    let type_registrations: Vec<_> = params
        .iter()
        .map(|p| {
            let ty = &p.ty;
            if let Some(svc_name) = direct_service_name(ty, svc_names) {
                let svc_ident = format_ident!("{}", svc_name);
                quote! { <::interoptopus::ffi::ServiceHandle<#svc_ident> as ::interoptopus::lang::types::TypeInfo>::register(inventory); }
            } else if let Some(svc_name) = ref_service_name(ty, svc_names) {
                let svc_ident = format_ident!("{}", svc_name);
                quote! { <::interoptopus::ffi::ServiceHandle<#svc_ident> as ::interoptopus::lang::types::TypeInfo>::register(inventory); }
            } else {
                quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory); }
            }
        })
        .collect();

    let callback_registration = async_callback_ty.as_ref().map(|cb_ty| {
        quote! { <#cb_ty as ::interoptopus::lang::types::TypeInfo>::register(inventory); }
    });

    let ret_registration = match ret {
        Some(ty) => Some(quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory); }),
        None => Some(quote! { <() as ::interoptopus::lang::types::TypeInfo>::register(inventory); }),
    };

    let arguments: Vec<_> = params
        .iter()
        .map(|p| {
            let pname_str = p.name.to_string();
            let pty = &p.ty;
            if let Some(svc_name) = direct_service_name(pty, svc_names) {
                let svc_ident = format_ident!("{}", svc_name);
                quote! { ::interoptopus::lang::function::Argument::new(#pname_str, <::interoptopus::ffi::ServiceHandle<#svc_ident> as ::interoptopus::lang::types::TypeInfo>::id()) }
            } else if let Some(svc_name) = ref_service_name(pty, svc_names) {
                let svc_ident = format_ident!("{}", svc_name);
                quote! { ::interoptopus::lang::function::Argument::new(#pname_str, <::interoptopus::ffi::ServiceHandle<#svc_ident> as ::interoptopus::lang::types::TypeInfo>::id()) }
            } else {
                quote! { ::interoptopus::lang::function::Argument::new(#pname_str, <#pty as ::interoptopus::lang::types::TypeInfo>::id()) }
            }
        })
        .collect();

    let callback_argument = async_callback_ty.as_ref().map(|cb_ty| {
        quote! { ::interoptopus::lang::function::Argument::new("cb", <#cb_ty as ::interoptopus::lang::types::TypeInfo>::id()) }
    });

    let rval = if let Some(ty) = ret {
        quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::id() }
    } else {
        quote! { <() as ::interoptopus::lang::types::TypeInfo>::id() }
    };

    quote! {
        {
            #(#type_registrations)*
            #callback_registration
            #ret_registration
            let id = ::interoptopus::inventory::FunctionId::from_id(
                ::interoptopus::inventory::Id::new(::interoptopus::inventory::hash_str(#fn_name_str))
            );
            let function = ::interoptopus::lang::function::Function {
                name: #fn_name_str.to_string(),
                visibility: ::interoptopus::lang::meta::Visibility::Public,
                docs: ::interoptopus::lang::meta::Docs::default(),
                emission: ::interoptopus::lang::meta::Emission::FileEmission(::interoptopus::lang::meta::FileEmission::Default),
                signature: ::interoptopus::lang::function::Signature {
                    arguments: vec![#(#arguments,)* #callback_argument],
                    rval: #rval,
                },
            };
            inventory.register_function(id, function);
        }
    }
}

// ---------------------------------------------------------------------------
// FFI type helpers — replace service types with ServiceHandle<Service>
// ---------------------------------------------------------------------------

fn ffi_param_ty(ty: &Type, svc_names: &HashSet<String>) -> TokenStream {
    if let Some(svc_name) = direct_service_name(ty, svc_names) {
        let svc_ident = format_ident!("{}", svc_name);
        quote! { ::interoptopus::ffi::ServiceHandle<#svc_ident> }
    } else if let Some(svc_name) = ref_service_name(ty, svc_names) {
        let svc_ident = format_ident!("{}", svc_name);
        quote! { ::interoptopus::ffi::ServiceHandle<#svc_ident> }
    } else {
        quote! { #ty }
    }
}

fn ffi_ret_arrow(ret: Option<&Type>, svc_names: &HashSet<String>) -> TokenStream {
    match ret {
        Some(ty) => {
            if let Some(svc_name) = direct_service_name(ty, svc_names) {
                let svc_ident = format_ident!("{}", svc_name);
                quote! { -> ::interoptopus::ffi::ServiceHandle<#svc_ident> }
            } else if let Some(svc_name) = service_in_type(ty, svc_names) {
                let svc_ident = format_ident!("{}", svc_name);
                quote! { -> <#ty as ::interoptopus::ffi::ServiceAs<#svc_ident>>::FFI }
            } else {
                quote! { -> #ty }
            }
        }
        None => quote! {},
    }
}

fn ffi_ret_or_unit(ret: Option<&Type>, svc_names: &HashSet<String>) -> TokenStream {
    match ret {
        Some(ty) => {
            if let Some(svc_name) = direct_service_name(ty, svc_names) {
                let svc_ident = format_ident!("{}", svc_name);
                quote! { ::interoptopus::ffi::ServiceHandle<#svc_ident> }
            } else if let Some(svc_name) = service_in_type(ty, svc_names) {
                let svc_ident = format_ident!("{}", svc_name);
                quote! { <#ty as ::interoptopus::ffi::ServiceAs<#svc_ident>>::FFI }
            } else {
                quote! { #ty }
            }
        }
        None => quote! { () },
    }
}

fn ffi_reg_ret(ty: &Type, svc_names: &HashSet<String>) -> Type {
    if let Some(svc_name) = direct_service_name(ty, svc_names) {
        let svc_ident = format_ident!("{}", svc_name);
        syn::parse_quote! { ::interoptopus::ffi::ServiceHandle<#svc_ident> }
    } else if let Some(svc_name) = service_in_type(ty, svc_names) {
        let svc_ident = format_ident!("{}", svc_name);
        syn::parse_quote! { <#ty as ::interoptopus::ffi::ServiceAs<#svc_ident>>::FFI }
    } else {
        ty.clone()
    }
}

// ---------------------------------------------------------------------------
// FFI fn type helpers for struct field declarations
// ---------------------------------------------------------------------------

fn ctor_ffi_fn_ty(ffi_ptys: &[TokenStream], c: &crate::plugin::model::PluginMethod, svc_ident: &Ident) -> TokenStream {
    if c.is_async {
        let cb_ret = ffi_ctor_cb_ret(c, svc_ident);
        quote! { extern "C" fn(#(#ffi_ptys,)* ::interoptopus::pattern::asynk::AsyncCallback<#cb_ret>) }
    } else if is_self_return(c.ret.as_ref()) {
        quote! { extern "C" fn(#(#ffi_ptys),*) -> ::interoptopus::ffi::ServiceHandle<#svc_ident> }
    } else {
        let user_ty = replace_self(c.ret.as_ref().unwrap(), svc_ident);
        quote! { extern "C" fn(#(#ffi_ptys),*) -> <#user_ty as ::interoptopus::ffi::ServiceAs<#svc_ident>>::FFI }
    }
}

fn ffi_ctor_cb_ret(c: &crate::plugin::model::PluginMethod, svc_ident: &Ident) -> TokenStream {
    if is_self_return(c.ret.as_ref()) {
        quote! { ::interoptopus::ffi::ServiceHandle<#svc_ident> }
    } else {
        let user_ty = replace_self(c.ret.as_ref().unwrap(), svc_ident);
        quote! { <#user_ty as ::interoptopus::ffi::ServiceAs<#svc_ident>>::FFI }
    }
}

fn method_ffi_fn_ty(ffi_ptys: &[TokenStream], ret: Option<&Type>, is_async: bool, self_svc: &Ident, svc_names: &HashSet<String>) -> TokenStream {
    if is_async {
        let cb_ret = ffi_ret_or_unit(ret, svc_names);
        quote! { extern "C" fn(::interoptopus::ffi::ServiceHandle<#self_svc>, #(#ffi_ptys,)* ::interoptopus::pattern::asynk::AsyncCallback<#cb_ret>) }
    } else {
        let ret_toks = ffi_ret_arrow(ret, svc_names);
        quote! { extern "C" fn(::interoptopus::ffi::ServiceHandle<#self_svc>, #(#ffi_ptys),*) #ret_toks }
    }
}

// ---------------------------------------------------------------------------
// Service struct field helpers — collecting fn pointer fields to copy
// ---------------------------------------------------------------------------

fn svc_field_copies(svc: &ServiceBlock, all_services: &[ServiceBlock], svc_names: &HashSet<String>, source: &TokenStream) -> Vec<TokenStream> {
    let mut copies = Vec::new();
    let prefix = svc.prefix();
    for m in svc.instance_methods() {
        let field = prefixed_ident(&prefix, &m.name);
        copies.push(quote! { #field: #source.#field });
    }
    let drop_field = format_ident!("{}_drop", prefix);
    copies.push(quote! { #drop_field: #source.#drop_field });

    for svc_name in transitive_returned_services(svc, all_services, svc_names) {
        let other = find_service(all_services, &svc_name);
        let other_prefix = other.prefix();
        for m in other.instance_methods() {
            let field = prefixed_ident(&other_prefix, &m.name);
            copies.push(quote! { #field: #source.#field });
        }
        let drop_field = format_ident!("{}_drop", other_prefix);
        copies.push(quote! { #drop_field: #source.#drop_field });
    }
    copies
}

fn svc_field_src_lets(svc: &ServiceBlock, all_services: &[ServiceBlock], svc_names: &HashSet<String>, source: &TokenStream) -> Vec<TokenStream> {
    let mut lets = Vec::new();
    let prefix = svc.prefix();
    for m in svc.instance_methods() {
        let field = prefixed_ident(&prefix, &m.name);
        lets.push(quote! { let #field = #source.#field; });
    }
    let drop_field = format_ident!("{}_drop", prefix);
    lets.push(quote! { let #drop_field = #source.#drop_field; });

    for svc_name in transitive_returned_services(svc, all_services, svc_names) {
        let other = find_service(all_services, &svc_name);
        let other_prefix = other.prefix();
        for m in other.instance_methods() {
            let field = prefixed_ident(&other_prefix, &m.name);
            lets.push(quote! { let #field = #source.#field; });
        }
        let drop_field = format_ident!("{}_drop", other_prefix);
        lets.push(quote! { let #drop_field = #source.#drop_field; });
    }
    lets
}

fn svc_field_inits(svc: &ServiceBlock, all_services: &[ServiceBlock], svc_names: &HashSet<String>) -> Vec<TokenStream> {
    let mut inits = Vec::new();
    let prefix = svc.prefix();
    for m in svc.instance_methods() {
        let field = prefixed_ident(&prefix, &m.name);
        inits.push(quote! { #field });
    }
    let drop_field = format_ident!("{}_drop", prefix);
    inits.push(quote! { #drop_field });

    for svc_name in transitive_returned_services(svc, all_services, svc_names) {
        let other = find_service(all_services, &svc_name);
        let other_prefix = other.prefix();
        for m in other.instance_methods() {
            let field = prefixed_ident(&other_prefix, &m.name);
            inits.push(quote! { #field });
        }
        let drop_field = format_ident!("{}_drop", other_prefix);
        inits.push(quote! { #drop_field });
    }
    inits
}

/// Generate a closure expression `|handle| ServiceName { handle, field1, field2, ... }`
/// suitable for use inside `async move` blocks (where field names are already captured).
fn svc_construct_expr(svc: &ServiceBlock, all_services: &[ServiceBlock], svc_names: &HashSet<String>) -> TokenStream {
    let svc_ident = &svc.name;
    let field_inits = svc_field_inits(svc, all_services, svc_names);
    quote! { |handle| #svc_ident { handle, #(#field_inits,)* } }
}

// ---------------------------------------------------------------------------
// Call argument helpers
// ---------------------------------------------------------------------------

fn ffi_call_args(params: &[PluginParam], svc_names: &HashSet<String>) -> Vec<TokenStream> {
    params
        .iter()
        .map(|p| {
            let pname = &p.name;
            if direct_service_name(&p.ty, svc_names).is_some() {
                // Owned service — pass handle by value.
                quote! { #pname.handle }
            } else if ref_service_name(&p.ty, svc_names).is_some() {
                // Reference to service — pass handle by value (same as owned).
                quote! { #pname.handle }
            } else {
                quote! { #pname }
            }
        })
        .collect()
}

fn forget_owned_services(params: &[PluginParam], svc_names: &HashSet<String>) -> Vec<TokenStream> {
    params
        .iter()
        .filter_map(|p| {
            if direct_service_name(&p.ty, svc_names).is_some() {
                let pname = &p.name;
                Some(quote! { let #pname = ::std::mem::ManuallyDrop::new(#pname); })
            } else {
                None
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// General helpers
// ---------------------------------------------------------------------------

fn ret_tokens(ret: Option<&Type>) -> TokenStream {
    match ret {
        Some(ty) => quote! { -> #ty },
        None => quote! {},
    }
}

fn typed_params(params: &[PluginParam]) -> Vec<TokenStream> {
    params.iter().map(|p| { let pname = &p.name; let pty = &p.ty; quote! { #pname: #pty } }).collect()
}

fn prefixed_ident(prefix: &str, name: &Ident) -> Ident {
    format_ident!("{}_{}", prefix, name)
}

fn find_service<'a>(all_services: &'a [ServiceBlock], name: &str) -> &'a ServiceBlock {
    all_services.iter().find(|s| s.name.to_string() == name).unwrap()
}

fn emit_load_field(field: &Ident, symbol: &str, fn_ty: TokenStream) -> TokenStream {
    quote! {
        #field: {
            let ptr = loader(#symbol);
            if ptr.is_null() {
                return Err(::interoptopus::lang::plugin::PluginLoadError::SymbolNotFound(#symbol.to_string()));
            }
            unsafe { ::std::mem::transmute::<*const u8, #fn_ty>(ptr) }
        }
    }
}

fn function_id_expr(fn_name: &str) -> TokenStream {
    quote! { ::interoptopus::inventory::FunctionId::from_id(::interoptopus::inventory::Id::new(::interoptopus::inventory::hash_str(#fn_name))) }
}
