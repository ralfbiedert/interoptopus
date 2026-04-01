interoptopus::plugin!(BadPlugin {
    fn good(x: u32) -> u32;
    fn bad(&self) -> u32;
});

fn main() {}
