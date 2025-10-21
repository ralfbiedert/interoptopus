use interoptopus_backends::template::pack_assets;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("templates.tar");

    pack_assets(&out_path, "templates/").unwrap();

    println!("cargo:rerun-if-changed=templates/");
    println!("cargo:warning=Packed assets to {}", out_path.display());
}
