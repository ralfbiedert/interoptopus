use syn::{Ident, Token, parse::Parse, punctuated::Punctuated};

#[derive(Debug, Clone, Default)]
pub struct FfiServiceArgs {
    pub debug: bool,
}

impl Parse for FfiServiceArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        if input.is_empty() {
            return Ok(args);
        }

        let parsed = Punctuated::<FfiServiceArg, Token![,]>::parse_terminated(input)?;

        for arg in parsed {
            match arg {
                FfiServiceArg::Debug => args.debug = true,
            }
        }

        Ok(args)
    }
}

#[derive(Debug, Clone)]
enum FfiServiceArg {
    Debug,
}

impl Parse for FfiServiceArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        match ident.to_string().as_str() {
            "debug" => Ok(Self::Debug),
            _ => Err(syn::Error::new_spanned(ident, "Unknown attribute")),
        }
    }
}