use crate::docs::extract_docs;
use crate::types::args::FfiTypeArgs;
use crate::utils::has_ffi_skip_attribute;
use syn::{Data, DeriveInput, Fields, Generics, Ident, Type, Visibility};

#[derive(Clone)]
pub struct TypeModel {
    pub name: Ident,
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
#[allow(unused)]
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
                            let skip = has_ffi_skip_attribute(&field.attrs);
                            FieldModel { name: field.ident, ty: field.ty, vis: field.vis, skip, docs: extract_docs(&field.attrs) }
                        })
                        .collect(),
                    Fields::Unnamed(fields) => fields
                        .unnamed
                        .into_iter()
                        .map(|field| {
                            let skip = has_ffi_skip_attribute(&field.attrs);
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

        let model = Self { name: input.ident, generics: input.generics, data, args, docs };

        Ok(model)
    }
}
