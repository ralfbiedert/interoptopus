use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{ReturnType, Type};

use crate::service::model::{ServiceModel, ServiceMethod, ReceiverKind};

impl ServiceModel {
    pub fn emit_ffi_functions(&self) -> TokenStream {
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

        quote! {
            #(#functions)*
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

        // Extract error type from the constructor's return type
        let error_type = self.extract_error_type_from_constructor(ctor);

        let constructor_params = if ctor.inputs.is_empty() {
            quote! { instance: &mut *const #service_type }
        } else {
            quote! { #params, instance: &mut *const #service_type }
        };


        if self.is_async {
            quote! {
                #docs
                #[::interoptopus_proc::ffi_function]
                pub fn #function_name(#constructor_params) -> <::interoptopus::ffi::Result<(), #error_type> as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr {
                    let result = #service_name::#ctor_name(#param_names);
                    match result {
                        ::interoptopus::ffi::Ok(service_instance) => {
                            let arc = ::std::sync::Arc::new(service_instance);
                            *instance = ::std::sync::Arc::into_raw(arc);
                            ::interoptopus::ffi::Ok(::std::ptr::null())
                        }
                        ::interoptopus::ffi::Err(err) => ::interoptopus::ffi::Err(err),
                        ::interoptopus::ffi::Result::Panic => ::interoptopus::ffi::Result::Panic,
                        ::interoptopus::ffi::Result::Null => ::interoptopus::ffi::Result::Null,
                    }
                }
            }
        } else {
            quote! {
                #docs
                #[::interoptopus_proc::ffi_function]
                pub fn #function_name(#constructor_params) -> <::interoptopus::ffi::Result<(), #error_type> as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr {
                    let result = #service_name::#ctor_name(#param_names);
                    match result {
                        ::interoptopus::ffi::Ok(service_instance) => {
                            let boxed = ::std::boxed::Box::new(service_instance);
                            *instance = ::std::boxed::Box::leak(boxed);
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

        if self.is_async {
            quote! {
                #[::interoptopus_proc::ffi_function]
                pub fn #function_name(instance: *const #service_type) {
                    if !instance.is_null() {
                        unsafe {
                            let _ = ::std::sync::Arc::from_raw(instance);
                        }
                    }
                }
            }
        } else {
            quote! {
                #[::interoptopus_proc::ffi_function]
                pub fn #function_name(instance: *const #service_type) {
                    if !instance.is_null() {
                        unsafe {
                            let _ = ::std::boxed::Box::from_raw(instance as *mut #service_type);
                        }
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
        let service_type = &self.service_type;

        match method.receiver_kind {
            ReceiverKind::Shared => self.emit_shared_method(method, &function_name, &docs),
            ReceiverKind::Mutable => self.emit_mutable_method(method, &function_name, &docs),
            ReceiverKind::AsyncThis => self.emit_async_method(method, &function_name, &docs),
            ReceiverKind::None => unreachable!("Constructors are handled separately"),
        }
    }

    fn emit_shared_method(&self, method: &ServiceMethod, function_name: &syn::Ident, docs: &TokenStream) -> TokenStream {
        let service_type = &self.service_type;
        let method_name = &method.name;
        let params = self.emit_params(&method.inputs);
        let param_names = self.emit_param_names(&method.inputs);
        let return_type = self.emit_return_type(&method.output);

        quote! {
            #docs
            #[::interoptopus_proc::ffi_function]
            pub fn #function_name(instance: *const #service_type, #params) #return_type {
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
        let return_type = self.emit_return_type(&method.output);

        quote! {
            #docs
            #[::interoptopus_proc::ffi_function]
            pub fn #function_name(instance: *mut #service_type, #params) #return_type {
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

        // Extract the inner type from ffi::Result<T, E>
        let callback_type = self.extract_async_callback_type(&method.output);

        quote! {
            #docs
            #[::interoptopus_proc::ffi_function]
            pub fn #function_name(
                instance: *const #service_type,
                #params,
                callback: ::interoptopus::pattern::asynk::AsyncCallback<#callback_type>
            ) -> <::interoptopus::ffi::Result<(), Error> as ::interoptopus::pattern::result::ResultAsPtr>::AsPtr {
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
                                callback.call(value);
                                ::std::mem::forget(value);
                            }
                            ::interoptopus::ffi::Err(_err) => {
                                // TODO: Handle async errors properly
                            }
                        }
                    });
                }
                ::interoptopus::ffi::Ok(())
            }
        }
    }

    fn emit_params(&self, inputs: &[crate::service::model::ServiceParameter]) -> TokenStream {
        let params = inputs.iter().map(|param| {
            let name = &param.name;
            let ty = &param.ty;
            quote! { #name: #ty }
        });

        quote! {
            #(#params),*
        }
    }

    fn emit_param_names(&self, inputs: &[crate::service::model::ServiceParameter]) -> TokenStream {
        let names = inputs.iter().map(|param| &param.name);
        quote! {
            #(#names),*
        }
    }

    fn emit_return_type(&self, output: &ReturnType) -> TokenStream {
        match output {
            ReturnType::Default => quote! {},
            ReturnType::Type(arrow, ty) => quote! { #arrow #ty },
        }
    }

    fn emit_docs(&self, docs: &[String]) -> TokenStream {
        if docs.is_empty() {
            quote! {}
        } else {
            let doc_strings = docs.iter().map(|doc| {
                quote! { #[doc = #doc] }
            });
            quote! {
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
                                    return quote! { #inner_type };
                                }
                            }
                        }
                    }
                }
                // Fallback to the whole type
                quote! { #ty }
            }
            ReturnType::Default => quote! { () },
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
                                        return quote! { #error_type };
                                    }
                                }
                            }
                        }
                    }
                }
                // Fallback - if we can't extract it, just use the Error type that should be in scope
                quote! { Error }
            }
            ReturnType::Default => quote! { Error },
        }
    }

    pub fn emit_service_info_impl(&self) -> TokenStream {
        let service_name = &self.service_name;
        let service_type = &self.service_type;
        let service_name_snake = self.service_name_snake_case();

        // Generate constructor function names
        let ctor_names: Vec<_> = self.constructors.iter().map(|ctor| {
            format_ident!("{}_{}", service_name_snake, ctor.name)
        }).collect();

        // Generate method function names
        let method_names: Vec<_> = self.methods.iter().map(|method| {
            format_ident!("{}_{}", service_name_snake, method.name)
        }).collect();

        let destructor_name = format_ident!("{}_destroy", service_name_snake);

        quote! {
            impl ::interoptopus::lang::service::ServiceInfo for #service_type {
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
        }
    }

    pub fn emit_verification_blocks(&self) -> TokenStream {
        // Generate compile-time verification blocks
        let service_type = &self.service_type;

        let async_verification = if self.is_async {
            quote! {
                const fn _assert_async_runtime() {
                    const fn _assert_async<T: ::interoptopus::pattern::asynk::AsyncRuntime>() {}
                    _assert_async::<#service_type>();
                }
                _assert_async_runtime();
            }
        } else {
            quote! {}
        };

        quote! {
            const _: () = {
                // Verify that the service type implements the required traits
                const fn _assert_service_type_is_valid() {
                    const fn _assert_type_info<T: ::interoptopus::lang::types::TypeInfo>() {}
                    _assert_type_info::<#service_type>();
                }

                // If this is an async service, verify AsyncRuntime is implemented
                #async_verification

                _assert_service_type_is_valid();
            };
        }
    }
}