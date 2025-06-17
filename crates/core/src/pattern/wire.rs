//! A protobuf-like marshaller across the rust-ffi border.<sup>🚧</sup>
// Generate serialization code on both sides, Rust and backend's language, to transfer
// type T over the FFI border in a byte array package.
use crate::lang::Ser;
use std::marker::PhantomData;
use std::borrow::Cow;

pub struct Wire<T>
where
    T: Ser,
{
    // buf: Cow<[u8]>,        // storage gotten from wherever -- define
    _type: PhantomData<T>, // what we're wiring
}

impl<T: Ser> Wire<T> {
    fn wire(&self) {
        // self.inner.ser(output);
    }
}
