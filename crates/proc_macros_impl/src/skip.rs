use syn::Attribute;

/// Check if an attribute is `#[ffi::skip]`
pub fn is_ffi_skip_attribute(attr: &Attribute) -> bool {
    if let syn::Meta::Path(path) = &attr.meta {
        path.segments.len() == 2
            && path.segments[0].ident == "ffi"
            && path.segments[1].ident == "skip"
    } else {
        false
    }
}

/// Check if any attribute in a slice is `#[ffi::skip]`
pub fn has_ffi_skip_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(is_ffi_skip_attribute)
}