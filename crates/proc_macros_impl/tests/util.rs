#[macro_export]
macro_rules! test_fn {
    ($item:expr) => {{
        // Extract just the arguments from inside the attribute (empty in this case)
        let attr_args = if let Some(attr) = $item.attrs.iter().find(|a| a.path().is_ident("ffi_function")) {
            match &attr.meta {
                syn::Meta::Path(_) => quote! {},              // #[ffi_function] with no args
                syn::Meta::List(list) => list.tokens.clone(), // #[ffi_function(...)]
                syn::Meta::NameValue(_) => quote! {},         // shouldn't happen for your use case
            }
        } else {
            quote! {}
        };

        // Create a clean function without the attribute
        let mut clean_item = $item.clone();
        clean_item.attrs.clear();
        let item_tokens = quote! { #clean_item };

        let output = interoptopus_proc_impl::ffi_function(attr_args, item_tokens);
        let formatted = prettyplease::unparse(&syn::parse2(output).unwrap());
        insta::assert_snapshot!(formatted);
    }};
}
