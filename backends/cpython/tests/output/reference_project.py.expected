from __future__ import annotations
import ctypes
import typing

T = typing.TypeVar("T")
c_lib = None

def init_lib(path):
    """Initializes the native library. Must be called at least once before anything else."""
    global c_lib
    c_lib = ctypes.cdll.LoadLibrary(path)

    c_lib.primitive_void.argtypes = []
    c_lib.primitive_void2.argtypes = []
    c_lib.primitive_bool.argtypes = [ctypes.c_bool]
    c_lib.primitive_u8.argtypes = [ctypes.c_uint8]
    c_lib.primitive_u16.argtypes = [ctypes.c_uint16]
    c_lib.primitive_u32.argtypes = [ctypes.c_uint32]
    c_lib.primitive_u64.argtypes = [ctypes.c_uint64]
    c_lib.primitive_i8.argtypes = [ctypes.c_int8]
    c_lib.primitive_i16.argtypes = [ctypes.c_int16]
    c_lib.primitive_i32.argtypes = [ctypes.c_int32]
    c_lib.primitive_i64.argtypes = [ctypes.c_int64]
    c_lib.many_args_5.argtypes = [ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64]
    c_lib.many_args_10.argtypes = [ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64]
    c_lib.ptr.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ptr_mut.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ptr_ptr.argtypes = [ctypes.POINTER(ctypes.POINTER(ctypes.c_int64))]
    c_lib.ref_simple.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ref_mut_simple.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ref_option.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.ref_mut_option.argtypes = [ctypes.POINTER(ctypes.c_int64)]
    c_lib.tupled.argtypes = [Tupled]
    c_lib.complex_args_1.argtypes = [Vec3f32, ctypes.POINTER(Tupled)]
    c_lib.complex_args_2.argtypes = [SomeForeignType]
    c_lib.callback.argtypes = [callbacks.fn_u8_rval_u8, ctypes.c_uint8]
    c_lib.generic_1a.argtypes = [Genericu32, Phantomu8]
    c_lib.generic_1b.argtypes = [Genericu8, Phantomu8]
    c_lib.generic_1c.argtypes = [ctypes.POINTER(Genericu8), ctypes.POINTER(Genericu8)]
    c_lib.generic_2.argtypes = [ctypes.c_void_p]
    c_lib.generic_3.argtypes = [ctypes.c_void_p]
    c_lib.generic_4.argtypes = [ctypes.c_void_p]
    c_lib.array_1.argtypes = [Array]
    c_lib.documented.argtypes = [StructDocumented]
    c_lib.ambiguous_1.argtypes = [Vec1]
    c_lib.ambiguous_2.argtypes = [Vec2]
    c_lib.ambiguous_3.argtypes = [Vec1, Vec2]
    c_lib.namespaced_type.argtypes = [Vec]
    c_lib.namespaced_inner_option.argtypes = [OptionVec]
    c_lib.namespaced_inner_slice.argtypes = [SliceVec]
    c_lib.namespaced_inner_slice_mut.argtypes = [SliceMutVec]
    c_lib.panics.argtypes = []
    c_lib.renamed.argtypes = [StructRenamed]
    c_lib.sleep.argtypes = [ctypes.c_uint64]
    c_lib.weird_1.argtypes = [Weird1u32, Weird2u8]
    c_lib.visibility.argtypes = [Visibility1, Visibility2]
    c_lib.repr_transparent.argtypes = [Tupled, ctypes.POINTER(Tupled)]
    c_lib.pattern_ascii_pointer_1.argtypes = [ctypes.POINTER(ctypes.c_char)]
    c_lib.pattern_ascii_pointer_2.argtypes = []
    c_lib.pattern_ascii_pointer_len.argtypes = [ctypes.POINTER(ctypes.c_char), UseAsciiStringPattern]
    c_lib.pattern_ascii_pointer_return_slice.argtypes = []
    c_lib.pattern_array_pointer_last_or_default.argtypes = [ctypes.c_void_p, ctypes.c_uint64, ctypes.c_uint32]
    c_lib.pattern_ffi_slice_1.argtypes = [Sliceu32]
    c_lib.pattern_ffi_slice_2.argtypes = [SliceVec3f32, ctypes.c_int32]
    c_lib.pattern_ffi_slice_3.argtypes = [SliceMutu8, callbacks.fn_SliceMutu8]
    c_lib.pattern_ffi_slice_4.argtypes = [Sliceu8, SliceMutu8]
    c_lib.pattern_ffi_slice_5.argtypes = [ctypes.POINTER(Sliceu8), ctypes.POINTER(SliceMutu8)]
    c_lib.pattern_ffi_slice_6.argtypes = [ctypes.POINTER(SliceMutu8), callbacks.fn_u8_rval_u8]
    c_lib.pattern_ffi_slice_delegate.argtypes = [callbacks.fn_Sliceu8_rval_u8]
    c_lib.pattern_ffi_slice_delegate_huge.argtypes = [callbacks.fn_SliceVec3f32_rval_Vec3f32]
    c_lib.pattern_ffi_option_1.argtypes = [OptionInner]
    c_lib.pattern_ffi_option_2.argtypes = [OptionInner]
    c_lib.pattern_ffi_bool.argtypes = [ctypes.c_uint8]
    c_lib.pattern_ffi_cchar.argtypes = [ctypes.c_char]
    c_lib.pattern_ffi_cchar_const_pointer.argtypes = [ctypes.POINTER(ctypes.c_char)]
    c_lib.pattern_ffi_cchar_mut_pointer.argtypes = [ctypes.POINTER(ctypes.c_char)]
    c_lib.pattern_api_guard.argtypes = []
    c_lib.pattern_callback_1.argtypes = [callbacks.fn_u32_rval_u32, ctypes.c_uint32]
    c_lib.pattern_callback_2.argtypes = [callbacks.fn_pconst]
    c_lib.simple_service_destroy.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    c_lib.simple_service_new_with.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_uint32]
    c_lib.simple_service_new_without.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    c_lib.simple_service_new_with_string.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.POINTER(ctypes.c_char)]
    c_lib.simple_service_new_failing.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_uint8]
    c_lib.simple_service_method_result.argtypes = [ctypes.c_void_p, ctypes.c_uint32]
    c_lib.simple_service_method_value.argtypes = [ctypes.c_void_p, ctypes.c_uint32]
    c_lib.simple_service_method_void.argtypes = [ctypes.c_void_p]
    c_lib.simple_service_method_mut_self.argtypes = [ctypes.c_void_p, Sliceu8]
    c_lib.simple_service_method_mut_self_void.argtypes = [ctypes.c_void_p, SliceBool]
    c_lib.simple_service_method_mut_self_ref.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8)]
    c_lib.simple_service_method_mut_self_ref_slice.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8), Sliceu8]
    c_lib.simple_service_method_mut_self_ref_slice_limited.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8), Sliceu8, Sliceu8]
    c_lib.simple_service_method_mut_self_ffi_error.argtypes = [ctypes.c_void_p, SliceMutu8]
    c_lib.simple_service_method_mut_self_no_error.argtypes = [ctypes.c_void_p, SliceMutu8]
    c_lib.simple_service_return_slice.argtypes = [ctypes.c_void_p]
    c_lib.simple_service_return_slice_mut.argtypes = [ctypes.c_void_p]
    c_lib.simple_service_return_string.argtypes = [ctypes.c_void_p]
    c_lib.simple_service_method_void_ffi_error.argtypes = [ctypes.c_void_p]
    c_lib.simple_service_method_callback.argtypes = [ctypes.c_void_p, callbacks.fn_u32_rval_u32]
    c_lib.simple_service_lt_destroy.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    c_lib.simple_service_lt_new_with.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.POINTER(ctypes.c_uint32)]
    c_lib.simple_service_lt_method_lt.argtypes = [ctypes.c_void_p, SliceBool]
    c_lib.simple_service_lt_method_lt2.argtypes = [ctypes.c_void_p, SliceBool]
    c_lib.simple_service_lt_return_string_accept_slice.argtypes = [ctypes.c_void_p, Sliceu8]
    c_lib.simple_service_lt_method_void_ffi_error.argtypes = [ctypes.c_void_p]

    c_lib.primitive_bool.restype = ctypes.c_bool
    c_lib.primitive_u8.restype = ctypes.c_uint8
    c_lib.primitive_u16.restype = ctypes.c_uint16
    c_lib.primitive_u32.restype = ctypes.c_uint32
    c_lib.primitive_u64.restype = ctypes.c_uint64
    c_lib.primitive_i8.restype = ctypes.c_int8
    c_lib.primitive_i16.restype = ctypes.c_int16
    c_lib.primitive_i32.restype = ctypes.c_int32
    c_lib.primitive_i64.restype = ctypes.c_int64
    c_lib.many_args_5.restype = ctypes.c_int64
    c_lib.many_args_10.restype = ctypes.c_int64
    c_lib.ptr.restype = ctypes.POINTER(ctypes.c_int64)
    c_lib.ptr_mut.restype = ctypes.POINTER(ctypes.c_int64)
    c_lib.ptr_ptr.restype = ctypes.POINTER(ctypes.POINTER(ctypes.c_int64))
    c_lib.ref_simple.restype = ctypes.POINTER(ctypes.c_int64)
    c_lib.ref_mut_simple.restype = ctypes.POINTER(ctypes.c_int64)
    c_lib.ref_option.restype = ctypes.c_bool
    c_lib.ref_mut_option.restype = ctypes.c_bool
    c_lib.tupled.restype = Tupled
    c_lib.complex_args_1.restype = ctypes.c_int
    c_lib.complex_args_2.restype = ctypes.c_void_p
    c_lib.callback.restype = ctypes.c_uint8
    c_lib.generic_1a.restype = ctypes.c_uint32
    c_lib.generic_1b.restype = ctypes.c_uint8
    c_lib.generic_1c.restype = ctypes.c_uint8
    c_lib.generic_2.restype = ctypes.c_uint8
    c_lib.generic_3.restype = ctypes.c_uint8
    c_lib.generic_4.restype = ctypes.c_uint8
    c_lib.array_1.restype = ctypes.c_uint8
    c_lib.documented.restype = ctypes.c_int
    c_lib.ambiguous_1.restype = Vec1
    c_lib.ambiguous_2.restype = Vec2
    c_lib.ambiguous_3.restype = ctypes.c_bool
    c_lib.namespaced_type.restype = Vec
    c_lib.namespaced_inner_option.restype = OptionVec
    c_lib.namespaced_inner_slice.restype = SliceVec
    c_lib.namespaced_inner_slice_mut.restype = SliceMutVec
    c_lib.panics.restype = ctypes.c_int
    c_lib.renamed.restype = ctypes.c_int
    c_lib.weird_1.restype = ctypes.c_bool
    c_lib.repr_transparent.restype = Tupled
    c_lib.pattern_ascii_pointer_1.restype = ctypes.c_uint32
    c_lib.pattern_ascii_pointer_2.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.pattern_ascii_pointer_len.restype = ctypes.c_uint32
    c_lib.pattern_ascii_pointer_return_slice.restype = SliceUseAsciiStringPattern
    c_lib.pattern_array_pointer_last_or_default.restype = ctypes.c_uint32
    c_lib.pattern_ffi_slice_1.restype = ctypes.c_uint32
    c_lib.pattern_ffi_slice_2.restype = Vec3f32
    c_lib.pattern_ffi_slice_delegate.restype = ctypes.c_uint8
    c_lib.pattern_ffi_slice_delegate_huge.restype = Vec3f32
    c_lib.pattern_ffi_option_1.restype = OptionInner
    c_lib.pattern_ffi_option_2.restype = Inner
    c_lib.pattern_ffi_bool.restype = ctypes.c_uint8
    c_lib.pattern_ffi_cchar.restype = ctypes.c_char
    c_lib.pattern_ffi_cchar_const_pointer.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.pattern_ffi_cchar_mut_pointer.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.pattern_api_guard.restype = ctypes.c_uint64
    c_lib.pattern_callback_1.restype = ctypes.c_uint32
    c_lib.pattern_callback_2.restype = callbacks.fn_pconst
    c_lib.simple_service_destroy.restype = ctypes.c_int
    c_lib.simple_service_new_with.restype = ctypes.c_int
    c_lib.simple_service_new_without.restype = ctypes.c_int
    c_lib.simple_service_new_with_string.restype = ctypes.c_int
    c_lib.simple_service_new_failing.restype = ctypes.c_int
    c_lib.simple_service_method_result.restype = ctypes.c_int
    c_lib.simple_service_method_value.restype = ctypes.c_uint32
    c_lib.simple_service_method_mut_self.restype = ctypes.c_uint8
    c_lib.simple_service_method_mut_self_ref.restype = ctypes.c_uint8
    c_lib.simple_service_method_mut_self_ref_slice.restype = ctypes.c_uint8
    c_lib.simple_service_method_mut_self_ref_slice_limited.restype = ctypes.c_uint8
    c_lib.simple_service_method_mut_self_ffi_error.restype = ctypes.c_int
    c_lib.simple_service_method_mut_self_no_error.restype = ctypes.c_int
    c_lib.simple_service_return_slice.restype = Sliceu32
    c_lib.simple_service_return_slice_mut.restype = SliceMutu32
    c_lib.simple_service_return_string.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.simple_service_method_void_ffi_error.restype = ctypes.c_int
    c_lib.simple_service_method_callback.restype = ctypes.c_int
    c_lib.simple_service_lt_destroy.restype = ctypes.c_int
    c_lib.simple_service_lt_new_with.restype = ctypes.c_int
    c_lib.simple_service_lt_return_string_accept_slice.restype = ctypes.POINTER(ctypes.c_char)
    c_lib.simple_service_lt_method_void_ffi_error.restype = ctypes.c_int

    c_lib.complex_args_1.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.panics.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_destroy.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_new_with.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_new_without.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_new_with_string.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_new_failing.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_method_result.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_method_mut_self_ffi_error.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_method_mut_self_no_error.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_method_void_ffi_error.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_method_callback.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_lt_destroy.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_lt_new_with.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_lt_method_void_ffi_error.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)


def primitive_void():
    return c_lib.primitive_void()

def primitive_void2():
    return c_lib.primitive_void2()

def primitive_bool(x: bool) -> bool:
    return c_lib.primitive_bool(x)

def primitive_u8(x: int) -> int:
    return c_lib.primitive_u8(x)

def primitive_u16(x: int) -> int:
    return c_lib.primitive_u16(x)

def primitive_u32(x: int) -> int:
    return c_lib.primitive_u32(x)

def primitive_u64(x: int) -> int:
    return c_lib.primitive_u64(x)

def primitive_i8(x: int) -> int:
    return c_lib.primitive_i8(x)

def primitive_i16(x: int) -> int:
    return c_lib.primitive_i16(x)

def primitive_i32(x: int) -> int:
    return c_lib.primitive_i32(x)

def primitive_i64(x: int) -> int:
    return c_lib.primitive_i64(x)

def many_args_5(x0: int, x1: int, x2: int, x3: int, x4: int) -> int:
    return c_lib.many_args_5(x0, x1, x2, x3, x4)

def many_args_10(x0: int, x1: int, x2: int, x3: int, x4: int, x5: int, x6: int, x7: int, x8: int, x9: int) -> int:
    return c_lib.many_args_10(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)

def ptr(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    return c_lib.ptr(x)

def ptr_mut(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    """ # Safety

 Parameter x must point to valid data."""
    return c_lib.ptr_mut(x)

def ptr_ptr(x: ctypes.POINTER(ctypes.POINTER(ctypes.c_int64))) -> ctypes.POINTER(ctypes.POINTER(ctypes.c_int64)):
    return c_lib.ptr_ptr(x)

def ref_simple(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    return c_lib.ref_simple(x)

def ref_mut_simple(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    return c_lib.ref_mut_simple(x)

def ref_option(x: ctypes.POINTER(ctypes.c_int64)) -> bool:
    return c_lib.ref_option(x)

def ref_mut_option(x: ctypes.POINTER(ctypes.c_int64)) -> bool:
    return c_lib.ref_mut_option(x)

def tupled(x: Tupled) -> Tupled:
    return c_lib.tupled(x)

def complex_args_1(a: Vec3f32, b: ctypes.POINTER(Tupled)):
    return c_lib.complex_args_1(a, b)

def complex_args_2(cmplx: SomeForeignType) -> ctypes.c_void_p:
    return c_lib.complex_args_2(cmplx)

def callback(callback, value: int) -> int:
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_u8_rval_u8(callback)

    return c_lib.callback(callback, value)

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

def array_1(x: Array) -> int:
    return c_lib.array_1(x)

def documented(x: StructDocumented) -> ctypes.c_int:
    """ This function has documentation."""
    return c_lib.documented(x)

def ambiguous_1(x: Vec1) -> Vec1:
    return c_lib.ambiguous_1(x)

def ambiguous_2(x: Vec2) -> Vec2:
    return c_lib.ambiguous_2(x)

def ambiguous_3(x: Vec1, y: Vec2) -> bool:
    return c_lib.ambiguous_3(x, y)

def namespaced_type(x: Vec) -> Vec:
    return c_lib.namespaced_type(x)

def namespaced_inner_option(x: OptionVec) -> OptionVec:
    return c_lib.namespaced_inner_option(x)

def namespaced_inner_slice(x: SliceVec | ctypes.Array[Vec]) -> SliceVec:
    if hasattr(x, "_length_") and getattr(x, "_type_", "") == Vec:
        x = SliceVec(data=ctypes.cast(x, ctypes.POINTER(Vec)), len=len(x))

    return c_lib.namespaced_inner_slice(x)

def namespaced_inner_slice_mut(x: SliceMutVec | ctypes.Array[Vec]) -> SliceMutVec:
    if hasattr(x, "_length_") and getattr(x, "_type_", "") == Vec:
        x = SliceMutVec(data=ctypes.cast(x, ctypes.POINTER(Vec)), len=len(x))

    return c_lib.namespaced_inner_slice_mut(x)

def panics():
    return c_lib.panics()

def renamed(x: StructRenamed) -> ctypes.c_int:
    return c_lib.renamed(x)

def sleep(millis: int):
    return c_lib.sleep(millis)

def weird_1(x: Weird1u32, y: Weird2u8) -> bool:
    return c_lib.weird_1(x, y)

def visibility(x: Visibility1, y: Visibility2):
    return c_lib.visibility(x, y)

def repr_transparent(x: Tupled, r: ctypes.POINTER(Tupled)) -> Tupled:
    return c_lib.repr_transparent(x, r)

def pattern_ascii_pointer_1(x: str) -> int:
    if not hasattr(x, "__ctypes_from_outparam__"):
        x = ctypes.cast(x, ctypes.POINTER(ctypes.c_char))
    return c_lib.pattern_ascii_pointer_1(x)

def pattern_ascii_pointer_2() -> str:
    rval = c_lib.pattern_ascii_pointer_2()
    return ctypes.string_at(rval)

def pattern_ascii_pointer_len(x: str, y: UseAsciiStringPattern) -> int:
    if not hasattr(x, "__ctypes_from_outparam__"):
        x = ctypes.cast(x, ctypes.POINTER(ctypes.c_char))
    return c_lib.pattern_ascii_pointer_len(x, y)

def pattern_ascii_pointer_return_slice() -> SliceUseAsciiStringPattern:
    return c_lib.pattern_ascii_pointer_return_slice()

def pattern_array_pointer_last_or_default(ptr, len: int, default: int) -> int:
    return c_lib.pattern_array_pointer_last_or_default(ptr, len, default)

def pattern_ffi_slice_1(ffi_slice: Sliceu32 | ctypes.Array[ctypes.c_uint32]) -> int:
    if hasattr(ffi_slice, "_length_") and getattr(ffi_slice, "_type_", "") == ctypes.c_uint32:
        ffi_slice = Sliceu32(data=ctypes.cast(ffi_slice, ctypes.POINTER(ctypes.c_uint32)), len=len(ffi_slice))

    return c_lib.pattern_ffi_slice_1(ffi_slice)

def pattern_ffi_slice_2(ffi_slice: SliceVec3f32 | ctypes.Array[Vec3f32], i: int) -> Vec3f32:
    if hasattr(ffi_slice, "_length_") and getattr(ffi_slice, "_type_", "") == Vec3f32:
        ffi_slice = SliceVec3f32(data=ctypes.cast(ffi_slice, ctypes.POINTER(Vec3f32)), len=len(ffi_slice))

    return c_lib.pattern_ffi_slice_2(ffi_slice, i)

def pattern_ffi_slice_3(slice: SliceMutu8 | ctypes.Array[ctypes.c_uint8], callback):
    if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
        slice = SliceMutu8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_SliceMutu8(callback)

    return c_lib.pattern_ffi_slice_3(slice, callback)

def pattern_ffi_slice_4(slice: Sliceu8 | ctypes.Array[ctypes.c_uint8], slice2: SliceMutu8 | ctypes.Array[ctypes.c_uint8]):
    if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
        slice = Sliceu8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

    if hasattr(slice2, "_length_") and getattr(slice2, "_type_", "") == ctypes.c_uint8:
        slice2 = SliceMutu8(data=ctypes.cast(slice2, ctypes.POINTER(ctypes.c_uint8)), len=len(slice2))

    return c_lib.pattern_ffi_slice_4(slice, slice2)

def pattern_ffi_slice_5(slice: ctypes.POINTER(Sliceu8), slice2: ctypes.POINTER(SliceMutu8)):
    return c_lib.pattern_ffi_slice_5(slice, slice2)

def pattern_ffi_slice_6(slice: ctypes.POINTER(SliceMutu8), callback):
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_u8_rval_u8(callback)

    return c_lib.pattern_ffi_slice_6(slice, callback)

def pattern_ffi_slice_delegate(callback) -> int:
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_Sliceu8_rval_u8(callback)

    return c_lib.pattern_ffi_slice_delegate(callback)

def pattern_ffi_slice_delegate_huge(callback) -> Vec3f32:
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_SliceVec3f32_rval_Vec3f32(callback)

    return c_lib.pattern_ffi_slice_delegate_huge(callback)

def pattern_ffi_option_1(ffi_slice: OptionInner) -> OptionInner:
    return c_lib.pattern_ffi_option_1(ffi_slice)

def pattern_ffi_option_2(ffi_slice: OptionInner) -> Inner:
    return c_lib.pattern_ffi_option_2(ffi_slice)

def pattern_ffi_bool(ffi_bool):
    return c_lib.pattern_ffi_bool(ffi_bool)

def pattern_ffi_cchar(ffi_cchar: ctypes.c_char) -> ctypes.c_char:
    return c_lib.pattern_ffi_cchar(ffi_cchar)

def pattern_ffi_cchar_const_pointer(ffi_cchar: ctypes.POINTER(ctypes.c_char)) -> ctypes.POINTER(ctypes.c_char):
    return c_lib.pattern_ffi_cchar_const_pointer(ffi_cchar)

def pattern_ffi_cchar_mut_pointer(ffi_cchar: ctypes.POINTER(ctypes.c_char)) -> ctypes.POINTER(ctypes.c_char):
    return c_lib.pattern_ffi_cchar_mut_pointer(ffi_cchar)

def pattern_api_guard():
    return c_lib.pattern_api_guard()

def pattern_callback_1(callback, x: int) -> int:
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_u32_rval_u32(callback)

    return c_lib.pattern_callback_1(callback, x)

def pattern_callback_2(callback):
    if not hasattr(callback, "__ctypes_from_outparam__"):
        callback = callbacks.fn_pconst(callback)

    return c_lib.pattern_callback_2(callback)



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


class EnumRenamed:
    X = 0


class FFIError:
    Ok = 0
    Null = 100
    Panic = 200
    Fail = 300


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


class SomeForeignType(ctypes.Structure):

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

    def __init__(self, e: ctypes.c_int = None):
        if e is not None:
            self.e = e

    @property
    def e(self) -> ctypes.c_int:
        return ctypes.Structure.__get__(self, "e")

    @e.setter
    def e(self, value: ctypes.c_int):
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


class UseAsciiStringPattern(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("ascii_string", ctypes.POINTER(ctypes.c_char)),
    ]

    def __init__(self, ascii_string: str = None):
        if ascii_string is not None:
            self.ascii_string = ascii_string

    @property
    def ascii_string(self) -> str:
        return ctypes.Structure.__get__(self, "ascii_string")

    @ascii_string.setter
    def ascii_string(self, value: str):
        return ctypes.Structure.__set__(self, "ascii_string", value)


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


class Sliceu32(ctypes.Structure):
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

    def copied(self) -> Sliceu32:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_uint32 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_uint32))
        rval = Sliceu32(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint32)), len=len(self))
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


class Sliceu8(ctypes.Structure):
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

    def copied(self) -> Sliceu8:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_uint8 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_uint8))
        rval = Sliceu8(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint8)), len=len(self))
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


class SliceMutu32(ctypes.Structure):
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

    def copied(self) -> SliceMutu32:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_uint32 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_uint32))
        rval = SliceMutu32(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint32)), len=len(self))
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


class SliceMutu8(ctypes.Structure):
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

    def copied(self) -> SliceMutu8:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (ctypes.c_uint8 * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(ctypes.c_uint8))
        rval = SliceMutu8(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint8)), len=len(self))
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


class OptionInner(ctypes.Structure):
    """May optionally hold a value."""

    _fields_ = [
        ("_t", Inner),
        ("_is_some", ctypes.c_uint8),
    ]

    @property
    def value(self) -> Inner:
        """Returns the value if it exists, or None."""
        if self._is_some == 1:
            return self._t
        else:
            return None

    def is_some(self) -> bool:
        """Returns true if the value exists."""
        return self._is_some == 1

    def is_none(self) -> bool:
        """Returns true if the value does not exist."""
        return self._is_some != 0


class OptionVec(ctypes.Structure):
    """May optionally hold a value."""

    _fields_ = [
        ("_t", Vec),
        ("_is_some", ctypes.c_uint8),
    ]

    @property
    def value(self) -> Vec:
        """Returns the value if it exists, or None."""
        if self._is_some == 1:
            return self._t
        else:
            return None

    def is_some(self) -> bool:
        """Returns true if the value exists."""
        return self._is_some == 1

    def is_none(self) -> bool:
        """Returns true if the value does not exist."""
        return self._is_some != 0


class SliceUseAsciiStringPattern(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(UseAsciiStringPattern)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> UseAsciiStringPattern:
        if i < 0:
            index = self.len+i
        else:
            index = i

        if index >= self.len:
            raise IndexError("Index out of range")

        return self.data[index]

    def copied(self) -> SliceUseAsciiStringPattern:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (UseAsciiStringPattern * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(UseAsciiStringPattern))
        rval = SliceUseAsciiStringPattern(data=ctypes.cast(array, ctypes.POINTER(UseAsciiStringPattern)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[UseAsciiStringPattern]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[UseAsciiStringPattern]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> UseAsciiStringPattern:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> UseAsciiStringPattern:
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




class callbacks:
    """Helpers to define callbacks."""
    fn_u8_rval_u8 = ctypes.CFUNCTYPE(ctypes.c_uint8, ctypes.c_uint8)
    fn_Sliceu8_rval_u8 = ctypes.CFUNCTYPE(ctypes.c_uint8, Sliceu8)
    fn_SliceVec3f32_rval_Vec3f32 = ctypes.CFUNCTYPE(Vec3f32, SliceVec3f32)
    fn_SliceMutu8 = ctypes.CFUNCTYPE(None, SliceMutu8)
    fn_u8_rval_u8 = ctypes.CFUNCTYPE(ctypes.c_uint8, ctypes.c_uint8)
    fn_u32_rval_u32 = ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32)
    fn_pconst = ctypes.CFUNCTYPE(None, ctypes.c_void_p)


class SimpleService:
    """ Some struct we want to expose as a class."""
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == SimpleService.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new_with(some_value: int) -> SimpleService:
        """ The constructor must return a `Result<Self, Error>`."""
        ctx = ctypes.c_void_p()
        c_lib.simple_service_new_with(ctx, some_value)
        self = SimpleService(SimpleService.__api_lock, ctx)
        return self

    @staticmethod
    def new_without() -> SimpleService:
        """"""
        ctx = ctypes.c_void_p()
        c_lib.simple_service_new_without(ctx, )
        self = SimpleService(SimpleService.__api_lock, ctx)
        return self

    @staticmethod
    def new_with_string(ascii: str) -> SimpleService:
        """"""
        ctx = ctypes.c_void_p()
        if not hasattr(ascii, "__ctypes_from_outparam__"):
            ascii = ctypes.cast(ascii, ctypes.POINTER(ctypes.c_char))
        c_lib.simple_service_new_with_string(ctx, ascii)
        self = SimpleService(SimpleService.__api_lock, ctx)
        return self

    @staticmethod
    def new_failing(some_value: int) -> SimpleService:
        """"""
        ctx = ctypes.c_void_p()
        c_lib.simple_service_new_failing(ctx, some_value)
        self = SimpleService(SimpleService.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.simple_service_destroy(self._ctx, )
    def method_result(self, anon1: int):
        """ Methods returning a Result<(), _> are the default and do not
 need annotations."""
        return c_lib.simple_service_method_result(self._ctx, anon1)

    def method_value(self, x: int) -> int:
        """"""
        return c_lib.simple_service_method_value(self._ctx, x)

    def method_void(self, ):
        """ This method should be documented.

 Multiple lines."""
        return c_lib.simple_service_method_void(self._ctx, )

    def method_mut_self(self, slice: Sliceu8 | ctypes.Array[ctypes.c_uint8]) -> int:
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = Sliceu8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.simple_service_method_mut_self(self._ctx, slice)

    def method_mut_self_void(self, slice: SliceBool | ctypes.Array[ctypes.c_uint8]):
        """ Single line."""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceBool(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.simple_service_method_mut_self_void(self._ctx, slice)

    def method_mut_self_ref(self, x: ctypes.POINTER(ctypes.c_uint8), y: ctypes.POINTER(ctypes.c_uint8)) -> int:
        """"""
        return c_lib.simple_service_method_mut_self_ref(self._ctx, x, y)

    def method_mut_self_ref_slice(self, x: ctypes.POINTER(ctypes.c_uint8), y: ctypes.POINTER(ctypes.c_uint8), slice: Sliceu8 | ctypes.Array[ctypes.c_uint8]) -> int:
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = Sliceu8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.simple_service_method_mut_self_ref_slice(self._ctx, x, y, slice)

    def method_mut_self_ref_slice_limited(self, x: ctypes.POINTER(ctypes.c_uint8), y: ctypes.POINTER(ctypes.c_uint8), slice: Sliceu8 | ctypes.Array[ctypes.c_uint8], slice2: Sliceu8 | ctypes.Array[ctypes.c_uint8]) -> int:
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = Sliceu8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        if hasattr(slice2, "_length_") and getattr(slice2, "_type_", "") == ctypes.c_uint8:
            slice2 = Sliceu8(data=ctypes.cast(slice2, ctypes.POINTER(ctypes.c_uint8)), len=len(slice2))

        return c_lib.simple_service_method_mut_self_ref_slice_limited(self._ctx, x, y, slice, slice2)

    def method_mut_self_ffi_error(self, slice: SliceMutu8 | ctypes.Array[ctypes.c_uint8]):
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceMutu8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.simple_service_method_mut_self_ffi_error(self._ctx, slice)

    def method_mut_self_no_error(self, slice: SliceMutu8 | ctypes.Array[ctypes.c_uint8]):
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceMutu8(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.simple_service_method_mut_self_no_error(self._ctx, slice)

    def return_slice(self, ) -> Sliceu32:
        """ Warning, you _must_ discard the returned slice object before calling into this service
 again, as otherwise undefined behavior might happen."""
        return c_lib.simple_service_return_slice(self._ctx, )

    def return_slice_mut(self, ) -> SliceMutu32:
        """ Warning, you _must_ discard the returned slice object before calling into this service
 again, as otherwise undefined behavior might happen."""
        return c_lib.simple_service_return_slice_mut(self._ctx, )

    def return_string(self, ) -> str:
        """ This function has no panic safeguards. If it panics your host app will be in an undefined state."""
        rval = c_lib.simple_service_return_string(self._ctx, )
        return ctypes.string_at(rval)

    def method_void_ffi_error(self, ):
        """"""
        return c_lib.simple_service_method_void_ffi_error(self._ctx, )

    def method_callback(self, callback):
        """"""
        if not hasattr(callback, "__ctypes_from_outparam__"):
            callback = callbacks.fn_u32_rval_u32(callback)

        return c_lib.simple_service_method_callback(self._ctx, callback)



class SimpleServiceLifetime:
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == SimpleServiceLifetime.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new_with(some_value: ctypes.POINTER(ctypes.c_uint32)) -> SimpleServiceLifetime:
        """"""
        ctx = ctypes.c_void_p()
        c_lib.simple_service_lt_new_with(ctx, some_value)
        self = SimpleServiceLifetime(SimpleServiceLifetime.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.simple_service_lt_destroy(self._ctx, )
    def method_lt(self, slice: SliceBool | ctypes.Array[ctypes.c_uint8]):
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceBool(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.simple_service_lt_method_lt(self._ctx, slice)

    def method_lt2(self, slice: SliceBool | ctypes.Array[ctypes.c_uint8]):
        """"""
        if hasattr(slice, "_length_") and getattr(slice, "_type_", "") == ctypes.c_uint8:
            slice = SliceBool(data=ctypes.cast(slice, ctypes.POINTER(ctypes.c_uint8)), len=len(slice))

        return c_lib.simple_service_lt_method_lt2(self._ctx, slice)

    def return_string_accept_slice(self, anon1: Sliceu8 | ctypes.Array[ctypes.c_uint8]) -> str:
        """"""
        if hasattr(anon1, "_length_") and getattr(anon1, "_type_", "") == ctypes.c_uint8:
            anon1 = Sliceu8(data=ctypes.cast(anon1, ctypes.POINTER(ctypes.c_uint8)), len=len(anon1))

        rval = c_lib.simple_service_lt_return_string_accept_slice(self._ctx, anon1)
        return ctypes.string_at(rval)

    def method_void_ffi_error(self, ):
        """"""
        return c_lib.simple_service_lt_method_void_ffi_error(self._ctx, )



