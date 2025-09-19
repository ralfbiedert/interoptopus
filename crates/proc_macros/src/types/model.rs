use syn::{Data, DeriveInput, Fields, Generics, Ident, Type, Visibility};

use crate::types::args::FfiTypeArgs;

#[derive(Clone)]
pub struct TypeModel {
    pub name: Ident,
    pub vis: Visibility,
    pub generics: Generics,
    pub data: TypeData,
    pub args: FfiTypeArgs,
    pub docs: Vec<String>,
}

#[derive(Clone)]
pub enum TypeData {
    Struct(StructData),
    Enum(EnumData),
}

#[derive(Clone)]
pub struct StructData {
    pub fields: Vec<FieldModel>,
}

#[derive(Clone)]
pub struct EnumData {
    pub variants: Vec<VariantModel>,
}

#[derive(Clone)]
pub struct FieldModel {
    pub name: Option<Ident>,
    pub ty: Type,
    pub vis: Visibility,
    pub skip: bool,
    pub docs: Vec<String>,
}

#[derive(Clone)]
pub struct VariantModel {
    pub name: Ident,
    pub data: VariantData,
    pub discriminant: Option<syn::Expr>,
    pub docs: Vec<String>,
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum VariantData {
    Unit,
    Tuple(Type),
}

impl TypeModel {
    pub fn from_derive_input(input: DeriveInput, args: FfiTypeArgs) -> syn::Result<Self> {
        let docs = extract_docs(&input.attrs);

        let data = match input.data {
            Data::Struct(data_struct) => {
                let fields = match data_struct.fields {
                    Fields::Named(fields) => fields
                        .named
                        .into_iter()
                        .map(|field| {
                            let skip = field.attrs.iter().any(|attr| attr.path().is_ident("skip"));
                            FieldModel { name: field.ident, ty: field.ty, vis: field.vis, skip, docs: extract_docs(&field.attrs) }
                        })
                        .collect(),
                    Fields::Unnamed(fields) => fields
                        .unnamed
                        .into_iter()
                        .map(|field| {
                            let skip = field.attrs.iter().any(|attr| attr.path().is_ident("skip"));
                            FieldModel { name: None, ty: field.ty, vis: field.vis, skip, docs: extract_docs(&field.attrs) }
                        })
                        .collect(),
                    Fields::Unit => vec![],
                };

                TypeData::Struct(StructData { fields })
            }
            Data::Enum(data_enum) => {
                let variants = data_enum
                    .variants
                    .into_iter()
                    .map(|variant| {
                        let data = match variant.fields {
                            Fields::Unit => VariantData::Unit,
                            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => VariantData::Tuple(fields.unnamed.into_iter().next().unwrap().ty),
                            Fields::Unnamed(_) => return Err(syn::Error::new_spanned(variant, "Tuple variants with multiple fields are not supported")),
                            Fields::Named(_) => return Err(syn::Error::new_spanned(variant, "Struct variants are not supported")),
                        };

                        Ok(VariantModel { name: variant.ident, data, discriminant: variant.discriminant.map(|(_, expr)| expr), docs: extract_docs(&variant.attrs) })
                    })
                    .collect::<syn::Result<Vec<_>>>()?;

                TypeData::Enum(EnumData { variants })
            }
            Data::Union(_) => return Err(syn::Error::new_spanned(input, "Unions are not supported")),
        };

        let model = Self { name: input.ident, vis: input.vis, generics: input.generics, data, args, docs };

        // Validate that empty types are not allowed unless opaque
        model.validate_non_empty()?;

        Ok(model)
    }

    fn validate_non_empty(&self) -> syn::Result<()> {
        match &self.data {
            TypeData::Struct(struct_data) => {
                // Count non-skipped fields
                let non_skipped_fields = struct_data.fields.iter().filter(|f| !f.skip).count();

                if non_skipped_fields == 0 && !self.args.opaque {
                    return Err(syn::Error::new_spanned(
                        &self.name,
                        "Empty structs and unit structs are not allowed unless marked as 'opaque'. Use #[ffi_type(opaque)] for types without meaningful fields."
                    ));
                }
            }
            TypeData::Enum(enum_data) => {
                if enum_data.variants.is_empty() {
                    return Err(syn::Error::new_spanned(
                        &self.name,
                        "Enums without variants are not allowed"
                    ));
                }
            }
        }

        Ok(())
    }
}

fn extract_docs(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(meta) = &attr.meta {
                    if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) = &meta.value {
                        Some(lit_str.value().trim().to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}
