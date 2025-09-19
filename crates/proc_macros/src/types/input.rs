use syn::{Data, DeriveInput, Fields, Generics, Ident, Result, Visibility, Error};
use crate::types::args::FfiTypeArgs;

pub struct ParsedInput {
    pub ident: Ident,
    pub vis: Visibility,
    pub generics: Generics,
    pub data: InputData,
    pub args: FfiTypeArgs,
    pub docs: Vec<String>,
    pub original_input: DeriveInput,
}

pub enum InputData {
    Struct(StructData),
    Enum(EnumData),
}

pub struct StructData {
    pub fields: Vec<FieldData>,
}

pub struct EnumData {
    pub variants: Vec<VariantData>,
}

pub struct FieldData {
    pub ident: Option<Ident>,
    pub ty: syn::Type,
    pub vis: Visibility,
    pub skip: bool,
    pub docs: Vec<String>,
}

pub struct VariantData {
    pub ident: Ident,
    pub discriminant: Option<syn::Expr>,
    pub fields: Vec<FieldData>,
    pub docs: Vec<String>,
}

impl ParsedInput {
    pub fn from_derive_input(input: DeriveInput, args: FfiTypeArgs) -> Result<Self> {
        args.validate()?;

        let docs = extract_docs(&input.attrs);

        let data = match &input.data {
            Data::Struct(data_struct) => {
                let fields = extract_fields(&data_struct.fields)?;
                InputData::Struct(StructData { fields })
            }
            Data::Enum(data_enum) => {
                let mut variants = Vec::new();
                for variant in &data_enum.variants {
                    let variant_docs = extract_docs(&variant.attrs);
                    let fields = extract_fields(&variant.fields)?;
                    variants.push(VariantData {
                        ident: variant.ident.clone(),
                        discriminant: variant.discriminant.as_ref().map(|(_, expr)| expr.clone()),
                        fields,
                        docs: variant_docs,
                    });
                }
                InputData::Enum(EnumData { variants })
            }
            Data::Union(_) => {
                return Err(Error::new_spanned(&input, "ffi_type does not support unions"));
            }
        };

        let ident = input.ident.clone();
        let vis = input.vis.clone();
        let generics = input.generics.clone();

        Ok(ParsedInput {
            ident,
            vis,
            generics,
            data,
            args,
            docs,
            original_input: input,
        })
    }
}

fn extract_fields(fields: &Fields) -> Result<Vec<FieldData>> {
    let mut field_data = Vec::new();

    match fields {
        Fields::Named(fields_named) => {
            for field in &fields_named.named {
                let skip = field.attrs.iter().any(|attr| attr.path().is_ident("skip"));
                let docs = extract_docs(&field.attrs);
                field_data.push(FieldData {
                    ident: field.ident.clone(),
                    ty: field.ty.clone(),
                    vis: field.vis.clone(),
                    skip,
                    docs,
                });
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            for field in &fields_unnamed.unnamed {
                let skip = field.attrs.iter().any(|attr| attr.path().is_ident("skip"));
                let docs = extract_docs(&field.attrs);
                field_data.push(FieldData {
                    ident: None,
                    ty: field.ty.clone(),
                    vis: field.vis.clone(),
                    skip,
                    docs,
                });
            }
        }
        Fields::Unit => {
            // No fields
        }
    }

    Ok(field_data)
}

fn extract_docs(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut docs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta_name_value) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta_name_value.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        let doc_string = lit_str.value();
                        // Remove leading space if present
                        let trimmed = if doc_string.starts_with(' ') {
                            &doc_string[1..]
                        } else {
                            &doc_string
                        };
                        docs.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    docs
}