use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ItemConst, Type};

use crate::docs::extract_docs;

use super::args::FfiConstantArgs;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ConstantModel {
    pub name: String,
    pub ty: Type,
    pub value: Expr,
    pub args: FfiConstantArgs,
    pub docs: Vec<String>,
}

impl ConstantModel {
    pub fn from_item_const(item: ItemConst, args: FfiConstantArgs) -> syn::Result<Self> {
        let name = item.ident.to_string();
        let ty = (*item.ty).clone();
        let value = (*item.expr).clone();
        let docs = extract_docs(&item.attrs);

        Ok(Self { name, ty, value, args, docs })
    }

    pub fn validate(&self) -> syn::Result<()> {
        // Validate that the constant type is a supported primitive
        self.validate_type()?;

        // Don't validate the value expression - Rust's const system handles that
        // Any valid const expression is allowed

        Ok(())
    }

    fn validate_type(&self) -> syn::Result<()> {
        // Type validation is now handled by the ConstantValue trait constraint
        // If the type doesn't implement ConstantValue, compilation will fail
        Ok(())
    }

    pub fn constant_value_tokens(&self) -> syn::Result<TokenStream> {
        let name_ident = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        // Use the ConstantValue trait to get the value from the actual constant
        Ok(quote! {
            ::interoptopus::lang::constant::ConstantValue::value(&#name_ident)
        })
    }

    pub fn effective_name(&self) -> String {
        self.args.name.as_ref().unwrap_or(&self.name).clone()
    }

    pub fn docs_content(&self) -> String {
        self.docs.join("\n")
    }
}
