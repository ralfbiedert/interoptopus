use crate::common::extract_docs;
use crate::function::args::FfiFunctionArgs;
use syn::spanned::Spanned;
use syn::{FnArg, Ident, ItemFn, Pat, Type, Visibility};

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
    pub generics: syn::Generics,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct FunctionParameter {
    pub name: Ident,
    pub ty: Type,
}

impl FunctionModel {
    pub fn from_item_fn(input: ItemFn, args: FfiFunctionArgs) -> syn::Result<Self> {
        let docs = extract_docs(&input.attrs);

        // Check for conflicting attributes
        let has_extern = input.sig.abi.is_some();
        let no_mangle_attr = input.attrs.iter().find(|attr| {
            // Check for both #[no_mangle] and #[unsafe(no_mangle)]
            if attr.path().is_ident("no_mangle") {
                true
            } else if attr.path().is_ident("unsafe") {
                // Check if this is #[unsafe(no_mangle)]
                match &attr.meta {
                    syn::Meta::List(list) => list.tokens.to_string().trim() == "no_mangle",
                    _ => false,
                }
            } else {
                false
            }
        });

        if has_extern {
            return Err(syn::Error::new_spanned(
                input.sig.abi.as_ref().unwrap(),
                "Functions with explicit extern declarations are not supported. Remove the declaration and let #[ffi_function] handle it.",
            ));
        }

        if let Some(attr) = no_mangle_attr {
            let message = "Functions with explicit #[no_mangle] are not supported, remove the attribute and let #[ffi_function] handle it.";
            return Err(syn::Error::new_spanned(attr, message));
        }

        // Parse function parameters
        let mut inputs = Vec::new();
        for (index, input_arg) in input.sig.inputs.iter().enumerate() {
            match input_arg {
                FnArg::Typed(typed_arg) => {
                    let param_name = if let Pat::Ident(pat_ident) = typed_arg.pat.as_ref() {
                        pat_ident.ident.clone()
                    } else {
                        // Generate a synthetic name for non-identifier patterns (like `_`)
                        syn::Ident::new(&format!("_{}", index), typed_arg.pat.span())
                    };
                    inputs.push(FunctionParameter { name: param_name, ty: (*typed_arg.ty).clone() });
                }
                FnArg::Receiver(_) => {
                    return Err(syn::Error::new_spanned(input_arg, "Methods with self parameters are not supported"));
                }
            }
        }

        let signature = FunctionSignature { inputs, output: input.sig.output.clone(), generics: input.sig.generics.clone() };

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
