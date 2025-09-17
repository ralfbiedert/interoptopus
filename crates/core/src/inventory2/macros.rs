/// Register an item.
#[macro_export]
macro_rules! item2 {
    ($x:ty) => {{
        use $crate::lang2::Register;

        |x: &mut $crate::inventory2::Inventory| {
            <$x as Register>::register(x);
        }
    }};
}
