use crate::model::TypeId;
use crate::Error;
use std::cmp::PartialEq;

/// Extract the inner data from a [`TypeKind`](interoptopus::lang::types::TypeKind) variant,
/// or `continue` if the type doesn't match.
///
/// ```ignore
/// let rust_array = try_extract_kind!(ty, Array);
/// ```
macro_rules! try_extract_kind {
    ($ty:expr, $variant:ident) => {
        match &$ty.kind {
            interoptopus::lang::types::TypeKind::$variant(x) => x,
            _ => continue,
        }
    };
}

/// Skip this loop iteration if `id_map` already has a mapping for `rust_id`.
///
/// ```ignore
/// skip_mapped!(id_map, rust_id);
/// ```
macro_rules! skip_mapped {
    ($id_map:expr, $rust_id:expr) => {
        if $id_map.ty(*$rust_id).is_some() {
            continue;
        }
    };
}

/// Unwrap an `Option`, or report a [`MissingItem`] to the [`PassMeta`] lost-and-found
/// and `continue` the enclosing loop.
///
/// Passes run repeatedly until convergence. When a dependency hasn't been resolved yet
/// (e.g. a Rust type has no C# mapping), this macro records the gap so the orchestrator
/// knows another iteration is needed, then skips the current item.
///
/// ```ignore
/// let cs_ty = try_resolve!(id_map.ty(rust_ty), pass_meta, self.info, MissingItem::RustType(rust_ty));
/// ```
macro_rules! try_resolve {
    ($option:expr, $pass_meta:expr, $info:expr, $missing:expr) => {
        match $option {
            Some(val) => val,
            None => {
                $pass_meta.lost_found.missing($info, $missing);
                continue;
            }
        }
    };
}

pub mod meta_info;
pub mod model_final;
pub mod model_fn_map;
pub mod model_id_maps;
pub mod model_type_blittable;
pub mod model_type_kind;
pub mod model_type_map;
pub mod model_type_map_array;
pub mod model_type_map_delegate;
pub mod model_type_map_enum;
pub mod model_type_map_enum_variants;
pub mod model_type_map_opaque;
pub mod model_type_map_patterns;
pub mod model_type_map_pointer;
pub mod model_type_map_primitives;
pub mod model_type_map_service;
pub mod model_type_map_struct;
pub mod model_type_map_struct_fields;
pub mod model_type_names;
pub mod output_final;
pub mod output_fn_imports;
pub mod output_header;
pub mod output_master;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Outcome {
    Unchanged,
    Changed,
}

impl Outcome {
    pub fn changed(&mut self) {
        *self = Outcome::Changed;
    }
}

pub type ModelResult = Result<Outcome, Error>;
pub type OutputResult = Result<(), Error>;

#[derive(Debug, Copy, Clone)]
pub struct PassInfo {
    pub name: &'static str,
}

#[derive(Debug, Copy, Clone)]
pub enum MissingItem {
    CsType(TypeId),
    RustType(interoptopus::inventory::TypeId),
}

#[derive(Debug, Copy, Clone)]
pub struct Missing {
    pub origin: PassInfo,
    pub item: MissingItem,
}

/// Tracks items not found by passes.
#[derive(Debug, Clone, Default)]
pub struct LostAndFound {
    entries: Vec<Missing>,
}

impl LostAndFound {
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn missing(&mut self, origin: PassInfo, item: MissingItem) {
        self.entries.push(Missing { origin, item });
    }

    pub fn print(&self) {
        for missing in &self.entries {
            println!("Missing in {:?}: {:?}", missing.origin.name, missing.item);
        }
    }
}

/// Collects common pass and debug information.
#[derive(Debug, Clone, Default)]
pub struct PassMeta {
    pub lost_found: LostAndFound,
}

impl PassMeta {
    pub fn clear(&mut self) {
        self.lost_found.entries.clear();
    }
}
