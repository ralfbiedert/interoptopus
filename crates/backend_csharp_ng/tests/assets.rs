use interoptopus_backends::template::Assets;

// Include the tar file that was created by build.rs
const ASSET_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/templates.tar"));

#[test]
fn test_asset_loading() {
    // Load assets from the embedded bytes
    let assets = Assets::from_reader(ASSET_BYTES).expect("Failed to load assets");

    // Load a specific file as a string
    let content = assets.load_string("file_header.cs").expect("Failed to load file_header.cs");

    // Verify it contains expected content
    assert!(!content.is_empty());
    println!("Loaded file_header.cs with {} bytes", content.len());

    // List all available assets
    let files = assets.list().collect::<Vec<_>>();
    println!("Available assets: {:?}", files);
    assert!(files.contains(&"file_header.cs"));
}
