#[macro_export]
macro_rules! expand_fn {
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
        clean_item.attrs.retain(|attr| !attr.path().is_ident("ffi_function"));
        let item_tokens = quote! { #clean_item };

        let output = interoptopus_proc_impl::ffi_function(attr_args, item_tokens);
        prettyplease::unparse(&syn::parse2(output).unwrap())
    }};
}

#[macro_export]
macro_rules! expand_ty {
    ($item:expr) => {{
        // Extract just the arguments from inside the attribute (empty in this case)
        let attr_args = if let Some(attr) = $item.attrs.iter().find(|a| a.path().is_ident("ffi_type")) {
            match &attr.meta {
                syn::Meta::Path(_) => quote! {},              // #[ffi_type] with no args
                syn::Meta::List(list) => list.tokens.clone(), // #[ffi_type(...)]
                syn::Meta::NameValue(_) => quote! {},         // shouldn't happen for your use case
            }
        } else {
            quote! {}
        };

        // Create a clean function without the attribute
        let mut clean_item = $item.clone();
        clean_item.attrs.retain(|attr| !attr.path().is_ident("ffi_type"));
        let item_tokens = quote! { #clean_item };

        let output = interoptopus_proc_impl::ffi_type(attr_args, item_tokens);
        prettyplease::unparse(&syn::parse2(output).unwrap())
    }};
}

#[macro_export]
macro_rules! expand_svc {
    ($item:expr) => {{
        // Extract just the arguments from inside the attribute (empty in this case)
        let attr_args = if let Some(attr) = $item.attrs.iter().find(|a| a.path().is_ident("ffi_service")) {
            match &attr.meta {
                syn::Meta::Path(_) => quote! {},              // #[ffi_service] with no args
                syn::Meta::List(list) => list.tokens.clone(), // #[ffi_service(...)]
                syn::Meta::NameValue(_) => quote! {},         // shouldn't happen for your use case
            }
        } else {
            quote! {}
        };

        // Create a clean function without the ffi_service attribute
        let mut clean_item = $item.clone();
        clean_item.attrs.retain(|attr| !attr.path().is_ident("ffi_service"));
        let item_tokens = quote! { #clean_item };

        let output = interoptopus_proc_impl::ffi_service(attr_args, item_tokens);
        prettyplease::unparse(&syn::parse2(output).unwrap())
    }};
}
