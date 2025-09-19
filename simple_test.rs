#[interoptopus_proc::ffi_type]
pub struct SimpleTest {
    pub x: u32,
    pub y: u32,
}

fn main() {
    println!("Compiled successfully! The macro works!");
}