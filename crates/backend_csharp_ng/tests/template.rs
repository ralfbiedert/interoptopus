use backend_csharp_ng::template::templates;
use interoptopus_backends::template::Context;

#[test]
fn load_templates() {
    let templates = templates();
    let file_header = templates.get("header.cs").unwrap();
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

    let rendered = templates.render("header.cs", &context).unwrap();

    assert!(rendered.contains("AAA"));
    assert!(rendered.contains("BBB"));
    assert!(rendered.contains("CCC"));
    assert!(rendered.contains("DDD"));
    assert!(rendered.contains("EEE"));
}
