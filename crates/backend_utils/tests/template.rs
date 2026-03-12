use interoptopus_backends::template::{TemplateEngine, pack_assets};
use tera::Context;

#[must_use]
pub fn engine_from_templates() -> TemplateEngine {
    let mut buf = Vec::new();
    pack_assets(&mut buf, "tests/templates").unwrap();
    TemplateEngine::from_bytes(buf.as_slice()).unwrap()
}

#[test]
fn indents() {
    let engine = engine_from_templates();
    let mut context = Context::new();
    context.insert("header", "// Foo");
    context.insert("types", "f32\nu32");

    insta::assert_snapshot!(engine.render("indented.cs", &context).unwrap());
}
