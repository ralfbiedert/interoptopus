use syn::Field;

pub struct FieldAttrs {
    pub forward: bool,
}

impl FieldAttrs {
    pub fn from_field(field: &Field) -> syn::Result<Self> {
        let mut forward = false;

        for attr in &field.attrs {
            if attr.path().is_ident("runtime") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("forward") {
                        forward = true;
                        Ok(())
                    } else {
                        Err(meta.error("Unknown runtime attribute; expected 'forward'"))
                    }
                })?;
            }
        }

        Ok(Self { forward })
    }
}
