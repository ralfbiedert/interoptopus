use proc_macro2::Span;
use std::collections::HashSet;
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

/// A method inside a plugin block. Supports `&self`, `-> Self`, and `async fn`.
pub struct PluginMethod {
    pub name: Ident,
    pub is_async: bool,
    pub async_span: Option<Span>,
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
    pub fn from_input(input: PluginInput) -> syn::Result<Self> {
        let mut functions = Vec::new();
        let mut services = Vec::new();

        for item in input.items {
            match item {
                PluginItem::Function(m) => {
                    if m.has_self {
                        return Err(syn::Error::new(
                            m.name.span(),
                            format!(
                                "Bare plugin function `{}` cannot take `&self`. \
                                 Move it into an `impl` block to make it a service method.",
                                m.name
                            ),
                        ));
                    }
                    functions.push(*m);
                }
                PluginItem::Service(s) => {
                    // Validate: every method in an impl block must either be a
                    // constructor (returns Self / contains Self in return type)
                    // or an instance method (has &self).
                    for m in &s.methods {
                        if !m.has_self && !contains_self_return(m.ret.as_ref()) {
                            return Err(syn::Error::new(m.name.span(), format!("Non-ctor static method `{}` in `impl {}` is not allowed.", m.name, s.name)));
                        }
                    }
                    services.push(s);
                }
            }
        }

        Ok(Self { name: input.name, functions, services })
    }

    /// Returns the set of all service type names defined in this plugin.
    pub fn service_names(&self) -> HashSet<String> {
        self.services.iter().map(|s| s.name.to_string()).collect()
    }
}

impl ServiceBlock {
    pub fn prefix(&self) -> String {
        pascal_to_snake(&self.name.to_string())
    }

    /// Constructors: methods without `&self` that return `Self` or `ffi::Result<Self, E>`.
    pub fn ctors(&self) -> Vec<&PluginMethod> {
        self.methods.iter().filter(|m| !m.has_self && contains_self_return(m.ret.as_ref())).collect()
    }

    pub fn instance_methods(&self) -> Vec<&PluginMethod> {
        self.methods.iter().filter(|m| m.has_self).collect()
    }

    /// Returns the names of other services that methods of this block can return.
    pub fn returned_service_names(&self, all_service_names: &HashSet<String>) -> HashSet<String> {
        let mut result = HashSet::new();
        for m in self.instance_methods() {
            if let Some(name) = service_in_return(m.ret.as_ref(), all_service_names) {
                result.insert(name);
            }
        }
        result
    }
}

/// Returns `true` if the return type is exactly `Self`.
pub fn is_self_return(ret: Option<&Type>) -> bool {
    match ret {
        Some(Type::Path(p)) => p.path.is_ident("Self"),
        _ => false,
    }
}

/// Returns `true` if the return type contains `Self` anywhere (bare, or inside
/// generic arguments like `Result<Self, E>`, `Try<Self>`, etc.).
pub fn contains_self_return(ret: Option<&Type>) -> bool {
    match ret {
        Some(Type::Path(p)) => {
            if p.path.is_ident("Self") {
                return true;
            }
            p.path.segments.iter().any(|seg| {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    args.args.iter().any(|arg| {
                        if let syn::GenericArgument::Type(inner) = arg {
                            contains_self_return(Some(inner))
                        } else {
                            false
                        }
                    })
                } else {
                    false
                }
            })
        }
        _ => false,
    }
}

/// Recursively replace every occurrence of `Self` in a type with `replacement`.
pub fn replace_self(ty: &Type, replacement: &Ident) -> Type {
    match ty {
        Type::Path(p) => {
            if p.path.is_ident("Self") {
                return syn::parse_quote! { #replacement };
            }
            let mut p = p.clone();
            for seg in &mut p.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &mut seg.arguments {
                    for arg in &mut args.args {
                        if let syn::GenericArgument::Type(inner) = arg {
                            *inner = replace_self(inner, replacement);
                        }
                    }
                }
            }
            Type::Path(p)
        }
        other => other.clone(),
    }
}

/// If `ty` is a known service type name, return it.
pub fn direct_service_name(ty: &Type, service_names: &HashSet<String>) -> Option<String> {
    if let Type::Path(p) = ty
        && let Some(ident) = p.path.get_ident()
    {
        let name = ident.to_string();
        if service_names.contains(&name) {
            return Some(name);
        }
    }
    None
}

/// If `ty` is `ffi::Result<ServiceType, E>`, return the service name.
pub fn result_service_name(ty: &Type, service_names: &HashSet<String>) -> Option<String> {
    if let Type::Path(p) = ty
        && let Some(seg) = p.path.segments.last()
        && seg.ident == "Result"
        && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
        && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
    {
        return direct_service_name(inner, service_names);
    }
    None
}

/// If `ty` is `&ServiceType`, return the service name.
pub fn ref_service_name(ty: &Type, service_names: &HashSet<String>) -> Option<String> {
    if let Type::Reference(r) = ty {
        return direct_service_name(&r.elem, service_names);
    }
    None
}

/// Extract the service name from a return type (direct or Result-wrapped). Does not match `Self`.
pub fn service_in_return(ret: Option<&Type>, service_names: &HashSet<String>) -> Option<String> {
    let ty = ret?;
    if let Some(name) = direct_service_name(ty, service_names) {
        return Some(name);
    }
    result_service_name(ty, service_names)
}

/// Recursively find a service name anywhere inside a type's generic arguments.
///
/// Handles `Service`, `ffi::Result<Service, E>`, `ffi::Option<Service>`,
/// `ffi::Result<ffi::Option<Service>, E>`, etc.
pub fn service_in_type(ty: &Type, service_names: &HashSet<String>) -> Option<String> {
    if let Some(name) = direct_service_name(ty, service_names) {
        return Some(name);
    }
    if let Type::Path(p) = ty
        && let Some(seg) = p.path.segments.last()
        && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
    {
        for arg in &args.args {
            if let syn::GenericArgument::Type(inner) = arg
                && let Some(name) = service_in_type(inner, service_names)
            {
                return Some(name);
            }
        }
    }
    None
}

/// Check if a parameter type involves a service (owned or by reference).
#[allow(dead_code)]
pub fn param_service_name(ty: &Type, service_names: &HashSet<String>) -> Option<String> {
    if let Some(name) = direct_service_name(ty, service_names) {
        return Some(name);
    }
    ref_service_name(ty, service_names)
}

/// Computes the transitive closure of service types reachable from `s`'s return types.
///
/// If service A returns B and B returns C, then A transitively needs B and C.
pub fn transitive_returned_services(s: &ServiceBlock, all_services: &[ServiceBlock], svc_names: &HashSet<String>) -> Vec<String> {
    let own_name = s.name.to_string();
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    // Pre-mark self as visited so we never include our own methods as "extra" fields.
    visited.insert(own_name);
    let mut queue: Vec<String> = s.returned_service_names(svc_names).into_iter().collect();
    while let Some(name) = queue.pop() {
        if visited.insert(name.clone()) {
            result.push(name.clone());
            if let Some(other) = all_services.iter().find(|o| o.name == name) {
                for sub in other.returned_service_names(svc_names) {
                    if !visited.contains(&sub) {
                        queue.push(sub);
                    }
                }
            }
        }
    }
    result.sort();
    result
}

/// Converts a `PascalCase` name to `snake_case`.
fn pascal_to_snake(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 4);
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            let prev = name.as_bytes()[i - 1];
            if prev.is_ascii_lowercase() || prev.is_ascii_digit() {
                result.push('_');
            }
        }
        result.extend(c.to_lowercase());
    }
    result
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
        let async_span = if input.peek(Token![async]) {
            let tok = input.parse::<Token![async]>()?;
            Some(tok.span)
        } else {
            None
        };
        let is_async = async_span.is_some();
        input.parse::<Token![fn]>()?;
        let name: Ident = input.parse()?;

        let content;
        syn::parenthesized!(content in input);

        let has_self = content.peek(Token![&]) && content.peek2(Token![self]);
        let has_mut_self = content.peek(Token![&]) && content.peek2(Token![mut]);
        if has_mut_self {
            return Err(syn::Error::new(name.span(), format!("`&mut self` is not supported in plugin service methods. Use `&self` instead.")));
        }
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

        Ok(Self { name, is_async, async_span, has_self, params, ret })
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
