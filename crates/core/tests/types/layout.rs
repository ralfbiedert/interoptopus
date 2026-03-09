use interoptopus::ffi;
use std::mem;

// IF ANY OF THESE TESTS FAIL AFTER A REFACTOR YOU MUST ALSO UPDATE ALL BACKEND CODEGEN
// TO REFLECT THE CHANGES MADE.

#[test]
fn option() {
    // repr(u32): discriminant is u32
    assert_eq!(mem::align_of::<ffi::Option<u8>>(), 4);
    assert_eq!(mem::size_of::<ffi::Option<u8>>(), 8);
    assert_eq!(mem::size_of::<ffi::Option<u64>>(), 16);
    assert_eq!(mem::align_of::<ffi::Option<u64>>(), 8);

    // Some = 0, None = 1
    let some: ffi::Option<u32> = ffi::Some(0xDEAD_BEEF);
    let raw = unsafe { &*(&some as *const ffi::Option<u32> as *const [u32; 2]) };
    assert_eq!(raw[0], 0, "Some discriminant must be 0");
    assert_eq!(raw[1], 0xDEAD_BEEF);

    let none: ffi::Option<u32> = ffi::None;
    let tag = unsafe { *(&none as *const ffi::Option<u32> as *const u32) };
    assert_eq!(tag, 1, "None discriminant must be 1");
}

#[test]
fn result() {
    // repr(u32): discriminant is u32
    assert_eq!(mem::align_of::<ffi::Result<u32, u32>>(), 4);
    assert_eq!(mem::size_of::<ffi::Result<u32, u32>>(), 8);
    assert_eq!(mem::size_of::<ffi::Result<u64, u8>>(), 16);
    assert_eq!(mem::align_of::<ffi::Result<u64, u8>>(), 8);

    // Ok = 0, Err = 1, Panic = 2, Null = 3
    let ok: ffi::Result<u32, u32> = ffi::Ok(0xCAFE);
    let raw = unsafe { &*(&ok as *const ffi::Result<u32, u32> as *const [u32; 2]) };
    assert_eq!(raw[0], 0, "Ok discriminant must be 0");
    assert_eq!(raw[1], 0xCAFE);

    let err: ffi::Result<u32, u32> = ffi::Result::Err(0xBEEF);
    let raw = unsafe { &*(&err as *const ffi::Result<u32, u32> as *const [u32; 2]) };
    assert_eq!(raw[0], 1, "Err discriminant must be 1");
    assert_eq!(raw[1], 0xBEEF);

    let panic: ffi::Result<u32, u32> = ffi::Result::Panic;
    let tag = unsafe { *(&panic as *const ffi::Result<u32, u32> as *const u32) };
    assert_eq!(tag, 2, "Panic discriminant must be 2");

    let null: ffi::Result<u32, u32> = ffi::Result::Null;
    let tag = unsafe { *(&null as *const ffi::Result<u32, u32> as *const u32) };
    assert_eq!(tag, 3, "Null discriminant must be 3");
}
