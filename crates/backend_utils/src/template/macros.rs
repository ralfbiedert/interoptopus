/// Renders a template.
// #[allow(clippy::crate_in_macro_def, reason = "We do want to access one of backend crates' templates, not this one")]
#[macro_export]
macro_rules! render {
    ($engine:expr, $template:expr) => {{
        let context = $crate::template::Context::new();
        $engine.render($template, &context)
    }};
    ($engine:expr, $template:expr, $(($key:expr,$value:expr)),+) => {{
        let mut context = $crate::template::Context::new();
        $(
            context.insert($key, $value);
        )+
        $engine.render($template, &context)
    }};
}
