use crate::forbidden::is_forbidden_name;
use crate::service::model::{ReceiverKind, ServiceModel};
use quote::ToTokens;
use syn::{ImplItem, ItemImpl};

impl ServiceModel {
    /// Consolidated validation for service constraints
    pub fn validate(&self, input: &ItemImpl) -> syn::Result<()> {
        self.validate_async_constraints(input)?;
        self.validate_forbidden_names()?;
        Ok(())
    }

    /// Validate async service constraints
    fn validate_async_constraints(&self, input: &ItemImpl) -> syn::Result<()> {
        if self.is_async {
            for method in &self.methods {
                if method.receiver_kind == ReceiverKind::Mutable {
                    // Find the receiver span in the method to point the error there
                    let error_span = if let Some(ImplItem::Fn(method_fn)) = input
                        .items
                        .iter()
                        .find(|item| if let ImplItem::Fn(f) = item { f.sig.ident == method.name } else { false })
                    {
                        // Find the receiver argument
                        if let Some(first_param) = method_fn.sig.inputs.first() {
                            first_param.to_token_stream()
                        } else {
                            method_fn.sig.ident.to_token_stream()
                        }
                    } else {
                        input.to_token_stream()
                    };
                    return Err(syn::Error::new_spanned(error_span, "Async services cannot have methods with &mut self. Use &self instead."));
                }
            }
        }
        Ok(())
    }

    /// Validates that no forbidden names are used for methods or parameters.
    fn validate_forbidden_names(&self) -> syn::Result<()> {
        // Check constructor method names and parameters
        for constructor in &self.constructors {
            // if is_forbidden_name(constructor.name.to_string()) {
            //     return Err(syn::Error::new_spanned(&constructor.name, format!("Using the name '{}' can cause conflicts in generated code.", constructor.name)));
            // }

            for param in &constructor.inputs {
                if is_forbidden_name(param.name.to_string()) {
                    return Err(syn::Error::new_spanned(&param.name, format!("Using the name '{}' can cause conflicts in generated code.", param.name)));
                }
            }
        }

        // Check regular method names and parameters
        for method in &self.methods {
            if is_forbidden_name(method.name.to_string()) {
                return Err(syn::Error::new_spanned(&method.name, format!("Using the name '{}' can cause conflicts in generated code.", method.name)));
            }

            for param in &method.inputs {
                if is_forbidden_name(param.name.to_string()) {
                    return Err(syn::Error::new_spanned(&param.name, format!("Using the name '{}' can cause conflicts in generated code.", param.name)));
                }
            }
        }

        Ok(())
    }
}
