/// Extract the inner data from a [`TypeKind`](interoptopus::lang::types::TypeKind) variant,
/// or `continue` if the type doesn't match.
///
/// ```ignore
/// let rust_array = try_extract_kind!(ty, Array);
/// ```
#[macro_export]
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
#[macro_export]
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
#[macro_export]
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
