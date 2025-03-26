from __future__ import annotations
import ctypes
import typing

T = typing.TypeVar("T")
c_lib = None

def init_lib(path):
    """Initializes the native library. Must be called at least once before anything else."""
    global c_lib
    c_lib = ctypes.cdll.LoadLibrary(path)

    c_lib.interoptopus_string_create.argtypes = [ctypes.c_void_p, ctypes.c_uint64, ctypes.POINTER(Utf8String)]
    c_lib.interoptopus_string_destroy.argtypes = [Utf8String]
    c_lib.interoptopus_vec_TODO_destroy.argtypes = [VecU8]
    c_lib.alignment_1.argtypes = [Packed1]
    c_lib.array_1.argtypes = [Array]
    c_lib.array_2.argtypes = []
    c_lib.array_3.argtypes = [ctypes.POINTER(Array)]
    c_lib.char_array_1.argtypes = []
    c_lib.char_array_2.argtypes = [CharArray]
    c_lib.char_array_3.argtypes = [ctypes.POINTER(CharArray)]
    c_lib.nested_array_1.argtypes = []
    c_lib.nested_array_2.argtypes = [ctypes.POINTER(NestedArray)]
    c_lib.nested_array_3.argtypes = [NestedArray]
    c_lib.behavior_sleep.argtypes = [ctypes.c_uint64]
    c_lib.behavior_panics.argtypes = []
    c_lib.enums_1.argtypes = [ctypes.c_int]
    c_lib.enums_2.argtypes = [ctypes.c_int]
    c_lib.enums_3.argtypes = [ctypes.POINTER(ctypes.c_int)]
    c_lib.fnptr_1.argtypes = [ctypes.CFUNCTYPE(ctypes.c_uint8, ctypes.c_uint8), ctypes.c_uint8]
    c_lib.fnptr_2.argtypes = [ctypes.CFUNCTYPE(None, CharArray), CharArray]
    c_lib.generic_1a.argtypes = [Genericu32, Phantomu8]
    c_lib.generic_1b.argtypes = [Genericu8, Phantomu8]
    c_lib.generic_1c.argtypes = [ctypes.POINTER(Genericu8), ctypes.POINTER(Genericu8)]
    c_lib.generic_2.argtypes = [ctypes.c_void_p]
    c_lib.generic_3.argtypes = [ctypes.c_void_p]
    c_lib.generic_4.argtypes = [ctypes.c_void_p]
    c_lib.generic_5.argtypes = [Weird1u32, Weird2u8]
    c_lib.meta_ambiguous_1.argtypes = [Vec1]
    c_lib.meta_ambiguous_2.argtypes = [Vec2]
    c_lib.meta_ambiguous_3.argtypes = [Vec1, Vec2]
    c_lib.meta_documented.argtypes = [StructDocumented]
    c_lib.meta_visibility1.argtypes = [Visibility1, Visibility2]
    c_lib.meta_renamed.argtypes = [StructRenamed]
    c_lib.namespaced_inner_option.argtypes = [TODO]
    c_lib.namespaced_inner_slice.argtypes = [SliceVec]
    c_lib.namespaced_inner_slice_mut.argtypes = [SliceMutVec]
    c_lib.namespaced_type.argtypes = [Vec]
    c_lib.primitive_args_5.argtypes = [ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64]
    c_lib.primitive_args_10.argtypes = [ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64]
    c_lib.primitive_bool.argtypes = [ctypes.c_bool]
    c_lib.primitive_f32.argtypes = [ctypes.c_float]
    c_lib.primitive_f64.argtypes = [ctypes.c_double]
    c_lib.primitive_i16.argtypes = [ctypes.c_int16]
    c_lib.primitive_i32.argtypes = [ctypes.c_int32]
    c_lib.primitive_i64.argtypes = [ctypes.c_int64]
    c_lib.primitive_i8.argtypes = [ctypes.c_int8]
    c_lib.primitive_u16.argtypes = [ctypes.c_uint16]
    c_lib.primitive_u32.argtypes = [ctypes.c_uint32]
    c_lib.primitive_u64.argtypes = [ctypes.c_uint64]
    c_lib.primitive_u8.argtypes = [ctypes.c_uint8]
    c_lib.primitive_void.argtypes = []
    c_lib.primitive_void2.argtypes = []
    c_lib.ptr1.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ptr2.argtypes = [ctypes.POINTER(ctypes.POINTER(ctypes.c_int64))]
    c_lib.ptr3.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ref1.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ref2.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ref3.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ref4.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.struct1.argtypes = [Tupled]
    c_lib.struct2.argtypes = [Vec3f32, ctypes.POINTER(Tupled)]
    c_lib.struct3.argtypes = [BoolField]
    c_lib.pattern_ascii_pointer_1.argtypes = [ctypes.POINTER(ctypes.c_char)]
    c_lib.pattern_ascii_pointer_2.argtypes = []
    c_lib.pattern_ascii_pointer_3.argtypes = [ctypes.POINTER(ctypes.c_char)]
    c_lib.pattern_ascii_pointer_4.argtypes = [ctypes.POINTER(ctypes.c_char), ctypes.c_uint32]
    c_lib.pattern_ascii_pointer_5.argtypes = [ctypes.POINTER(ctypes.c_char), ctypes.c_uint32]
    c_lib.pattern_ascii_pointer_return_slice.argtypes = []
    c_lib.pattern_string_1.argtypes = [Utf8String]
    c_lib.pattern_string_2.argtypes = [Utf8String]
    c_lib.pattern_string_3.argtypes = []
    c_lib.pattern_string_4.argtypes = [UseString]
    c_lib.pattern_string_5.argtypes = [UseString]
    c_lib.pattern_string_6a.argtypes = [ctypes.POINTER(UseString)]
    c_lib.pattern_string_6b.argtypes = [ctypes.POINTER(UseString)]
    c_lib.pattern_string_7.argtypes = [SliceUtf8String, ctypes.c_uint64]
    c_lib.pattern_string_8.argtypes = [SliceUseString, ctypes.c_uint64]
    c_lib.pattern_string_9.argtypes = []
    c_lib.pattern_ffi_slice_1.argtypes = [SliceU32]
    c_lib.pattern_ffi_slice_1b.argtypes = [SliceMutU32]
    c_lib.pattern_ffi_slice_2.argtypes = [SliceVec3f32, ctypes.c_int32]
    c_lib.pattern_ffi_slice_3.argtypes = [SliceMutU8, ctypes.CFUNCTYPE(None, SliceMutU8, ctypes.c_void_p)]
    c_lib.pattern_ffi_slice_4.argtypes = [SliceU8, SliceMutU8]
    c_lib.pattern_ffi_slice_5.argtypes = [ctypes.POINTER(SliceU8), ctypes.POINTER(SliceMutU8)]
    c_lib.pattern_ffi_slice_6.argtypes = [ctypes.POINTER(SliceMutU8), ctypes.CFUNCTYPE(ctypes.c_uint8, ctypes.c_uint8, ctypes.c_void_p)]
    c_lib.pattern_ffi_slice_8.argtypes = [ctypes.POINTER(SliceMutCharArray), ctypes.CFUNCTYPE(None, CharArray, ctypes.c_void_p)]
    c_lib.pattern_ffi_slice_delegate.argtypes = [ctypes.CFUNCTYPE(ctypes.c_uint8, SliceU8, ctypes.c_void_p)]
    c_lib.pattern_ffi_slice_delegate_huge.argtypes = [ctypes.CFUNCTYPE(Vec3f32, SliceVec3f32, ctypes.c_void_p)]
    c_lib.pattern_ffi_option_1.argtypes = [TODO]
    c_lib.pattern_ffi_option_2.argtypes = [TODO]
    c_lib.pattern_ffi_option_3.argtypes = [TODO]
    c_lib.pattern_ffi_bool.argtypes = [ctypes.c_uint8]
    c_lib.pattern_ffi_cchar.argtypes = [ctypes.c_char]
    c_lib.pattern_ffi_cchar_const_pointer.argtypes = [ctypes.POINTER(ctypes.c_char)]
    c_lib.pattern_ffi_cchar_mut_pointer.argtypes = [ctypes.POINTER(ctypes.c_char)]
    c_lib.pattern_result_1.argtypes = [ResultU32Error]
    c_lib.pattern_result_2.argtypes = []
    c_lib.pattern_result_3.argtypes = [ResultError]
    c_lib.pattern_api_guard.argtypes = []
    c_lib.pattern_callback_1.argtypes = [ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p), ctypes.c_uint32]
    c_lib.pattern_callback_2.argtypes = [ctypes.CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_void_p)]
    c_lib.pattern_callback_4.argtypes = [ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p), ctypes.c_uint32]
    c_lib.pattern_callback_5.argtypes = []
    c_lib.pattern_callback_6.argtypes = []
    c_lib.pattern_callback_7.argtypes = [ctypes.CFUNCTYPE(ResultError, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p), ctypes.CFUNCTYPE(None, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p), ctypes.c_int32, ctypes.c_int32, ctypes.POINTER(ctypes.c_int32)]
    c_lib.pattern_callback_8.argtypes = [ctypes.CFUNCTYPE(None, Utf8String, ctypes.c_void_p), ctypes.CFUNCTYPE(None, UseString, ctypes.c_void_p), Utf8String]
    c_lib.pattern_surrogates_1.argtypes = [Local, ctypes.POINTER(Container)]
    c_lib.pattern_vec_1.argtypes = []
    c_lib.pattern_vec_2.argtypes = [VecU8]
    c_lib.pattern_vec_3.argtypes = [VecU8]
    c_lib.pattern_vec_4.argtypes = [ctypes.POINTER(VecU8)]
    c_lib.service_async_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_async_new.argtypes = []
    c_lib.service_async_return_after_ms.argtypes = [ctypes.c_void_p, ctypes.c_uint64, ctypes.c_uint64, ctypes.CFUNCTYPE(None, ctypes.POINTER(ResultU64Error), ctypes.c_void_p)]
    c_lib.service_async_process_struct.argtypes = [ctypes.c_void_p, NestedArray, ctypes.CFUNCTYPE(None, ctypes.POINTER(ResultNestedArrayError), ctypes.c_void_p)]
    c_lib.service_async_handle_string.argtypes = [ctypes.c_void_p, Utf8String, ctypes.CFUNCTYPE(None, ctypes.POINTER(ResultUtf8StringError), ctypes.c_void_p)]
    c_lib.service_async_handle_nested_string.argtypes = [ctypes.c_void_p, Utf8String, ctypes.CFUNCTYPE(None, ctypes.POINTER(ResultUseStringError), ctypes.c_void_p)]
    c_lib.service_async_callback_string.argtypes = [ctypes.c_void_p, Utf8String, ctypes.CFUNCTYPE(None, Utf8String, ctypes.c_void_p)]
    c_lib.service_async_fail.argtypes = [ctypes.c_void_p, ctypes.CFUNCTYPE(None, ctypes.POINTER(ResultError), ctypes.c_void_p)]
    c_lib.service_async_bad.argtypes = [ctypes.c_void_p]
    c_lib.service_basic_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_basic_new.argtypes = []
    c_lib.service_main_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_main_new.argtypes = [ctypes.c_uint32]
    c_lib.service_dependent_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_dependent_from_main.argtypes = [ctypes.c_void_p]
    c_lib.service_dependent_get.argtypes = [ctypes.c_void_p]
    c_lib.service_result_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_result_new.argtypes = []
    c_lib.service_result_test.argtypes = [ctypes.c_void_p]
    c_lib.service_on_panic_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_on_panic_new.argtypes = []
    c_lib.service_on_panic_return_result.argtypes = [ctypes.c_void_p, ctypes.c_uint32]
    c_lib.service_on_panic_return_default_value.argtypes = [ctypes.c_void_p, ctypes.c_uint32]
    c_lib.service_on_panic_return_ub_on_panic.argtypes = [ctypes.c_void_p]
    c_lib.service_callbacks_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_callbacks_new.argtypes = []
    c_lib.service_callbacks_callback_simple.argtypes = [ctypes.c_void_p, ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p)]
    c_lib.service_callbacks_callback_ffi_return.argtypes = [ctypes.c_void_p, ctypes.CFUNCTYPE(ResultError, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p)]
    c_lib.service_callbacks_callback_with_slice.argtypes = [ctypes.c_void_p, ctypes.CFUNCTYPE(ResultError, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p), SliceI32]
    c_lib.service_callbacks_set_delegate_table.argtypes = [ctypes.c_void_p, CallbackTable]
    c_lib.service_callbacks_invoke_delegates.argtypes = [ctypes.c_void_p]
    c_lib.service_ignoring_methods_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_ignoring_methods_new.argtypes = []
    c_lib.service_multiple_ctors_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_multiple_ctors_new_with.argtypes = [ctypes.c_uint32]
    c_lib.service_multiple_ctors_new_without.argtypes = []
    c_lib.service_multiple_ctors_new_with_string.argtypes = [ctypes.POINTER(ctypes.c_char)]
    c_lib.service_multiple_ctors_new_failing.argtypes = [ctypes.c_uint8]
    c_lib.service_various_slices_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_various_slices_new.argtypes = []
    c_lib.service_various_slices_mut_self.argtypes = [ctypes.c_void_p, SliceU8]
    c_lib.service_various_slices_mut_self_void.argtypes = [ctypes.c_void_p, SliceBool]
    c_lib.service_various_slices_mut_self_ref.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8)]
    c_lib.service_various_slices_mut_self_ref_slice.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8), SliceU8]
    c_lib.service_various_slices_mut_self_ref_slice_limited.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8), SliceU8, SliceU8]
    c_lib.service_various_slices_mut_self_ffi_error.argtypes = [ctypes.c_void_p, SliceMutU8]
    c_lib.service_various_slices_mut_self_no_error.argtypes = [ctypes.c_void_p, SliceMutU8]
    c_lib.service_various_slices_return_slice.argtypes = [ctypes.c_void_p]
    c_lib.service_various_slices_return_slice_mut.argtypes = [ctypes.c_void_p]
    c_lib.service_strings_destroy.argtypes = [ctypes.c_void_p]
    c_lib.service_strings_new.argtypes = []
    c_lib.service_strings_new_string.argtypes = [Utf8String]
    c_lib.service_strings_pass_cstr.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_char)]
    c_lib.service_strings_return_cstr.argtypes = [ctypes.c_void_p]
    c_lib.service_strings_callback_string.argtypes = [ctypes.c_void_p, Utf8String, ctypes.CFUNCTYPE(None, Utf8String, ctypes.c_void_p)]

    c_lib.interoptopus_string_create.restype = ctypes.c_int64
    c_lib.interoptopus_string_destroy.restype = ctypes.c_int64
    c_lib.alignment_1.restype = Packed2
    c_lib.array_1.restype = ctypes.c_uint8
    c_lib.array_2.restype = Array
    c_lib.char_array_1.restype = CharArray
    c_lib.char_array_2.restype = CharArray
    c_lib.char_array_3.restype = ctypes.c_uint8
    c_lib.nested_array_1.restype = NestedArray
    c_lib.nested_array_3.restype = ctypes.c_uint8
    c_lib.enums_2.restype = ctypes.c_int
    c_lib.enums_3.restype = ctypes.POINTER(ctypes.c_int)
    c_lib.fnptr_1.restype = ctypes.c_uint8
    c_lib.generic_1a.restype = ctypes.c_uint32
    c_lib.generic_1b.restype = ctypes.c_uint8
    c_lib.generic_1c.restype = ctypes.c_uint8
    c_lib.generic_2.restype = ctypes.c_uint8
    c_lib.generic_3.restype = ctypes.c_uint8
    c_lib.generic_4.restype = ctypes.c_uint8
    c_lib.generic_5.restype = ctypes.c_bool
    c_lib.meta_ambiguous_1.restype = Vec1
    c_lib.meta_ambiguous_2.restype = Vec2
    c_lib.meta_ambiguous_3.restype = ctypes.c_bool
    c_lib.meta_documented.restype = ctypes.c_int
    c_lib.meta_renamed.restype = ctypes.c_int
    c_lib.namespaced_inner_option.restype = TODO
    c_lib.namespaced_inner_slice.restype = SliceVec
    c_lib.namespaced_inner_slice_mut.restype = SliceMutVec
    c_lib.namespaced_type.restype = Vec
    c_lib.primitive_args_5.restype = ctypes.c_int64
    c_lib.primitive_args_10.restype = ctypes.c_int64
    c_lib.primitive_bool.restype = ctypes.c_bool
    c_lib.primitive_f32.restype = ctypes.c_float
    c_lib.primitive_f64.restype = ctypes.c_double
    c_lib.primitive_i16.restype = ctypes.c_int16
    c_lib.primitive_i32.restype = ctypes.c_int32
    c_lib.primitive_i64.restype = ctypes.c_int64
    c_lib.primitive_i8.restype = ctypes.c_int8
    c_lib.primitive_u16.restype = ctypes.c_uint16
    c_lib.primitive_u32.restype = ctypes.c_uint32
    c_lib.primitive_u64.restype = ctypes.c_uint64
    c_lib.primitive_u8.restype = ctypes.c_uint8
    c_lib.ptr1.restype = ctypes.POINTER(ctypes.c_int64)
    c_lib.ptr2.restype = ctypes.POINTER(ctypes.POINTER(ctypes.c_int64))
    c_lib.ptr3.restype = ctypes.POINTER(ctypes.c_int64)
    c_lib.ref1.restype = ctypes.POINTER(ctypes.c_int64)
    c_lib.ref2.restype = ctypes.POINTER(ctypes.c_int64)
    c_lib.ref3.restype = ctypes.c_bool
    c_lib.ref4.restype = ctypes.c_bool
    c_lib.struct1.restype = Tupled
    c_lib.struct2.restype = ResultError
    c_lib.struct3.restype = ctypes.c_bool
    c_lib.pattern_ascii_pointer_1.restype = ctypes.c_uint32
    c_lib.pattern_ascii_pointer_2.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.pattern_ascii_pointer_3.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.pattern_ascii_pointer_4.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.pattern_ascii_pointer_5.restype = ctypes.c_uint8
    c_lib.pattern_ascii_pointer_return_slice.restype = SliceUseCStrPtr
    c_lib.pattern_string_1.restype = Utf8String
    c_lib.pattern_string_2.restype = ctypes.c_uint32
    c_lib.pattern_string_3.restype = Utf8String
    c_lib.pattern_string_4.restype = UseString
    c_lib.pattern_string_5.restype = ResultUseStringError
    c_lib.pattern_string_6a.restype = ResultError
    c_lib.pattern_string_6b.restype = ResultError
    c_lib.pattern_string_7.restype = ResultUtf8StringError
    c_lib.pattern_string_8.restype = ResultUseStringError
    c_lib.pattern_string_9.restype = ResultUtf8StringError
    c_lib.pattern_ffi_slice_1.restype = ctypes.c_uint32
    c_lib.pattern_ffi_slice_1b.restype = ctypes.c_uint32
    c_lib.pattern_ffi_slice_2.restype = Vec3f32
    c_lib.pattern_ffi_slice_delegate.restype = ctypes.c_uint8
    c_lib.pattern_ffi_slice_delegate_huge.restype = Vec3f32
    c_lib.pattern_ffi_option_1.restype = TODO
    c_lib.pattern_ffi_option_2.restype = Inner
    c_lib.pattern_ffi_option_3.restype = TODO
    c_lib.pattern_ffi_bool.restype = ctypes.c_uint8
    c_lib.pattern_ffi_cchar.restype = ctypes.c_char
    c_lib.pattern_ffi_cchar_const_pointer.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.pattern_ffi_cchar_mut_pointer.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.pattern_result_1.restype = ResultU32Error
    c_lib.pattern_result_2.restype = ResultError
    c_lib.pattern_result_3.restype = ResultError
    c_lib.pattern_api_guard.restype = ctypes.c_uint64
    c_lib.pattern_callback_1.restype = ctypes.c_uint32
    c_lib.pattern_callback_2.restype = ctypes.CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_void_p)
    c_lib.pattern_callback_4.restype = ctypes.c_uint32
    c_lib.pattern_callback_5.restype = ctypes.CFUNCTYPE(None, ctypes.c_void_p)
    c_lib.pattern_callback_6.restype = ctypes.CFUNCTYPE(ctypes.c_int32, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p)
    c_lib.pattern_callback_7.restype = ResultError
    c_lib.pattern_vec_1.restype = VecU8
    c_lib.pattern_vec_3.restype = VecU8
    c_lib.pattern_vec_4.restype = VecU8
    c_lib.service_async_destroy.restype = ResultConstPtrServiceAsyncError
    c_lib.service_async_new.restype = ResultConstPtrServiceAsyncError
    c_lib.service_async_return_after_ms.restype = ResultError
    c_lib.service_async_process_struct.restype = ResultError
    c_lib.service_async_handle_string.restype = ResultError
    c_lib.service_async_handle_nested_string.restype = ResultError
    c_lib.service_async_fail.restype = ResultError
    c_lib.service_basic_destroy.restype = ResultConstPtrServiceBasicError
    c_lib.service_basic_new.restype = ResultConstPtrServiceBasicError
    c_lib.service_main_destroy.restype = ResultConstPtrServiceMainError
    c_lib.service_main_new.restype = ResultConstPtrServiceMainError
    c_lib.service_dependent_destroy.restype = ResultConstPtrServiceDependentError
    c_lib.service_dependent_from_main.restype = ResultConstPtrServiceDependentError
    c_lib.service_dependent_get.restype = ctypes.c_uint32
    c_lib.service_result_destroy.restype = ResultConstPtrServiceResultError
    c_lib.service_result_new.restype = ResultConstPtrServiceResultError
    c_lib.service_result_test.restype = ResultError
    c_lib.service_on_panic_destroy.restype = ResultConstPtrServiceOnPanicError
    c_lib.service_on_panic_new.restype = ResultConstPtrServiceOnPanicError
    c_lib.service_on_panic_return_result.restype = ResultError
    c_lib.service_on_panic_return_default_value.restype = ctypes.c_uint32
    c_lib.service_on_panic_return_ub_on_panic.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.service_callbacks_destroy.restype = ResultConstPtrServiceCallbacksError
    c_lib.service_callbacks_new.restype = ResultConstPtrServiceCallbacksError
    c_lib.service_callbacks_callback_simple.restype = ResultError
    c_lib.service_callbacks_callback_ffi_return.restype = ResultError
    c_lib.service_callbacks_callback_with_slice.restype = ResultError
    c_lib.service_callbacks_invoke_delegates.restype = ResultError
    c_lib.service_ignoring_methods_destroy.restype = ResultConstPtrServiceIgnoringMethodsError
    c_lib.service_ignoring_methods_new.restype = ResultConstPtrServiceIgnoringMethodsError
    c_lib.service_multiple_ctors_destroy.restype = ResultConstPtrServiceMultipleCtorsError
    c_lib.service_multiple_ctors_new_with.restype = ResultConstPtrServiceMultipleCtorsError
    c_lib.service_multiple_ctors_new_without.restype = ResultConstPtrServiceMultipleCtorsError
    c_lib.service_multiple_ctors_new_with_string.restype = ResultConstPtrServiceMultipleCtorsError
    c_lib.service_multiple_ctors_new_failing.restype = ResultConstPtrServiceMultipleCtorsError
    c_lib.service_various_slices_destroy.restype = ResultConstPtrServiceVariousSlicesError
    c_lib.service_various_slices_new.restype = ResultConstPtrServiceVariousSlicesError
    c_lib.service_various_slices_mut_self.restype = ctypes.c_uint8
    c_lib.service_various_slices_mut_self_ref.restype = ctypes.c_uint8
    c_lib.service_various_slices_mut_self_ref_slice.restype = ctypes.c_uint8
    c_lib.service_various_slices_mut_self_ref_slice_limited.restype = ctypes.c_uint8
    c_lib.service_various_slices_mut_self_ffi_error.restype = ResultError
    c_lib.service_various_slices_mut_self_no_error.restype = ResultError
    c_lib.service_various_slices_return_slice.restype = SliceU32
    c_lib.service_various_slices_return_slice_mut.restype = SliceMutU32
    c_lib.service_strings_destroy.restype = ResultConstPtrServiceStringsError
    c_lib.service_strings_new.restype = ResultConstPtrServiceStringsError
    c_lib.service_strings_new_string.restype = ResultConstPtrServiceStringsError
    c_lib.service_strings_return_cstr.restype = ctypes.POINTER(ctypes.c_char)


def interoptopus_string_create(utf8: ctypes.c_void_p, len: int, rval: ctypes.POINTER(Utf8String)) -> int:
    return c_lib.interoptopus_string_create(utf8, len, rval)

def interoptopus_string_destroy(utf8) -> int:
    return c_lib.interoptopus_string_destroy(utf8)

def interoptopus_vec_TODO_destroy(ignored):
    """ TODO: This should be macro generated."""
    return c_lib.interoptopus_vec_TODO_destroy(ignored)

def alignment_1(a: Packed1) -> Packed2:
    return c_lib.alignment_1(a)

def array_1(x: Array) -> int:
    return c_lib.array_1(x)

def array_2() -> Array:
    return c_lib.array_2()

def array_3(arr: ctypes.POINTER(Array)):
    return c_lib.array_3(arr)

def char_array_1() -> CharArray:
    return c_lib.char_array_1()

def char_array_2(arr: CharArray) -> CharArray:
    return c_lib.char_array_2(arr)

def char_array_3(arr: ctypes.POINTER(CharArray)) -> int:
    return c_lib.char_array_3(arr)

def nested_array_1() -> NestedArray:
    return c_lib.nested_array_1()

def nested_array_2(result: ctypes.POINTER(NestedArray)):
    return c_lib.nested_array_2(result)

def nested_array_3(input: NestedArray) -> int:
    return c_lib.nested_array_3(input)

def behavior_sleep(millis: int):
    return c_lib.behavior_sleep(millis)

def behavior_panics():
    return c_lib.behavior_panics()

def enums_1(ignored: TODO):
    return c_lib.enums_1(ignored)

def enums_2(x: TODO) -> TODO:
    return c_lib.enums_2(x)

def enums_3(x: ctypes.POINTER(ctypes.c_int)) -> ctypes.POINTER(ctypes.c_int):
    return c_lib.enums_3(x)

def fnptr_1(callback, value: int) -> int:
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_u8_rval_u8(callback)

    return c_lib.fnptr_1(callback, value)

def fnptr_2(callback, value: CharArray):
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_CharArray(callback)

    return c_lib.fnptr_2(callback, value)

def generic_1a(x: Genericu32, y: Phantomu8) -> int:
    return c_lib.generic_1a(x, y)

def generic_1b(x: Genericu8, y: Phantomu8) -> int:
    return c_lib.generic_1b(x, y)

def generic_1c(x: ctypes.POINTER(Genericu8), y: ctypes.POINTER(Genericu8)) -> int:
    return c_lib.generic_1c(x, y)

def generic_2(x: ctypes.c_void_p) -> int:
    return c_lib.generic_2(x)

def generic_3(x: ctypes.c_void_p) -> int:
    return c_lib.generic_3(x)

def generic_4(x: ctypes.c_void_p) -> int:
    return c_lib.generic_4(x)

def generic_5(x: Weird1u32, y: Weird2u8) -> bool:
    return c_lib.generic_5(x, y)

def meta_ambiguous_1(x: Vec1) -> Vec1:
    return c_lib.meta_ambiguous_1(x)

def meta_ambiguous_2(x: Vec2) -> Vec2:
    return c_lib.meta_ambiguous_2(x)

def meta_ambiguous_3(x: Vec1, y: Vec2) -> bool:
    return c_lib.meta_ambiguous_3(x, y)

def meta_documented(x: StructDocumented) -> TODO:
    """ This function has documentation."""
    return c_lib.meta_documented(x)

def meta_visibility1(x: Visibility1, y: Visibility2):
    return c_lib.meta_visibility1(x, y)

def meta_renamed(x: StructRenamed) -> TODO:
    return c_lib.meta_renamed(x)

def namespaced_inner_option(x: TODO) -> TODO:
    return c_lib.namespaced_inner_option(x)

def namespaced_inner_slice(x: SliceVec | ctypes.Array[Vec]) -> SliceVec:
    if hasattr(x, "_length_") and getattr(x, "_type_", "") == Vec:
        x = SliceVec(data=ctypes.cast(x, ctypes.POINTER(Vec)), len=len(x))

    return c_lib.namespaced_inner_slice(x)

def namespaced_inner_slice_mut(x: SliceMutVec | ctypes.Array[Vec]) -> SliceMutVec:
    if hasattr(x, "_length_") and getattr(x, "_type_", "") == Vec:
        x = SliceMutVec(data=ctypes.cast(x, ctypes.POINTER(Vec)), len=len(x))

    return c_lib.namespaced_inner_slice_mut(x)

def namespaced_type(x: Vec) -> Vec:
    return c_lib.namespaced_type(x)

def primitive_args_5(x0: int, x1: int, x2: int, x3: int, x4: int) -> int:
    return c_lib.primitive_args_5(x0, x1, x2, x3, x4)

def primitive_args_10(x0: int, x1: int, x2: int, x3: int, x4: int, x5: int, x6: int, x7: int, x8: int, x9: int) -> int:
    return c_lib.primitive_args_10(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)

def primitive_bool(x: bool) -> bool:
    return c_lib.primitive_bool(x)

def primitive_f32(x: float) -> float:
    return c_lib.primitive_f32(x)

def primitive_f64(x: float) -> float:
    return c_lib.primitive_f64(x)

def primitive_i16(x: int) -> int:
    return c_lib.primitive_i16(x)

def primitive_i32(x: int) -> int:
    return c_lib.primitive_i32(x)

def primitive_i64(x: int) -> int:
    return c_lib.primitive_i64(x)

def primitive_i8(x: int) -> int:
    return c_lib.primitive_i8(x)

def primitive_u16(x: int) -> int:
    return c_lib.primitive_u16(x)

def primitive_u32(x: int) -> int:
    return c_lib.primitive_u32(x)

def primitive_u64(x: int) -> int:
    return c_lib.primitive_u64(x)

def primitive_u8(x: int) -> int:
    return c_lib.primitive_u8(x)

def primitive_void():
    return c_lib.primitive_void()

def primitive_void2():
    return c_lib.primitive_void2()

def ptr1(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    return c_lib.ptr1(x)

def ptr2(x: ctypes.POINTER(ctypes.POINTER(ctypes.c_int64))) -> ctypes.POINTER(ctypes.POINTER(ctypes.c_int64)):
    return c_lib.ptr2(x)

def ptr3(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    """ # Safety

 Parameter x must point to valid data."""
    return c_lib.ptr3(x)

def ref1(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    return c_lib.ref1(x)

def ref2(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    return c_lib.ref2(x)

def ref3(x: ctypes.POINTER(ctypes.c_int64)) -> bool:
    return c_lib.ref3(x)

def ref4(x: ctypes.POINTER(ctypes.c_int64)) -> bool:
    return c_lib.ref4(x)

def struct1(x: Tupled) -> Tupled:
    return c_lib.struct1(x)

def struct2(a: Vec3f32, b: ctypes.POINTER(Tupled)):
    return c_lib.struct2(a, b)

def struct3(x: BoolField) -> bool:
    return c_lib.struct3(x)

def pattern_ascii_pointer_1(x: bytes) -> int:
    if not hasattr(x, "__ctypes_from_outparam__"):
        x = ctypes.cast(x, ctypes.POINTER(ctypes.c_char))
    return c_lib.pattern_ascii_pointer_1(x)

def pattern_ascii_pointer_2() -> bytes:
    rval = c_lib.pattern_ascii_pointer_2()
    return ctypes.string_at(rval)

def pattern_ascii_pointer_3(x: bytes) -> bytes:
    if not hasattr(x, "__ctypes_from_outparam__"):
        x = ctypes.cast(x, ctypes.POINTER(ctypes.c_char))
    rval = c_lib.pattern_ascii_pointer_3(x)
    return ctypes.string_at(rval)

def pattern_ascii_pointer_4(x: bytes, l: int) -> bytes:
    if not hasattr(x, "__ctypes_from_outparam__"):
        x = ctypes.cast(x, ctypes.POINTER(ctypes.c_char))
    rval = c_lib.pattern_ascii_pointer_4(x, l)
    return ctypes.string_at(rval)

def pattern_ascii_pointer_5(x: bytes, i: int) -> int:
    if not hasattr(x, "__ctypes_from_outparam__"):
        x = ctypes.cast(x, ctypes.POINTER(ctypes.c_char))
    return c_lib.pattern_ascii_pointer_5(x, i)

def pattern_ascii_pointer_return_slice() -> SliceUseCStrPtr:
    return c_lib.pattern_ascii_pointer_return_slice()

def pattern_string_1(x):
    return c_lib.pattern_string_1(x)

def pattern_string_2(x) -> int:
    return c_lib.pattern_string_2(x)

def pattern_string_3():
    return c_lib.pattern_string_3()

def pattern_string_4(x: UseString) -> UseString:
    return c_lib.pattern_string_4(x)

def pattern_string_5(x: UseString):
    return c_lib.pattern_string_5(x)

def pattern_string_6a(ignored: ctypes.POINTER(UseString)):
    return c_lib.pattern_string_6a(ignored)

def pattern_string_6b(y: ctypes.POINTER(UseString)):
    return c_lib.pattern_string_6b(y)

def pattern_string_7(x: SliceUtf8String | ctypes.Array[Utf8String], i: int):
    if hasattr(x, "_length_") and getattr(x, "_type_", "") == Utf8String:
        x = SliceUtf8String(data=ctypes.cast(x, ctypes.POINTER(Utf8String)), len=len(x))

    return c_lib.pattern_string_7(x, i)

def pattern_string_8(x: SliceUseString | ctypes.Array[UseString], i: int):
    if hasattr(x, "_length_") and getattr(x, "_type_", "") == UseString:
        x = SliceUseString(data=ctypes.cast(x, ctypes.POINTER(UseString)), len=len(x))

    return c_lib.pattern_string_8(x, i)

def pattern_string_9():
    return c_lib.pattern_string_9()

def pattern_ffi_slice_1(ffi_slice: SliceU32 | ctypes.Array[ctypes.c_uint32]) -> int:
    if hasattr(ffi_slice, "_length_") and getattr(ffi_slice, "_type_", "") == ctypes.c_uint32:
        ffi_slice = SliceU32(data=ctypes.cast(ffi_slice, ctypes.POINTER(ctypes.c_uint32)), len=len(ffi_slice))

    return c_lib.pattern_ffi_slice_1(ffi_slice)

def pattern_ffi_slice_1b(ffi_slice: SliceMutU32 | ctypes.Array[ctypes.c_uint32]) -> int:
    if hasattr(ffi_slice, "_length_") and getattr(ffi_slice, "_type_", "") == ctypes.c_uint32:
        ffi_slice = SliceMutU32(data=ctypes.cast(ffi_slice, ctypes.POINTER(ctypes.c_uint32)), len=len(ffi_slice))

    return c_lib.pattern_ffi_slice_1b(ffi_slice)

def pattern_ffi_slice_2(ffi_slice: SliceVec3f32 | ctypes.Array[Vec3f32], i: int) -> Vec3f32:
    if hasattr(ffi_slice, "_length_") and getattr(ffi_slice, "_type_", "") == Vec3f32:
        ffi_slice = SliceVec3f32(data=ctypes.cast(ffi_slice, ctypes.POINTER(Vec3f32)), len=len(ffi_slice))

    return c_lib.pattern_ffi_slice_2(ffi_slice, i)

def pattern_ffi_slice_3(slice: SliceMutU8 | ctypes.Array[ctypes.c_uint8], callback):
    if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
        slice = SliceMutU8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_SliceMutU8_ConstPtr(callback)

    return c_lib.pattern_ffi_slice_3(slice, callback)

def pattern_ffi_slice_4(slice: SliceU8 | ctypes.Array[ctypes.c_uint8], slice2: SliceMutU8 | ctypes.Array[ctypes.c_uint8]):
    if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
        slice = SliceU8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

    if hasattr(slice2, "_length_") and getattr(slice2, "_type_", "") == ctypes.c_uint8:
        slice2 = SliceMutU8(data=ctypes.cast(slice2, ctypes.POINTER(ctypes.c_uint8)), len=len(slice2))

    return c_lib.pattern_ffi_slice_4(slice, slice2)

def pattern_ffi_slice_5(slice: ctypes.POINTER(SliceU8), slice2: ctypes.POINTER(SliceMutU8)):
    """ It is (probably?) UB to call this function with the same FFI slice data at the same time."""
    return c_lib.pattern_ffi_slice_5(slice, slice2)

def pattern_ffi_slice_6(slice: ctypes.POINTER(SliceMutU8), callback):
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_u8_ConstPtr_rval_u8(callback)

    return c_lib.pattern_ffi_slice_6(slice, callback)

def pattern_ffi_slice_8(slice: ctypes.POINTER(SliceMutCharArray), callback):
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_CharArray_ConstPtr(callback)

    return c_lib.pattern_ffi_slice_8(slice, callback)

def pattern_ffi_slice_delegate(callback) -> int:
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_SliceU8_ConstPtr_rval_u8(callback)

    return c_lib.pattern_ffi_slice_delegate(callback)

def pattern_ffi_slice_delegate_huge(callback) -> Vec3f32:
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_SliceVec3f32_ConstPtr_rval_Vec3f32(callback)

    return c_lib.pattern_ffi_slice_delegate_huge(callback)

def pattern_ffi_option_1(x: TODO) -> TODO:
    return c_lib.pattern_ffi_option_1(x)

def pattern_ffi_option_2(x: TODO) -> Inner:
    return c_lib.pattern_ffi_option_2(x)

def pattern_ffi_option_3(x: TODO) -> TODO:
    return c_lib.pattern_ffi_option_3(x)

def pattern_ffi_bool(ffi_bool):
    return c_lib.pattern_ffi_bool(ffi_bool)

def pattern_ffi_cchar(ffi_cchar: ctypes.c_char) -> ctypes.c_char:
    return c_lib.pattern_ffi_cchar(ffi_cchar)

def pattern_ffi_cchar_const_pointer(ffi_cchar: ctypes.POINTER(ctypes.c_char)) -> ctypes.POINTER(ctypes.c_char):
    return c_lib.pattern_ffi_cchar_const_pointer(ffi_cchar)

def pattern_ffi_cchar_mut_pointer(ffi_cchar: ctypes.POINTER(ctypes.c_char)) -> ctypes.POINTER(ctypes.c_char):
    return c_lib.pattern_ffi_cchar_mut_pointer(ffi_cchar)

def pattern_result_1(x):
    return c_lib.pattern_result_1(x)

def pattern_result_2():
    return c_lib.pattern_result_2()

def pattern_result_3(x):
    return c_lib.pattern_result_3(x)

def pattern_api_guard():
    return c_lib.pattern_api_guard()

def pattern_callback_1(callback, x: int) -> int:
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_u32_ConstPtr_rval_u32(callback)

    return c_lib.pattern_callback_1(callback, x)

def pattern_callback_2(callback):
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_ConstPtr_ConstPtr(callback)

    return c_lib.pattern_callback_2(callback)

def pattern_callback_4(callback, x: int) -> int:
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_u32_ConstPtr_rval_u32(callback)

    return c_lib.pattern_callback_4(callback, x)

def pattern_callback_5():
    return c_lib.pattern_callback_5()

def pattern_callback_6():
    return c_lib.pattern_callback_6()

def pattern_callback_7(c1, c2, x: int, i: int, o: ctypes.POINTER(ctypes.c_int32)):
    if not hasattr(c1, "__ctypes_from_outparam__"):
        c1 = callbacks.fn_i32_i32_ConstPtr_rval_ResultError(c1)

    if not hasattr(c2, "__ctypes_from_outparam__"):
        c2 = callbacks.fn_i32_i32_ConstPtr(c2)

    return c_lib.pattern_callback_7(c1, c2, x, i, o)

def pattern_callback_8(cb, cb2, s):
    if not hasattr(cb, "__ctypes_from_outparam__"):
        cb = callbacks.fn_Utf8String_ConstPtr(cb)

    if not hasattr(cb2, "__ctypes_from_outparam__"):
        cb2 = callbacks.fn_UseString_ConstPtr(cb2)

    return c_lib.pattern_callback_8(cb, cb2, s)

def pattern_surrogates_1(s: Local, c: ctypes.POINTER(Container)):
    return c_lib.pattern_surrogates_1(s, c)

def pattern_vec_1():
    return c_lib.pattern_vec_1()

def pattern_vec_2(ignored):
    return c_lib.pattern_vec_2(ignored)

def pattern_vec_3(v):
    return c_lib.pattern_vec_3(v)

def pattern_vec_4(v: ctypes.POINTER(VecU8)):
    return c_lib.pattern_vec_4(v)



U8 = 255
F32_MIN_POSITIVE = 0.000000000000000000000000000000000000011754944
COMPUTED_I32 = -2147483647


TRUE = ctypes.c_uint8(1)
FALSE = ctypes.c_uint8(0)


def _errcheck(returned, success):
    """Checks for FFIErrors and converts them to an exception."""
    if returned == success: return
    else: raise Exception(f"Function returned error: {returned}")


class CallbackVars(object):
    """Helper to be used `lambda x: setattr(cv, "x", x)` when getting values from callbacks."""
    def __str__(self):
        rval = ""
        for var in  filter(lambda x: "__" not in x, dir(self)):
            rval += f"{var}: {getattr(self, var)}"
        return rval


class _Iter(object):
    """Helper for slice iterators."""
    def __init__(self, target):
        self.i = 0
        self.target = target

    def __iter__(self):
        self.i = 0
        return self

    def __next__(self):
        if self.i >= self.target.len:
            raise StopIteration()
        rval = self.target[self.i]
        self.i += 1
        return rval


class EnumDocumented:
    """ Documented enum."""
    #  Variant A.
    A = 0
    #  Variant B.
    B = 1
    #  Variant B.
    C = 2


class EnumPayload:
    A = 0
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN


class EnumRenamed:
    X = 0


class Error:
    Fail = 0


class Utf8String(ctypes.Structure):
    """ UTF-8 string marshalling helper.
 A highly dangerous 'use once type' that has ownership semantics!
 Once passed over an FFI boundary 'the other side' is meant to own
 (and free) it. Rust handles that fine, but if in C# you put this
 in a struct and then call Rust multiple times with that struct 
 you'll free the same pointer multiple times, and get UB!"""

    # These fields represent the underlying C data layout
    _fields_ = [
        ("ptr", ctypes.POINTER(ctypes.c_uint8)),
        ("len", ctypes.c_uint64),
        ("capacity", ctypes.c_uint64),
    ]

    def __init__(self, ptr: ctypes.POINTER(ctypes.c_uint8) = None, len: int = None, capacity: int = None):
        if ptr is not None:
            self.ptr = ptr
        if len is not None:
            self.len = len
        if capacity is not None:
            self.capacity = capacity

    @property
    def ptr(self) -> ctypes.POINTER(ctypes.c_uint8):
        return ctypes.Structure.__get__(self, "ptr")

    @ptr.setter
    def ptr(self, value: ctypes.POINTER(ctypes.c_uint8)):
        return ctypes.Structure.__set__(self, "ptr", value)

    @property
    def len(self) -> int:
        return ctypes.Structure.__get__(self, "len")

    @len.setter
    def len(self, value: int):
        return ctypes.Structure.__set__(self, "len", value)

    @property
    def capacity(self) -> int:
        return ctypes.Structure.__get__(self, "capacity")

    @capacity.setter
    def capacity(self, value: int):
        return ctypes.Structure.__set__(self, "capacity", value)


class BoolField(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("val", ctypes.c_bool),
    ]

    def __init__(self, val: bool = None):
        if val is not None:
            self.val = val

    @property
    def val(self) -> bool:
        return ctypes.Structure.__get__(self, "val")

    @val.setter
    def val(self, value: bool):
        return ctypes.Structure.__set__(self, "val", value)


class ExtraTypef32(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_float),
    ]

    def __init__(self, x: float = None):
        if x is not None:
            self.x = x

    @property
    def x(self) -> float:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: float):
        return ctypes.Structure.__set__(self, "x", value)


class Inner(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_float),
    ]

    def __init__(self, x: float = None):
        if x is not None:
            self.x = x

    @property
    def x(self) -> float:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: float):
        return ctypes.Structure.__set__(self, "x", value)


class Local(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_uint32),
    ]

    def __init__(self, x: int = None):
        if x is not None:
            self.x = x

    @property
    def x(self) -> int:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: int):
        return ctypes.Structure.__set__(self, "x", value)


class Packed1(ctypes.Structure):
    _pack_ = 1

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_uint8),
        ("y", ctypes.c_uint16),
    ]

    def __init__(self, x: int = None, y: int = None):
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y

    @property
    def x(self) -> int:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: int):
        return ctypes.Structure.__set__(self, "x", value)

    @property
    def y(self) -> int:
        return ctypes.Structure.__get__(self, "y")

    @y.setter
    def y(self, value: int):
        return ctypes.Structure.__set__(self, "y", value)


class Packed2(ctypes.Structure):
    _pack_ = 1

    # These fields represent the underlying C data layout
    _fields_ = [
        ("y", ctypes.c_uint16),
        ("x", ctypes.c_uint8),
    ]

    def __init__(self, y: int = None, x: int = None):
        if y is not None:
            self.y = y
        if x is not None:
            self.x = x

    @property
    def y(self) -> int:
        return ctypes.Structure.__get__(self, "y")

    @y.setter
    def y(self, value: int):
        return ctypes.Structure.__set__(self, "y", value)

    @property
    def x(self) -> int:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: int):
        return ctypes.Structure.__set__(self, "x", value)


class Phantomu8(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_uint32),
    ]

    def __init__(self, x: int = None):
        if x is not None:
            self.x = x

    @property
    def x(self) -> int:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: int):
        return ctypes.Structure.__set__(self, "x", value)


class StructDocumented(ctypes.Structure):
    """ Documented struct."""

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_float),
    ]

    def __init__(self, x: float = None):
        if x is not None:
            self.x = x

    @property
    def x(self) -> float:
        """ Documented field."""
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: float):
        """ Documented field."""
        return ctypes.Structure.__set__(self, "x", value)


class StructRenamed(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("e", ctypes.c_int),
    ]

    def __init__(self, e: TODO = None):
        if e is not None:
            self.e = e

    @property
    def e(self) -> TODO:
        return ctypes.Structure.__get__(self, "e")

    @e.setter
    def e(self, value: TODO):
        return ctypes.Structure.__set__(self, "e", value)


class Tupled(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x0", ctypes.c_uint8),
    ]

    def __init__(self, x0: int = None):
        if x0 is not None:
            self.x0 = x0

    @property
    def x0(self) -> int:
        return ctypes.Structure.__get__(self, "x0")

    @x0.setter
    def x0(self, value: int):
        return ctypes.Structure.__set__(self, "x0", value)


class UseCStrPtr(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("ascii_string", ctypes.POINTER(ctypes.c_char)),
    ]

    def __init__(self, ascii_string: bytes = None):
        if ascii_string is not None:
            self.ascii_string = ascii_string

    @property
    def ascii_string(self) -> bytes:
        return ctypes.Structure.__get__(self, "ascii_string")

    @ascii_string.setter
    def ascii_string(self, value: bytes):
        return ctypes.Structure.__set__(self, "ascii_string", value)


class UseString(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("s1", Utf8String),
        ("s2", Utf8String),
    ]

    def __init__(self, s1 = None, s2 = None):
        if s1 is not None:
            self.s1 = s1
        if s2 is not None:
            self.s2 = s2

    @property
    def s1(self):
        return ctypes.Structure.__get__(self, "s1")

    @s1.setter
    def s1(self, value):
        return ctypes.Structure.__set__(self, "s1", value)

    @property
    def s2(self):
        return ctypes.Structure.__get__(self, "s2")

    @s2.setter
    def s2(self, value):
        return ctypes.Structure.__set__(self, "s2", value)


class Vec(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_double),
        ("z", ctypes.c_double),
    ]

    def __init__(self, x: float = None, z: float = None):
        if x is not None:
            self.x = x
        if z is not None:
            self.z = z

    @property
    def x(self) -> float:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: float):
        return ctypes.Structure.__set__(self, "x", value)

    @property
    def z(self) -> float:
        return ctypes.Structure.__get__(self, "z")

    @z.setter
    def z(self, value: float):
        return ctypes.Structure.__set__(self, "z", value)


class Vec1(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
    ]

    def __init__(self, x: float = None, y: float = None):
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y

    @property
    def x(self) -> float:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: float):
        return ctypes.Structure.__set__(self, "x", value)

    @property
    def y(self) -> float:
        return ctypes.Structure.__get__(self, "y")

    @y.setter
    def y(self, value: float):
        return ctypes.Structure.__set__(self, "y", value)


class Vec2(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_double),
        ("z", ctypes.c_double),
    ]

    def __init__(self, x: float = None, z: float = None):
        if x is not None:
            self.x = x
        if z is not None:
            self.z = z

    @property
    def x(self) -> float:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: float):
        return ctypes.Structure.__set__(self, "x", value)

    @property
    def z(self) -> float:
        return ctypes.Structure.__get__(self, "z")

    @z.setter
    def z(self, value: float):
        return ctypes.Structure.__set__(self, "z", value)


class Vec3f32(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
        ("z", ctypes.c_float),
    ]

    def __init__(self, x: float = None, y: float = None, z: float = None):
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y
        if z is not None:
            self.z = z

    @property
    def x(self) -> float:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: float):
        return ctypes.Structure.__set__(self, "x", value)

    @property
    def y(self) -> float:
        return ctypes.Structure.__get__(self, "y")

    @y.setter
    def y(self, value: float):
        return ctypes.Structure.__set__(self, "y", value)

    @property
    def z(self) -> float:
        return ctypes.Structure.__get__(self, "z")

    @z.setter
    def z(self, value: float):
        return ctypes.Structure.__set__(self, "z", value)


class Visibility1(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("pblc", ctypes.c_uint8),
        ("prvt", ctypes.c_uint8),
    ]

    def __init__(self, pblc: int = None, prvt: int = None):
        if pblc is not None:
            self.pblc = pblc
        if prvt is not None:
            self.prvt = prvt

    @property
    def pblc(self) -> int:
        return ctypes.Structure.__get__(self, "pblc")

    @pblc.setter
    def pblc(self, value: int):
        return ctypes.Structure.__set__(self, "pblc", value)

    @property
    def prvt(self) -> int:
        return ctypes.Structure.__get__(self, "prvt")

    @prvt.setter
    def prvt(self, value: int):
        return ctypes.Structure.__set__(self, "prvt", value)


class Visibility2(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("pblc1", ctypes.c_uint8),
        ("pblc2", ctypes.c_uint8),
    ]

    def __init__(self, pblc1: int = None, pblc2: int = None):
        if pblc1 is not None:
            self.pblc1 = pblc1
        if pblc2 is not None:
            self.pblc2 = pblc2

    @property
    def pblc1(self) -> int:
        return ctypes.Structure.__get__(self, "pblc1")

    @pblc1.setter
    def pblc1(self, value: int):
        return ctypes.Structure.__set__(self, "pblc1", value)

    @property
    def pblc2(self) -> int:
        return ctypes.Structure.__get__(self, "pblc2")

    @pblc2.setter
    def pblc2(self, value: int):
        return ctypes.Structure.__set__(self, "pblc2", value)


class Weird1u32(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_uint32),
    ]

    def __init__(self, x: int = None):
        if x is not None:
            self.x = x

    @property
    def x(self) -> int:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: int):
        return ctypes.Structure.__set__(self, "x", value)


class SliceBool(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_uint8)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i):
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceBool:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_uint8 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_uint8))
        rval = SliceBool(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint8)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[ctypes.c_uint8]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[ctypes.c_uint8]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self):
        """Returns the first element of this slice."""
        return self[0]

    def last(self):
        """Returns the last element of this slice."""
        return self[len(self)-1]


class SliceI32(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_int32)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> int:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceI32:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_int32 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_int32))
        rval = SliceI32(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_int32)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[ctypes.c_int32]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[ctypes.c_int32]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> int:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> int:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class SliceU32(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_uint32)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> int:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceU32:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_uint32 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_uint32))
        rval = SliceU32(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint32)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[ctypes.c_uint32]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[ctypes.c_uint32]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> int:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> int:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class SliceU8(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_uint8)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> int:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceU8:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_uint8 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_uint8))
        rval = SliceU8(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint8)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[ctypes.c_uint8]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[ctypes.c_uint8]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> int:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> int:
        """Returns the last element of this slice."""
        return self[len(self)-1]

    def bytearray(self):
        """Returns a bytearray with the content of this slice."""
        rval = bytearray(len(self))
        for i in range(len(self)):
            rval[i] = self[i]
        return rval


class SliceUtf8String(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(Utf8String)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i):
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceUtf8String:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (Utf8String * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(Utf8String))
        rval = SliceUtf8String(data=ctypes.cast(array, ctypes.POINTER(Utf8String)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[Utf8String]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[Utf8String]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self):
        """Returns the first element of this slice."""
        return self[0]

    def last(self):
        """Returns the last element of this slice."""
        return self[len(self)-1]


class SliceMutU32(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_uint32)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> int:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def __setitem__(self, i, v: int):
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        self.data[index] = v

    def copied(self) -> SliceMutU32:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_uint32 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_uint32))
        rval = SliceMutU32(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint32)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[ctypes.c_uint32]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[ctypes.c_uint32]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> int:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> int:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class SliceMutU8(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_uint8)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> int:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def __setitem__(self, i, v: int):
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        self.data[index] = v

    def copied(self) -> SliceMutU8:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_uint8 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_uint8))
        rval = SliceMutU8(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint8)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[ctypes.c_uint8]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[ctypes.c_uint8]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> int:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> int:
        """Returns the last element of this slice."""
        return self[len(self)-1]

    def bytearray(self):
        """Returns a bytearray with the content of this slice."""
        rval = bytearray(len(self))
        for i in range(len(self)):
            rval[i] = self[i]
        return rval


class OptionUtf8String:
    """Option that contains Some(value) or None."""
    # Element if Some().
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    None = 1


class ResultError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultU32Error:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultU64Error:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultUtf8StringError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class Array(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.c_uint8 * 16),
    ]

    def __init__(self, data = None):
        if data is not None:
            self.data = data

    @property
    def data(self):
        return ctypes.Structure.__get__(self, "data")

    @data.setter
    def data(self, value):
        return ctypes.Structure.__set__(self, "data", value)


class Container(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("foreign", Local),
    ]

    def __init__(self, foreign: Local = None):
        if foreign is not None:
            self.foreign = foreign

    @property
    def foreign(self) -> Local:
        return ctypes.Structure.__get__(self, "foreign")

    @foreign.setter
    def foreign(self, value: Local):
        return ctypes.Structure.__set__(self, "foreign", value)


class FixedString(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.c_uint8 * 32),
    ]

    def __init__(self, data = None):
        if data is not None:
            self.data = data

    @property
    def data(self):
        return ctypes.Structure.__get__(self, "data")

    @data.setter
    def data(self, value):
        return ctypes.Structure.__set__(self, "data", value)


class Genericu32(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.POINTER(ctypes.c_uint32)),
    ]

    def __init__(self, x: ctypes.POINTER(ctypes.c_uint32) = None):
        if x is not None:
            self.x = x

    @property
    def x(self) -> ctypes.POINTER(ctypes.c_uint32):
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: ctypes.POINTER(ctypes.c_uint32)):
        return ctypes.Structure.__set__(self, "x", value)


class Genericu8(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.POINTER(ctypes.c_uint8)),
    ]

    def __init__(self, x: ctypes.POINTER(ctypes.c_uint8) = None):
        if x is not None:
            self.x = x

    @property
    def x(self) -> ctypes.POINTER(ctypes.c_uint8):
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: ctypes.POINTER(ctypes.c_uint8)):
        return ctypes.Structure.__set__(self, "x", value)


class Weird2u8(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("t", ctypes.c_uint8),
        ("a", ctypes.c_uint8 * 5),
        ("r", ctypes.POINTER(ctypes.c_uint8)),
    ]

    def __init__(self, t: int = None, a = None, r: ctypes.POINTER(ctypes.c_uint8) = None):
        if t is not None:
            self.t = t
        if a is not None:
            self.a = a
        if r is not None:
            self.r = r

    @property
    def t(self) -> int:
        return ctypes.Structure.__get__(self, "t")

    @t.setter
    def t(self, value: int):
        return ctypes.Structure.__set__(self, "t", value)

    @property
    def a(self):
        return ctypes.Structure.__get__(self, "a")

    @a.setter
    def a(self, value):
        return ctypes.Structure.__set__(self, "a", value)

    @property
    def r(self) -> ctypes.POINTER(ctypes.c_uint8):
        return ctypes.Structure.__get__(self, "r")

    @r.setter
    def r(self, value: ctypes.POINTER(ctypes.c_uint8)):
        return ctypes.Structure.__set__(self, "r", value)


class SliceUseCStrPtr(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(UseCStrPtr)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> UseCStrPtr:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceUseCStrPtr:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (UseCStrPtr * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(UseCStrPtr))
        rval = SliceUseCStrPtr(data=ctypes.cast(array, ctypes.POINTER(UseCStrPtr)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[UseCStrPtr]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[UseCStrPtr]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> UseCStrPtr:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> UseCStrPtr:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class SliceUseString(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(UseString)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> UseString:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceUseString:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (UseString * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(UseString))
        rval = SliceUseString(data=ctypes.cast(array, ctypes.POINTER(UseString)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[UseString]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[UseString]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> UseString:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> UseString:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class SliceVec(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(Vec)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> Vec:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceVec:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (Vec * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(Vec))
        rval = SliceVec(data=ctypes.cast(array, ctypes.POINTER(Vec)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[Vec]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[Vec]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> Vec:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> Vec:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class SliceVec3f32(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(Vec3f32)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> Vec3f32:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceVec3f32:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (Vec3f32 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(Vec3f32))
        rval = SliceVec3f32(data=ctypes.cast(array, ctypes.POINTER(Vec3f32)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[Vec3f32]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[Vec3f32]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> Vec3f32:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> Vec3f32:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class SliceMutVec(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(Vec)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> Vec:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def __setitem__(self, i, v: Vec):
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        self.data[index] = v

    def copied(self) -> SliceMutVec:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (Vec * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(Vec))
        rval = SliceMutVec(data=ctypes.cast(array, ctypes.POINTER(Vec)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[Vec]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[Vec]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> Vec:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> Vec:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class OptionInner:
    """Option that contains Some(value) or None."""
    # Element if Some().
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    None = 1


class OptionVec:
    """Option that contains Some(value) or None."""
    # Element if Some().
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    None = 1


class ResultConstPtrServiceAsyncError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceBasicError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceCallbacksError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceDependentError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceIgnoringMethodsError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceMainError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceMultipleCtorsError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceOnPanicError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceResultError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceStringsError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultConstPtrServiceVariousSlicesError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultOptionUtf8StringError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class ResultUseStringError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3


class CallbackTable(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("my_callback", ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p)),
        ("my_callback_namespaced", ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p)),
        ("my_callback_void", ctypes.CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_void_p)),
        ("my_callback_contextual", ctypes.CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_uint32, ctypes.c_void_p)),
        ("sum_delegate_1", ctypes.CFUNCTYPE(None, ctypes.c_void_p)),
        ("sum_delegate_2", ctypes.CFUNCTYPE(ctypes.c_int32, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p)),
        ("sum_delegate_return", ctypes.CFUNCTYPE(ResultError, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p)),
        ("sum_delegate_return_2", ctypes.CFUNCTYPE(None, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p)),
    ]

    def __init__(self, my_callback = None, my_callback_namespaced = None, my_callback_void = None, my_callback_contextual = None, sum_delegate_1 = None, sum_delegate_2 = None, sum_delegate_return = None, sum_delegate_return_2 = None):
        if my_callback is not None:
            self.my_callback = my_callback
        if my_callback_namespaced is not None:
            self.my_callback_namespaced = my_callback_namespaced
        if my_callback_void is not None:
            self.my_callback_void = my_callback_void
        if my_callback_contextual is not None:
            self.my_callback_contextual = my_callback_contextual
        if sum_delegate_1 is not None:
            self.sum_delegate_1 = sum_delegate_1
        if sum_delegate_2 is not None:
            self.sum_delegate_2 = sum_delegate_2
        if sum_delegate_return is not None:
            self.sum_delegate_return = sum_delegate_return
        if sum_delegate_return_2 is not None:
            self.sum_delegate_return_2 = sum_delegate_return_2

    @property
    def my_callback(self):
        return ctypes.Structure.__get__(self, "my_callback")

    @my_callback.setter
    def my_callback(self, value):
        return ctypes.Structure.__set__(self, "my_callback", value)

    @property
    def my_callback_namespaced(self):
        return ctypes.Structure.__get__(self, "my_callback_namespaced")

    @my_callback_namespaced.setter
    def my_callback_namespaced(self, value):
        return ctypes.Structure.__set__(self, "my_callback_namespaced", value)

    @property
    def my_callback_void(self):
        return ctypes.Structure.__get__(self, "my_callback_void")

    @my_callback_void.setter
    def my_callback_void(self, value):
        return ctypes.Structure.__set__(self, "my_callback_void", value)

    @property
    def my_callback_contextual(self):
        return ctypes.Structure.__get__(self, "my_callback_contextual")

    @my_callback_contextual.setter
    def my_callback_contextual(self, value):
        return ctypes.Structure.__set__(self, "my_callback_contextual", value)

    @property
    def sum_delegate_1(self):
        return ctypes.Structure.__get__(self, "sum_delegate_1")

    @sum_delegate_1.setter
    def sum_delegate_1(self, value):
        return ctypes.Structure.__set__(self, "sum_delegate_1", value)

    @property
    def sum_delegate_2(self):
        return ctypes.Structure.__get__(self, "sum_delegate_2")

    @sum_delegate_2.setter
    def sum_delegate_2(self, value):
        return ctypes.Structure.__set__(self, "sum_delegate_2", value)

    @property
    def sum_delegate_return(self):
        return ctypes.Structure.__get__(self, "sum_delegate_return")

    @sum_delegate_return.setter
    def sum_delegate_return(self, value):
        return ctypes.Structure.__set__(self, "sum_delegate_return", value)

    @property
    def sum_delegate_return_2(self):
        return ctypes.Structure.__get__(self, "sum_delegate_return_2")

    @sum_delegate_return_2.setter
    def sum_delegate_return_2(self, value):
        return ctypes.Structure.__set__(self, "sum_delegate_return_2", value)


class CharArray(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("str", FixedString),
        ("str_2", FixedString),
    ]

    def __init__(self, str: FixedString = None, str_2: FixedString = None):
        if str is not None:
            self.str = str
        if str_2 is not None:
            self.str_2 = str_2

    @property
    def str(self) -> FixedString:
        return ctypes.Structure.__get__(self, "str")

    @str.setter
    def str(self, value: FixedString):
        return ctypes.Structure.__set__(self, "str", value)

    @property
    def str_2(self) -> FixedString:
        return ctypes.Structure.__get__(self, "str_2")

    @str_2.setter
    def str_2(self, value: FixedString):
        return ctypes.Structure.__set__(self, "str_2", value)


class NestedArray(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("field_enum", ctypes.c_int),
        ("field_vec", Vec3f32),
        ("field_bool", ctypes.c_bool),
        ("field_int", ctypes.c_int32),
        ("field_array", ctypes.c_uint16 * 5),
        ("field_array_2", ctypes.c_uint16 * 5),
        ("field_struct", Array),
    ]

    def __init__(self, field_enum: TODO = None, field_vec: Vec3f32 = None, field_bool: bool = None, field_int: int = None, field_array = None, field_array_2 = None, field_struct: Array = None):
        if field_enum is not None:
            self.field_enum = field_enum
        if field_vec is not None:
            self.field_vec = field_vec
        if field_bool is not None:
            self.field_bool = field_bool
        if field_int is not None:
            self.field_int = field_int
        if field_array is not None:
            self.field_array = field_array
        if field_array_2 is not None:
            self.field_array_2 = field_array_2
        if field_struct is not None:
            self.field_struct = field_struct

    @property
    def field_enum(self) -> TODO:
        return ctypes.Structure.__get__(self, "field_enum")

    @field_enum.setter
    def field_enum(self, value: TODO):
        return ctypes.Structure.__set__(self, "field_enum", value)

    @property
    def field_vec(self) -> Vec3f32:
        return ctypes.Structure.__get__(self, "field_vec")

    @field_vec.setter
    def field_vec(self, value: Vec3f32):
        return ctypes.Structure.__set__(self, "field_vec", value)

    @property
    def field_bool(self) -> bool:
        return ctypes.Structure.__get__(self, "field_bool")

    @field_bool.setter
    def field_bool(self, value: bool):
        return ctypes.Structure.__set__(self, "field_bool", value)

    @property
    def field_int(self) -> int:
        return ctypes.Structure.__get__(self, "field_int")

    @field_int.setter
    def field_int(self, value: int):
        return ctypes.Structure.__set__(self, "field_int", value)

    @property
    def field_array(self):
        return ctypes.Structure.__get__(self, "field_array")

    @field_array.setter
    def field_array(self, value):
        return ctypes.Structure.__set__(self, "field_array", value)

    @property
    def field_array_2(self):
        return ctypes.Structure.__get__(self, "field_array_2")

    @field_array_2.setter
    def field_array_2(self, value):
        return ctypes.Structure.__set__(self, "field_array_2", value)

    @property
    def field_struct(self) -> Array:
        return ctypes.Structure.__get__(self, "field_struct")

    @field_struct.setter
    def field_struct(self, value: Array):
        return ctypes.Structure.__set__(self, "field_struct", value)


class OptionResultOptionUtf8StringError:
    """Option that contains Some(value) or None."""
    # Element if Some().
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    None = 1


class SliceMutCharArray(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(CharArray)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> CharArray:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def __setitem__(self, i, v: CharArray):
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        self.data[index] = v

    def copied(self) -> SliceMutCharArray:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (CharArray * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(CharArray))
        rval = SliceMutCharArray(data=ctypes.cast(array, ctypes.POINTER(CharArray)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[CharArray]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[CharArray]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> CharArray:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> CharArray:
        """Returns the last element of this slice."""
        return self[len(self)-1]


class OptionOptionResultOptionUtf8StringError:
    """Option that contains Some(value) or None."""
    # Element if Some().
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    None = 1


class ResultNestedArrayError:
    """Result that contains value or an error."""
    # Element if err is `Ok`.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    # Error value.
# TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    Panic = 2
    Null = 3




class callbacks:
    """Helpers to define callbacks."""
    fn_u8_rval_u8 = ctypes.CFUNCTYPE(ctypes.c_uint8, ctypes.c_uint8)
    fn_CharArray = ctypes.CFUNCTYPE(None, CharArray)
    fn_CharArray_ConstPtr = ctypes.CFUNCTYPE(None, CharArray, ctypes.c_void_p)
    fn_SliceU8_ConstPtr_rval_u8 = ctypes.CFUNCTYPE(ctypes.c_uint8, SliceU8, ctypes.c_void_p)
    fn_SliceVec3f32_ConstPtr_rval_Vec3f32 = ctypes.CFUNCTYPE(Vec3f32, SliceVec3f32, ctypes.c_void_p)
    fn_SliceMutU8_ConstPtr = ctypes.CFUNCTYPE(None, SliceMutU8, ctypes.c_void_p)
    fn_u8_ConstPtr_rval_u8 = ctypes.CFUNCTYPE(ctypes.c_uint8, ctypes.c_uint8, ctypes.c_void_p)
    fn_u32_ConstPtr_rval_u32 = ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p)
    fn_ConstPtr_u32_ConstPtr = ctypes.CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_uint32, ctypes.c_void_p)
    fn_u32_ConstPtr_rval_u32 = ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32, ctypes.c_void_p)
    fn_ConstPtr_ConstPtr = ctypes.CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_void_p)
    fn_UseString_ConstPtr = ctypes.CFUNCTYPE(None, UseString, ctypes.c_void_p)
    fn_Utf8String_ConstPtr = ctypes.CFUNCTYPE(None, Utf8String, ctypes.c_void_p)
    fn_ConstPtr = ctypes.CFUNCTYPE(None, ctypes.c_void_p)
    fn_i32_i32_ConstPtr_rval_i32 = ctypes.CFUNCTYPE(ctypes.c_int32, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p)
    fn_i32_i32_ConstPtr_rval_ResultError = ctypes.CFUNCTYPE(ResultError, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p)
    fn_i32_i32_ConstPtr = ctypes.CFUNCTYPE(None, ctypes.c_int32, ctypes.c_int32, ctypes.c_void_p)


class ServiceAsync:
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceAsync.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new() -> ServiceAsync:
        """"""
        ctx = c_lib.service_async_new().t
        self = ServiceAsync(ServiceAsync.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_async_destroy(self._ctx, )
    def return_after_ms(self, x: int, ms: int, _async_callback):
        """"""
        return c_lib.service_async_return_after_ms(self._ctx, x, ms, _async_callback)

    def process_struct(self, x: NestedArray, _async_callback):
        """"""
        return c_lib.service_async_process_struct(self._ctx, x, _async_callback)

    def handle_string(self, s, _async_callback):
        """"""
        return c_lib.service_async_handle_string(self._ctx, s, _async_callback)

    def handle_nested_string(self, s, _async_callback):
        """"""
        return c_lib.service_async_handle_nested_string(self._ctx, s, _async_callback)

    def callback_string(self, s, cb):
        """"""
        if not hasattr(cb, "__ctypes_from_outparam__"):
            cb = callbacks.fn_Utf8String_ConstPtr(cb)

        return c_lib.service_async_callback_string(self._ctx, s, cb)

    def fail(self, _async_callback):
        """"""
        return c_lib.service_async_fail(self._ctx, _async_callback)

    def bad(self, ):
        """"""
        return c_lib.service_async_bad(self._ctx, )



class ServiceBasic:
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceBasic.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new() -> ServiceBasic:
        """"""
        ctx = c_lib.service_basic_new().t
        self = ServiceBasic(ServiceBasic.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_basic_destroy(self._ctx, )


class ServiceMain:
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceMain.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new() -> ServiceMain:
        """"""
        ctx = c_lib.service_main_new().t
        self = ServiceMain(ServiceMain.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_main_destroy(self._ctx, )


class ServiceDependent:
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceDependent.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def from_main() -> ServiceDependent:
        """"""
        ctx = c_lib.service_dependent_from_main().t
        self = ServiceDependent(ServiceDependent.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_dependent_destroy(self._ctx, )
    def get(self, ) -> int:
        """"""
        return c_lib.service_dependent_get(self._ctx, )



class ServiceResult:
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceResult.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new() -> ServiceResult:
        """"""
        ctx = c_lib.service_result_new().t
        self = ServiceResult(ServiceResult.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_result_destroy(self._ctx, )
    def test(self, ):
        """"""
        return c_lib.service_result_test(self._ctx, )



class ServiceOnPanic:
    """ Some struct we want to expose as a class."""
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceOnPanic.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new() -> ServiceOnPanic:
        """"""
        ctx = c_lib.service_on_panic_new().t
        self = ServiceOnPanic(ServiceOnPanic.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_on_panic_destroy(self._ctx, )
    def return_result(self, anon1: int):
        """ Methods returning a Result<(), _> are the default and do not
 need annotations."""
        return c_lib.service_on_panic_return_result(self._ctx, anon1)

    def return_default_value(self, x: int) -> int:
        """ Methods returning a value need an `on_panic` annotation."""
        return c_lib.service_on_panic_return_default_value(self._ctx, x)

    def return_ub_on_panic(self, ) -> bytes:
        """ This function has no panic safeguards. It will be a bit faster to
 call, but if it panics your host app will abort."""
        rval = c_lib.service_on_panic_return_ub_on_panic(self._ctx, )
        return ctypes.string_at(rval)



class ServiceCallbacks:
    """ Some struct we want to expose as a class."""
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceCallbacks.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new() -> ServiceCallbacks:
        """"""
        ctx = c_lib.service_callbacks_new().t
        self = ServiceCallbacks(ServiceCallbacks.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_callbacks_destroy(self._ctx, )
    def callback_simple(self, callback):
        """"""
        if not hasattr(callback, "__ctypes_from_outparam__"):
            callback = callbacks.fn_u32_ConstPtr_rval_u32(callback)

        return c_lib.service_callbacks_callback_simple(self._ctx, callback)

    def callback_ffi_return(self, callback):
        """"""
        if not hasattr(callback, "__ctypes_from_outparam__"):
            callback = callbacks.fn_i32_i32_ConstPtr_rval_ResultError(callback)

        return c_lib.service_callbacks_callback_ffi_return(self._ctx, callback)

    def callback_with_slice(self, callback, input: SliceI32 | ctypes.Array[ctypes.c_int32]):
        """"""
        if not hasattr(callback, "__ctypes_from_outparam__"):
            callback = callbacks.fn_i32_i32_ConstPtr_rval_ResultError(callback)

        if hasattr(input, "_length_") and getattr(input, "_type_", "") == ctypes.c_int32:
            input = SliceI32(data=ctypes.cast(input, ctypes.POINTER(ctypes.c_int32)), len=len(input))

        return c_lib.service_callbacks_callback_with_slice(self._ctx, callback, input)

    def set_delegate_table(self, table: CallbackTable):
        """"""
        return c_lib.service_callbacks_set_delegate_table(self._ctx, table)

    def invoke_delegates(self, ):
        """"""
        return c_lib.service_callbacks_invoke_delegates(self._ctx, )



class ServiceIgnoringMethods:
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceIgnoringMethods.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new() -> ServiceIgnoringMethods:
        """"""
        ctx = c_lib.service_ignoring_methods_new().t
        self = ServiceIgnoringMethods(ServiceIgnoringMethods.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_ignoring_methods_destroy(self._ctx, )


class ServiceMultipleCtors:
    """ Some struct we want to expose as a class."""
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceMultipleCtors.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new_with() -> ServiceMultipleCtors:
        """"""
        ctx = c_lib.service_multiple_ctors_new_with().t
        self = ServiceMultipleCtors(ServiceMultipleCtors.__api_lock, ctx)
        return self

    @staticmethod
    def new_without() -> ServiceMultipleCtors:
        """"""
        ctx = c_lib.service_multiple_ctors_new_without().t
        self = ServiceMultipleCtors(ServiceMultipleCtors.__api_lock, ctx)
        return self

    @staticmethod
    def new_with_string() -> ServiceMultipleCtors:
        """"""
        if not hasattr(anon0, "__ctypes_from_outparam__"):
            anon0 = ctypes.cast(anon0, ctypes.POINTER(ctypes.c_char))
        ctx = c_lib.service_multiple_ctors_new_with_string().t
        self = ServiceMultipleCtors(ServiceMultipleCtors.__api_lock, ctx)
        return self

    @staticmethod
    def new_failing() -> ServiceMultipleCtors:
        """"""
        ctx = c_lib.service_multiple_ctors_new_failing().t
        self = ServiceMultipleCtors(ServiceMultipleCtors.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_multiple_ctors_destroy(self._ctx, )


class ServiceVariousSlices:
    """ Some struct we want to expose as a class."""
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceVariousSlices.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new() -> ServiceVariousSlices:
        """"""
        ctx = c_lib.service_various_slices_new().t
        self = ServiceVariousSlices(ServiceVariousSlices.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_various_slices_destroy(self._ctx, )
    def mut_self(self, slice: SliceU8 | ctypes.Array[ctypes.c_uint8]) -> int:
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceU8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.service_various_slices_mut_self(self._ctx, slice)

    def mut_self_void(self, slice: SliceBool | ctypes.Array[ctypes.c_uint8]):
        """ Single line."""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceBool(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.service_various_slices_mut_self_void(self._ctx, slice)

    def mut_self_ref(self, x: ctypes.POINTER(ctypes.c_uint8), y: ctypes.POINTER(ctypes.c_uint8)) -> int:
        """"""
        return c_lib.service_various_slices_mut_self_ref(self._ctx, x, y)

    def mut_self_ref_slice(self, x: ctypes.POINTER(ctypes.c_uint8), y: ctypes.POINTER(ctypes.c_uint8), slice: SliceU8 | ctypes.Array[ctypes.c_uint8]) -> int:
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceU8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.service_various_slices_mut_self_ref_slice(self._ctx, x, y, slice)

    def mut_self_ref_slice_limited(self, x: ctypes.POINTER(ctypes.c_uint8), y: ctypes.POINTER(ctypes.c_uint8), slice: SliceU8 | ctypes.Array[ctypes.c_uint8], slice2: SliceU8 | ctypes.Array[ctypes.c_uint8]) -> int:
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceU8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        if hasattr(slice2, "_length_") and getattr(slice2, "_type_", "") == ctypes.c_uint8:
            slice2 = SliceU8(data=ctypes.cast(slice2, ctypes.POINTER(ctypes.c_uint8)), len=len(slice2))

        return c_lib.service_various_slices_mut_self_ref_slice_limited(self._ctx, x, y, slice, slice2)

    def mut_self_ffi_error(self, slice: SliceMutU8 | ctypes.Array[ctypes.c_uint8]):
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceMutU8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.service_various_slices_mut_self_ffi_error(self._ctx, slice)

    def mut_self_no_error(self, slice: SliceMutU8 | ctypes.Array[ctypes.c_uint8]):
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceMutU8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.service_various_slices_mut_self_no_error(self._ctx, slice)

    def return_slice(self, ) -> SliceU32:
        """ Warning, you _must_ discard the returned slice object before calling into this service
 again, as otherwise undefined behavior might happen."""
        return c_lib.service_various_slices_return_slice(self._ctx, )

    def return_slice_mut(self, ) -> SliceMutU32:
        """ Warning, you _must_ discard the returned slice object before calling into this service
 again, as otherwise undefined behavior might happen."""
        return c_lib.service_various_slices_return_slice_mut(self._ctx, )



class ServiceStrings:
    """ Some struct we want to expose as a class."""
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == ServiceStrings.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new() -> ServiceStrings:
        """"""
        ctx = c_lib.service_strings_new().t
        self = ServiceStrings(ServiceStrings.__api_lock, ctx)
        return self

    @staticmethod
    def new_string() -> ServiceStrings:
        """"""
        ctx = c_lib.service_strings_new_string().t
        self = ServiceStrings(ServiceStrings.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.service_strings_destroy(self._ctx, )
    def pass_cstr(self, anon1: bytes):
        """"""
        if not hasattr(anon1, "__ctypes_from_outparam__"):
            anon1 = ctypes.cast(anon1, ctypes.POINTER(ctypes.c_char))
        return c_lib.service_strings_pass_cstr(self._ctx, anon1)

    def return_cstr(self, ) -> bytes:
        """"""
        rval = c_lib.service_strings_return_cstr(self._ctx, )
        return ctypes.string_at(rval)

    def callback_string(self, s, cb):
        """"""
        if not hasattr(cb, "__ctypes_from_outparam__"):
            cb = callbacks.fn_Utf8String_ConstPtr(cb)

        return c_lib.service_strings_callback_string(self._ctx, s, cb)



