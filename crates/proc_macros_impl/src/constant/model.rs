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
    pub fn from_item_const(item: ItemConst, args: FfiConstantArgs) -> Self {
        let name = item.ident.to_string();
        let ty = (*item.ty).clone();
        let value = (*item.expr).clone();
        let docs = extract_docs(&item.attrs);

        Self { name, ty, value, args, docs }
    }

    pub fn validate() {
        // Type validation is now handled by the ConstantValue trait constraint
        // If the type doesn't implement ConstantValue, compilation will fail
    }

    pub fn constant_value_tokens(&self) -> TokenStream {
        let name_ident = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        // Use the ConstantValue trait to get the value from the actual constant
        quote! {
            ::interoptopus::lang::constant::ConstantValue::value(&#name_ident)
        }
    }

    pub fn effective_name(&self) -> String {
        self.args.name.as_ref().unwrap_or(&self.name).clone()
    }

    pub fn docs_content(&self) -> String {
        self.docs.join("\n")
    }
}
