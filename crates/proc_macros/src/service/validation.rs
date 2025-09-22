use crate::service::model::{ReceiverKind, ServiceModel};
use quote::ToTokens;
use syn::{ImplItem, ItemImpl};

impl ServiceModel {
    /// Consolidated validation for service constraints
    pub fn validate(&self, input: &ItemImpl) -> syn::Result<()> {
        self.validate_async_constraints(input)?;
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
}
