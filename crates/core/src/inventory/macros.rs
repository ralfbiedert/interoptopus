/// Register an item.
#[macro_export]
macro_rules! item2 {
    ($x:ty) => {{
        use $crate::lang::Register;

        |x: &mut $crate::inventory::Inventory| {
            <$x as Register>::register(x);
        }
    }};
}
