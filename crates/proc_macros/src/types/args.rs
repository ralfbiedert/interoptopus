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

        // Validate conflicting attributes
        args.validate_mutually_exclusive_attributes(opaque_span, transparent_span, service_span)?;

        Ok(args)
    }
}

impl FfiTypeArgs {
    fn validate_mutually_exclusive_attributes(
        &self,
        opaque_span: Option<proc_macro2::Span>,
        transparent_span: Option<proc_macro2::Span>,
        service_span: Option<proc_macro2::Span>,
    ) -> syn::Result<()> {
        let mut conflicts = Vec::new();

        if self.opaque {
            conflicts.push(("opaque", opaque_span));
        }
        if self.transparent {
            conflicts.push(("transparent", transparent_span));
        }
        if self.service {
            conflicts.push(("service", service_span));
        }

        if conflicts.len() > 1 {
            let names: Vec<&str> = conflicts.iter().map(|(name, _)| *name).collect();
            let span = conflicts[1].1.or(conflicts[0].1).unwrap_or(proc_macro2::Span::call_site());

            return Err(syn::Error::new(
                span,
                format!(
                    "Cannot use {} attributes together - they are mutually exclusive",
                    match names.len() {
                        2 => format!("'{}' and '{}'", names[0], names[1]),
                        3 => format!("'{}', '{}', and '{}'", names[0], names[1], names[2]),
                        _ => names.join(", "),
                    }
                )
            ));
        }

        Ok(())
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
            "packed" => Ok(FfiTypeArg::Packed),
            "transparent" => Ok(FfiTypeArg::Transparent(span)),
            "opaque" => Ok(FfiTypeArg::Opaque(span)),
            "service" => Ok(FfiTypeArg::Service(span)),
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