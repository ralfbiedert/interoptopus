use crate::docs::extract_docs;
use crate::service::args::FfiServiceArgs;
use crate::utils::has_ffi_skip_attribute;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{FnArg, Generics, Ident, ImplItem, ItemImpl, Pat, ReturnType, Type, Visibility};

#[derive(Clone)]
#[allow(dead_code)]
pub struct ServiceModel {
    pub service_name: Ident,
    pub service_type: Type,
    pub generics: syn::Generics,
    pub args: FfiServiceArgs,
    pub constructors: Vec<ServiceMethod>,
    pub methods: Vec<ServiceMethod>,
    pub is_async: bool,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct ServiceMethod {
    pub name: Ident,
    pub docs: Vec<String>,
    pub inputs: Vec<ServiceParameter>,
    pub output: ReturnType,
    pub is_async: bool,
    pub receiver_kind: ReceiverKind,
    pub vis: Visibility,
    pub span: Span,
    pub skip: bool,
    pub generics: Generics,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct ServiceParameter {
    pub name: Ident,
    pub ty: Type,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReceiverKind {
    None,      // Constructor
    Shared,    // &self
    Mutable,   // &mut self
    AsyncThis, // Async<Self>
}

impl ServiceModel {
    pub fn from_impl_item(input: ItemImpl, args: FfiServiceArgs) -> syn::Result<Self> {
        // Extract service type and name
        let service_type = match input.self_ty.as_ref() {
            Type::Path(path) => {
                if let Some(segment) = path.path.segments.last() {
                    (segment.ident.clone(), (*input.self_ty).clone())
                } else {
                    return Err(syn::Error::new_spanned(&input.self_ty, "Invalid service type"));
                }
            }
            _ => return Err(syn::Error::new_spanned(&input.self_ty, "Service type must be a path")),
        };

        let (service_name, service_type) = service_type;
        let generics = input.generics.clone();

        let mut constructors = Vec::new();
        let mut methods = Vec::new();
        let mut has_async = false;

        // Process each method in the impl block
        for item in &input.items {
            if let ImplItem::Fn(method) = item {
                // Check if method should be skipped
                let skip = has_ffi_skip_attribute(&method.attrs);

                // Skip processing this method if it has the skip attribute
                if skip {
                    continue;
                }

                let docs = extract_docs(&method.attrs);
                let method_name = method.sig.ident.clone();
                let is_async = method.sig.asyncness.is_some();
                let vis = method.vis.clone();
                let span = method.span();

                if is_async {
                    has_async = true;
                }

                // Parse parameters and determine receiver kind
                let mut inputs = Vec::new();
                let mut receiver_kind = ReceiverKind::None;
                let mut param_index = 0; // Track index for generated parameter names

                for (i, input_arg) in method.sig.inputs.iter().enumerate() {
                    match input_arg {
                        FnArg::Receiver(receiver) => {
                            receiver_kind = if receiver.mutability.is_some() {
                                ReceiverKind::Mutable
                            } else {
                                ReceiverKind::Shared
                            };
                        }
                        FnArg::Typed(typed_arg) => {
                            let param_type = (*typed_arg.ty).clone();

                            // Check for special Async<Self> parameter (first parameter in async functions)
                            if i == 0 && is_async {
                                if let Type::Path(path) = &param_type {
                                    if let Some(segment) = path.path.segments.last() {
                                        if segment.ident == "Async" {
                                            receiver_kind = ReceiverKind::AsyncThis;
                                            continue; // Don't add to inputs, regardless of pattern
                                        }
                                    }
                                }
                            }

                            let param_name = if let Pat::Ident(pat_ident) = typed_arg.pat.as_ref() {
                                pat_ident.ident.clone()
                            } else {
                                // Generate a synthetic name for non-identifier patterns (like `_`)
                                syn::Ident::new(&format!("_{}", param_index), typed_arg.pat.span())
                            };

                            inputs.push(ServiceParameter { name: param_name, ty: param_type, span: typed_arg.span() });
                            param_index += 1;
                        }
                    }
                }

                let service_method =
                    ServiceMethod { name: method_name, docs, inputs, output: method.sig.output.clone(), is_async, receiver_kind: receiver_kind.clone(), vis, span, skip: false, generics: method.sig.generics.clone() };

                // Validate async methods
                if is_async && receiver_kind == ReceiverKind::None {
                    return Err(syn::Error::new_spanned(&method.sig.inputs.first(), "Async methods must use Async<Self> as their first parameter"));
                }

                // Async methods are never constructors, regardless of receiver kind
                match (receiver_kind, is_async) {
                    (ReceiverKind::None, false) => constructors.push(service_method),
                    _ => methods.push(service_method),
                }
            }
        }

        // Note: We now support lifetime generics, but not type generics
        for param in &generics.params {
            if let syn::GenericParam::Type(_) = param {
                return Err(syn::Error::new_spanned(param, "Generic services are not supported by #[ffi_service], only lifetime work."));
            }
        }

        let model = ServiceModel { service_name, service_type, generics, args, constructors, methods, is_async: has_async };

        Ok(model)
    }


    pub fn service_name_snake_case(&self) -> String {
        // Check if a manual prefix is provided
        if let Some(ref prefix) = self.args.prefix {
            // Remove trailing underscore if present, we'll add it back when needed
            prefix.trim_end_matches('_').to_string()
        } else {
            // Convert CamelCase to snake_case
            let name = self.service_name.to_string();
            let mut result = String::new();
            let mut chars = name.chars().peekable();

            while let Some(ch) = chars.next() {
                if ch.is_uppercase() && !result.is_empty() {
                    // Check if next char is lowercase (to handle acronyms correctly)
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch.is_lowercase() {
                            result.push('_');
                        }
                    }
                }
                result.push(ch.to_lowercase().next().unwrap_or(ch));
            }

            result
        }
    }
}
