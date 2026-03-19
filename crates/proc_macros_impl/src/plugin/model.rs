use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Token, Type, braced, token};

/// Top-level plugin declaration: `MyPlugin { ... }`.
pub struct PluginInput {
    pub name: Ident,
    pub _brace: token::Brace,
    pub items: Vec<PluginItem>,
}

/// An item inside the plugin block: either a free function or a service impl block.
pub enum PluginItem {
    Function(PluginFunction),
    Service(PluginService),
}

/// A free function signature: `fn name(args...) -> ReturnType;`
pub struct PluginFunction {
    pub name: Ident,
    pub params: Vec<PluginParam>,
    pub ret: Option<Type>,
}

/// A service block: `impl Foo { fn new() -> Self; fn bar(&self, x: u32); }`
pub struct PluginService {
    pub name: Ident,
    pub methods: Vec<PluginMethod>,
}

/// A method inside a service impl block.
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
    pub functions: Vec<PluginFunction>,
    pub services: Vec<PluginService>,
}

impl PluginModel {
    pub fn from_input(input: PluginInput) -> syn::Result<Self> {
        let mut functions = Vec::new();
        let mut services = Vec::new();

        for item in input.items {
            match item {
                PluginItem::Function(f) => functions.push(f),
                PluginItem::Service(s) => services.push(s),
            }
        }

        Ok(Self { name: input.name, functions, services })
    }
}

// --- Parsing ---

impl Parse for PluginInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        let brace = braced!(content in input);
        let mut items = Vec::new();

        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(Self { name, _brace: brace, items })
    }
}

impl Parse for PluginItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![impl]) {
            Ok(PluginItem::Service(input.parse()?))
        } else if lookahead.peek(Token![fn]) {
            Ok(PluginItem::Function(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for PluginFunction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![fn]>()?;
        let name: Ident = input.parse()?;

        let content;
        syn::parenthesized!(content in input);
        let params = parse_params(&content)?;

        let ret = parse_return_type(input)?;
        input.parse::<Token![;]>()?;

        Ok(Self { name, params, ret })
    }
}

impl Parse for PluginService {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![impl]>()?;
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);
        let mut methods = Vec::new();

        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        Ok(Self { name, methods })
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
