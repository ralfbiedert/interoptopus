use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Type;

use crate::plugin::model::{PluginMethod, PluginModel, PluginParam};

impl PluginModel {
    pub fn emit(&self) -> TokenStream {
        if self.is_service() {
            self.emit_service_plugin()
        } else {
            self.emit_function_plugin()
        }
    }

    // -----------------------------------------------------------------------
    // Function plugin (no &self methods) — e.g., AAA { fn do_math(...) -> ...; }
    // -----------------------------------------------------------------------

    fn emit_function_plugin(&self) -> TokenStream {
        let struc = self.emit_fn_struct();
        let imp = self.emit_fn_impl();
        let plugin_trait = self.emit_fn_plugin_trait();
        let info = self.emit_fn_info();

        quote! {
            #struc
            #imp
            #plugin_trait
            #info
        }
    }

    fn emit_fn_struct(&self) -> TokenStream {
        let name = &self.name;
        let fields = self.methods.iter().map(|f| {
            let field_name = &f.name;
            let param_tys = f.params.iter().map(|p| &p.ty);
            let ret = ret_tokens(&f.ret);
            quote! { #field_name: unsafe extern "C" fn(#(#param_tys),*) #ret }
        });

        quote! { struct #name { #(#fields,)* } }
    }

    fn emit_fn_impl(&self) -> TokenStream {
        let name = &self.name;
        let methods = self.methods.iter().map(|f| {
            let fn_name = &f.name;
            let params = typed_params(&f.params);
            let arg_names: Vec<_> = f.params.iter().map(|p| &p.name).collect();
            let ret = ret_tokens(&f.ret);
            quote! {
                pub fn #fn_name(&self, #(#params),*) #ret {
                    unsafe { (self.#fn_name)(#(#arg_names),*) }
                }
            }
        });

        quote! {
            impl #name {
                pub fn new(loader: &impl ::interoptopus::lang::plugin::Loader) -> Result<Self, ::interoptopus::lang::plugin::PluginLoadError> {
                    loader.load_plugin()
                }

                #(#methods)*
            }
        }
    }

    fn emit_fn_plugin_trait(&self) -> TokenStream {
        let name = &self.name;
        let field_loads = self.methods.iter().map(|f| {
            let field_name = &f.name;
            let symbol = field_name.to_string();
            let param_tys = f.params.iter().map(|p| &p.ty);
            let ret = ret_tokens(&f.ret);
            emit_load_field(field_name, &symbol, quote! { unsafe extern "C" fn(#(#param_tys),*) #ret })
        });

        quote! {
            impl ::interoptopus::lang::plugin::Plugin for #name {
                fn load_from(loader: impl Fn(&str) -> *const u8) -> Result<Self, ::interoptopus::lang::plugin::PluginLoadError> {
                    Ok(Self { #(#field_loads,)* })
                }
            }
        }
    }

    fn emit_fn_info(&self) -> TokenStream {
        let name = &self.name;
        let name_str = name.to_string();
        let registrations = self.methods.iter().map(|f| {
            emit_function_registration(&f.name.to_string(), &f.params, &f.ret, None)
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
                    #(#registrations)*
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Service plugin (has &self methods) — e.g., BBB { fn create() -> Self; fn bar(&self, ...); }
    // -----------------------------------------------------------------------

    fn emit_service_plugin(&self) -> TokenStream {
        let struc = self.emit_svc_struct();
        let imp = self.emit_svc_impl();
        let plugin_trait = self.emit_svc_plugin_trait();
        let info = self.emit_svc_info();
        let drop_impl = self.emit_svc_drop();

        quote! {
            #struc
            #imp
            #plugin_trait
            #info
            #drop_impl
        }
    }

    fn svc_prefix(&self) -> String {
        self.name.to_string().to_lowercase()
    }

    fn svc_ctors(&self) -> Vec<&PluginMethod> {
        self.methods.iter().filter(|m| !m.has_self && is_self_return(&m.ret)).collect()
    }

    fn svc_methods(&self) -> Vec<&PluginMethod> {
        self.methods.iter().filter(|m| m.has_self).collect()
    }

    fn emit_svc_struct(&self) -> TokenStream {
        let name = &self.name;
        let prefix = self.svc_prefix();
        let methods = self.svc_methods();

        let method_fields = methods.iter().map(|m| {
            let field = prefixed_ident(&prefix, &m.name);
            let param_tys: Vec<_> = m.params.iter().map(|p| &p.ty).collect();
            let ret = ret_tokens(&m.ret);
            quote! { #field: unsafe extern "C" fn(isize, #(#param_tys),*) #ret }
        });

        let drop_field = format_ident!("{}_drop", prefix);

        quote! {
            struct #name {
                handle: isize,
                #(#method_fields,)*
                #drop_field: unsafe extern "C" fn(isize),
            }
        }
    }

    fn emit_svc_impl(&self) -> TokenStream {
        let name = &self.name;
        let prefix = self.svc_prefix();
        let methods = self.svc_methods();

        // Instance methods: bar(&self, x) delegates to self.bbb_bar(self.handle, x)
        let method_impls = methods.iter().map(|m| {
            let method_name = &m.name;
            let field = prefixed_ident(&prefix, &m.name);
            let params = typed_params(&m.params);
            let arg_names: Vec<_> = m.params.iter().map(|p| &p.name).collect();
            let ret = ret_tokens(&m.ret);
            quote! {
                pub fn #method_name(&self, #(#params),*) #ret {
                    unsafe { (self.#field)(self.handle, #(#arg_names),*) }
                }
            }
        });

        // Static constructors: BBB::create(loader, path) -> Result<Self, ...>
        let ctors = self.svc_ctors();
        let ctor_methods = ctors.iter().map(|c| {
            let ctor_name = &c.name;
            quote! {
                pub fn #ctor_name(loader: &impl ::interoptopus::lang::plugin::Loader) -> Result<Self, ::interoptopus::lang::plugin::PluginLoadError> {
                    loader.load_plugin()
                }
            }
        });

        quote! {
            impl #name {
                #(#ctor_methods)*
                #(#method_impls)*
            }
        }
    }

    fn emit_svc_plugin_trait(&self) -> TokenStream {
        let name = &self.name;
        let prefix = self.svc_prefix();
        let ctors = self.svc_ctors();
        let methods = self.svc_methods();

        // We use the first ctor for load_from (zero-param ctors only for now)
        let ctor = ctors.first().expect("service plugin must have at least one constructor");
        let ctor_symbol = format!("{}_{}", prefix, ctor.name);

        let method_field_loads = methods.iter().map(|m| {
            let field = prefixed_ident(&prefix, &m.name);
            let symbol = format!("{}_{}", prefix, m.name);
            let param_tys: Vec<_> = m.params.iter().map(|p| &p.ty).collect();
            let ret = ret_tokens(&m.ret);
            emit_load_field(&field, &symbol, quote! { unsafe extern "C" fn(isize, #(#param_tys),*) #ret })
        });

        let drop_field = format_ident!("{}_drop", prefix);
        let drop_symbol = format!("{}_drop", prefix);
        let drop_load = emit_load_field(&drop_field, &drop_symbol, quote! { unsafe extern "C" fn(isize) });

        quote! {
            impl ::interoptopus::lang::plugin::Plugin for #name {
                fn load_from(loader: impl Fn(&str) -> *const u8) -> Result<Self, ::interoptopus::lang::plugin::PluginLoadError> {
                    // Load the constructor
                    let create_ptr = loader(#ctor_symbol);
                    if create_ptr.is_null() {
                        return Err(::interoptopus::lang::plugin::PluginLoadError::SymbolNotFound(
                            #ctor_symbol.to_string()
                        ));
                    }
                    let create_fn = unsafe {
                        ::std::mem::transmute::<*const u8, unsafe extern "C" fn() -> isize>(create_ptr)
                    };

                    let handle = unsafe { create_fn() };

                    Ok(Self {
                        handle,
                        #(#method_field_loads,)*
                        #drop_load,
                    })
                }
            }
        }
    }

    fn emit_svc_drop(&self) -> TokenStream {
        let name = &self.name;
        let drop_field = format_ident!("{}_drop", self.svc_prefix());

        quote! {
            impl Drop for #name {
                fn drop(&mut self) {
                    unsafe { (self.#drop_field)(self.handle) }
                }
            }
        }
    }

    fn emit_svc_info(&self) -> TokenStream {
        let name = &self.name;
        let name_str = name.to_string();
        let prefix = self.svc_prefix();

        let ctors = self.svc_ctors();
        let methods = self.svc_methods();

        let type_id_expr = quote! {
            ::interoptopus::inventory::TypeId::from_id(
                ::interoptopus::inventory::Id::new(
                    ::interoptopus::inventory::hash_str(#name_str)
                )
            )
        };

        let register_type = quote! {
            {
                let ty = ::interoptopus::lang::types::Type {
                    name: #name_str.to_string(),
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

        let ctor_fn_names: Vec<String> = ctors.iter().map(|c| format!("{}_{}", prefix, c.name)).collect();
        let method_fn_names: Vec<String> = methods.iter().map(|m| format!("{}_{}", prefix, m.name)).collect();
        let destructor_fn_name = format!("{}_drop", prefix);

        let ctor_registrations = ctors.iter().zip(ctor_fn_names.iter()).map(|(ctor, fn_name)| {
            emit_function_registration(fn_name, &ctor.params, &ctor.ret, Some(&type_id_expr))
        });

        let method_registrations = methods.iter().zip(method_fn_names.iter()).map(|(method, fn_name)| {
            emit_function_registration(fn_name, &method.params, &method.ret, Some(&type_id_expr))
        });

        let destructor_registration = emit_function_registration(&destructor_fn_name, &[], &None, None);

        let ctor_id_exprs = ctor_fn_names.iter().map(|n| function_id_expr(n));
        let method_id_exprs = method_fn_names.iter().map(|n| function_id_expr(n));
        let destructor_id_expr = function_id_expr(&destructor_fn_name);

        let service_id_expr = quote! {
            ::interoptopus::inventory::ServiceId::from_id(
                ::interoptopus::inventory::Id::new(
                    ::interoptopus::inventory::hash_str(#name_str)
                )
            )
        };

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
    params.iter().map(|p| {
        let pname = &p.name;
        let pty = &p.ty;
        quote! { #pname: #pty }
    }).collect()
}

fn prefixed_ident(prefix: &str, name: &Ident) -> Ident {
    format_ident!("{}_{}", prefix, name)
}

/// Returns true if the given return type is `Self`.
fn is_self_return(ret: &Option<Type>) -> bool {
    match ret {
        Some(Type::Path(p)) => p.path.is_ident("Self"),
        _ => false,
    }
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
fn emit_function_registration(
    fn_name_str: &str,
    params: &[PluginParam],
    ret: &Option<Type>,
    self_type_id: Option<&TokenStream>,
) -> TokenStream {
    let type_registrations = params.iter().map(|p| {
        let ty = &p.ty;
        quote! { <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory); }
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
