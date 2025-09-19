use proc_macro2::TokenStream;
use syn::{Attribute, Error, Lit, Meta, Result};

#[derive(Debug, Default)]
pub struct FfiTypeArgs {
    pub packed: bool,
    pub transparent: bool,
    pub opaque: bool,
    pub service: bool,
    pub debug: bool,
    pub name: Option<String>,
    pub module: Option<String>,
}

impl FfiTypeArgs {
    pub fn from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut args = Self::default();

        for attr in attrs {
            if attr.path().is_ident("ffi_type") {
                args.parse_ffi_type_attr(attr)?;
            }
        }

        Ok(args)
    }

    pub fn from_token_stream(tokens: TokenStream) -> Result<Self> {
        if tokens.is_empty() {
            return Ok(Self::default());
        }

        // For `#[ffi_type(name = "Vec2")]`, the tokens will be `name = "Vec2"`
        // We need to wrap this in parentheses to make it parse as a Meta::List
        let wrapped_tokens = quote::quote! { (#tokens) };
        let meta: Meta = syn::parse2(wrapped_tokens)?;
        let mut args = Self::default();
        args.parse_meta(&meta)?;
        Ok(args)
    }

    fn parse_ffi_type_attr(&mut self, attr: &Attribute) -> Result<()> {
        match &attr.meta {
            Meta::Path(_) => {
                // #[ffi_type] without parameters
                Ok(())
            }
            Meta::List(list) => {
                self.parse_meta(&Meta::List(list.clone()))
            }
            Meta::NameValue(_) => {
                Err(Error::new_spanned(attr, "ffi_type does not support name-value syntax"))
            }
        }
    }

    fn parse_meta(&mut self, meta: &Meta) -> Result<()> {
        match meta {
            Meta::Path(path) => {
                if path.is_ident("packed") {
                    self.packed = true;
                } else if path.is_ident("transparent") {
                    self.transparent = true;
                } else if path.is_ident("opaque") {
                    self.opaque = true;
                } else if path.is_ident("service") {
                    self.service = true;
                } else if path.is_ident("debug") {
                    self.debug = true;
                } else {
                    return Err(Error::new_spanned(path, format!("Unknown ffi_type parameter: {}", path.get_ident().unwrap())));
                }
                Ok(())
            }
            Meta::List(list) => {
                list.parse_nested_meta(|nested| {
                    if nested.path.is_ident("packed") {
                        self.packed = true;
                        Ok(())
                    } else if nested.path.is_ident("transparent") {
                        self.transparent = true;
                        Ok(())
                    } else if nested.path.is_ident("opaque") {
                        self.opaque = true;
                        Ok(())
                    } else if nested.path.is_ident("service") {
                        self.service = true;
                        Ok(())
                    } else if nested.path.is_ident("debug") {
                        self.debug = true;
                        Ok(())
                    } else if nested.path.is_ident("name") {
                        let value = nested.value()?;
                        let name: Lit = value.parse()?;
                        if let Lit::Str(lit_str) = name {
                            self.name = Some(lit_str.value());
                            Ok(())
                        } else {
                            Err(nested.error("name must be a string literal"))
                        }
                    } else if nested.path.is_ident("module") {
                        let value = nested.value()?;
                        let module: Lit = value.parse()?;
                        if let Lit::Str(lit_str) = module {
                            self.module = Some(lit_str.value());
                            Ok(())
                        } else {
                            Err(nested.error("module must be a string literal"))
                        }
                    } else {
                        Err(nested.error(format!("Unknown ffi_type parameter: {}", nested.path.get_ident().map(|i| i.to_string()).unwrap_or_else(|| "unknown".to_string()))))
                    }
                })
            }
            Meta::NameValue(name_value) => {
                Err(Error::new_spanned(name_value, "ffi_type does not support this name-value syntax"))
            }
        }
    }

    pub fn validate(&self) -> Result<()> {
        let exclusive_count = [self.packed, self.transparent, self.opaque, self.service]
            .iter()
            .map(|&x| if x { 1 } else { 0 })
            .sum::<i32>();

        if exclusive_count > 1 {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "Cannot combine packed, transparent, opaque, or service attributes"
            ));
        }

        if self.service && (self.name.is_some() || self.module.is_some()) {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "service attribute cannot be combined with name or module"
            ));
        }

        Ok(())
    }
}