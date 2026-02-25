//! Model pass — transforms the Rust inventory into the C language model.
//!
//! Unlike the C# backend which needs iterative convergence due to
//! managed/unmanaged type resolution, the C model can be built in a
//! single linear pass because all type mappings are straightforward.

use crate::lang::{
    CArray, CCallback, CEnumVariant, CField, CFnPointer, CFunction, CModel, COption, CPointer, CPrimitive, CResult, CSimpleEnum, CSlice, CStruct, CTaggedUnion,
    CTaggedUnionVariant, CType, CTypeKind, CVec,
};
use interoptopus::inventory::{RustInventory, TypeId};
use interoptopus::lang::function::Function;
use interoptopus::lang::types::{Layout, Primitive, Repr, Type, TypeKind, TypePattern, VariantKind};
use std::collections::{HashMap, HashSet};

/// Build the complete C model from a Rust inventory.
#[must_use]
pub fn build_model(inv: &RustInventory) -> CModel {
    let mut types = HashMap::new();

    // First pass: map every Rust type to a C type.
    for (&tid, ty) in &inv.types {
        if let Some(ctype) = map_type(inv, ty) {
            types.insert(tid, ctype);
        }
    }

    // Topological sort for emission order.
    let types_ordered = topo_sort(inv);

    // Map functions.
    let functions = build_functions(inv);

    CModel { types_ordered, types, functions }
}

/// Turn a Rust type name (e.g. `Option<Vec2>`) into a valid C identifier (`OPTIONVEC2`).
fn c_name(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

/// Resolve the C type name for a Rust `TypeId`.
fn type_name(inv: &RustInventory, tid: &TypeId) -> String {
    let ty = &inv.types[tid];
    match &ty.kind {
        TypeKind::Primitive(p) => map_primitive(*p).c_name().to_string(),
        TypeKind::ReadPointer(inner) => format!("const {}*", type_name(inv, inner)),
        TypeKind::ReadWritePointer(inner) => format!("{}*", type_name(inv, inner)),
        TypeKind::TypePattern(TypePattern::CChar) => "char".to_string(),
        TypeKind::TypePattern(TypePattern::Bool) => "bool".to_string(),
        TypeKind::TypePattern(TypePattern::CVoid) => "void".to_string(),
        _ => c_name(&ty.name),
    }
}

/// Resolve the C type specifier (maps void primitives to `"void"`).
fn type_spec(inv: &RustInventory, tid: &TypeId) -> String {
    let ty = &inv.types[tid];
    if matches!(ty.kind, TypeKind::Primitive(Primitive::Void)) {
        "void".to_string()
    } else {
        type_name(inv, tid)
    }
}

/// Map a Rust enum `Repr` to the corresponding fixed-width C integer type name.
///
/// The `#[ffi]` proc macro always assigns a `Layout::Primitive` repr to enums
/// (e.g. `u8`, `u32`), so the fallback branch should never be reached in
/// practice — it exists only as a defensive default.
fn repr_to_c_tag_type(repr: &Repr) -> String {
    match repr.layout {
        Layout::Primitive(p) => map_primitive(p).c_name().to_string(),
        _ => "uint32_t".to_string(),
    }
}

fn map_primitive(p: Primitive) -> CPrimitive {
    match p {
        Primitive::Void => CPrimitive::Void,
        Primitive::Bool => CPrimitive::Bool,
        Primitive::U8 => CPrimitive::U8,
        Primitive::U16 => CPrimitive::U16,
        Primitive::U32 => CPrimitive::U32,
        Primitive::U64 => CPrimitive::U64,
        Primitive::Usize => CPrimitive::Usize,
        Primitive::I8 => CPrimitive::I8,
        Primitive::I16 => CPrimitive::I16,
        Primitive::I32 => CPrimitive::I32,
        Primitive::I64 => CPrimitive::I64,
        Primitive::Isize => CPrimitive::Isize,
        Primitive::F32 => CPrimitive::F32,
        Primitive::F64 => CPrimitive::F64,
    }
}

fn map_type(inv: &RustInventory, ty: &Type) -> Option<CType> {
    let name = c_name(&ty.name);
    let kind = match &ty.kind {
        TypeKind::Primitive(p) => CTypeKind::Primitive(map_primitive(*p)),

        TypeKind::ReadPointer(inner) => CTypeKind::Pointer(CPointer { target_name: type_name(inv, inner), is_const: true }),

        TypeKind::ReadWritePointer(inner) => CTypeKind::Pointer(CPointer { target_name: type_name(inv, inner), is_const: false }),

        TypeKind::Struct(s) => {
            let fields = s
                .fields
                .iter()
                .map(|f| {
                    let resolved = &inv.types[&f.ty];
                    if let TypeKind::Array(arr) = &resolved.kind {
                        CField { name: f.name.clone(), type_name: type_spec(inv, &arr.ty), array_len: Some(arr.len) }
                    } else {
                        CField { name: f.name.clone(), type_name: type_spec(inv, &f.ty), array_len: None }
                    }
                })
                .collect();
            CTypeKind::Struct(CStruct { fields })
        }

        TypeKind::Enum(e) => {
            let tag_c_type = repr_to_c_tag_type(&e.repr);
            let has_data = e.variants.iter().any(|v| matches!(v.kind, VariantKind::Tuple(_)));
            if has_data {
                let tag_name = format!("{name}_TAG");
                let variants = e
                    .variants
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        let data_type = match &v.kind {
                            VariantKind::Tuple(tid) => Some(type_spec(inv, tid)),
                            VariantKind::Unit(_) => None,
                        };
                        CTaggedUnionVariant { name: format!("{}_{}", name, v.name.to_uppercase()), tag: i, data_type }
                    })
                    .collect();
                CTypeKind::TaggedUnion(CTaggedUnion { tag_name, variants, tag_c_type })
            } else {
                let variants = e
                    .variants
                    .iter()
                    .enumerate()
                    .map(|(i, v)| CEnumVariant { name: format!("{}_{}", name, v.name.to_uppercase()), value: i })
                    .collect();
                CTypeKind::SimpleEnum(CSimpleEnum { variants, tag_c_type })
            }
        }

        TypeKind::TypePattern(TypePattern::NamedCallback(sig)) => {
            let rval = type_spec(inv, &sig.rval);
            let mut params: Vec<String> = sig.arguments.iter().map(|a| type_spec(inv, &a.ty)).collect();
            params.push("const void*".to_string());
            CTypeKind::Callback(CCallback { fn_typedef: format!("{name}_fn"), rval, params: params.join(", ") })
        }

        TypeKind::TypePattern(TypePattern::Slice(inner)) => CTypeKind::Slice(CSlice { inner_type: type_spec(inv, inner), is_const: true }),

        TypeKind::TypePattern(TypePattern::SliceMut(inner)) => CTypeKind::SliceMut(CSlice { inner_type: type_spec(inv, inner), is_const: false }),

        TypeKind::TypePattern(TypePattern::Vec(inner)) => CTypeKind::Vec(CVec { inner_type: type_spec(inv, inner) }),

        TypeKind::TypePattern(TypePattern::Utf8String) => CTypeKind::Utf8String,

        TypeKind::TypePattern(TypePattern::Option(inner)) => {
            let tag_name = format!("{name}_TAG");
            // ffi::Option is #[repr(u32)]
            CTypeKind::Option(COption { tag_name, inner_type: type_spec(inv, inner), tag_c_type: "uint32_t".to_string() })
        }

        TypeKind::TypePattern(TypePattern::Result(ok, err)) => {
            let tag_name = format!("{name}_TAG");
            // ffi::Result is #[repr(u32)]
            CTypeKind::Result(CResult { tag_name, ok_type: type_spec(inv, ok), err_type: type_spec(inv, err), tag_c_type: "uint32_t".to_string() })
        }

        TypeKind::FnPointer(sig) => {
            let rval = type_spec(inv, &sig.rval);
            let params = if sig.arguments.is_empty() {
                "void".to_string()
            } else {
                sig.arguments.iter().map(|a| type_spec(inv, &a.ty)).collect::<Vec<_>>().join(", ")
            };
            CTypeKind::FnPointer(CFnPointer { rval, params })
        }

        TypeKind::Opaque | TypeKind::Service => CTypeKind::Opaque,

        TypeKind::Array(arr) => CTypeKind::Array(CArray { element_type: type_spec(inv, &arr.ty), len: arr.len }),

        TypeKind::TypePattern(TypePattern::CChar | TypePattern::Bool | TypePattern::CVoid) => CTypeKind::Primitive(match &ty.kind {
            TypeKind::TypePattern(TypePattern::CChar) => CPrimitive::I8,
            TypeKind::TypePattern(TypePattern::Bool) => CPrimitive::Bool,
            _ => CPrimitive::Void,
        }),

        _ => return None,
    };

    Some(CType { name, kind })
}

fn param_list(inv: &RustInventory, f: &Function) -> String {
    if f.signature.arguments.is_empty() {
        return "void".to_string();
    }
    f.signature
        .arguments
        .iter()
        .map(|a| format!("{} {}", type_spec(inv, &a.ty), a.name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn param_types(inv: &RustInventory, f: &Function) -> String {
    if f.signature.arguments.is_empty() {
        return "void".to_string();
    }
    f.signature.arguments.iter().map(|a| type_spec(inv, &a.ty)).collect::<Vec<_>>().join(", ")
}

fn build_functions(inv: &RustInventory) -> Vec<CFunction> {
    let mut fns: Vec<&Function> = inv.functions.values().collect();
    fns.sort_by_key(|f| &f.name);

    fns.iter()
        .map(|f| CFunction {
            name: f.name.clone(),
            rval: type_spec(inv, &f.signature.rval),
            params: param_list(inv, f),
            param_types: param_types(inv, f),
            is_internal: f.name.starts_with("interoptopus_"),
        })
        .collect()
}

// ── Topological sort ──

fn topo_sort(inv: &RustInventory) -> Vec<TypeId> {
    let mut visited = HashSet::new();
    let mut order = Vec::new();

    let mut tids: Vec<TypeId> = inv.types.keys().copied().collect();
    tids.sort_by(|a, b| inv.types[a].name.cmp(&inv.types[b].name));

    for tid in tids {
        visit(tid, inv, &mut visited, &mut order);
    }
    order
}

fn visit(tid: TypeId, inv: &RustInventory, visited: &mut HashSet<TypeId>, order: &mut Vec<TypeId>) {
    if !inv.types.contains_key(&tid) || !visited.insert(tid) {
        return;
    }
    for dep in deps(&inv.types[&tid]) {
        visit(dep, inv, visited, order);
    }
    order.push(tid);
}

fn deps(ty: &Type) -> Vec<TypeId> {
    match &ty.kind {
        TypeKind::Struct(s) => s.fields.iter().map(|f| f.ty).collect(),
        TypeKind::Enum(e) => e
            .variants
            .iter()
            .filter_map(|v| match &v.kind {
                VariantKind::Tuple(tid) => Some(*tid),
                VariantKind::Unit(_) => None,
            })
            .collect(),
        TypeKind::TypePattern(TypePattern::Slice(t) | TypePattern::SliceMut(t) | TypePattern::Vec(t) | TypePattern::Option(t)) => {
            vec![*t]
        }
        TypeKind::TypePattern(TypePattern::Result(ok, err)) => vec![*ok, *err],
        TypeKind::TypePattern(TypePattern::NamedCallback(sig)) | TypeKind::FnPointer(sig) => {
            let mut d: Vec<_> = sig.arguments.iter().map(|a| a.ty).collect();
            d.push(sig.rval);
            d
        }
        TypeKind::Array(arr) => vec![arr.ty],
        TypeKind::ReadPointer(t) | TypeKind::ReadWritePointer(t) => vec![*t],
        _ => vec![],
    }
}
