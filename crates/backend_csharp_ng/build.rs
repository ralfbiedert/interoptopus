use interoptopus_backends::template::pack_assets;

fn main() {
    pack_assets("templates.tar", "templates/").unwrap();
}
