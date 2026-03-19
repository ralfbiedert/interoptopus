use syn::{Expr, Ident, Token, parse::Parse, punctuated::Punctuated};

#[derive(Debug, Clone)]
pub enum ServiceExportKind {
    Unique,
}

#[derive(Debug, Clone, Default)]
pub struct FfiServiceArgs {
    pub debug: bool,
    pub prefix: Option<String>,
    pub export: Option<ServiceExportKind>,
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
                FfiServiceArg::Prefix(prefix) => args.prefix = Some(prefix),
                FfiServiceArg::Export(kind) => args.export = Some(kind),
            }
        }

        Ok(args)
    }
}

#[derive(Debug, Clone)]
enum FfiServiceArg {
    Debug,
    Prefix(String),
    Export(ServiceExportKind),
}

impl Parse for FfiServiceArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        match ident.to_string().as_str() {
            "debug" => Ok(Self::Debug),
            "prefix" => {
                input.parse::<Token![=]>()?;
                let expr: syn::Expr = input.parse()?;
                if let syn::Expr::Lit(lit) = expr {
                    if let syn::Lit::Str(lit_str) = lit.lit {
                        Ok(Self::Prefix(lit_str.value()))
                    } else {
                        Err(syn::Error::new_spanned(lit, "Expected string literal"))
                    }
                } else {
                    Err(syn::Error::new_spanned(expr, "Expected string literal"))
                }
            }
            "export" => {
                input.parse::<Token![=]>()?;
                let expr: Expr = input.parse()?;

                match expr {
                    Expr::Path(path) => {
                        if path.path.is_ident("unique") {
                            Ok(Self::Export(ServiceExportKind::Unique))
                        } else {
                            Err(syn::Error::new_spanned(path, "Expected 'unique'"))
                        }
                    }
                    _ => Err(syn::Error::new_spanned(expr, "Expected 'unique'")),
                }
            }
            _ => Err(syn::Error::new_spanned(ident, "Unknown attribute")),
        }
    }
}
