interoptopus::plugin!(BadPlugin {
    impl MyService {
        fn create() -> Self;
        fn not_a_method(x: u32) -> u32;
        fn get(&self) -> u32;
    }
});

fn main() {}
