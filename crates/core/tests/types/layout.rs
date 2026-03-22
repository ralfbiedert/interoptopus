#![allow(clippy::ptr_as_ptr, clippy::borrow_as_ptr, clippy::cast_ptr_alignment, clippy::ref_as_ptr)]

use interoptopus::ffi;
use interoptopus::pattern::api_guard::ApiVersion;
use std::mem;

const PTR_SIZE: usize = mem::size_of::<*const u8>();

// IF ANY OF THESE TESTS FAIL AFTER A REFACTOR YOU MUST ALSO UPDATE ALL BACKEND CODEGEN
// TO REFLECT THE CHANGES MADE.

#[test]
fn api_version() {
    // repr(transparent) over u64
    assert_eq!(mem::size_of::<ApiVersion>(), mem::size_of::<u64>());
    assert_eq!(mem::align_of::<ApiVersion>(), mem::align_of::<u64>());

    let v = ApiVersion::new(0x1234_5678_9ABC_DEF0);
    let raw = unsafe { *(&v as *const ApiVersion as *const u64) };
    assert_eq!(raw, 0x1234_5678_9ABC_DEF0);
}

#[test]
fn bool() {
    // repr(transparent) over u8
    assert_eq!(mem::size_of::<ffi::Bool>(), 1);
    assert_eq!(mem::align_of::<ffi::Bool>(), 1);

    // 1 = true, 0 = false
    let raw_t = unsafe { *(&ffi::Bool::TRUE as *const ffi::Bool as *const u8) };
    let raw_f = unsafe { *(&ffi::Bool::FALSE as *const ffi::Bool as *const u8) };
    assert_eq!(raw_t, 1);
    assert_eq!(raw_f, 0);
}

#[test]
fn cchar() {
    // repr(transparent) over c_char (i8/u8)
    assert_eq!(mem::size_of::<ffi::CChar>(), 1);
    assert_eq!(mem::align_of::<ffi::CChar>(), 1);
}

#[test]
fn cstr_ptr() {
    // repr(transparent) over *const c_char
    assert_eq!(mem::size_of::<ffi::CStrPtr>(), PTR_SIZE);
    assert_eq!(mem::align_of::<ffi::CStrPtr>(), PTR_SIZE);

    // Inner pointer matches the source CStr
    let cstr = c"hello";
    let ffi_cstr = ffi::CStrPtr::from_cstr(cstr);
    let raw_ptr = unsafe { *(&ffi_cstr as *const ffi::CStrPtr as *const *const u8) };
    assert_eq!(raw_ptr, cstr.as_ptr() as *const u8);
}

#[test]
fn utf8_string() {
    // repr(C): ptr, len, capacity — three pointer-width words
    assert_eq!(mem::size_of::<ffi::String>(), PTR_SIZE + 8 + 8);
    assert_eq!(mem::align_of::<ffi::String>(), PTR_SIZE);

    let s = ffi::String::from_string("hello".to_string());
    let raw = &s as *const ffi::String as *const u8;
    unsafe {
        let ptr = *(raw as *const *const u8);
        let len = *(raw.add(PTR_SIZE) as *const u64);
        let cap = *(raw.add(PTR_SIZE + 8) as *const u64);
        assert!(!ptr.is_null());
        assert_eq!(len, 5);
        assert!(cap >= 5);
    }
    // Prevent drop from double-freeing
    let _ = s.into_string();
}

#[test]
fn slice() {
    // repr(C): *const T, u64 len (+ ZST PhantomData)
    assert_eq!(mem::size_of::<ffi::Slice<u8>>(), PTR_SIZE + 8);
    assert_eq!(mem::align_of::<ffi::Slice<u8>>(), PTR_SIZE);
    assert_eq!(mem::size_of::<ffi::Slice<u64>>(), PTR_SIZE + 8);

    let data: [u32; 3] = [10, 20, 30];
    let s = ffi::Slice::from(&data[..]);
    let raw = &s as *const ffi::Slice<u32> as *const u8;
    unsafe {
        let ptr = *(raw as *const *const u32);
        let len = *(raw.add(PTR_SIZE) as *const u64);
        assert_eq!(ptr, data.as_ptr());
        assert_eq!(len, 3);
    }
}

#[test]
fn slice_mut() {
    // repr(C): *mut T, u64 len (+ ZST PhantomData)
    assert_eq!(mem::size_of::<ffi::SliceMut<u8>>(), PTR_SIZE + 8);
    assert_eq!(mem::align_of::<ffi::SliceMut<u8>>(), PTR_SIZE);

    let mut data: [u32; 4] = [1, 2, 3, 4];
    let expected_ptr = data.as_mut_ptr();
    let s = ffi::SliceMut::from(&mut data[..]);
    let raw = &s as *const ffi::SliceMut<u32> as *const u8;
    unsafe {
        let ptr = *(raw as *const *mut u32);
        let len = *(raw.add(PTR_SIZE) as *const u64);
        assert_eq!(ptr, expected_ptr);
        assert_eq!(len, 4);
    }
}

#[test]
fn vec() {
    // repr(C): *mut T, u64 len, u64 capacity
    assert_eq!(mem::size_of::<ffi::Vec<u8>>(), PTR_SIZE + 8 + 8);
    assert_eq!(mem::align_of::<ffi::Vec<u8>>(), PTR_SIZE);

    let v = ffi::Vec::from_vec(vec![1u32, 2, 3]);
    let raw = &v as *const ffi::Vec<u32> as *const u8;
    unsafe {
        let ptr = *(raw as *const *const u32);
        let len = *(raw.add(PTR_SIZE) as *const u64);
        let cap = *(raw.add(PTR_SIZE + 8) as *const u64);
        assert!(!ptr.is_null());
        assert_eq!(len, 3);
        assert!(cap >= 3);
    }
    // Prevent drop from double-freeing
    let _ = v.into_vec();
}

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
fn named_callback() {
    use interoptopus::callback;
    callback!(LayoutCallback(x: i32) -> i32);

    // repr(C): fn_ptr, ctx_ptr, dtor_ptr — three pointer-width words
    assert_eq!(mem::size_of::<LayoutCallback>(), 3 * PTR_SIZE);
    assert_eq!(mem::align_of::<LayoutCallback>(), PTR_SIZE);

    // new(): fn_ptr non-null, ctx_ptr null, dtor_ptr null
    extern "C" fn my_fn(x: i32, _: *const std::ffi::c_void) -> i32 { x }
    let cb = LayoutCallback::new(my_fn);
    let raw = &cb as *const LayoutCallback as *const u8;
    unsafe {
        let fn_ptr = *(raw as *const *const u8);
        assert!(!fn_ptr.is_null(), "fn_ptr must be non-null");
        let ctx_ptr = *(raw.add(PTR_SIZE) as *const *const u8);
        assert!(ctx_ptr.is_null(), "ctx_ptr must be null for new()");
        let dtor_ptr = *(raw.add(2 * PTR_SIZE) as *const *const u8);
        assert!(dtor_ptr.is_null(), "dtor_ptr must be null for new()");
    }

    // from_closure(): dtor_ptr non-null, ctx_ptr non-null (points to the boxed closure)
    let cb2 = LayoutCallback::from_closure(|x| x + 1);
    let raw2 = &cb2 as *const LayoutCallback as *const u8;
    unsafe {
        let ctx_ptr = *(raw2.add(PTR_SIZE) as *const *const u8);
        assert!(!ctx_ptr.is_null(), "ctx_ptr must be non-null for from_closure()");
        let dtor_ptr = *(raw2.add(2 * PTR_SIZE) as *const *const u8);
        assert!(!dtor_ptr.is_null(), "dtor_ptr must be non-null for from_closure()");
    }
    // cb2 dropped here, destructor runs automatically
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
