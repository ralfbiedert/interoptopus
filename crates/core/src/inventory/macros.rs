/// Register an item.
#[macro_export]
macro_rules! item {
    ($x:ty) => {{
        use $crate::lang::Register;

        |x: &mut $crate::inventory::Inventory| {
            <$x as Register>::register(x);
        }
    }};
}

/// Register an item.
#[macro_export]
macro_rules! extra_type {
    ($x:ty) => {{
        use $crate::lang::types::TypeInfo;

        |x: &mut $crate::inventory::Inventory| {
            <$x as TypeInfo>::register(x);
        }
    }};
}
