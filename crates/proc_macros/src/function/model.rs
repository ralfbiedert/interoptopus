use crate::common::extract_docs;
use crate::function::args::FfiFunctionArgs;
use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{FnArg, Ident, ItemFn, Pat, PatType, Type, Visibility};

#[derive(Clone)]
pub struct FunctionModel {
    pub name: Ident,
    pub vis: Visibility,
    pub args: FfiFunctionArgs,
    pub docs: Vec<String>,
    pub signature: FunctionSignature,
    pub is_unsafe: bool,
}

#[derive(Clone)]
pub struct FunctionSignature {
    pub inputs: Vec<FunctionParameter>,
    pub output: syn::ReturnType,
    pub output_span: Option<Span>,
    pub generics: syn::Generics,
}

#[derive(Clone)]
pub struct FunctionParameter {
    pub name: Ident,
    pub ty: Type,
    pub ty_span: Span,
}

impl FunctionModel {
    pub fn from_item_fn(input: ItemFn, args: FfiFunctionArgs) -> syn::Result<Self> {
        let docs = extract_docs(&input.attrs);

        // Check for conflicting attributes
        let has_extern = input.sig.abi.is_some();
        let has_no_mangle = input.attrs.iter().any(|attr| attr.path().is_ident("no_mangle"));

        if has_extern {
            return Err(syn::Error::new_spanned(
                input.sig.abi.as_ref().unwrap(),
                "Functions with explicit extern declarations are not supported. Remove the extern declaration and let the macro handle it.",
            ));
        }

        if has_no_mangle {
            return Err(syn::Error::new_spanned(
                input.attrs.iter().find(|attr| attr.path().is_ident("no_mangle")).unwrap(),
                "Functions with #[no_mangle] are not supported. Remove the #[no_mangle] attribute and let the macro handle it.",
            ));
        }

        // Parse function parameters
        let mut inputs = Vec::new();
        for input_arg in &input.sig.inputs {
            match input_arg {
                FnArg::Typed(typed_arg) => {
                    if let Pat::Ident(pat_ident) = typed_arg.pat.as_ref() {
                        // The span is captured correctly by syn

                        // Use the span of the type
                        let ty_span = typed_arg.ty.span();

                        inputs.push(FunctionParameter {
                            name: pat_ident.ident.clone(),
                            ty: (*typed_arg.ty).clone(),
                            ty_span
                        });
                    } else {
                        return Err(syn::Error::new_spanned(&typed_arg.pat, "Only simple parameter names are supported"));
                    }
                }
                FnArg::Receiver(_) => {
                    return Err(syn::Error::new_spanned(input_arg, "Methods with self parameters are not supported"));
                }
            }
        }

        let output_span = match &input.sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => {
                // Get the span that covers the entire return type
                let tokens = ty.to_token_stream();
                if let (Some(first), Some(last)) = (tokens.clone().into_iter().next(), tokens.into_iter().last()) {
                    Some(first.span().join(last.span()).unwrap_or_else(|| ty.span()))
                } else {
                    Some(ty.span())
                }
            }
        };

        let signature = FunctionSignature { inputs, output: input.sig.output.clone(), output_span, generics: input.sig.generics.clone() };

        // Check for generics constraints - we only support lifetime generics, not type generics
        for param in &signature.generics.params {
            if let syn::GenericParam::Type(_) = param {
                return Err(syn::Error::new_spanned(param, "Functions with type generics are not supported at FFI boundaries"));
            }
        }

        let model = Self { name: input.sig.ident.clone(), vis: input.vis.clone(), args, docs, signature, is_unsafe: input.sig.unsafety.is_some() };

        Ok(model)
    }

    pub fn generate_export_name(&self) -> String {
        match &self.args.export {
            Some(crate::function::args::ExportKind::Custom(name)) => name.clone(),
            Some(crate::function::args::ExportKind::Unique) => {
                // Generate a pseudo-random suffix
                let base_name = self.name.to_string();
                let hash = {
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::{Hash, Hasher};
                    let mut hasher = DefaultHasher::new();
                    base_name.hash(&mut hasher);
                    file!().hash(&mut hasher);
                    line!().hash(&mut hasher);
                    hasher.finish()
                };
                format!("{}_{}", base_name, hash % 100000)
            }
            None => self.name.to_string(),
        }
    }
}
