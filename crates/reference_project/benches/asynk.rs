#![feature(test)]

extern crate test;

use interoptopus_reference_project::patterns::string::pattern_string_2;
use test::Bencher;

#[bench]
fn tokio_baseline(b: &mut Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    b.iter(|| {
        rt.block_on(async {});
    });
}
