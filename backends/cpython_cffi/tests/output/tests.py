import sys
import unittest
import ctypes
import random

import reference_project as r
from reference_project import api

r.init_api("../../../../target/debug/interoptopus_reference_project.dll")


# https://stackoverflow.com/questions/52475749/maximum-and-minimum-value-of-c-types-integers-from-python
def limits(c_int_type):
    signed = c_int_type(-1).value < c_int_type(0).value
    bit_size = ctypes.sizeof(c_int_type) * 8
    signed_limit = 2 ** (bit_size - 1)
    return (-signed_limit, signed_limit - 1) if signed else (0, 2 * signed_limit - 1)


i8_min = limits(ctypes.c_int8)[0]
i16_min = limits(ctypes.c_int16)[0]
i32_min = limits(ctypes.c_int32)[0]
i64_min = limits(ctypes.c_int64)[0]

i8_max = limits(ctypes.c_int8)[1]
i16_max = limits(ctypes.c_int16)[1]
i32_max = limits(ctypes.c_int32)[1]
i64_max = limits(ctypes.c_int64)[1]

u8_max = limits(ctypes.c_uint8)[1]
u16_max = limits(ctypes.c_uint16)[1]
u32_max = limits(ctypes.c_uint32)[1]
u64_max = limits(ctypes.c_uint64)[1]

null = r.ffi.NULL


class TestConstants(unittest.TestCase):

    def test_U8(self):
        self.assertEqual(u8_max, r.U8)

    def test_COMPUTED_I32(self):
        self.assertEqual(-i32_max, r.COMPUTED_I32)

    def test_F32_MIN_POSITIVE(self):
        self.assertEqual(0.000000000000000000000000000000000000011754944, r.F32_MIN_POSITIVE)


class TestFunctions(unittest.TestCase):

    def test_U8(self):
        self.assertEqual(u8_max, r.U8)

    def test_primitives(self):
        api.primitive_void()
        api.primitive_void2()

        self.assertEqual(True, api.primitive_bool(False))

        self.assertEqual(0, api.primitive_u8(u8_max))
        self.assertEqual(0, api.primitive_u16(u16_max))
        self.assertEqual(0, api.primitive_u32(u32_max))
        self.assertEqual(0, api.primitive_u64(u64_max))

        self.assertEqual(-i8_max, api.primitive_i8(i8_max))
        self.assertEqual(-i16_max, api.primitive_i16(i16_max))
        self.assertEqual(-i32_max, api.primitive_i32(i32_max))
        self.assertEqual(-i64_max, api.primitive_i64(i64_max))

    def test_ptr(self):
        ptr = r.int64_t(100)
        # ptr = r.ffi.new("int64_t*", 100)
        ptr_ptr = r.ffi.new("int64_t**", ptr.c_ptr())

        self.assertEqual(ptr.c_ptr(), api.ptr(ptr))
        self.assertEqual(ptr_ptr, api.ptr_ptr(ptr_ptr))

        self.assertEqual(True, api.ref_option(ptr))
        self.assertEqual(True, api.ref_mut_option(ptr))

        self.assertEqual(ptr.c_ptr(), api.ptr_mut(ptr))
        self.assertEqual(ptr.c_value(), -100)

        self.assertEqual(ptr.c_ptr(), api.ref_mut_simple(ptr))
        self.assertEqual(ptr.c_value(), 100)

    def test_tuple(self):
        tupled = r.Tupled(x0=100)
        self.assertEqual(200, api.tupled(tupled).x0)

    def test_complex(self):
        vec = r.Vec3f32()
        foreign = r.SomeForeignType()

        self.assertEqual(r.FFIError.Ok, api.complex_args_1(vec, null))
        self.assertEqual(null, api.complex_args_2(foreign))

    def test_callback(self):
        def my_callback(param):
            return param * 3

        self.assertEqual(9, api.callback(my_callback, 3))

    def test_generic(self):
        uint32 = r.uint32_t(10)
        uint8 = r.uint8_t(10)

        genericu32 = r.Genericu32(x=uint32.c_ptr())
        genericu8 = r.Genericu8(x=uint8.c_ptr())
        phantom = r.Phantomu8()

        self.assertEqual(10, api.generic_1a(genericu32, phantom))
        # self.assertEqual(10, api.generic_2(genericu8.ptr()))

    def test_documented(self):
        documented = r.StructDocumented()

        self.assertEqual(r.EnumDocumented.A, api.documented(documented))

    def test_ambiguous(self):
        vec1 = r.Vec1(x=10.0)
        vec2 = r.Vec2(x=10.0, z=11.0)

        for i in range(1000):
            self.assertEqual(10.0, api.ambiguous_1(vec1).x)
            self.assertEqual(11.0, api.ambiguous_2(vec2).z)
            self.assertEqual(True, api.ambiguous_3(vec1, vec2))


    def test_namespaces(self):
        vec = r.Vec(x=10)

        self.assertEqual(10.0, api.namespaced_type(vec).x)

    def test_panics(self):
        worked = False
        try:
            api.panics()
            worked = True
        except:
            pass

        self.assertFalse(worked)


class TestPatterns(unittest.TestCase):

    def test_ascii_pointers(self):
        some_str_5 = b"01234"
        some_str_5_ascii = r.ascii_string(some_str_5)
        use_ascii = r.UseAsciiStringPattern(ascii_string=some_str_5_ascii)

        self.assertEqual(0, api.pattern_ascii_pointer_1(null))
        self.assertEqual(10, api.pattern_ascii_pointer_1(b"0123456789"))

        self.assertEqual(10, api.pattern_ascii_pointer_len(some_str_5, use_ascii))
        self.assertEqual(5, api.pattern_ascii_pointer_len(null, use_ascii))

        # Test these really hard because we've had some spurious errors
        for i in range(1000):
            s = b"x" * random.randint(0, 100)
            self.assertEqual(len(s), api.pattern_ascii_pointer_1(s))


    def test_slices(self):
        # uint32 = r.CArray("uint32_t", 10_000)
        uints = r.uint32_t.c_array(10_000)
        vecs = r.Vec3f32.c_array(100)

        api.pattern_ffi_slice_1(uints)
        api.pattern_ffi_slice_2(vecs, 1000)

        some_value = 0

        def my_callback(param):
            nonlocal some_value
            some_value = param[4]
            return 0

        api.pattern_ffi_slice_delegate(my_callback)

        self.assertEqual(some_value, 4)

    def test_services(self):
        s1 = r.SimpleService.new_without()
        s2 = r.SimpleService.new_with(123)

        try:
            s3 = r.SimpleService.new_failing()
            s3.method_void()
        except:
            pass

        assert (s1.method_value(123) == s2.method_value(123))


if __name__ == '__main__':
    unittest.main()
    sys.exit(0)

