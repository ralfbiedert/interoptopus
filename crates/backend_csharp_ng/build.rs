use interoptopus_backends::template::pack_assets;

fn main() {
    pack_assets("foo.assets", "templates/").unwrap();
}
