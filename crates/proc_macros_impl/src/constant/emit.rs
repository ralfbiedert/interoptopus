use proc_macro2::TokenStream;
use quote::quote;

use super::model::ConstantModel;

impl ConstantModel {
    pub fn emit_constant_info_impl(&self) -> syn::Result<TokenStream> {
        let name_ident = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let effective_name = self.effective_name();
        let ty = &self.ty;
        let constant_value = self.constant_value_tokens()?;
        let docs_content = self.docs_content();
        let visibility = quote! { ::interoptopus::lang::meta::Visibility::Public };
        let emission = quote! { ::interoptopus::lang::meta::Emission::Common };

        let constant_info_impl = quote! {
            #[allow(non_camel_case_types)]
            #[allow(clippy::redundant_pub_crate)]
            pub struct #name_ident {}

            impl ::interoptopus::lang::constant::ConstantInfo for #name_ident {
                fn id() -> ::interoptopus::inventory::ConstantId {
                    ::interoptopus::inventory::ConstantId::from_id(::interoptopus::id!(#name_ident))
                }

                fn constant() -> ::interoptopus::lang::constant::Constant {
                    ::interoptopus::lang::constant::Constant {
                        name: #effective_name.to_string(),
                        visibility: #visibility,
                        docs: ::interoptopus::lang::meta::Docs::from_line(#docs_content),
                        emission: #emission,
                        ty: <#ty as ::interoptopus::lang::types::TypeInfo>::id(),
                        value: #constant_value,
                    }
                }

                fn register(inventory: &mut ::interoptopus::inventory::Inventory) {
                    <#ty as ::interoptopus::lang::types::TypeInfo>::register(inventory);
                    inventory.register_constant(Self::id(), Self::constant());
                }
            }
        };

        Ok(constant_info_impl)
    }
}
