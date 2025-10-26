/// Renders a template.
// #[allow(clippy::crate_in_macro_def, reason = "We do want to access one of backend crates' templates, not this one")]
#[macro_export]
macro_rules! render {
    ($engine:expr, $template:expr) => {};
    ($engine:expr, $template:expr, $(($key:expr,$value:expr)),+) => {{}};
}
