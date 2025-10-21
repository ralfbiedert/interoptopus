use backend_csharp_ng::template::Templates;

#[test]
fn load_templates() {
    let templates = Templates::builtins();
    let file_header = templates.load_string("file_header.cs").unwrap();
    assert!(file_header.contains("auto-generated"));
}
