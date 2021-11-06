use crate::surrogates::read_surrogates;
use crate::types::Attributes;
use crate::util::extract_doc_lines;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{GenericParam, ItemStruct, Type};

#[derive(Debug, Copy, Clone)]
pub enum TypeRepr {
    C,
    Transparent,
    Opaque,
}

fn type_repr(attributes: &Attributes, item: &ItemStruct) -> TypeRepr {
    if attributes.opaque {
        return TypeRepr::Opaque;
    }

    let repr = item
        .attrs
        .iter()
        .find(|x| x.to_token_stream().to_string().contains("repr"))
        .unwrap_or_else(|| panic!("Struct {} must have `#[repr()] annotation.", item.ident));

    if repr.to_token_stream().to_string().contains("transparent") {
        TypeRepr::Transparent
    } else {
        TypeRepr::C
    }
}

// Various Struct examples
//
// ```
// pub struct S1<'a, T>
// where
//     T: 'static,
//     T: CTypeInfo,
// {
//     pub x: u32,
//     pub p: PhantomData<&'a T>,
// }
//
// pub struct S2<'a, T: Clone>
// {
//     pub x: u32,
//     pub p: PhantomData<&'a T>,
// }
//
// pub struct Tupled(pub u8);
//
// pub struct S3<const N: usize>
// {
//     pub f: [u32; N],
// }
//
// struct Weird2<'a, T: Clone, const N: usize>
//     where
//         T: Copy + Copy + 'a,
//         T: Debug,
// {
//     t: &'a T,
//     a: [T; N],
//     r: &'a u8,
// }
//
// ```
//
pub fn ffi_type_struct(attributes: &Attributes, input: TokenStream, item: ItemStruct) -> TokenStream {
    let namespace = attributes.namespace.clone().unwrap_or_else(|| "".to_string());
    let doc_line = extract_doc_lines(&item.attrs).join("\n");

    let type_repr = type_repr(attributes, &item);

    let struct_ident_str = item.ident.to_string();
    let struct_ident = syn::Ident::new(&struct_ident_str, item.ident.span());
    let struct_ident_c = attributes.name.clone().unwrap_or(struct_ident_str);
    let surrogates = read_surrogates(&item.attrs);

    let mut field_names = Vec::new();
    let mut field_type_info = Vec::new();
    let mut field_types = Vec::new();
    let mut field_docs = Vec::new();
    let mut field_visibilities = Vec::new();

    let mut has_generics = false;
    let mut generic_params_needing_ctypeinfo_bounds = Vec::new();
    let mut generic_parameter_tokens = Vec::new();
    let mut generic_struct_tokens = Vec::new();
    let mut generic_where_tokens = Vec::new();

    for generic in &item.generics.params {
        match generic {
            GenericParam::Lifetime(lt) => {
                let ident = lt.lifetime.ident.clone();
                let lt = syn::Lifetime::new(&format!("'{}", ident.to_string()), item.span());
                generic_parameter_tokens.push(quote! { #lt });
                generic_struct_tokens.push(quote! { #lt });
            }
            GenericParam::Type(ty) => {
                let ident = ty.ident.clone();
                let whre = ty.bounds.to_token_stream();
                generic_parameter_tokens.push(quote! { #ident });
                generic_struct_tokens.push(quote! { #ident });
                generic_params_needing_ctypeinfo_bounds.push(ty.ident.clone());
                generic_where_tokens.push(quote! { #ident: interoptopus::lang::rust::CTypeInfo });
                if !whre.to_string().is_empty() {
                    generic_where_tokens.push(quote! { #ident: #whre });
                }
            }
            GenericParam::Const(x) => {
                let ident = x.ident.clone();
                let ty = x.ty.to_token_stream();
                generic_parameter_tokens.push(quote! { const #ident: #ty });
                generic_struct_tokens.push(quote! { #ident });
            }
        }

        has_generics = true;
    }

    if let Some(whre) = item.generics.where_clause {
        for pred in whre.predicates {
            generic_where_tokens.push(quote! { #pred });
        }
    }

    let mut param_param = quote! {};
    let mut param_struct = quote! {};
    let mut param_where = quote! {};

    if has_generics {
        param_param = quote! { < #(#generic_parameter_tokens),* > };
        param_struct = quote! { < #(#generic_struct_tokens),* > };
        param_where = quote! { where #(#generic_where_tokens),*  };
    }

    for (i, field) in item.fields.iter().enumerate() {
        let name = field.ident.as_ref().map(|x| x.to_string()).unwrap_or_else(|| format!("x{}", i.to_string()));

        if attributes.skip.contains_key(&name) {
            continue;
        }

        let visibility = attributes.visibility_for_field(field, &name);

        field_names.push(name.clone());
        field_docs.push(extract_doc_lines(&field.attrs).join("\n"));
        field_visibilities.push(visibility);

        let token = match &field.ty {
            Type::Path(x) => {
                if let Some(qself) = &x.qself {
                    let ty = &qself.ty;

                    // Ok, I really don't know if this is kosher. After some debugging it seems to work,
                    // but this is probably brittle AF.

                    let first = x.path.segments.first().expect("Must have last path segment.");
                    let middle = x.path.segments.iter().skip(1).rev().skip(1).rev().collect::<Vec<_>>();
                    let last = x.path.segments.last().expect("Must have last path segment.");
                    quote! { < #ty as #first #(:: #middle)*> :: #last }
                } else {
                    x.path.to_token_stream()
                }
            }
            Type::Ptr(x) => x.to_token_stream(),
            Type::Array(x) => x.to_token_stream(),
            Type::Reference(x) => x.to_token_stream(),
            _ => {
                panic!("Unknown token: {:?}", field);
            }
        };

        if surrogates.1.contains_key(&name) {
            let lookup = surrogates.1.get(&name).unwrap();
            let ident = syn::Ident::new(&lookup, surrogates.0.unwrap());
            field_type_info.push(quote! { #ident()  });
            field_types.push(quote! { #ident()  }); // TODO: are these 2 correct?
        } else {
            field_type_info.push(quote! { < #token as ::interoptopus::lang::rust::CTypeInfo >::type_info()  });
            field_types.push(quote! { #token });
        }
    }

    let rval_builder = if attributes.opaque {
        quote! {
            let mut rval = ::interoptopus::lang::c::OpaqueType::new(name, meta);
            ::interoptopus::lang::c::CType::Opaque(rval)
        }
    } else {
        quote! {
            let rval = ::interoptopus::lang::c::CompositeType::with_meta(name, fields, meta);
            ::interoptopus::lang::c::CType::Composite(rval)
        }
    };

    let let_fields = if attributes.opaque {
        quote! {}
    } else {
        quote! {
            #({
                let documentation = ::interoptopus::lang::c::Documentation::from_line(#field_docs);
                let the_type = #field_type_info;
                let field = ::interoptopus::lang::c::Field::with_documentation(#field_names.to_string(), the_type, #field_visibilities, documentation);
                fields.push(field);
            })*
        }
    };

    let let_name = if let Some(name) = &attributes.name {
        quote! {
            let name = #name.to_string();
        }
    } else {
        quote! {
            #({
                generics.push(<#generic_params_needing_ctypeinfo_bounds as ::interoptopus::lang::rust::CTypeInfo>::type_info().name_within_lib());
            })*

            let name = format!("{}{}", #struct_ident_c.to_string(), generics.join(""));
        }
    };

    match type_repr {
        TypeRepr::C | TypeRepr::Opaque => {
            quote! {
                #input

                unsafe impl #param_param ::interoptopus::lang::rust::CTypeInfo for #struct_ident #param_struct #param_where {

                    fn type_info() -> ::interoptopus::lang::c::CType {
                        let documentation = ::interoptopus::lang::c::Documentation::from_line(#doc_line);
                        let mut meta = ::interoptopus::lang::c::Meta::with_namespace_documentation(#namespace.to_string(), documentation);
                        let mut fields: ::std::vec::Vec<interoptopus::lang::c::Field> = ::std::vec::Vec::new();
                        let mut generics: ::std::vec::Vec<String> = ::std::vec::Vec::new();

                        #let_name

                        #let_fields

                        #rval_builder
                    }
                }
            }
        }
        TypeRepr::Transparent => {
            let first_field_type = field_types.get(0).expect("Transparent structs must have at least one field");

            quote! {
                #input

                unsafe impl #param_param ::interoptopus::lang::rust::CTypeInfo for #struct_ident #param_struct #param_where {

                    fn type_info() -> ::interoptopus::lang::c::CType {
                        < #first_field_type > :: type_info()
                    }
                }
            }
        }
    }
}
