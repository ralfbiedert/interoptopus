use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;

use crate::types::model::{TypeData, TypeModel, VariantData};

/// Compute the wire discriminant tag for each variant of an enum.
/// Mirrors Rust auto-numbering: explicit discriminants reset the counter,
/// auto-numbered variants are previous + 1. This matches what the C# backend
/// uses on its side (unit variants use Rust auto/explicit values; tuple variants
/// also fall out of auto-numbering when starting at 0 with no explicit values).
fn compute_variant_tags(enum_data: &crate::types::model::EnumData) -> Vec<isize> {
    let mut next_auto: isize = 0;
    let mut tags = Vec::with_capacity(enum_data.variants.len());
    for v in &enum_data.variants {
        let value = if let Some(expr) = &v.discriminant {
            match crate::types::discriminant::try_eval(expr) {
                Some(val) => {
                    next_auto = val + 1;
                    val
                }
                None => {
                    let val = next_auto;
                    next_auto += 1;
                    val
                }
            }
        } else {
            let val = next_auto;
            next_auto += 1;
            val
        };
        tags.push(value);
    }
    tags
}

impl TypeModel {
    pub fn emit_wireio_impl(&self) -> TokenStream {
        let name = &self.name;
        let generics = &self.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let write_impl = self.emit_wireio_write();
        let read_impl = self.emit_wireio_read();
        let live_size_impl = self.emit_wireio_live_size();

        // Use underscore prefix for params if we won't use them (enum, opaque, service, or has skipped fields)
        let has_skipped_fields = match &self.data {
            TypeData::Struct(struct_data) => struct_data.fields.iter().any(|f| f.skip),
            TypeData::Enum(_) => false,
        };

        let is_unsupported_enum = false;

        let use_underscore_params = is_unsupported_enum || self.args.opaque || self.args.service || has_skipped_fields;

        let (write_param, read_param) = if use_underscore_params {
            (quote_spanned! { name.span() => _out }, quote_spanned! { name.span() => _input })
        } else {
            (quote_spanned! { name.span() => out }, quote_spanned! { name.span() => input })
        };

        // Build where clause with WireIO bounds for all field types
        let wireio_where_clause = self.build_wireio_where_clause(where_clause);

        quote_spanned! { name.span() =>
            #[allow(clippy::used_underscore_binding, clippy::type_repetition_in_bounds)]
            unsafe impl #impl_generics ::interoptopus::lang::types::WireIO for #name #ty_generics #wireio_where_clause {
                fn write(&self, #write_param: &mut impl ::std::io::Write) -> ::std::result::Result<(), ::interoptopus::wire::SerializationError> {
                    #write_impl
                }

                fn read(#read_param: &mut impl ::std::io::Read) -> ::std::result::Result<Self, ::interoptopus::wire::SerializationError>
                where
                    Self: Sized
                {
                    #read_impl
                }

                fn live_size(&self) -> usize {
                    #live_size_impl
                }
            }
        }
    }

    fn build_wireio_where_clause(&self, existing_where: Option<&syn::WhereClause>) -> TokenStream {
        // For opaque and service types, inner fields are not emitted, so we don't add WireIO bounds for them
        if self.args.opaque || self.args.service {
            return existing_where.map_or_else(|| quote_spanned! { self.name.span() => }, |w| quote_spanned! { self.name.span() => #w });
        }

        match &self.data {
            TypeData::Struct(struct_data) => {
                // If there are skipped fields, we don't generate WireIO, so no extra where clause needed
                if struct_data.fields.iter().any(|f| f.skip) {
                    return existing_where.map_or_else(|| quote_spanned! { self.name.span() => }, |w| quote_spanned! { self.name.span() => #w });
                }

                let field_bounds: Vec<_> = struct_data
                    .fields
                    .iter()
                    .filter(|f| !f.skip)
                    .map(|field| {
                        let ty = &field.ty;
                        let span = field.name.as_ref().map_or_else(|| ty.span(), syn::Ident::span);
                        quote_spanned! { span => #ty: ::interoptopus::lang::types::WireIO, }
                    })
                    .collect();

                if let Some(existing_where) = existing_where {
                    let existing_predicates = &existing_where.predicates;
                    if field_bounds.is_empty() {
                        quote_spanned! { self.name.span() => where #existing_predicates }
                    } else {
                        quote_spanned! { self.name.span() => where #existing_predicates #(#field_bounds)* }
                    }
                } else if field_bounds.is_empty() {
                    quote_spanned! { self.name.span() => }
                } else {
                    quote_spanned! { self.name.span() => where #(#field_bounds)* }
                }
            }
            TypeData::Enum(enum_data) => {
                let field_bounds: Vec<_> = enum_data
                    .variants
                    .iter()
                    .filter_map(|v| match &v.data {
                        VariantData::Unit => None,
                        VariantData::Tuple(ty) => Some(quote_spanned! { v.name.span() => #ty: ::interoptopus::lang::types::WireIO, }),
                    })
                    .collect();

                if let Some(existing_where) = existing_where {
                    let existing_predicates = &existing_where.predicates;
                    if field_bounds.is_empty() {
                        quote_spanned! { self.name.span() => where #existing_predicates }
                    } else {
                        quote_spanned! { self.name.span() => where #existing_predicates #(#field_bounds)* }
                    }
                } else if field_bounds.is_empty() {
                    quote_spanned! { self.name.span() => }
                } else {
                    quote_spanned! { self.name.span() => where #(#field_bounds)* }
                }
            }
        }
    }

    fn emit_wireio_write(&self) -> TokenStream {
        // For opaque and service types, fields aren't exposed, so can't serialize
        if self.args.opaque || self.args.service {
            return quote_spanned! { self.name.span() =>
                ::interoptopus::bad_wire!()
            };
        }

        match &self.data {
            TypeData::Struct(struct_data) => {
                // If there are skipped fields, we can't properly serialize/deserialize
                if struct_data.fields.iter().any(|f| f.skip) {
                    return quote_spanned! { self.name.span() =>
                        ::interoptopus::bad_wire!()
                    };
                }

                let is_packed = self.args.packed;

                let field_writes = struct_data.fields.iter().filter(|f| !f.skip).enumerate().map(|(index, field)| {
                    let ty = &field.ty;
                    let span = field.name.as_ref().map_or_else(|| ty.span(), syn::Ident::span);

                    let field_access = if let Some(name) = &field.name {
                        quote_spanned! { span => self.#name }
                    } else {
                        let idx = syn::Index::from(index);
                        quote_spanned! { span => self.#idx }
                    };

                    if is_packed {
                        // For packed structs, read the value to avoid taking a reference to an unaligned field
                        quote_spanned! { span =>
                            {
                                let __value = #field_access;
                                <#ty as ::interoptopus::lang::types::WireIO>::write(&__value, out)?;
                            }
                        }
                    } else {
                        quote_spanned! { span =>
                            <#ty as ::interoptopus::lang::types::WireIO>::write(&#field_access, out)?;
                        }
                    }
                });

                quote_spanned! { self.name.span() =>
                    #(#field_writes)*
                    ::std::result::Result::Ok(())
                }
            }
            TypeData::Enum(enum_data) => {
                let name = &self.name;
                let wire_ty = crate::types::discriminant::wire_type_tokens(&enum_data.discriminant, self.name.span());
                let tags = compute_variant_tags(enum_data);
                let arms = enum_data.variants.iter().zip(tags.iter()).map(|(v, tag)| {
                    let vname = &v.name;
                    let tag_lit = proc_macro2::Literal::isize_unsuffixed(*tag);
                    match &v.data {
                        VariantData::Unit => quote_spanned! { vname.span() =>
                            #name::#vname => {
                                <#wire_ty as ::interoptopus::lang::types::WireIO>::write(&(#tag_lit as #wire_ty), out)?;
                            }
                        },
                        VariantData::Tuple(ty) => quote_spanned! { vname.span() =>
                            #name::#vname(__inner) => {
                                <#wire_ty as ::interoptopus::lang::types::WireIO>::write(&(#tag_lit as #wire_ty), out)?;
                                <#ty as ::interoptopus::lang::types::WireIO>::write(__inner, out)?;
                            }
                        },
                    }
                });

                quote_spanned! { self.name.span() =>
                    match self {
                        #(#arms)*
                    }
                    ::std::result::Result::Ok(())
                }
            }
        }
    }

    fn emit_wireio_read(&self) -> TokenStream {
        // For opaque and service types, fields aren't exposed, so can't deserialize
        if self.args.opaque || self.args.service {
            return quote_spanned! { self.name.span() =>
                ::interoptopus::bad_wire!()
            };
        }

        match &self.data {
            TypeData::Struct(struct_data) => {
                // If there are skipped fields, we can't properly serialize/deserialize
                if struct_data.fields.iter().any(|f| f.skip) {
                    return quote_spanned! { self.name.span() =>
                        ::interoptopus::bad_wire!()
                    };
                }

                let field_reads = struct_data.fields.iter().filter(|f| !f.skip).enumerate().map(|(index, field)| {
                    let ty = &field.ty;
                    let span = field.name.as_ref().map_or_else(|| ty.span(), syn::Ident::span);

                    let field_name = if let Some(name) = &field.name {
                        quote_spanned! { span => #name }
                    } else {
                        let idx = syn::Index::from(index);
                        let field_ident = quote::format_ident!("field_{}", idx);
                        quote_spanned! { span => #field_ident }
                    };

                    quote_spanned! { span =>
                        let #field_name = <#ty as ::interoptopus::lang::types::WireIO>::read(input)?;
                    }
                });

                let struct_construction = if struct_data.fields.is_empty() {
                    quote_spanned! { self.name.span() =>
                        ::std::result::Result::Ok(Self)
                    }
                } else if struct_data.fields.iter().any(|f| f.name.is_some()) {
                    // Named fields
                    let field_names = struct_data.fields.iter().filter(|f| !f.skip).map(|field| {
                        let name = field.name.as_ref().unwrap();
                        let span = name.span();
                        quote_spanned! { span => #name }
                    });

                    quote_spanned! { self.name.span() =>
                        ::std::result::Result::Ok(Self { #(#field_names),* })
                    }
                } else {
                    // Unnamed fields
                    let field_names = struct_data.fields.iter().filter(|f| !f.skip).enumerate().map(|(index, _field)| {
                        let idx = syn::Index::from(index);
                        let field_ident = quote::format_ident!("field_{}", idx);
                        quote_spanned! { self.name.span() => #field_ident }
                    });

                    quote_spanned! { self.name.span() =>
                        ::std::result::Result::Ok(Self(#(#field_names),*))
                    }
                };

                quote_spanned! { self.name.span() =>
                    #(#field_reads)*
                    #struct_construction
                }
            }
            TypeData::Enum(enum_data) => {
                let name = &self.name;
                let wire_ty = crate::types::discriminant::wire_type_tokens(&enum_data.discriminant, self.name.span());
                let tags = compute_variant_tags(enum_data);
                let arms = enum_data.variants.iter().zip(tags.iter()).map(|(v, tag)| {
                    let vname = &v.name;
                    let tag_lit = proc_macro2::Literal::isize_unsuffixed(*tag);
                    match &v.data {
                        VariantData::Unit => quote_spanned! { vname.span() =>
                            x if x == (#tag_lit as #wire_ty) => ::std::result::Result::Ok(#name::#vname),
                        },
                        VariantData::Tuple(ty) => quote_spanned! { vname.span() =>
                            x if x == (#tag_lit as #wire_ty) => {
                                let __inner = <#ty as ::interoptopus::lang::types::WireIO>::read(input)?;
                                ::std::result::Result::Ok(#name::#vname(__inner))
                            }
                        },
                    }
                });

                quote_spanned! { self.name.span() =>
                    let __discriminant = <#wire_ty as ::interoptopus::lang::types::WireIO>::read(input)?;
                    match __discriminant {
                        #(#arms)*
                        _ => ::std::result::Result::Err(::interoptopus::wire::SerializationError::invalid_discriminant(
                            stringify!(#name),
                            __discriminant as isize,
                        )),
                    }
                }
            }
        }
    }

    fn emit_wireio_live_size(&self) -> TokenStream {
        // For opaque and service types, fields aren't exposed, so can't compute size
        if self.args.opaque || self.args.service {
            return quote_spanned! { self.name.span() =>
                ::interoptopus::bad_wire!()
            };
        }

        match &self.data {
            TypeData::Struct(struct_data) => {
                // If there are skipped fields, we can't properly serialize/deserialize
                if struct_data.fields.iter().any(|f| f.skip) {
                    return quote_spanned! { self.name.span() =>
                        ::interoptopus::bad_wire!()
                    };
                }

                let is_packed = self.args.packed;

                let field_sizes = struct_data.fields.iter().filter(|f| !f.skip).enumerate().map(|(index, field)| {
                    let ty = &field.ty;
                    let span = field.name.as_ref().map_or_else(|| ty.span(), syn::Ident::span);

                    let field_access = if let Some(name) = &field.name {
                        quote_spanned! { span => self.#name }
                    } else {
                        let idx = syn::Index::from(index);
                        quote_spanned! { span => self.#idx }
                    };

                    if is_packed {
                        // For packed structs, read the value to avoid taking a reference to an unaligned field
                        quote_spanned! { span =>
                            ({
                                let __value = #field_access;
                                <#ty as ::interoptopus::lang::types::WireIO>::live_size(&__value)
                            })
                        }
                    } else {
                        quote_spanned! { span =>
                            <#ty as ::interoptopus::lang::types::WireIO>::live_size(&#field_access)
                        }
                    }
                });

                let sizes: Vec<_> = field_sizes.collect();
                if sizes.is_empty() {
                    quote_spanned! { self.name.span() => 0 }
                } else {
                    quote_spanned! { self.name.span() => #(#sizes)+* }
                }
            }
            TypeData::Enum(enum_data) => {
                let name = &self.name;
                let wire_ty = crate::types::discriminant::wire_type_tokens(&enum_data.discriminant, self.name.span());
                let has_tuple = enum_data.variants.iter().any(|v| matches!(v.data, VariantData::Tuple(_)));
                if !has_tuple {
                    quote_spanned! { self.name.span() =>
                        ::std::mem::size_of::<#wire_ty>()
                    }
                } else {
                    let arms = enum_data.variants.iter().map(|v| {
                        let vname = &v.name;
                        match &v.data {
                            VariantData::Unit => quote_spanned! { vname.span() =>
                                #name::#vname => 0,
                            },
                            VariantData::Tuple(ty) => quote_spanned! { vname.span() =>
                                #name::#vname(__inner) => <#ty as ::interoptopus::lang::types::WireIO>::live_size(__inner),
                            },
                        }
                    });
                    quote_spanned! { self.name.span() =>
                        ::std::mem::size_of::<#wire_ty>() + match self {
                            #(#arms)*
                        }
                    }
                }
            }
        }
    }
}
