use syn::Field;

pub struct FieldAttrs {
    pub has_runtime_attr: bool,
}

impl FieldAttrs {
    pub fn from_field(field: &Field) -> Self {
        let mut has_runtime_attr = false;

        for attr in &field.attrs {
            // Check for #[runtime] attribute (simple identifier, not a path)
            if attr.path().is_ident("runtime") {
                has_runtime_attr = true;
            }
        }

        Self { has_runtime_attr }
    }
}
