use crate::runtime::args::FieldAttrs;
use syn::{DeriveInput, Field, Fields, Generics, Ident, Type};

pub struct RuntimeModel {
    pub name: Ident,
    pub generics: Generics,
    pub forward_field: ForwardField,
}

pub struct ForwardField {
    pub name: Ident,
    pub ty: Type,
}

impl RuntimeModel {
    pub fn from_derive_input(input: DeriveInput) -> syn::Result<Self> {
        let name = input.ident;
        let generics = input.generics;

        let fields = match input.data {
            syn::Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => fields.named.into_iter().collect::<Vec<_>>(),
                Fields::Unnamed(_) => {
                    return Err(syn::Error::new_spanned(
                        name,
                        "AsyncRuntime can only be derived for structs with named fields",
                    ))
                }
                Fields::Unit => {
                    return Err(syn::Error::new_spanned(
                        name,
                        "AsyncRuntime can only be derived for structs with named fields",
                    ))
                }
            },
            syn::Data::Enum(_) => {
                return Err(syn::Error::new_spanned(name, "AsyncRuntime can only be derived for structs"))
            }
            syn::Data::Union(_) => {
                return Err(syn::Error::new_spanned(name, "AsyncRuntime can only be derived for structs"))
            }
        };

        let forward_field = Self::find_forward_field(&fields)?;

        Ok(Self { name, generics, forward_field })
    }

    fn find_forward_field(fields: &[Field]) -> syn::Result<ForwardField> {
        let mut forward_fields = Vec::new();

        for field in fields {
            let attrs = FieldAttrs::from_field(field)?;
            if attrs.forward {
                let name = field.ident.clone().ok_or_else(|| {
                    syn::Error::new_spanned(field, "Expected named field with #[runtime::forward]")
                })?;
                forward_fields.push(ForwardField { name, ty: field.ty.clone() });
            }
        }

        match forward_fields.len() {
            0 => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "AsyncRuntime requires exactly one field marked with #[runtime::forward]",
            )),
            1 => Ok(forward_fields.into_iter().next().unwrap()),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "AsyncRuntime requires exactly one field marked with #[runtime::forward], found multiple",
            )),
        }
    }
}
