use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Type;

use crate::plugin::model::{PluginMethod, PluginModel, PluginService};

/// Helper: build the prefixed field name for a service symbol stored on the plugin struct.
/// e.g., service "Foo", method "bar" -> field name "foo_bar"
fn svc_field_name(svc_name_lower: &str, method_name: &Ident) -> Ident {
    format_ident!("{}_{}", svc_name_lower, method_name)
}

impl PluginModel {
    pub fn emit(&self) -> TokenStream {
        let plugin_struct = self.emit_plugin_struct();
        let plugin_impl = self.emit_plugin_impl();
        let plugin_info_impl = self.emit_plugin_info_impl();
        let plugin_trait_impl = self.emit_plugin_trait_impl();
        let services = self.services.iter().map(|s| self.emit_service(s));

        quote! {
            #plugin_struct
            #plugin_impl
            #plugin_info_impl
            #plugin_trait_impl
            #(#services)*
        }
    }

    fn emit_plugin_struct(&self) -> TokenStream {
        let name = &self.name;

        // Free function fields
        let fn_fields = self.functions.iter().map(|f| {
            let field_name = &f.name;
            let param_tys = f.params.iter().map(|p| &p.ty);
            let ret = match &f.ret {
                Some(ty) => quote! { -> #ty },
                None => quote! {},
            };
            quote! {
                #field_name: unsafe extern "C" fn(#(#param_tys),*) #ret
            }
        });

        // Service fields: ctor, methods (with isize handle prefix), drop
        let svc_fields = self.services.iter().flat_map(|s| {
            let svc_lower = s.name.to_string().to_lowercase();
            let ctors: Vec<_> = s.methods.iter().filter(|m| !m.has_self && is_self_return(&m.ret)).collect();
            let methods: Vec<_> = s.methods.iter().filter(|m| m.has_self).collect();

            let mut fields = Vec::new();

            // Ctor fields: fn() -> isize
            for c in &ctors {
                let field = svc_field_name(&svc_lower, &c.name);
                fields.push(quote! { #field: unsafe extern "C" fn() -> isize });
            }

            // Method fields: fn(isize, params...) -> ret
            for m in &methods {
                let field = svc_field_name(&svc_lower, &m.name);
                let param_tys: Vec<_> = m.params.iter().map(|p| &p.ty).collect();
                let ret = match &m.ret {
                    Some(ty) => quote! { -> #ty },
                    None => quote! {},
                };
                fields.push(quote! { #field: unsafe extern "C" fn(isize, #(#param_tys),*) #ret });
            }

            // Drop field
            let drop_field = format_ident!("{}_drop", svc_lower);
            fields.push(quote! { #drop_field: unsafe extern "C" fn(isize) });

            fields
        });

        quote! {
            struct #name {
                #(#fn_fields,)*
                #(#svc_fields,)*
            }
        }
    }

    fn emit_plugin_impl(&self) -> TokenStream {
        let name = &self.name;

        // Free function methods
        let fn_methods = self.functions.iter().map(|f| {
            let fn_name = &f.name;
            let params = f.params.iter().map(|p| {
                let pname = &p.name;
                let pty = &p.ty;
                quote! { #pname: #pty }
            });
            let arg_names = f.params.iter().map(|p| &p.name);
            let ret = match &f.ret {
                Some(ty) => quote! { -> #ty },
                None => quote! {},
            };
            quote! {
                pub fn #fn_name(&self, #(#params),*) #ret {
                    unsafe { (self.#fn_name)(#(#arg_names),*) }
                }
            }
        });

        // Service factory methods: e.g., foo_create(&self) -> Foo
        let svc_factory_methods = self.services.iter().flat_map(|s| {
            let svc_name = &s.name;
            let svc_lower = s.name.to_string().to_lowercase();
            let ctors: Vec<_> = s.methods.iter().filter(|m| !m.has_self && is_self_return(&m.ret)).collect();
            let methods: Vec<_> = s.methods.iter().filter(|m| m.has_self).collect();

            ctors.into_iter().map(move |c| {
                let factory_name = svc_field_name(&svc_lower, &c.name);

                // Build the field copies for each method fn pointer
                let method_field_copies = methods.iter().map(|m| {
                    let field = svc_field_name(&svc_lower, &m.name);
                    quote! { #field: self.#field }
                });

                let drop_field = format_ident!("{}_drop", svc_lower);

                quote! {
                    pub fn #factory_name(&self) -> #svc_name {
                        let handle = unsafe { (self.#factory_name)() };
                        #svc_name {
                            handle,
                            #(#method_field_copies,)*
                            #drop_field: self.#drop_field,
                        }
                    }
                }
            }).collect::<Vec<_>>()
        });

        quote! {
            impl #name {
                #(#fn_methods)*
                #(#svc_factory_methods)*
            }
        }
    }

    fn emit_plugin_trait_impl(&self) -> TokenStream {
        let name = &self.name;

        // Free function field loads
        let fn_field_loads = self.functions.iter().map(|f| {
            let field_name = &f.name;
            let symbol_str = field_name.to_string();
            let param_tys = f.params.iter().map(|p| &p.ty);
            let ret = match &f.ret {
                Some(ty) => quote! { -> #ty },
                None => quote! {},
            };
            quote! {
                #field_name: {
                    let ptr = loader(#symbol_str);
                    if ptr.is_null() {
                        return Err(::interoptopus::lang::plugin::PluginLoadError::SymbolNotFound(
                            #symbol_str.to_string()
                        ));
                    }
                    unsafe {
                        ::std::mem::transmute::<*const u8, unsafe extern "C" fn(#(#param_tys),*) #ret>(ptr)
                    }
                }
            }
        });

        // Service field loads
        let svc_field_loads = self.services.iter().flat_map(|s| {
            let svc_lower = s.name.to_string().to_lowercase();
            let ctors: Vec<_> = s.methods.iter().filter(|m| !m.has_self && is_self_return(&m.ret)).collect();
            let methods: Vec<_> = s.methods.iter().filter(|m| m.has_self).collect();

            let mut loads = Vec::new();

            // Ctor loads
            for c in &ctors {
                let field = svc_field_name(&svc_lower, &c.name);
                let symbol = format!("{}_{}", svc_lower, c.name);
                loads.push(quote! {
                    #field: {
                        let ptr = loader(#symbol);
                        if ptr.is_null() {
                            return Err(::interoptopus::lang::plugin::PluginLoadError::SymbolNotFound(
                                #symbol.to_string()
                            ));
                        }
                        unsafe {
                            ::std::mem::transmute::<*const u8, unsafe extern "C" fn() -> isize>(ptr)
                        }
                    }
                });
            }

            // Method loads
            for m in &methods {
                let field = svc_field_name(&svc_lower, &m.name);
                let symbol = format!("{}_{}", svc_lower, m.name);
                let param_tys: Vec<_> = m.params.iter().map(|p| &p.ty).collect();
                let ret = match &m.ret {
                    Some(ty) => quote! { -> #ty },
                    None => quote! {},
                };
                loads.push(quote! {
                    #field: {
                        let ptr = loader(#symbol);
                        if ptr.is_null() {
                            return Err(::interoptopus::lang::plugin::PluginLoadError::SymbolNotFound(
                                #symbol.to_string()
                            ));
                        }
                        unsafe {
                            ::std::mem::transmute::<*const u8, unsafe extern "C" fn(isize, #(#param_tys),*) #ret>(ptr)
                        }
                    }
                });
            }

            // Drop load
            let drop_field = format_ident!("{}_drop", svc_lower);
            let drop_symbol = format!("{}_drop", svc_lower);
            loads.push(quote! {
                #drop_field: {
                    let ptr = loader(#drop_symbol);
                    if ptr.is_null() {
                        return Err(::interoptopus::lang::plugin::PluginLoadError::SymbolNotFound(
                            #drop_symbol.to_string()
                        ));
                    }
                    unsafe {
                        ::std::mem::transmute::<*const u8, unsafe extern "C" fn(isize)>(ptr)
                    }
                }
            });

            loads
        });

        quote! {
            impl ::interoptopus::lang::plugin::Plugin for #name {
                fn load_from(loader: impl Fn(&str) -> *const u8) -> Result<Self, ::interoptopus::lang::plugin::PluginLoadError> {
                    Ok(Self {
                        #(#fn_field_loads,)*
                        #(#svc_field_loads,)*
                    })
                }
            }
        }
    }

    fn emit_plugin_info_impl(&self) -> TokenStream {
        let name = &self.name;
        let name_str = name.to_string();

        let fn_registrations = self.functions.iter().map(|f| {
            emit_function_registration(&f.name.to_string(), &f.params, &f.ret)
        });

        let svc_registrations = self.services.iter().map(|s| {
            emit_service_registration(s)
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
                    #(#svc_registrations)*
                }
            }
        }
    }

    fn emit_service(&self, service: &PluginService) -> TokenStream {
        let service_name = &service.name;
        let svc_lower = service_name.to_string().to_lowercase();

        let methods: Vec<&PluginMethod> = service.methods.iter().filter(|m| m.has_self).collect();

        // --- Foo struct: handle + fn pointer fields (prefixed with svc name) ---
        let struct_fields = methods.iter().map(|m| {
            let field = svc_field_name(&svc_lower, &m.name);
            let param_tys: Vec<_> = m.params.iter().map(|p| &p.ty).collect();
            let ret = match &m.ret {
                Some(ty) => quote! { -> #ty },
                None => quote! {},
            };
            quote! {
                #field: unsafe extern "C" fn(isize, #(#param_tys),*) #ret
            }
        });

        let drop_field = format_ident!("{}_drop", svc_lower);

        let service_struct = quote! {
            struct #service_name {
                handle: isize,
                #(#struct_fields,)*
                #drop_field: unsafe extern "C" fn(isize),
            }
        };

        // --- Methods impl: delegate to fn pointers ---
        let method_impls = methods.iter().map(|m| {
            let method_name = &m.name;
            let field = svc_field_name(&svc_lower, &m.name);
            let params = m.params.iter().map(|p| {
                let pname = &p.name;
                let pty = &p.ty;
                quote! { #pname: #pty }
            });
            let arg_names = m.params.iter().map(|p| &p.name);
            let ret = match &m.ret {
                Some(ty) => quote! { -> #ty },
                None => quote! {},
            };
            quote! {
                pub fn #method_name(&self, #(#params),*) #ret {
                    unsafe { (self.#field)(self.handle, #(#arg_names),*) }
                }
            }
        });

        let methods_impl = quote! {
            impl #service_name {
                #(#method_impls)*
            }
        };

        // --- Drop impl ---
        let drop_impl = quote! {
            impl Drop for #service_name {
                fn drop(&mut self) {
                    unsafe { (self.#drop_field)(self.handle) }
                }
            }
        };

        quote! {
            #service_struct
            #methods_impl
            #drop_impl
        }
    }
}

/// Returns true if the given return type is `Self`.
fn is_self_return(ret: &Option<Type>) -> bool {
    match ret {
        Some(Type::Path(p)) => p.path.is_ident("Self"),
        _ => false,
    }
}

use crate::plugin::model::PluginParam;

/// Emits code to register a single function with the inventory.
///
/// For each parameter type, calls `TypeInfo::register`. Then builds a `Function`
/// and registers it. The `self_type_id` is used as the rval for ctors that return `Self`.
fn emit_function_registration_inner(
    fn_name_str: &str,
    params: &[PluginParam],
    ret: &Option<Type>,
    self_type_id: Option<&TokenStream>,
) -> TokenStream {
    let type_registrations = params.iter().map(|p| {
        let ty = &p.ty;
        quote! {
            <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
        }
    });

    let ret_registration = match ret {
        Some(ty) if !is_self_return(&Some(ty.clone())) => Some(quote! {
            <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
        }),
        _ => None,
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

/// Emits function registration for a free function (no `Self` type context).
fn emit_function_registration(
    fn_name_str: &str,
    params: &[PluginParam],
    ret: &Option<Type>,
) -> TokenStream {
    emit_function_registration_inner(fn_name_str, params, ret, None)
}

/// Emits registration for an entire service: type, functions, and service info.
fn emit_service_registration(service: &PluginService) -> TokenStream {
    let svc_name_str = service.name.to_string();
    let svc_name_lower = svc_name_str.to_lowercase();

    // Register the fictitious service type
    let type_id_expr = quote! {
        ::interoptopus::inventory::TypeId::from_id(
            ::interoptopus::inventory::Id::new(
                ::interoptopus::inventory::hash_str(#svc_name_str)
            )
        )
    };

    let register_type = quote! {
        {
            let ty_id = #type_id_expr;

            let ty = ::interoptopus::lang::types::Type {
                name: #svc_name_str.to_string(),
                visibility: ::interoptopus::lang::meta::Visibility::Public,
                docs: ::interoptopus::lang::meta::Docs::default(),
                emission: ::interoptopus::lang::meta::Emission::FileEmission(
                    ::interoptopus::lang::meta::FileEmission::Default,
                ),
                kind: ::interoptopus::lang::types::TypeKind::Service,
            };

            inventory.register_type(ty_id, ty);
        }
    };

    // Classify methods into ctors and regular methods
    let ctors: Vec<&PluginMethod> = service.methods.iter().filter(|m| !m.has_self && is_self_return(&m.ret)).collect();
    let methods: Vec<&PluginMethod> = service.methods.iter().filter(|m| m.has_self).collect();

    // Build prefixed function names
    let ctor_fn_names: Vec<String> = ctors.iter().map(|c| format!("{}_{}", svc_name_lower, c.name)).collect();
    let method_fn_names: Vec<String> = methods.iter().map(|m| format!("{}_{}", svc_name_lower, m.name)).collect();
    let destructor_fn_name = format!("{}_drop", svc_name_lower);

    // Register each ctor function
    let ctor_registrations = ctors.iter().zip(ctor_fn_names.iter()).map(|(ctor, fn_name)| {
        emit_function_registration_inner(fn_name, &ctor.params, &ctor.ret, Some(&type_id_expr))
    });

    // Register each method function
    let method_registrations = methods.iter().zip(method_fn_names.iter()).map(|(method, fn_name)| {
        emit_function_registration_inner(fn_name, &method.params, &method.ret, Some(&type_id_expr))
    });

    // Register the synthetic destructor
    let destructor_registration = emit_function_registration_inner(&destructor_fn_name, &[], &None, None);

    // Build FunctionId expressions for the Service struct
    let ctor_id_exprs = ctor_fn_names.iter().map(|fn_name| {
        quote! {
            ::interoptopus::inventory::FunctionId::from_id(
                ::interoptopus::inventory::Id::new(
                    ::interoptopus::inventory::hash_str(#fn_name)
                )
            )
        }
    });

    let method_id_exprs = method_fn_names.iter().map(|fn_name| {
        quote! {
            ::interoptopus::inventory::FunctionId::from_id(
                ::interoptopus::inventory::Id::new(
                    ::interoptopus::inventory::hash_str(#fn_name)
                )
            )
        }
    });

    let destructor_id_expr = quote! {
        ::interoptopus::inventory::FunctionId::from_id(
            ::interoptopus::inventory::Id::new(
                ::interoptopus::inventory::hash_str(#destructor_fn_name)
            )
        )
    };

    let service_id_expr = quote! {
        ::interoptopus::inventory::ServiceId::from_id(
            ::interoptopus::inventory::Id::new(
                ::interoptopus::inventory::hash_str(#svc_name_str)
            )
        )
    };

    quote! {
        // Register the service type
        #register_type

        // Register ctor functions
        #(#ctor_registrations)*

        // Register method functions
        #(#method_registrations)*

        // Register destructor
        #destructor_registration

        // Register the service itself
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
