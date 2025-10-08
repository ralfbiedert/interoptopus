use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;

use crate::types::model::{TypeData, TypeModel};

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

        let use_underscore_params = matches!(&self.data, TypeData::Enum(_)) || self.args.opaque || self.args.service || has_skipped_fields;

        let (write_param, read_param) = if use_underscore_params {
            (quote_spanned! { name.span() => _out }, quote_spanned! { name.span() => _input })
        } else {
            (quote_spanned! { name.span() => out }, quote_spanned! { name.span() => input })
        };

        // Build where clause with WireIO bounds for all field types
        let wireio_where_clause = self.build_wireio_where_clause(where_clause);

        quote_spanned! { name.span() =>
            impl #impl_generics ::interoptopus::lang::types::WireIO for #name #ty_generics #wireio_where_clause {
                fn write(&self, #write_param: &mut impl ::std::io::Write) -> ::std::result::Result<(), ::interoptopus::lang::types::SerializationError> {
                    #write_impl
                }

                fn read(#read_param: &mut impl ::std::io::Read) -> ::std::result::Result<Self, ::interoptopus::lang::types::SerializationError>
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
            TypeData::Enum(_) => {
                // For enums, just use existing where clause
                existing_where.map_or_else(|| quote_spanned! { self.name.span() => }, |w| quote_spanned! { self.name.span() => #w })
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
            TypeData::Enum(_) => {
                quote_spanned! { self.name.span() =>
                    ::interoptopus::bad_wire!()
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
            TypeData::Enum(_) => {
                quote_spanned! { self.name.span() =>
                    ::interoptopus::bad_wire!()
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
            TypeData::Enum(_) => {
                quote_spanned! { self.name.span() =>
                    ::interoptopus::bad_wire!()
                }
            }
        }
    }
}
