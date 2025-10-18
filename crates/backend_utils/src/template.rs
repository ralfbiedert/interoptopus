/// Indents each line in the given text with the specified prefix.
pub fn indent_all_with(prefix: impl AsRef<str>, text: impl AsRef<str>) -> String {
    if text.as_ref().is_empty() {
        return text.as_ref().to_string();
    }

    let ends_with_newline = text.as_ref().ends_with('\n');
    let lines: Vec<&str> = text.as_ref().lines().collect();

    let indented: Vec<String> = lines
        .iter()
        .map(|line| {
            if line.is_empty() {
                (*line).to_string()
            } else {
                format!("{}{}", prefix.as_ref(), line)
            }
        })
        .collect();

    let mut result = indented.join("\n");
    if ends_with_newline && !result.ends_with('\n') {
        result.push('\n');
    }

    result
}

/// Create a templated codegen engine for backends.
///
/// This utilizes templates stored in the `$CARGO_MANIFEST_DIR/templates` directory
/// with glob pattern matching `template_glob`. Use the `render!()` macro to render
/// templates.
///
/// # Example
///
/// ```ignore
/// codegen_template_engine("**/*.py");
/// ```
#[macro_export]
macro_rules! template_engine {
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
