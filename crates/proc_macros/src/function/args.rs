use syn::{Expr, Ident, Lit, Token, parse::Parse, punctuated::Punctuated};

#[derive(Debug, Clone, Default)]
pub struct FfiFunctionArgs {
    pub debug: bool,
    pub export: Option<ExportKind>,
    pub module: Option<ModuleKind>,
}

#[derive(Debug, Clone)]
pub enum ExportKind {
    Unique,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum ModuleKind {
    Named(String),
    Common,
}

impl Parse for FfiFunctionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        if input.is_empty() {
            return Ok(args);
        }

        let parsed = Punctuated::<FfiFunctionArg, Token![,]>::parse_terminated(input)?;

        let mut export_span = None;
        let mut module_span = None;

        for arg in parsed {
            match arg {
                FfiFunctionArg::Debug => args.debug = true,
                FfiFunctionArg::Export(kind, span) => {
                    if export_span.is_some() {
                        return Err(syn::Error::new(span, "export can only be specified once"));
                    }
                    args.export = Some(kind);
                    export_span = Some(span);
                }
                FfiFunctionArg::Module(kind, span) => {
                    if module_span.is_some() {
                        return Err(syn::Error::new(span, "module can only be specified once"));
                    }
                    args.module = Some(kind);
                    module_span = Some(span);
                }
            }
        }

        Ok(args)
    }
}

#[derive(Debug, Clone)]
enum FfiFunctionArg {
    Debug,
    Export(ExportKind, proc_macro2::Span),
    Module(ModuleKind, proc_macro2::Span),
}

impl Parse for FfiFunctionArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let span = ident.span();

        match ident.to_string().as_str() {
            "debug" => Ok(Self::Debug),
            "export" => {
                input.parse::<Token![=]>()?;
                let expr: Expr = input.parse()?;

                match expr {
                    Expr::Lit(lit) => {
                        if let Lit::Str(lit_str) = lit.lit {
                            Ok(Self::Export(ExportKind::Custom(lit_str.value()), span))
                        } else {
                            Err(syn::Error::new_spanned(lit, "Expected string literal or identifier"))
                        }
                    }
                    Expr::Path(path) => {
                        if path.path.is_ident("unique") {
                            Ok(Self::Export(ExportKind::Unique, span))
                        } else {
                            Err(syn::Error::new_spanned(path, "Expected 'unique' or string literal"))
                        }
                    }
                    _ => Err(syn::Error::new_spanned(expr, "Expected 'unique' or string literal")),
                }
            }
            "module" => {
                input.parse::<Token![=]>()?;
                let expr: Expr = input.parse()?;

                match expr {
                    Expr::Lit(lit) => {
                        if let Lit::Str(lit_str) = lit.lit {
                            Ok(Self::Module(ModuleKind::Named(lit_str.value()), span))
                        } else {
                            Err(syn::Error::new_spanned(lit, "Expected string literal or 'common'"))
                        }
                    }
                    Expr::Path(path) => {
                        if path.path.is_ident("common") {
                            Ok(Self::Module(ModuleKind::Common, span))
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
