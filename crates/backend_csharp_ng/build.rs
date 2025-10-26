use interoptopus_backends::template::pack_assets;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("templates.tar");
    let out_file = File::create(&out_path).unwrap();

    pack_assets(out_file, "templates/").unwrap();

    println!("cargo:rerun-if-changed=templates/");
    println!("cargo:warning=Packed assets to {}", out_path.display());
}
