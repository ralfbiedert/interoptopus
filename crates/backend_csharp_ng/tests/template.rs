use backend_csharp_ng::template::templates;

#[test]
fn load_templates() {
    let templates = templates();
    let file_header = templates.get("file_header.cs").unwrap();
    assert!(file_header.contains("auto-generated"));
}
