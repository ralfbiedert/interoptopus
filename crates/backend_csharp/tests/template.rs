use interoptopus_backends::template::Context;
use interoptopus_csharp::template::templates;

#[test]
fn load_templates() {
    let templates = templates();
    let file_header = templates.get("rust/header.cs").unwrap();
    assert!(file_header.contains("auto-generated"));
}

#[test]
fn render_templates() {
    let templates = templates();
    let mut context = Context::new();

    context.insert("INTEROP_DLL_NAME", "AAA");
    context.insert("INTEROP_HASH", "BBB");
    context.insert("INTEROP_NAMESPACE", "CCC");
    context.insert("INTEROPTOPUS_CRATE", "DDD");
    context.insert("INTEROPTOPUS_VERSION", "EEE");
    context.insert("emit_version", &true);

    let rendered = templates.render("rust/header.cs", &context).unwrap();

    assert!(rendered.contains("AAA"));
    assert!(rendered.contains("BBB"));
    assert!(rendered.contains("CCC"));
    assert!(rendered.contains("DDD"));
    assert!(rendered.contains("EEE"));
}
