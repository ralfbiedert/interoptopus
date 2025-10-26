/// Renders a template.
#[allow(clippy::crate_in_macro_def, reason = "We do want to access one of backend crates' templates, not this one")]
#[macro_export]
macro_rules! render {
    ($writer:expr, $template:expr) => {
        {
            let context = tera::Context::new();
            let template = crate::TEMPLATES.render($template, &context).map_err($crate::Error::Templating)?;
            let indented = $crate::indent_all_with($writer.indent_prefix(), template);
            write!($writer.writer(), "{}", indented).map_err($crate::Error::Io)
        }
    };
    ($writer:expr, $template:expr, $(($key:expr,$value:expr)),+) => {
        {
            let mut context = tera::Context::new();
            $(
                context.insert($key, $value);
            )*
            let template = crate::TEMPLATES.render($template, &context).map_err($crate::Error::Templating)?;
            let indented = $crate::indent_all_with($writer.indent_prefix(), template);
            write!($writer.writer(), "{}", indented).map_err($crate::Error::Io)
        }
    };
}
