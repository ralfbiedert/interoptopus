//! Determines the smallest `#[repr]` discriminant type for an enum.
//!
//! Analyses variant discriminant expressions (integer literals, negation of
//! literals, or auto-numbered) and picks the smallest Rust primitive that fits
//! all values.  Falls back to `i32` when an expression cannot be evaluated
//! statically.

/// The discriminant analysis result, expressed as a Rust `#[repr]` token and
/// the matching `Primitive` variant for the inventory metadata.
#[derive(Clone)]
pub struct DiscriminantChoice {
    /// Token for `#[repr(<ty>)]`, e.g. `u8`, `i16`, `u32`.
    pub repr_ident: &'static str,
    /// Token for `Primitive::<Variant>`.
    pub primitive_ident: &'static str,
}

/// Try to evaluate a discriminant expression to an `isize`.
///
/// Handles:
/// - Integer literals: `42`
/// - Negated integer literals: `-1`
///
/// Returns `None` for anything more complex (const generics, paths, arithmetic …).
fn try_eval(expr: &syn::Expr) -> Option<isize> {
    match expr {
        syn::Expr::Lit(lit) => {
            if let syn::Lit::Int(int_lit) = &lit.lit {
                int_lit.base10_parse::<isize>().ok()
            } else {
                None
            }
        }
        syn::Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            if let syn::Expr::Lit(lit) = &*unary.expr {
                if let syn::Lit::Int(int_lit) = &lit.lit {
                    int_lit.base10_parse::<isize>().ok().map(|v| -v)
                } else {
                    None
                }
            } else {
                None
            }
        }
        // Group expressions (e.g. `(42)`) — peel one layer
        syn::Expr::Group(g) => try_eval(&g.expr),
        syn::Expr::Paren(p) => try_eval(&p.expr),
        _ => None,
    }
}

/// Compute the smallest discriminant type for an enum given its variants.
///
/// Each item in `discriminants` is `Option<&syn::Expr>` — `None` for
/// auto-numbered variants, `Some(expr)` for explicit `= expr` discriminants.
///
/// The Rust auto-numbering rule is: first variant starts at 0, each subsequent
/// variant is previous + 1, unless an explicit discriminant resets the counter.
pub fn optimal_discriminant<'a>(discriminants: impl Iterator<Item = Option<&'a syn::Expr>>) -> DiscriminantChoice {
    let mut has_negative = false;
    let mut max_val: isize = 0;
    let mut min_val: isize = 0;
    let mut next_auto: isize = 0;

    for disc in discriminants {
        let value = if let Some(expr) = disc {
            match try_eval(expr) {
                Some(val) => {
                    next_auto = val + 1;
                    val
                }
                None => {
                    // Can't evaluate — conservatively fall back to i32
                    return DiscriminantChoice { repr_ident: "i32", primitive_ident: "I32" };
                }
            }
        } else {
            let val = next_auto;
            next_auto += 1;
            val
        };

        if value < 0 {
            has_negative = true;
            min_val = min_val.min(value);
        }
        max_val = max_val.max(value);
    }

    #[allow(clippy::cast_possible_wrap)]
    if has_negative {
        if min_val >= i8::MIN as isize && max_val <= i8::MAX as isize {
            DiscriminantChoice { repr_ident: "i8", primitive_ident: "I8" }
        } else if min_val >= i16::MIN as isize && max_val <= i16::MAX as isize {
            DiscriminantChoice { repr_ident: "i16", primitive_ident: "I16" }
        } else {
            DiscriminantChoice { repr_ident: "i32", primitive_ident: "I32" }
        }
    } else if max_val <= u8::MAX as isize {
        DiscriminantChoice { repr_ident: "u8", primitive_ident: "U8" }
    } else if max_val <= u16::MAX as isize {
        DiscriminantChoice { repr_ident: "u16", primitive_ident: "U16" }
    } else {
        DiscriminantChoice { repr_ident: "u32", primitive_ident: "U32" }
    }
}

/// Build the `#[repr(…)]` attribute for the given discriminant choice.
pub fn repr_attribute(choice: &DiscriminantChoice) -> syn::Attribute {
    let ident = syn::Ident::new(choice.repr_ident, proc_macro2::Span::call_site());
    syn::parse_quote! { #[repr(#ident)] }
}

/// Build the `Layout::Primitive(Primitive::…)` token stream for `generate_repr`.
pub fn layout_tokens(choice: &DiscriminantChoice, span: proc_macro2::Span) -> proc_macro2::TokenStream {
    let ident = syn::Ident::new(choice.primitive_ident, span);
    quote::quote_spanned! { span =>
        ::interoptopus::lang::types::Layout::Primitive(
            ::interoptopus::lang::types::Primitive::#ident
        )
    }
}

/// Build the Rust type token for `WireIO` read/write (e.g. `u8`, `i16`).
pub fn wire_type_tokens(choice: &DiscriminantChoice, span: proc_macro2::Span) -> syn::Ident {
    syn::Ident::new(choice.repr_ident, span)
}
