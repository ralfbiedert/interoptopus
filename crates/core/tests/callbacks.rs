use interoptopus::callback;
use interoptopus::ffi::Slice;
use std::sync::{
    Arc,
    atomic::{AtomicI32, Ordering},
};

#[test]
fn callback_default() {
    callback!(MyCallback());
    // MyCallback::default();
}

callback!(CallbackSlice(x: Slice<'_, u8>) -> u8);

// Closure capturing an environment variable is called correctly.
#[test]
fn closure_call() {
    callback!(Add(x: i32, y: i32) -> i32);

    let offset = 10;
    let cb = Add::from_fn(move |x, y| x + y + offset);
    assert_eq!(cb.call(3, 4), 17);
}

// Closure that mutates shared state via an atomic to verify it actually runs.
#[test]
fn closure_call_side_effect() {
    callback!(Increment(delta: i32));

    let counter = Arc::new(AtomicI32::new(0));
    let c = Arc::clone(&counter);
    let cb = Increment::from_fn(move |delta| {
        c.fetch_add(delta, Ordering::SeqCst);
    });

    cb.call(3);
    cb.call(7);
    assert_eq!(counter.load(Ordering::SeqCst), 10);
    // cb dropped here, destructor runs automatically
}

// Drop frees the allocation: the Arc refcount reaches zero when cb goes out of scope.
#[test]
fn closure_drop_frees_allocation() {
    callback!(Noop());

    let shared = Arc::new(());
    let weak = Arc::downgrade(&shared);
    let captured = Arc::clone(&shared);
    drop(shared);

    let cb = Noop::from_fn(move || {
        let _ = &captured;
    });
    assert!(weak.upgrade().is_some(), "closure should still hold the Arc");

    drop(cb);
    assert!(weak.upgrade().is_none(), "Arc must be dropped after cb is dropped");
}

// Dropping a plain function-pointer callback (no destructor) is a no-op.
#[test]
fn drop_noop_on_fn_ptr_callback() {
    callback!(Noop2());

    extern "C" fn nothing(_: *const std::ffi::c_void) {}
    let cb = Noop2 { callback: Some(nothing), data: std::ptr::null(), destructor: None };
    drop(cb); // must not crash or double-free
}
