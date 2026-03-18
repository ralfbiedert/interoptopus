use interoptopus::ffi;
use interoptopus::wire::Wire;

fn main() {
    // Wire::from requires T::WIRE_SAFE at compile time.
    // ffi::Vec is not WIRE_SAFE, so this must not compile.
    let v: ffi::Vec<u32> = ffi::Vec::from_vec(vec![1, 2, 3]);
    let _ = Wire::from(v);
}
