use syn::{Expr, Ident, Lit, Token, parse::Parse, punctuated::Punctuated};

#[derive(Debug, Clone, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct FfiTypeArgs {
    pub packed: bool,
    pub transparent: bool,
    pub opaque: bool,
    pub service: bool,
    pub debug: bool,
    pub name: Option<String>,
    pub module: Option<String>,
    // Track source tokens for error reporting
    pub transparent_token: Option<Ident>,
    pub opaque_token: Option<Ident>,
    pub service_token: Option<Ident>,
}

impl Parse for FfiTypeArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        if input.is_empty() {
            return Ok(args);
        }

        let parsed = Punctuated::<FfiTypeArg, Token![,]>::parse_terminated(input)?;

        for arg in parsed {
            match arg {
                FfiTypeArg::Packed => args.packed = true,
                FfiTypeArg::Transparent(ident) => {
                    args.transparent = true;
                    args.transparent_token = Some(ident);
                }
                FfiTypeArg::Opaque(ident) => {
                    args.opaque = true;
                    args.opaque_token = Some(ident);
                }
                FfiTypeArg::Service(ident) => {
                    args.service = true;
                    args.service_token = Some(ident);
                }
                FfiTypeArg::Debug => args.debug = true,
                FfiTypeArg::Name(name) => args.name = Some(name),
                FfiTypeArg::Module(module) => args.module = Some(module),
            }
        }

        Ok(args)
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
enum FfiTypeArg {
    Packed,
    Transparent(Ident),
    Opaque(Ident),
    Service(Ident),
    Debug,
    Name(String),
    Module(String),
}

impl Parse for FfiTypeArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        match ident.to_string().as_str() {
            "packed" => Ok(Self::Packed),
            "transparent" => Ok(Self::Transparent(ident)),
            "opaque" => Ok(Self::Opaque(ident)),
            "service" => Ok(Self::Service(ident)),
            "debug" => Ok(Self::Debug),
            "name" => {
                input.parse::<Token![=]>()?;
                let expr: Expr = input.parse()?;
                if let Expr::Lit(lit) = expr {
                    if let Lit::Str(lit_str) = lit.lit {
                        Ok(Self::Name(lit_str.value()))
                    } else {
                        Err(syn::Error::new_spanned(lit, "Expected string literal"))
                    }
                } else {
                    Err(syn::Error::new_spanned(expr, "Expected string literal"))
                }
            }
            "module" => {
                input.parse::<Token![=]>()?;
                let expr: Expr = input.parse()?;
                if let Expr::Lit(lit) = expr {
                    if let Lit::Str(lit_str) = lit.lit {
                        Ok(Self::Module(lit_str.value()))
                    } else {
                        Err(syn::Error::new_spanned(lit, "Expected string literal"))
                    }
                } else {
                    Err(syn::Error::new_spanned(expr, "Expected string literal"))
                }
            }
            _ => Err(syn::Error::new_spanned(ident, "Unknown attribute")),
        }
    }
}
