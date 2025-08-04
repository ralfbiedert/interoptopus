//! Create a templated codegen engine, utilising templates stored in
//! `$CARGO_MANIFEST_DIR/templates` directory
//! with glob pattern matching `template_glob`. Use a render!() macro.
//! Typical usage
//! ```nocompile
//! codegen_template_engine("**/*.py");
//! ```
#[macro_export]
macro_rules! codegen_template_engine {
    ($template_glob:literal) => {
        static TEMPLATE_FILES: std::sync::LazyLock<include_dir::Dir<'_>> = std::sync::LazyLock::new(|| include_dir::include_dir!("$CARGO_MANIFEST_DIR/templates"));

        pub(crate) static TEMPLATES: std::sync::LazyLock<tera::Tera> = std::sync::LazyLock::new(|| {
            let mut tera = tera::Tera::default();
            for file in TEMPLATE_FILES.find($template_glob).unwrap() {
                file.as_file().map(|template| {
                    tera.add_raw_template(&template.path().to_str().unwrap(), template.contents_utf8().unwrap())
                        .unwrap();
                });
            }
            tera
        });
    };
}

#[allow(clippy::crate_in_macro_def, reason = "We do want to access one of backend crates' templates, not this one")]
#[macro_export]
macro_rules! render {
    ($writer:expr, $template:expr) => {
        {
            let context = tera::Context::new();
            let template = crate::TEMPLATES.render($template, &context).map_err(interoptopus::Error::Templating)?;
            let indented = indent::indent_all_with($writer.indent_prefix(), template);
            write!($writer.writer(), "{}", indented).map_err(interoptopus::Error::IO)
        }
    };
    ($writer:expr, $template:expr, $(($key:expr,$value:expr)),+) => {
        {
            let mut context = tera::Context::new();
            $(
                context.insert($key, $value);
            )*
            let template = crate::TEMPLATES.render($template, &context).map_err(interoptopus::Error::Templating)?;
            let indented = indent::indent_all_with($writer.indent_prefix(), template);
            write!($writer.writer(), "{}", indented).map_err(interoptopus::Error::IO)
        }
    };
}
