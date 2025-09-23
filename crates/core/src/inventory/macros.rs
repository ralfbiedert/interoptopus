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

/// Register an item.
#[macro_export]
macro_rules! function {
    ($x:ty) => {{
        |x: &mut $crate::inventory::Inventory| {
            <$x as $crate::lang::function::FunctionInfo>::register(x);
        }
    }};
}
