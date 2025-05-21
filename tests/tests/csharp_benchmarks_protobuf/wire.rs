// Wire type means types inside might not be a part of inventory - they are just
// used for serializing into a buffer.
#[ffi_function]
fn foo(i: Wire<Input>) -> Wire<Outputs> {
    let buf = i.wire();
    my_rust_function(buf);
    Wire::from(output)
}

trait Wired {
    fn ser(&self);
    fn de() -> Self;
    fn max_buffer_size(self) -> usize { // TODO: for some types we can't calculate ahead of time
0usize //        4 + self.item_id.len() + 4;
    }
}

struct Wire<T> {
    buf: Vec<u8>, // ? who owns
    marker: PhantomData<T>,
}

// TODO: this should go through the C# generated shim that would unwire the result type
// private static unsafe extern void WireRustClient(byte[] structPointer, uint structLength, void** result, uint* resultLength);
