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
}

impl Parse for FfiTypeArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        if input.is_empty() {
            return Ok(args);
        }

        let parsed = Punctuated::<FfiTypeArg, Token![,]>::parse_terminated(input)?;

        let mut opaque_span = None;
        let mut transparent_span = None;
        let mut service_span = None;

        for arg in parsed {
            match arg {
                FfiTypeArg::Packed => args.packed = true,
                FfiTypeArg::Transparent(span) => {
                    args.transparent = true;
                    transparent_span = Some(span);
                }
                FfiTypeArg::Opaque(span) => {
                    args.opaque = true;
                    opaque_span = Some(span);
                }
                FfiTypeArg::Service(span) => {
                    args.service = true;
                    service_span = Some(span);
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
enum FfiTypeArg {
    Packed,
    Transparent(proc_macro2::Span),
    Opaque(proc_macro2::Span),
    Service(proc_macro2::Span),
    Debug,
    Name(String),
    Module(String),
}

impl Parse for FfiTypeArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let span = ident.span();

        match ident.to_string().as_str() {
            "packed" => Ok(Self::Packed),
            "transparent" => Ok(Self::Transparent(span)),
            "opaque" => Ok(Self::Opaque(span)),
            "service" => Ok(Self::Service(span)),
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
