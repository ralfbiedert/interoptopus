interoptopus::plugin!(BadPlugin {
    impl MyService {
        fn create() -> Self;
        fn mutate(&mut self);
    }
});

fn main() {}
