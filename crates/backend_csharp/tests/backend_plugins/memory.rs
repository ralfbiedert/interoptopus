use crate::{define_plugin, load_plugin};
use interoptopus_csharp::pattern::{Try, TryExtension};
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
        async fn replace_left_async(&self, heavy: Heavy) -> Heavy;
        async fn replace_right_async(&self, heavy: &Heavy) -> Heavy;
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

    // 128 MB worth of u32 elements = 32M elements
    const CHUNK: usize = 32 * 1024 * 1024;
    const NUM_CYCLES: usize = 128;

    let rss = || memory_stats::memory_stats().map(|s| s.physical_mem).unwrap_or(0);

    // Warm up: one allocation + GC so the runtime has settled.
    {
        let _warm = plugin.heavy_new_self(1024, 0xDEAD).ok()?;
    }
    plugin.gc();
    std::thread::sleep(std::time::Duration::from_millis(200));

    let mut post_first_cycle = 0;

    for cycle in 0..NUM_CYCLES {
        // Allocate 4 large Heavy objects, each 128 MB, filled with distinct values.
        let h1 = plugin.heavy_new_self(CHUNK, 1).ok()?;
        let h2 = plugin.heavy_new_self(CHUNK, 2).ok()?;
        let h3 = plugin.heavy_new_self(CHUNK, 3).ok()?;
        let h4 = plugin.heavy_new_self(CHUNK, 4).ok()?;

        // Verify initial contents.
        assert_eq!(h1.get(0), 1, "cycle {cycle}: h1[0]");
        assert_eq!(h2.get(CHUNK - 1), 2, "cycle {cycle}: h2[last]");
        assert_eq!(h3.get(CHUNK / 2), 3, "cycle {cycle}: h3[mid]");
        assert_eq!(h4.get(0), 4, "cycle {cycle}: h4[0]");

        // Create Fliparoo via create_1 (takes ownership of h1, h2).
        let flip = plugin.fliparoo_create_1(h1, h2).ok()?;

        // Verify left/right are h1/h2 by checking values.
        let left = flip.get_left();
        let right = flip.get_right();
        assert_eq!(left.get(0), 1, "cycle {cycle}: flip left should be h1");
        assert_eq!(right.get(0), 2, "cycle {cycle}: flip right should be h2");

        // Replace left with h3 (ownership transfer), get old left back.
        let old_left = flip.replace_left_1(h3);
        assert_eq!(old_left.get(0), 1, "cycle {cycle}: old left should still be h1");

        // Verify new left is h3.
        let new_left = flip.get_left();
        assert_eq!(new_left.get(0), 3, "cycle {cycle}: new left should be h3");

        // Replace right with h4.
        let old_right = flip.replace_right_1(h4);
        assert_eq!(old_right.get(0), 2, "cycle {cycle}: old right should still be h2");
        let new_right = flip.get_right();
        assert_eq!(new_right.get(0), 4, "cycle {cycle}: new right should be h4");

        // replace_left_2 takes a ref — left gets a copy, flip keeps its own.
        let copy_left = flip.replace_left_2(&old_left);
        assert_eq!(copy_left.get(0), 3, "cycle {cycle}: copy of old left");

        // replace_right_2 takes ownership.
        let copy_right = flip.replace_right_2(old_right);
        assert_eq!(copy_right.get(0), 4, "cycle {cycle}: copy of old right");

        // Verify final state: left should be old_left (h1 value), right should be old_right (h2 value).
        let final_left = flip.get_left();
        let final_right = flip.get_right();
        assert_eq!(final_left.get(0), 1, "cycle {cycle}: final left");
        assert_eq!(final_right.get(0), 2, "cycle {cycle}: final right");

        // Drop all handles — the .NET side should be free to GC everything.
        drop(flip);
        drop(old_left);
        drop(new_left);
        drop(new_right);
        drop(left);
        drop(right);
        drop(copy_left);
        drop(copy_right);
        drop(final_left);
        drop(final_right);

        // Force GC and let memory settle.
        plugin.gc();

        let current = rss();
        // eprintln!("cycle {cycle}: current={current}");

        // After the first cycle, the runtime has its peak committed memory.
        // Subsequent cycles must not grow significantly — that would indicate a leak.
        if cycle == 0 {
            post_first_cycle = current;
        } else {
            let growth_since_first = current.saturating_sub(post_first_cycle);
            assert!(growth_since_first < 256 * 1024 * 1024, "cycle {cycle}: memory grew {growth_since_first} bytes since cycle 0 ({post_first_cycle}), expected stable");
        }
    }

    Ok(())
}

#[tokio::test]
async fn load_plugin_async() -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin!(Memory, "memory.dll", super::BASE);

    const CHUNK: usize = 8 * 1024 * 1024;
    const NUM_CYCLES: usize = 128;
    const ALLOWED_GROWTH: usize = 8 * CHUNK;

    let rss = || memory_stats::memory_stats().map(|s| s.physical_mem).unwrap_or(0);

    // Warm up.
    {
        let _warm = plugin.heavy_new_self_async(1024).await.ok()?;
    }
    plugin.gc();
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let mut post_first_cycle = 0;

    for cycle in 0..NUM_CYCLES {
        // Allocate via async.
        let h1 = plugin.heavy_new_self_async(CHUNK).await.ok()?;
        let h2 = plugin.heavy_new_self(CHUNK, 2).ok()?;
        let h3 = plugin.heavy_new_self(CHUNK, 3).ok()?;
        let h4 = plugin.heavy_new_self(CHUNK, 4).ok()?;

        // Create Fliparoo via async create_2 (takes refs).
        let flip = plugin.fliparoo_create_2(&h1, &h2).await.ok()?;

        // Verify initial state.
        assert_eq!(flip.get_left().get(0), 0, "cycle {cycle}: async-created left");
        assert_eq!(flip.get_right().get(0), 2, "cycle {cycle}: async-created right");

        // Async replace left (ownership transfer).
        let old_left = flip.replace_left_async(h3).await;
        assert_eq!(old_left.get(0), 0, "cycle {cycle}: async old left");
        assert_eq!(flip.get_left().get(0), 3, "cycle {cycle}: async new left");

        // Async replace right (ref — no ownership transfer).
        let old_right = flip.replace_right_async(&h4).await;
        assert_eq!(old_right.get(0), 2, "cycle {cycle}: async old right");
        assert_eq!(flip.get_right().get(0), 4, "cycle {cycle}: async new right");

        // Create via async create_3 (mixed: ref + owned).
        let flip2 = plugin.fliparoo_create_3(&old_left, old_right).await.ok()?;
        assert_eq!(flip2.get_left().get(0), 0, "cycle {cycle}: flip2 left");
        assert_eq!(flip2.get_right().get(0), 2, "cycle {cycle}: flip2 right");

        // Drop everything.
        drop(flip);
        drop(flip2);
        drop(h1);
        drop(h2);
        drop(h4);
        drop(old_left);

        plugin.gc();
        // tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        // let stats = memory_stats::memory_stats();
        // println!("cycle {cycle}: val={:?}", stats.unwrap().physical_mem);

        let current = rss();

        if cycle == 0 {
            post_first_cycle = current;
        } else {
            let growth_since_first = current.saturating_sub(post_first_cycle);
            assert!(growth_since_first < ALLOWED_GROWTH, "cycle {cycle}: memory grew {growth_since_first} bytes since cycle 0, expected stable");
        }
    }

    Ok(())
}
