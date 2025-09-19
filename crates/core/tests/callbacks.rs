use interoptopus::callback;

#[test]
fn callback_default() {
    callback!(MyCallback());
    MyCallback::default();
}
