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

/// Register an item.
#[macro_export]
macro_rules! constant {
    ($x:ty) => {{
        |x: &mut $crate::inventory::Inventory| {
            <$x as $crate::lang::constant::ConstantInfo>::register(x);
        }
    }};
}

/// Register an item.
#[macro_export]
macro_rules! service {
    ($x:ty) => {{
        |x: &mut $crate::inventory::Inventory| {
            <$x as $crate::lang::service::ServiceInfo>::register(x);
        }
    }};
}
