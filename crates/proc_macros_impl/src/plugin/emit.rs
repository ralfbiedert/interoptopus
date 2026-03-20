use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Type;

use crate::plugin::model::{PluginModel, PluginParam, ServiceBlock, is_self_return};

impl PluginModel {
    pub fn emit(&self) -> TokenStream {
        let plugin_struct = self.emit_plugin_struct();
        let plugin_impl = self.emit_plugin_impl();
        let plugin_trait = self.emit_plugin_trait();
        let plugin_info = self.emit_plugin_info();
        let service_structs = self.services.iter().map(|s| emit_service_struct(s));
        let service_impls = self.services.iter().map(|s| emit_service_impl(s));
        let service_drops = self.services.iter().map(|s| emit_service_drop(s));

        quote! {
            #plugin_struct
            #plugin_impl
            #plugin_trait
            #plugin_info
            #(#service_structs)*
            #(#service_impls)*
            #(#service_drops)*
        }
    }

    // -----------------------------------------------------------------------
    // Plugin struct — holds all fn pointers (bare fns + service fns)
    // -----------------------------------------------------------------------

    fn emit_plugin_struct(&self) -> TokenStream {
        let name = &self.name;

        let bare_fields = self.functions.iter().map(|f| {
            let field_name = &f.name;
            let param_tys = f.params.iter().map(|p| &p.ty);
            let ret = ret_tokens(&f.ret);
            quote! { #field_name: extern "C" fn(#(#param_tys),*) #ret }
        });

        let service_fields = self.services.iter().flat_map(|s| {
            let prefix = s.prefix();
            let mut fields = Vec::new();

            // Ctor fields
            for c in s.ctors() {
                let field = prefixed_ident(&prefix, &c.name);
                let param_tys: Vec<_> = c.params.iter().map(|p| &p.ty).collect();
                fields.push(quote! { #field: extern "C" fn(#(#param_tys),*) -> isize });
            }

            // Method fields (with isize handle as first param)
            for m in s.instance_methods() {
                let field = prefixed_ident(&prefix, &m.name);
                let param_tys: Vec<_> = m.params.iter().map(|p| &p.ty).collect();
                let ret = ret_tokens(&m.ret);
                fields.push(quote! { #field: extern "C" fn(isize, #(#param_tys),*) #ret });
            }

            // Drop field
            let drop_field = format_ident!("{}_drop", prefix);
            fields.push(quote! { #drop_field: extern "C" fn(isize) });

            fields
        });

        // Trampoline registration function pointer
        let register_trampoline_field = quote! {
            register_trampoline: extern "C" fn(i64, *const u8)
        };

        quote! { struct #name { #(#bare_fields,)* #(#service_fields,)* #register_trampoline_field, } }
    }

    // -----------------------------------------------------------------------
    // Plugin impl — bare fn delegates + service ctor methods
    // -----------------------------------------------------------------------

    fn emit_plugin_impl(&self) -> TokenStream {
        let name = &self.name;

        let bare_methods = self.functions.iter().map(|f| {
            let fn_name = &f.name;
            let params = typed_params(&f.params);
            let arg_names: Vec<_> = f.params.iter().map(|p| &p.name).collect();
            let ret = ret_tokens(&f.ret);
            quote! {
                pub fn #fn_name(&self, #(#params),*) #ret {
                    (self.#fn_name)(#(#arg_names),*)
                }
            }
        });

        let ctor_methods = self.services.iter().flat_map(|s| {
            let prefix = s.prefix();
            let svc_name = &s.name;

            s.ctors()
                .into_iter()
                .map(move |c| {
                    let method_name = prefixed_ident(&prefix, &c.name);
                    let ctor_field = prefixed_ident(&prefix, &c.name);
                    let params = typed_params(&c.params);
                    let arg_names: Vec<_> = c.params.iter().map(|p| &p.name).collect();

                    // Copy method + drop fn pointers into the service struct
                    let method_copies = s
                        .instance_methods()
                        .iter()
                        .map(|m| {
                            let field = prefixed_ident(&prefix, &m.name);
                            quote! { #field: self.#field }
                        })
                        .collect::<Vec<_>>();

                    let drop_field = format_ident!("{}_drop", prefix);

                    quote! {
                        pub fn #method_name(&self, #(#params),*) -> #svc_name {
                            let handle = (self.#ctor_field)(#(#arg_names),*);
                            #svc_name {
                                handle,
                                #(#method_copies,)*
                                #drop_field: self.#drop_field,
                            }
                        }
                    }
                })
                .collect::<Vec<_>>()
        });

        quote! {
            impl #name {
                pub fn new(loader: &impl ::interoptopus::lang::plugin::Loader) -> Result<Self, ::interoptopus::lang::plugin::PluginLoadError> {
                    let plugin: Self = loader.load_plugin()?;
                    plugin.register_trampolines();
                    Ok(plugin)
                }

                /// Registers Rust runtime trampolines (wire alloc/free) with the foreign plugin.
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

    fn emit_plugin_trait(&self) -> TokenStream {
        let name = &self.name;

        let bare_loads = self.functions.iter().map(|f| {
            let field_name = &f.name;
            let symbol = field_name.to_string();
            let param_tys = f.params.iter().map(|p| &p.ty);
            let ret = ret_tokens(&f.ret);
            emit_load_field(field_name, &symbol, quote! { extern "C" fn(#(#param_tys),*) #ret })
        });

        let service_loads = self.services.iter().flat_map(|s| {
            let prefix = s.prefix();
            let mut loads = Vec::new();

            for c in s.ctors() {
                let field = prefixed_ident(&prefix, &c.name);
                let symbol = format!("{}_{}", prefix, c.name);
                let param_tys: Vec<_> = c.params.iter().map(|p| &p.ty).collect();
                loads.push(emit_load_field(&field, &symbol, quote! { extern "C" fn(#(#param_tys),*) -> isize }));
            }

            for m in s.instance_methods() {
                let field = prefixed_ident(&prefix, &m.name);
                let symbol = format!("{}_{}", prefix, m.name);
                let param_tys: Vec<_> = m.params.iter().map(|p| &p.ty).collect();
                let ret = ret_tokens(&m.ret);
                loads.push(emit_load_field(&field, &symbol, quote! { extern "C" fn(isize, #(#param_tys),*) #ret }));
            }

            let drop_field = format_ident!("{}_drop", prefix);
            let drop_symbol = format!("{}_drop", prefix);
            loads.push(emit_load_field(&drop_field, &drop_symbol, quote! { extern "C" fn(isize) }));

            loads
        });

        let register_trampoline_field = format_ident!("register_trampoline");
        let register_trampoline_load = emit_load_field(&register_trampoline_field, "register_trampoline", quote! { extern "C" fn(i64, *const u8) });

        quote! {
            impl ::interoptopus::lang::plugin::Plugin for #name {
                fn load_from(loader: impl Fn(&str) -> *const u8) -> Result<Self, ::interoptopus::lang::plugin::PluginLoadError> {
                    Ok(Self {
                        #(#bare_loads,)*
                        #(#service_loads,)*
                        #register_trampoline_load,
                    })
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // PluginInfo — registers types, functions, and services with the inventory
    // -----------------------------------------------------------------------

    fn emit_plugin_info(&self) -> TokenStream {
        let name = &self.name;
        let name_str = name.to_string();

        let bare_registrations = self
            .functions
            .iter()
            .map(|f| emit_function_registration(&f.name.to_string(), &f.params, &f.ret, None));

        let service_registrations = self.services.iter().map(|s| emit_service_registration(s));

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
// Service struct + impl + drop (generated per `impl Foo { ... }` block)
// ---------------------------------------------------------------------------

fn emit_service_struct(s: &ServiceBlock) -> TokenStream {
    let name = &s.name;
    let prefix = s.prefix();
    let inst_methods = s.instance_methods();

    let method_fields = inst_methods.iter().map(|m| {
        let field = prefixed_ident(&prefix, &m.name);
        let param_tys: Vec<_> = m.params.iter().map(|p| &p.ty).collect();
        let ret = ret_tokens(&m.ret);
        quote! { #field: extern "C" fn(isize, #(#param_tys),*) #ret }
    });

    let drop_field = format_ident!("{}_drop", prefix);

    quote! {
        struct #name {
            handle: isize,
            #(#method_fields,)*
            #drop_field: extern "C" fn(isize),
        }
    }
}

fn emit_service_impl(s: &ServiceBlock) -> TokenStream {
    let name = &s.name;
    let prefix = s.prefix();
    let inst_methods = s.instance_methods();

    let methods = inst_methods.iter().map(|m| {
        let method_name = &m.name;
        let field = prefixed_ident(&prefix, &m.name);
        let params = typed_params(&m.params);
        let arg_names: Vec<_> = m.params.iter().map(|p| &p.name).collect();
        let ret = ret_tokens(&m.ret);
        quote! {
            pub fn #method_name(&self, #(#params),*) #ret {
                (self.#field)(self.handle, #(#arg_names),*)
            }
        }
    });

    quote! {
        impl #name {
            #(#methods)*
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

fn emit_service_registration(s: &ServiceBlock) -> TokenStream {
    let svc_name_str = s.name.to_string();
    let prefix = s.prefix();

    let type_id_expr = quote! {
        ::interoptopus::inventory::TypeId::from_id(
            ::interoptopus::inventory::Id::new(
                ::interoptopus::inventory::hash_str(#svc_name_str)
            )
        )
    };

    let register_type = quote! {
        {
            let ty = ::interoptopus::lang::types::Type {
                name: #svc_name_str.to_string(),
                visibility: ::interoptopus::lang::meta::Visibility::Public,
                docs: ::interoptopus::lang::meta::Docs::default(),
                emission: ::interoptopus::lang::meta::Emission::FileEmission(
                    ::interoptopus::lang::meta::FileEmission::Default,
                ),
                kind: ::interoptopus::lang::types::TypeKind::Service,
            };
            inventory.register_type(#type_id_expr, ty);
        }
    };

    let ctors = s.ctors();
    let methods = s.instance_methods();

    let ctor_fn_names: Vec<String> = ctors.iter().map(|c| format!("{}_{}", prefix, c.name)).collect();
    let method_fn_names: Vec<String> = methods.iter().map(|m| format!("{}_{}", prefix, m.name)).collect();
    let destructor_fn_name = format!("{}_drop", prefix);

    let ctor_registrations = ctors
        .iter()
        .zip(ctor_fn_names.iter())
        .map(|(ctor, fn_name)| emit_function_registration(fn_name, &ctor.params, &ctor.ret, Some(&type_id_expr)));

    let method_registrations = methods
        .iter()
        .zip(method_fn_names.iter())
        .map(|(method, fn_name)| emit_function_registration(fn_name, &method.params, &method.ret, Some(&type_id_expr)));

    let destructor_registration = emit_function_registration(&destructor_fn_name, &[], &None, None);

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

    quote! {
        #register_type
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
// Helpers
// ---------------------------------------------------------------------------

fn ret_tokens(ret: &Option<Type>) -> TokenStream {
    match ret {
        Some(ty) => quote! { -> #ty },
        None => quote! {},
    }
}

fn typed_params(params: &[PluginParam]) -> Vec<TokenStream> {
    params
        .iter()
        .map(|p| {
            let pname = &p.name;
            let pty = &p.ty;
            quote! { #pname: #pty }
        })
        .collect()
}

fn prefixed_ident(prefix: &str, name: &Ident) -> Ident {
    format_ident!("{}_{}", prefix, name)
}

/// Emits a field initializer that loads a symbol and transmutes it to the given fn pointer type.
fn emit_load_field(field: &Ident, symbol: &str, fn_ty: TokenStream) -> TokenStream {
    quote! {
        #field: {
            let ptr = loader(#symbol);
            if ptr.is_null() {
                return Err(::interoptopus::lang::plugin::PluginLoadError::SymbolNotFound(
                    #symbol.to_string()
                ));
            }
            unsafe { ::std::mem::transmute::<*const u8, #fn_ty>(ptr) }
        }
    }
}

fn function_id_expr(fn_name: &str) -> TokenStream {
    quote! {
        ::interoptopus::inventory::FunctionId::from_id(
            ::interoptopus::inventory::Id::new(
                ::interoptopus::inventory::hash_str(#fn_name)
            )
        )
    }
}

/// Emits code to register a single function with the inventory.
fn emit_function_registration(fn_name_str: &str, params: &[PluginParam], ret: &Option<Type>, self_type_id: Option<&TokenStream>) -> TokenStream {
    let type_registrations = params.iter().map(|p| {
        let ty = &p.ty;
        quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory); }
    });

    let ret_registration = match ret {
        Some(ty) if !is_self_return(&Some(ty.clone())) => Some(quote! {
            <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
        }),
        Some(_) => None, // Self return — type already registered via register_type
        None => Some(quote! {
            <() as ::interoptopus::lang::types::TypeInfo>::register(inventory);
        }),
    };

    let arguments = params.iter().map(|p| {
        let pname_str = p.name.to_string();
        let pty = &p.ty;
        quote! {
            ::interoptopus::lang::function::Argument::new(
                #pname_str,
                <#pty as ::interoptopus::lang::types::TypeInfo>::id(),
            )
        }
    });

    let rval = if is_self_return(ret) {
        if let Some(tid) = self_type_id {
            quote! { #tid }
        } else {
            quote! { <() as ::interoptopus::lang::types::TypeInfo>::id() }
        }
    } else {
        match ret {
            Some(ty) => quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::id() },
            None => quote! { <() as ::interoptopus::lang::types::TypeInfo>::id() },
        }
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
}
