import common
import reference_project as r
import unittest
import sys
import ctypes

r.init_lib(common.DLL)

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
        self.assertEqual(200, r.call_tupled(tupled).x0)

    def test_callback(self):

        def my_callback(param):
            return param * 3

        self.assertEqual(9, r.callback(my_callback, 3))


if __name__ == '__main__':
    unittest.main()
    sys.exit(0)
