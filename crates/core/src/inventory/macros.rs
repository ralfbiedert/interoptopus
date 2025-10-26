/// Register an item.
#[macro_export]
macro_rules! extra_type {
    ($x:ty) => {{
        use $crate::lang::types::TypeInfo;

        |inventory| {
            <$x as TypeInfo>::register(inventory);
        }
    }};
}

/// Register an item.
#[macro_export]
macro_rules! function {
    ($x:ty) => {{
        |inventory| {
            <$x as $crate::lang::function::FunctionInfo>::register(inventory);
        }
    }};
}

/// Register an item.
#[macro_export]
macro_rules! constant {
    ($x:ty) => {{
        |inventory| {
            <$x as $crate::lang::constant::ConstantInfo>::register(inventory);
        }
    }};
}

/// Register an item.
#[macro_export]
macro_rules! service {
    ($x:ty) => {{
        |inventory| {
            <$x as $crate::lang::service::ServiceInfo>::register(inventory);
        }
    }};
}
