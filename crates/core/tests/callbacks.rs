use interoptopus::callback;
use interoptopus::ffi::Slice;

#[test]
fn callback_default() {
    callback!(MyCallback());
    // MyCallback::default();
}

callback!(CallbackSlice(x: Slice<'_, u8>) -> u8);
