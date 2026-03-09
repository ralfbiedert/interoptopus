use interoptopus::lang::types::{type_id_ptr, type_id_ptr_mut, TypeInfo};

#[test]
fn type_id_ptr_matches_concrete_const_pointer() {
    // type_id_ptr(T::id()) must equal <*const T>::id() for concrete types.
    assert_eq!(type_id_ptr(u8::id()), <*const u8>::id());
    assert_eq!(type_id_ptr(u32::id()), <*const u32>::id());
    assert_eq!(type_id_ptr(u64::id()), <*const u64>::id());
    assert_eq!(type_id_ptr(i8::id()), <*const i8>::id());
    assert_eq!(type_id_ptr(std::ffi::c_void::id()), <*const std::ffi::c_void>::id());
    assert_eq!(type_id_ptr(std::ffi::c_char::id()), <*const std::ffi::c_char>::id());
}

#[test]
fn type_id_ptr_mut_matches_concrete_mut_pointer() {
    // type_id_ptr_mut(T::id()) must equal <*mut T>::id() for concrete types.
    assert_eq!(type_id_ptr_mut(u8::id()), <*mut u8>::id());
    assert_eq!(type_id_ptr_mut(u32::id()), <*mut u32>::id());
    assert_eq!(type_id_ptr_mut(u64::id()), <*mut u64>::id());
    assert_eq!(type_id_ptr_mut(i8::id()), <*mut i8>::id());
    assert_eq!(type_id_ptr_mut(std::ffi::c_void::id()), <*mut std::ffi::c_void>::id());
    assert_eq!(type_id_ptr_mut(std::ffi::c_char::id()), <*mut std::ffi::c_char>::id());
}

#[test]
fn const_and_mut_pointer_ids_differ() {
    // *const T and *mut T must have different TypeIds.
    assert_ne!(type_id_ptr(u8::id()), type_id_ptr_mut(u8::id()));
    assert_ne!(type_id_ptr(u64::id()), type_id_ptr_mut(u64::id()));
    assert_ne!(<*const u8>::id(), <*mut u8>::id());
}
