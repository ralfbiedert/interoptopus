pub mod lang;
pub mod plugin;
pub mod stage;

mod buffer;
mod macros;
mod pipeline;

use interoptopus_backends::template_engine;

// template_engine!("**/*.cs");

use interoptopus_backends::reexport::include_dir::include_dir;
use interoptopus_backends::reexport::{include_dir, tera};

static TEMPLATE_FILES: ::std::sync::LazyLock<include_dir::Dir<'_>> = ::std::sync::LazyLock::new(|| include_dir::include_dir!("$CARGO_MANIFEST_DIR/templates"));

pub(crate) static TEMPLATES: ::std::sync::LazyLock<tera::Tera> = ::std::sync::LazyLock::new(|| {
    let mut tera = tera::Tera::default();
    for file in TEMPLATE_FILES.find("**.cs").unwrap() {
        file.as_file().map(|template| {
            tera.add_raw_template(&template.path().to_str().unwrap(), template.contents_utf8().unwrap()).unwrap();
        });
    }
    tera
});
