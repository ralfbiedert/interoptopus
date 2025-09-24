use syn::{parse::Parse, punctuated::Punctuated, Expr, Ident, Lit, Token};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ModuleKind {
    Named(String),
    Common,
}

#[derive(Debug, Clone, Default)]
pub struct FfiConstantArgs {
    pub debug: bool,
    pub name: Option<String>,
    pub module: Option<ModuleKind>,
}

impl Parse for FfiConstantArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        if input.is_empty() {
            return Ok(args);
        }

        let parsed = Punctuated::<FfiConstantArg, Token![,]>::parse_terminated(input)?;

        for arg in parsed {
            match arg {
                FfiConstantArg::Debug => args.debug = true,
                FfiConstantArg::Name(name) => args.name = Some(name),
                FfiConstantArg::Module(module) => args.module = Some(module),
            }
        }

        Ok(args)
    }
}

impl FfiConstantArgs {
    pub fn validate(&self) -> syn::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum FfiConstantArg {
    Debug,
    Name(String),
    Module(ModuleKind),
}

impl Parse for FfiConstantArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        match ident.to_string().as_str() {
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

                match expr {
                    Expr::Lit(lit) => {
                        if let Lit::Str(lit_str) = lit.lit {
                            Ok(Self::Module(ModuleKind::Named(lit_str.value())))
                        } else {
                            Err(syn::Error::new_spanned(lit, "Expected string literal or 'common'"))
                        }
                    }
                    Expr::Path(path) => {
                        if path.path.is_ident("common") {
                            Ok(Self::Module(ModuleKind::Common))
                        } else {
                            Err(syn::Error::new_spanned(path, "Expected 'common' or string literal"))
                        }
                    }
                    _ => Err(syn::Error::new_spanned(expr, "Expected 'common' or string literal")),
                }
            }
            _ => Err(syn::Error::new_spanned(ident, "Unknown attribute")),
        }
    }
}
