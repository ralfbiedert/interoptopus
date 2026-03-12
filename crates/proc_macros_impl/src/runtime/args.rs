use syn::Field;

pub struct FieldAttrs {
    pub has_runtime_attr: bool,
}

impl FieldAttrs {
    pub fn from_field(field: &Field) -> Self {
        let has_runtime_attr = field.attrs.iter().any(|attr| attr.path().is_ident("runtime"));
        Self { has_runtime_attr }
    }
}
