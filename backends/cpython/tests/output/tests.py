import ctypes
import reference_project as r
import unittest
import sys

r.init_lib("../../../../target/debug/interoptopus_reference_project.dll")


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


class TestFunctions(unittest.TestCase):

    def test_primitives(self):
        r.primitive_void()
        r.primitive_void2()

        self.assertEqual(True, r.primitive_bool(False))

        self.assertEqual(0, r.primitive_u8(u8_max))
        self.assertEqual(0, r.primitive_u16(u16_max))
        self.assertEqual(0, r.primitive_u32(u32_max))
        self.assertEqual(0, r.primitive_u64(u64_max))

        self.assertEqual(-i8_max, r.primitive_i8(i8_max))
        self.assertEqual(-i16_max, r.primitive_i16(i16_max))
        self.assertEqual(-i32_max, r.primitive_i32(i32_max))
        self.assertEqual(-i64_max, r.primitive_i64(i64_max))

    def test_ptr(self):
        ptr = (ctypes.c_int64 * 100)(100, 2, 3)
        ptr_ptr = ctypes.POINTER(ctypes.POINTER(ctypes.c_int64))()

        r.ptr_ptr(ptr_ptr)

        # This is unfortunately a bit ugly in ctypes. To compare pointers for equality you must
        # cast them to `c_void_p` AND compare `.value`.
        self.assertEqual(ctypes.cast(ptr, ctypes.c_void_p).value, ctypes.cast(r.ptr(ptr), ctypes.c_void_p).value)
        self.assertEqual(ctypes.cast(ptr_ptr, ctypes.c_void_p).value,
                         ctypes.cast(r.ptr_ptr(ptr_ptr), ctypes.c_void_p).value)

        self.assertEqual(True, r.ref_option(ptr))
        self.assertEqual(True, r.ref_mut_option(ptr))

        self.assertEqual(ctypes.cast(ptr, ctypes.c_void_p).value, ctypes.cast(r.ptr_mut(ptr), ctypes.c_void_p).value)
        self.assertEqual(-100, ptr[0])

    def test_tuple(self):
        tupled = r.Tupled(x0=100)
        self.assertEqual(200, r.tupled(tupled).x0)

    def test_complex(self):
        vec = r.Vec3f32()
        foreign = r.SomeForeignType()

        r.complex_args_1(vec, None)
        self.assertEqual(None, r.complex_args_2(foreign))

    def test_callback(self):

        def my_callback(param):
            return param * 3

        self.assertEqual(9, r.callback(my_callback, 3))

    def test_generic(self):
        uint32 = (ctypes.c_uint32 * 10)(123)

        genericu32 = r.Genericu32(uint32)
        phantom = r.Phantomu8()

        self.assertEqual(123, r.generic_1a(genericu32, phantom))

    def test_documented(self):
        documented = r.StructDocumented()

        self.assertEqual(r.EnumDocumented.A, r.documented(documented))

    def test_ambiguous(self):
        vec1 = r.Vec1(x=10.0)
        vec2 = r.Vec2(x=10.0, z=11.0)

        for i in range(1000):
            self.assertEqual(10.0, r.ambiguous_1(vec1).x)
            self.assertEqual(11.0, r.ambiguous_2(vec2).z)
            self.assertEqual(True, r.ambiguous_3(vec1, vec2))

    def test_namespaces(self):
        vec = r.Vec(x=10)

        self.assertEqual(10.0, r.namespaced_type(vec).x)

    def test_panics(self):
        worked = False
        try:
            r.panics()
            worked = True
        except:
            pass

        self.assertFalse(worked)


class TestPatterns(unittest.TestCase):

    def test_services(self):
        service = r.SimpleService.new_with(123)
        slice = service.return_slice_mut()

        self.assertEqual(10, service.method_value(10))
        self.assertEqual(123, slice[0])

    def test_ascii_pointer(self):
        self.assertEqual(3, r.pattern_ascii_pointer_1(b"111"))
        self.assertEqual(b'', r.pattern_ascii_pointer_2())


if __name__ == '__main__':
    unittest.main()
    sys.exit(0)
