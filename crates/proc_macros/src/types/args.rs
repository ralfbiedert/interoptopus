use syn::{parse::Parse, punctuated::Punctuated, Expr, Ident, Lit, Token};

#[derive(Debug, Clone, Default)]
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
        let mut args = FfiTypeArgs::default();
        
        if input.is_empty() {
            return Ok(args);
        }

        let parsed = Punctuated::<FfiTypeArg, Token![,]>::parse_terminated(input)?;
        
        for arg in parsed {
            match arg {
                FfiTypeArg::Packed => args.packed = true,
                FfiTypeArg::Transparent => args.transparent = true,
                FfiTypeArg::Opaque => args.opaque = true,
                FfiTypeArg::Service => args.service = true,
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
    Transparent,
    Opaque,
    Service,
    Debug,
    Name(String),
    Module(String),
}

impl Parse for FfiTypeArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        
        match ident.to_string().as_str() {
            "packed" => Ok(FfiTypeArg::Packed),
            "transparent" => Ok(FfiTypeArg::Transparent),
            "opaque" => Ok(FfiTypeArg::Opaque),
            "service" => Ok(FfiTypeArg::Service),
            "debug" => Ok(FfiTypeArg::Debug),
            "name" => {
                input.parse::<Token![=]>()?;
                let expr: Expr = input.parse()?;
                if let Expr::Lit(lit) = expr {
                    if let Lit::Str(lit_str) = lit.lit {
                        Ok(FfiTypeArg::Name(lit_str.value()))
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
                        Ok(FfiTypeArg::Module(lit_str.value()))
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