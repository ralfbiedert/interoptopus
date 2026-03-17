/// Renders a Tera template, optionally with key-value context pairs.
///
/// # Usage
///
/// Without context:
/// ```ignore
/// let output = render!(engine, "my_template.cs")?;
/// ```
///
/// With context variables:
/// ```ignore
/// let output = render!(engine, "my_template.cs", ("name", &name), ("is_public", &true))?;
/// ```
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
