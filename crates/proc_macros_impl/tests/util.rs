#[macro_export]
macro_rules! expand_ffi {
    ($item:expr) => {{
        // Extract just the arguments from inside the attribute (empty in this case)
        let attr_args = if let Some(attr) = $item.attrs.iter().find(|a| a.path().is_ident("ffi")) {
            match &attr.meta {
                syn::Meta::Path(_) => quote! {},              // #[ffi] with no args
                syn::Meta::List(list) => list.tokens.clone(), // #[ffi(...)]
                syn::Meta::NameValue(_) => quote! {},         // shouldn't happen for your use case
            }
        } else {
            quote! {}
        };

        // Create a clean item without the ffi attribute
        let mut clean_item = $item.clone();
        clean_item.attrs.retain(|attr| !attr.path().is_ident("ffi"));
        let item_tokens = quote! { #clean_item };

        let output = interoptopus_proc_impl::ffi(attr_args, item_tokens);
        prettyplease::unparse(&syn::parse2(output).unwrap())
    }};
}
