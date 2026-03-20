use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Token, Type, braced, token};

/// Top-level plugin declaration: `Plugin { fn ...; impl Foo { ... } }`.
pub struct PluginInput {
    pub name: Ident,
    pub _brace: token::Brace,
    pub items: Vec<PluginItem>,
}

/// An item inside a plugin block: either a bare function or a service impl block.
pub enum PluginItem {
    Function(Box<PluginMethod>),
    Service(ServiceBlock),
}

/// An `impl Foo { fn create() -> Self; fn bar(&self, x: i32); }` block.
pub struct ServiceBlock {
    pub name: Ident,
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
    pub functions: Vec<PluginMethod>,
    pub services: Vec<ServiceBlock>,
}

impl PluginModel {
    pub fn from_input(input: PluginInput) -> Self {
        let mut functions = Vec::new();
        let mut services = Vec::new();

        for item in input.items {
            match item {
                PluginItem::Function(m) => functions.push(*m),
                PluginItem::Service(s) => services.push(s),
            }
        }

        Self { name: input.name, functions, services }
    }
}

impl ServiceBlock {
    pub fn prefix(&self) -> String {
        self.name.to_string().to_lowercase()
    }

    pub fn ctors(&self) -> Vec<&PluginMethod> {
        self.methods.iter().filter(|m| !m.has_self && is_self_return(m.ret.as_ref())).collect()
    }

    pub fn instance_methods(&self) -> Vec<&PluginMethod> {
        self.methods.iter().filter(|m| m.has_self).collect()
    }
}

pub fn is_self_return(ret: Option<&Type>) -> bool {
    match ret {
        Some(Type::Path(p)) => p.path.is_ident("Self"),
        _ => false,
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
        if input.peek(Token![impl]) {
            Ok(Self::Service(input.parse()?))
        } else {
            Ok(Self::Function(Box::new(input.parse()?)))
        }
    }
}

impl Parse for ServiceBlock {
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
