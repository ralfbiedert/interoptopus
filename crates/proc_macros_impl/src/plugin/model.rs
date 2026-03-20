use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Token, Type, braced, token};

/// Top-level plugin declaration: `MyPlugin { ... }`.
pub struct PluginInput {
    pub name: Ident,
    pub _brace: token::Brace,
    pub methods: Vec<PluginMethod>,
}

/// A method inside a plugin block. Supports `&self` and `-> Self`.
pub struct PluginMethod {
    pub name: Ident,
    pub has_self: bool,
    pub params: Vec<PluginParam>,
    pub ret: Option<Type>,
}

/// A typed parameter: `name: Type`.
pub struct PluginParam {
    pub name: Ident,
    pub ty: Type,
}

/// The fully resolved model ready for emission.
pub struct PluginModel {
    pub name: Ident,
    pub methods: Vec<PluginMethod>,
}

impl PluginModel {
    pub fn from_input(input: PluginInput) -> syn::Result<Self> {
        Ok(Self { name: input.name, methods: input.methods })
    }

    /// A service plugin has `&self` methods (instance methods on a GCHandle-backed object).
    /// A function plugin has only plain functions.
    pub fn is_service(&self) -> bool {
        self.methods.iter().any(|m| m.has_self)
    }
}

// --- Parsing ---

impl Parse for PluginInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        let brace = braced!(content in input);
        let mut methods = Vec::new();

        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        Ok(Self { name, _brace: brace, methods })
    }
}

impl Parse for PluginMethod {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![fn]>()?;
        let name: Ident = input.parse()?;

        let content;
        syn::parenthesized!(content in input);

        let has_self = content.peek(Token![&]) && content.peek2(Token![self]);
        if has_self {
            content.parse::<Token![&]>()?;
            content.parse::<Token![self]>()?;
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        let params = parse_params(&content)?;
        let ret = parse_return_type(input)?;
        input.parse::<Token![;]>()?;

        Ok(Self { name, has_self, params, ret })
    }
}

fn parse_params(input: ParseStream) -> syn::Result<Vec<PluginParam>> {
    let punctuated: Punctuated<PluginParam, Token![,]> = Punctuated::parse_terminated(input)?;
    Ok(punctuated.into_iter().collect())
}

fn parse_return_type(input: ParseStream) -> syn::Result<Option<Type>> {
    if input.peek(Token![->]) {
        input.parse::<Token![->]>()?;
        Ok(Some(input.parse()?))
    } else {
        Ok(None)
    }
}

impl Parse for PluginParam {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(Self { name, ty })
    }
}
