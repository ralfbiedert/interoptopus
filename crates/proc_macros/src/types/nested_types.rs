//use quote::ToTokens;
//use syn::{GenericArgument, Type, TypeArray, TypePath, TypePtr, TypeReference, TypeSlice, TypeTuple};

// Recursively extract all type names from a syn::Type
// pub fn extract_wire_type_names(ty: &Type) -> Vec<String> {
//     let mut type_names = Vec::new();

//     match ty {
//         Type::Path(TypePath { path, .. }) => {
//             // Extract the main type name (last segment)
//             if let Some(last_segment) = path.segments.last() {
//                 type_names.push(last_segment.ident.to_string());

//                 // Process generic arguments recursively
//                 if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
//                     for arg in &args.args {
//                         if let GenericArgument::Type(inner_ty) = arg {
//                             type_names.extend(extract_wire_type_names(inner_ty));
//                         }
//                     }
//                 }
//             }

//             // Also extract intermediate path segments for fully qualified paths
//             for segment in &path.segments {
//                 if segment != path.segments.last().unwrap() {
//                     type_names.push(segment.ident.to_string());
//                 }
//             }
//         }
//         Type::Array(TypeArray { elem, .. }) => {
//             type_names.extend(extract_wire_type_names(elem));
//         }
//         Type::Ptr(TypePtr { elem, .. }) => {
//             type_names.extend(extract_wire_type_names(elem));
//         }
//         Type::Reference(TypeReference { elem, .. }) => {
//             type_names.extend(extract_wire_type_names(elem));
//         }
//         Type::Slice(TypeSlice { elem, .. }) => {
//             type_names.extend(extract_wire_type_names(elem));
//         }
//         Type::Tuple(TypeTuple { elems, .. }) => {
//             for elem in elems {
//                 type_names.extend(extract_wire_type_names(elem));
//             }
//         }
//         // Add other type variants as needed TODO
//         _ => {
//             // For unsupported types, just use the token stream as fallback
//             type_names.push(ty.to_token_stream().to_string());
//         }
//     }

//     // Remove duplicates while preserving order
//     let mut unique_names = Vec::new();
//     for name in type_names {
//         if !unique_names.contains(&name) {
//             unique_names.push(name);
//         }
//     }

//     unique_names
// }
