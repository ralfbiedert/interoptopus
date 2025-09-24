use crate::service::model::{ReceiverKind, ServiceMethod, ServiceModel};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{Error, Generics, Lifetime, ReturnType, Type};

impl ServiceModel {
    pub fn emit_ffi_functions(&self) -> Result<TokenStream, Error> {
        let mut functions = Vec::new();

        // Generate constructor functions
        for ctor in &self.constructors {
            functions.push(self.emit_constructor_function(ctor));
        }

        // Generate destructor function
        functions.push(self.emit_destructor_function());

        // Generate method functions
        for method in &self.methods {
            functions.push(self.emit_method_function(method));
        }

        Ok(quote_spanned! { self.service_name.span() =>
            #(#functions)*
        })
    }

    /// Get the base service name without any generic parameters (for const contexts)
    fn get_base_service_name(&self) -> syn::Ident {
        self.service_name.clone()
    }

    /// Replace anonymous lifetimes ('_) with explicit lifetime parameters
    fn replace_anonymous_lifetimes(&self, ty: &Type, method_generics: &Generics) -> Type {
        use syn::{GenericArgument, PathArguments, Type};

        match ty {
            Type::Reference(type_ref) => {
                let mut new_ref = type_ref.clone();
                if let Some(ref lifetime) = type_ref.lifetime {
                    // Check if it's an anonymous lifetime
                    if lifetime.ident == "_" {
                        // Create or find a suitable lifetime parameter
                        let replacement_lifetime = self.get_replacement_lifetime(method_generics);
                        new_ref.lifetime = Some(replacement_lifetime);
                    }
                }
                // Recursively process the referenced type
                new_ref.elem = Box::new(self.replace_anonymous_lifetimes(&type_ref.elem, method_generics));
                Type::Reference(new_ref)
            }
            Type::Path(type_path) => {
                let mut new_path = type_path.clone();

                // Process each segment of the path
                for segment in &mut new_path.path.segments {
                    // Recursively process any generic arguments
                    if let PathArguments::AngleBracketed(ref mut args) = segment.arguments {
                        for arg in &mut args.args {
                            match arg {
                                GenericArgument::Type(inner_type) => {
                                    *inner_type = self.replace_anonymous_lifetimes(inner_type, method_generics);
                                }
                                GenericArgument::Lifetime(lifetime) => {
                                    // Replace anonymous lifetime with explicit one
                                    if lifetime.ident == "_" {
                                        *lifetime = self.get_replacement_lifetime(method_generics);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                Type::Path(new_path)
            }
            _ => ty.clone(),
        }
    }

    /// Get a suitable lifetime to replace anonymous lifetimes
    fn get_replacement_lifetime(&self, method_generics: &Generics) -> Lifetime {
        use syn::parse_quote;

        // First, try to find an existing lifetime parameter
        if let Some(first_lifetime) = method_generics.params.iter().find_map(|param| {
            if let syn::GenericParam::Lifetime(lifetime_def) = param {
                Some(&lifetime_def.lifetime)
            } else {
                None
            }
        }) {
            first_lifetime.clone()
        } else {
            // If no lifetime parameters exist, create one that will be added to the generics
            parse_quote!('a)
        }
    }

    /// Check if a type contains anonymous lifetimes that need explicit parameters
    fn type_contains_anonymous_lifetimes(&self, ty: &Type) -> bool {
        use syn::{GenericArgument, PathArguments, Type};

        match ty {
            Type::Reference(type_ref) => {
                if let Some(ref lifetime) = type_ref.lifetime {
                    if lifetime.ident == "_" {
                        return true;
                    }
                }
                self.type_contains_anonymous_lifetimes(&type_ref.elem)
            }
            Type::Path(type_path) => {
                for segment in &type_path.path.segments {
                    if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                        for arg in &args.args {
                            match arg {
                                GenericArgument::Type(inner_type) => {
                                    if self.type_contains_anonymous_lifetimes(inner_type) {
                                        return true;
                                    }
                                }
                                GenericArgument::Lifetime(lifetime) => {
                                    if lifetime.ident == "_" {
                                        return true;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// Add a lifetime parameter to generics if needed for anonymous lifetimes
    fn ensure_lifetime_parameter(&self, method_generics: &Generics, return_type: &syn::ReturnType) -> Generics {
        use syn::{parse_quote, GenericParam, ReturnType};

        // Check if the return type contains anonymous lifetimes
        let needs_lifetime = match return_type {
            ReturnType::Type(_, ty) => self.type_contains_anonymous_lifetimes(ty),
            _ => false,
        };

        if needs_lifetime && method_generics.params.iter().all(|param| !matches!(param, GenericParam::Lifetime(_))) {
            // Add a lifetime parameter 'a
            let mut new_generics = method_generics.clone();
            let lifetime_param: GenericParam = parse_quote!('a);
            new_generics.params.insert(0, lifetime_param);
            new_generics
        } else {
            method_generics.clone()
        }
    }

    /// Replace Self with the actual service type in a type expression
    /// For const contexts, lifetime parameters are stripped
    fn replace_self_with_service_type(&self, ty: &Type) -> Type {
        use syn::{GenericArgument, PathArguments, Type};

        match ty {
            Type::Path(type_path) => {
                let mut new_path = type_path.clone();

                // Process each segment of the path
                for segment in &mut new_path.path.segments {
                    if segment.ident == "Self" {
                        // Replace Self with the base service type name (no generics for const contexts)
                        segment.ident = self.service_name.clone();
                        // Don't add generic parameters in const contexts
                        segment.arguments = PathArguments::None;
                    } else {
                        // Recursively process any generic arguments
                        if let PathArguments::AngleBracketed(ref mut args) = segment.arguments {
                            for arg in &mut args.args {
                                if let GenericArgument::Type(inner_type) = arg {
                                    *inner_type = self.replace_self_with_service_type(inner_type);
                                }
                            }
                        }
                    }
                }

                Type::Path(new_path)
            }
            _ => ty.clone(),
        }
    }

    fn emit_constructor_function(&self, ctor: &ServiceMethod) -> TokenStream {
        let service_name_snake = self.service_name_snake_case();
        let ctor_name = &ctor.name;
        let function_name = format_ident!("{}_{}", service_name_snake, ctor_name);

        let docs = self.emit_docs(&ctor.docs);
        let params = self.emit_params(&ctor.inputs);
        let param_names = self.emit_param_names(&ctor.inputs);

        let service_type = &self.service_type;
        let service_name = &self.service_name;
        let generics = &self.generics;

        // For generic types, we need to use the concrete type with turbofish syntax
        let service_call = if self.generics.params.is_empty() {
            quote_spanned! { ctor.name.span() => #service_name::#ctor_name }
        } else {
            // Use the full generic type with angle brackets for function calls
            quote_spanned! { ctor.name.span() => <#service_type>::#ctor_name }
        };

        // Extract error type from the constructor's return type
        let error_type = self.extract_error_type_from_constructor(ctor);

        let constructor_params = if ctor.inputs.is_empty() {
            quote_spanned! { ctor.name.span() => instance: *mut *const #service_type }
        } else {
            quote_spanned! { ctor.name.span() => #params, instance: *mut *const #service_type }
        };

        // Use Box for non-async services, Arc for async services
        let (wrapper_creation, into_raw_call) = if self.is_async {
            (
                quote_spanned! { ctor.name.span() => let arc = ::std::sync::Arc::new(service_instance); },
                quote_spanned! { ctor.name.span() => ::std::sync::Arc::into_raw(arc) },
            )
        } else {
            (
                quote_spanned! { ctor.name.span() => let boxed = ::std::boxed::Box::new(service_instance); },
                quote_spanned! { ctor.name.span() => ::std::boxed::Box::into_raw(boxed) },
            )
        };

        quote_spanned! { ctor.name.span() =>
            #docs
            #[::interoptopus::ffi]
            unsafe fn #function_name #generics(#constructor_params) -> <::interoptopus::ffi::Result<(), #error_type> as ::interoptopus::pattern::result::ResultAs>::AsT<*const #service_type> {
                unsafe {
                   let result = #service_call(#param_names);
                    match result {
                        ::interoptopus::ffi::Ok(service_instance) => {
                            #wrapper_creation
                            *instance = #into_raw_call;
                            ::interoptopus::ffi::Ok(::std::ptr::null())
                        }
                        ::interoptopus::ffi::Err(err) => ::interoptopus::ffi::Err(err),
                        ::interoptopus::ffi::Result::Panic => ::interoptopus::ffi::Result::Panic,
                        ::interoptopus::ffi::Result::Null => ::interoptopus::ffi::Result::Null,
                    }
                }
            }
        }
    }

    fn emit_destructor_function(&self) -> TokenStream {
        let service_name_snake = self.service_name_snake_case();
        let function_name = format_ident!("{}_destroy", service_name_snake);

        let service_type = &self.service_type;
        let generics = &self.generics;

        // Use Box for non-async services, Arc for async services
        let from_raw_call = if self.is_async {
            quote_spanned! { self.service_name.span() => let _ = ::std::sync::Arc::from_raw(instance); }
        } else {
            quote_spanned! { self.service_name.span() => let _ = ::std::boxed::Box::from_raw(instance as *mut #service_type); }
        };

        quote_spanned! { self.service_name.span() =>
            #[::interoptopus::ffi]
            fn #function_name #generics(instance: *const #service_type) {
                if !instance.is_null() {
                    unsafe {
                        #from_raw_call
                    }
                }
            }
        }
    }

    fn emit_method_function(&self, method: &ServiceMethod) -> TokenStream {
        let service_name_snake = self.service_name_snake_case();
        let method_name = &method.name;
        let function_name = format_ident!("{}_{}", service_name_snake, method_name);

        let docs = self.emit_docs(&method.docs);
        let _service_type = &self.service_type;

        match method.receiver_kind {
            ReceiverKind::Shared => self.emit_shared_method(method, &function_name, &docs),
            ReceiverKind::Mutable => self.emit_mutable_method(method, &function_name, &docs),
            ReceiverKind::AsyncThis => self.emit_async_method(method, &function_name, &docs),
            ReceiverKind::None => {
                if method.is_async {
                    // This shouldn't happen as async methods should have Async<Self> parameter
                    panic!("Async methods in services should have Async<Self> as their first parameter")
                } else {
                    unreachable!("Non-async methods with no receiver should be constructors")
                }
            }
        }
    }

    fn emit_shared_method(&self, method: &ServiceMethod, function_name: &syn::Ident, docs: &TokenStream) -> TokenStream {
        let service_type = &self.service_type;
        let method_name = &method.name;
        let params = self.emit_params(&method.inputs);
        let param_names = self.emit_param_names(&method.inputs);

        // Ensure we have proper lifetime parameters and process types
        let enhanced_generics = self.ensure_lifetime_parameter(&method.generics, &method.output);
        let return_type = self.emit_return_type_processed(&method.output, &enhanced_generics);
        let where_clause = &enhanced_generics.where_clause;

        quote_spanned! { method.name.span() =>
            #docs
            #[::interoptopus::ffi]
            unsafe fn #function_name #enhanced_generics(instance: *const #service_type, #params) #return_type #where_clause {
                unsafe {
                    let instance_ref = &*instance;
                    instance_ref.#method_name(#param_names)
                }
           }
        }
    }

    fn emit_mutable_method(&self, method: &ServiceMethod, function_name: &syn::Ident, docs: &TokenStream) -> TokenStream {
        let service_type = &self.service_type;
        let method_name = &method.name;
        let params = self.emit_params(&method.inputs);
        let param_names = self.emit_param_names(&method.inputs);

        // Ensure we have proper lifetime parameters and process types
        let enhanced_generics = self.ensure_lifetime_parameter(&method.generics, &method.output);
        let return_type = self.emit_return_type_processed(&method.output, &enhanced_generics);
        let where_clause = &enhanced_generics.where_clause;

        quote_spanned! { method.name.span() =>
            #docs
            #[::interoptopus::ffi]
            unsafe fn #function_name #enhanced_generics(instance: *mut #service_type, #params) #return_type #where_clause {
                unsafe {
                    let instance_ref = &mut *instance;
                    instance_ref.#method_name(#param_names)
                }
            }
        }
    }

    fn emit_async_method(&self, method: &ServiceMethod, function_name: &syn::Ident, docs: &TokenStream) -> TokenStream {
        let service_type = &self.service_type;
        let method_name = &method.name;
        let params = self.emit_params(&method.inputs);
        let param_names = self.emit_param_names(&method.inputs);

        // Ensure we have proper lifetime parameters
        let enhanced_generics = self.ensure_lifetime_parameter(&method.generics, &method.output);
        let where_clause = &enhanced_generics.where_clause;

        // Extract the inner type from ffi::Result<T, E>
        let callback_type = self.extract_async_callback_type(&method.output);

        let async_params = if method.inputs.is_empty() {
            quote_spanned! { method.name.span() =>
                instance: *const #service_type,
                callback: ::interoptopus::pattern::asynk::AsyncCallback<#callback_type>
            }
        } else {
            quote_spanned! { method.name.span() =>
                instance: *const #service_type,
                #params,
                callback: ::interoptopus::pattern::asynk::AsyncCallback<#callback_type>
            }
        };

        quote_spanned! { method.name.span() =>
            #docs
            #[::interoptopus::ffi]
            unsafe fn #function_name #enhanced_generics(
                #async_params
            ) -> <::interoptopus::ffi::Result<(), Error> as ::interoptopus::pattern::result::ResultAs>::AsT<*const #service_type> #where_clause {
                unsafe {
                    let instance_arc = ::std::sync::Arc::from_raw(instance);
                    let instance_clone = ::std::sync::Arc::clone(&instance_arc);
                    ::std::mem::forget(instance_arc); // Don't drop the original

                    let async_this = ::interoptopus::pattern::asynk::Async::new(instance_clone.clone());

                    use ::interoptopus::pattern::asynk::AsyncRuntime;
                    instance_clone.spawn(move |_| async move {
                        let result = #service_type::#method_name(async_this, #param_names).await;
                        match result {
                            ::interoptopus::ffi::Ok(value) => {
                                callback.call(&value);
                            }
                            ::interoptopus::ffi::Err(_err) => {
                                // TODO: Handle async errors properly
                            }
                            ::interoptopus::ffi::Result::Panic => {
                                // TODO: Handle async panics properly
                            }
                            ::interoptopus::ffi::Result::Null => {
                                // TODO: Handle async null properly
                            }
                        }
                    });
                    ::interoptopus::ffi::Ok(::std::ptr::null())
                }

            }
        }
    }

    fn emit_params(&self, inputs: &[crate::service::model::ServiceParameter]) -> TokenStream {
        if inputs.is_empty() {
            quote_spanned! { self.service_name.span() => }
        } else {
            let params = inputs.iter().map(|param| {
                let name = &param.name;
                let ty = &param.ty;
                quote_spanned! { name.span() => #name: #ty }
            });

            quote_spanned! { self.service_name.span() =>
                #(#params),*
            }
        }
    }

    fn emit_param_names(&self, inputs: &[crate::service::model::ServiceParameter]) -> TokenStream {
        if inputs.is_empty() {
            quote_spanned! { self.service_name.span() => }
        } else {
            let names = inputs.iter().map(|param| &param.name);
            quote_spanned! { self.service_name.span() =>
                #(#names),*
            }
        }
    }

    fn emit_return_type_processed(&self, output: &ReturnType, method_generics: &Generics) -> TokenStream {
        match output {
            ReturnType::Default => quote_spanned! { self.service_name.span() => },
            ReturnType::Type(arrow, ty) => {
                let service_ty = &self.replace_self_with_service_type(ty);
                let processed_ty = self.replace_anonymous_lifetimes(service_ty, method_generics);
                quote_spanned! { self.service_name.span() => #arrow #processed_ty }
            }
        }
    }

    fn emit_docs(&self, docs: &[String]) -> TokenStream {
        if docs.is_empty() {
            quote_spanned! { self.service_name.span() => }
        } else {
            let doc_strings = docs.iter().map(|doc| {
                quote_spanned! { self.service_name.span() => #[doc = #doc] }
            });
            quote_spanned! { self.service_name.span() =>
                #(#doc_strings)*
            }
        }
    }

    fn extract_async_callback_type(&self, return_type: &ReturnType) -> TokenStream {
        match return_type {
            ReturnType::Type(_, ty) => {
                // Try to extract T from ffi::Result<T, E>
                if let Type::Path(path) = ty.as_ref() {
                    if let Some(segment) = path.path.segments.last() {
                        if segment.ident == "Result" {
                            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                                    return quote_spanned! { self.service_name.span() => #inner_type };
                                }
                            }
                        }
                    }
                }
                // Fallback to the whole type
                quote_spanned! { self.service_name.span() => #ty }
            }
            ReturnType::Default => quote_spanned! { self.service_name.span() => () },
        }
    }

    fn extract_error_type_from_constructor(&self, ctor: &ServiceMethod) -> TokenStream {
        match &ctor.output {
            ReturnType::Type(_, ty) => {
                // Try to extract E from ffi::Result<T, E>
                if let Type::Path(path) = ty.as_ref() {
                    if let Some(segment) = path.path.segments.last() {
                        if segment.ident == "Result" {
                            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                                if let Some(syn::GenericArgument::Type(_)) = args.args.first() {
                                    if let Some(syn::GenericArgument::Type(error_type)) = args.args.iter().nth(1) {
                                        return quote_spanned! { ctor.name.span() => #error_type };
                                    }
                                }
                            }
                        }
                    }
                }
                // Fallback - if we can't extract it, just use the Error type that should be in scope
                quote_spanned! { ctor.name.span() => Error }
            }
            ReturnType::Default => quote_spanned! { ctor.name.span() => Error },
        }
    }

    pub fn emit_service_info_impl(&self) -> Result<TokenStream, Error> {
        let _service_name = &self.service_name;
        let service_type = &self.service_type;
        let service_name_snake = self.service_name_snake_case();
        let generics = &self.generics;

        // Generate constructor function names
        let ctor_names: Vec<_> = self.constructors.iter().map(|ctor| format_ident!("{}_{}", service_name_snake, ctor.name)).collect();

        // Generate method function names
        let method_names: Vec<_> = self.methods.iter().map(|method| format_ident!("{}_{}", service_name_snake, method.name)).collect();

        let destructor_name = format_ident!("{}_destroy", service_name_snake);

        Ok(quote_spanned! { self.service_name.span() =>
            impl #generics ::interoptopus::lang::service::ServiceInfo for #service_type {
                fn id() -> ::interoptopus::inventory::ServiceId {
                    ::interoptopus::inventory::ServiceId::from_id(::interoptopus::id!(#service_type))
                }

                fn service() -> ::interoptopus::lang::service::Service {
                    ::interoptopus::lang::service::Service::new(
                        <#service_type as ::interoptopus::lang::types::TypeInfo>::id(),
                        vec![
                            #(<#ctor_names as ::interoptopus::lang::function::FunctionInfo>::id()),*
                        ],
                        <#destructor_name as ::interoptopus::lang::function::FunctionInfo>::id(),
                        vec![
                            #(<#method_names as ::interoptopus::lang::function::FunctionInfo>::id()),*
                        ],
                    )
                }

                fn register(inventory: &mut ::interoptopus::inventory::Inventory) {
                    // Register the service type itself
                    <#service_type as ::interoptopus::lang::types::TypeInfo>::register(inventory);

                    // Register all constructor functions
                    #(
                        <#ctor_names as ::interoptopus::lang::function::FunctionInfo>::register(inventory);
                    )*

                    // Register destructor function
                    <#destructor_name as ::interoptopus::lang::function::FunctionInfo>::register(inventory);

                    // Register all method functions
                    #(
                        <#method_names as ::interoptopus::lang::function::FunctionInfo>::register(inventory);
                    )*

                    // Register the service itself
                    inventory.register_service(Self::id(), Self::service());
                }
            }
        })
    }

    pub fn emit_const_verification_blocks(&self) -> Result<TokenStream, Error> {
        // Generate compile-time verification blocks
        let service_type = &self.service_type;
        let base_service_name = self.get_base_service_name();

        let async_verification = if self.is_async {
            quote_spanned! { self.service_name.span() =>
                const fn _assert_async<T: ::interoptopus::pattern::asynk::AsyncRuntime>() {}
                _assert_async::<#service_type>();
            }
        } else {
            quote_spanned! { self.service_name.span() => }
        };

        // Generate SERVICE_CTOR_SAFE checks for constructor return types
        let ctor_verification_blocks: Vec<TokenStream> = self
            .constructors
            .iter()
            .map(|ctor| match &ctor.output {
                ReturnType::Type(_, return_type) => {
                    let return_type_resolved = self.replace_self_with_service_type(return_type);
                    let ctor_span = ctor.name.span();
                    Ok(quote::quote_spanned! { ctor_span =>
                        ::interoptopus::lang::types::assert_service_ctor_safe::<#return_type_resolved>();
                    })
                }
                ReturnType::Default => {
                    return Err(Error::new_spanned(ctor.name.to_token_stream(), "This method looks like a constructor, but it does not return ffi::Result<Self, _>"));
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;

        // Generate validation for async methods - they should have Async<Self> as first parameter
        let async_method_verification_blocks: Vec<TokenStream> = self
            .methods
            .iter()
            .filter(|method| method.is_async && method.receiver_kind == ReceiverKind::AsyncThis)
            .map(|method| {
                let method_span = method.name.span();
                // Create a validation that checks if Async<ServiceType> can be used
                let method_rval = match &method.output {
                    ReturnType::Default => quote_spanned! { method_span => () },
                    ReturnType::Type(_, x) => quote_spanned! { x.span() => #x },
                };
                Ok(quote_spanned! { method_span =>
                    {
                        const fn _assert_rval_result1<T: ::interoptopus::lang::types::TypeInfo, E: ::interoptopus::lang::types::TypeInfo>(_: &::interoptopus::ffi::Result<T, E>) {}
                        const fn _assert_rval_result2(x: &#method_rval) {
                            _assert_rval_result1(&x);
                        }
                    }
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        // Note: Skipping ASYNC_SAFE checks for now due to const context limitations with generics
        let async_safe_verification = quote_spanned! { self.service_name.span() => };

        let x = self.service_type.span();
        let base_service_verification = quote_spanned! { x =>
            const fn _assert_type_info<T: ::interoptopus::lang::types::TypeInfo>() {}
            _assert_type_info::<#base_service_name>();
        };

        Ok(quote_spanned! { x=>
            const _: () = {
                #base_service_verification
                #async_verification
                #async_safe_verification
                #(#ctor_verification_blocks)*
                #(#async_method_verification_blocks)*

                ::interoptopus::lang::types::assert_service_safe::<#service_type>();
            };
        })
    }
}
