use crate::{define_plugin, load_plugin};
use interoptopus_csharp::pattern::Try;
use std::error::Error;

interoptopus::plugin!(Memory {
    fn gc();

    impl Heavy {
        fn new_self(size: usize, value: u32) -> Try<Self>;
        async fn new_self_async(size: usize) -> Try<Self>;
        fn get(&self, i: usize) -> u32;
    }

    impl Fliparoo {
        fn create_1(heavy_1: Heavy, heavy_2: Heavy) -> Try<Self>;
        async fn create_2(heavy1: &Heavy, heavy2: &Heavy) -> Try<Self>;
        async fn create_3(heavy1: &Heavy, heavy3: Heavy) -> Try<Self>;
        fn replace_left_1(&self, heavy: Heavy) -> Heavy;
        fn replace_left_2(&self, heavy: &Heavy) -> Heavy;
        fn replace_right_1(&self, heavy: Heavy) -> Heavy;
        fn replace_right_2(&self, heavy: Heavy) -> Heavy;
        fn get_left(&self) -> Heavy;
        fn get_right(&self) -> Heavy;
    }
});

#[test]
fn build_plugin() -> Result<(), Box<dyn Error>> {
    define_plugin!(Memory, "memory.dll", super::BASE);
    Ok(())
}

#[test]
fn load_plugin() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Memory, "memory.dll", super::BASE);

    Ok(())
}
