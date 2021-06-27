import unittest
import ctypes
import random
import reference_project as r

r.init_api("../../../target/debug/interoptopus_reference_project.dll")

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
        r.raw.primitive_void()
        r.raw.primitive_void2()

        self.assertEqual(True, r.raw.primitive_bool(False))

        self.assertEqual(0, r.raw.primitive_u8(u8_max))
        self.assertEqual(0, r.raw.primitive_u16(u16_max))
        self.assertEqual(0, r.raw.primitive_u32(u32_max))
        self.assertEqual(0, r.raw.primitive_u64(u64_max))

        self.assertEqual(-i8_max, r.raw.primitive_i8(i8_max))
        self.assertEqual(-i16_max, r.raw.primitive_i16(i16_max))
        self.assertEqual(-i32_max, r.raw.primitive_i32(i32_max))
        self.assertEqual(-i64_max, r.raw.primitive_i64(i64_max))

    def test_ptr(self):
        ptr = r.ffi.new("int64_t*", 100)
        ptr_ptr = r.ffi.new("int64_t**", ptr)

        self.assertEqual(ptr, r.raw.ptr(ptr))
        self.assertEqual(ptr_ptr, r.raw.ptr_ptr(ptr_ptr))

        self.assertEqual(True, r.raw.ref_option(ptr))
        self.assertEqual(True, r.raw.ref_mut_option(ptr))

        self.assertEqual(ptr, r.raw.ptr_mut(ptr))
        self.assertEqual(ptr[0], -100)

        self.assertEqual(ptr, r.raw.ref_mut_simple(ptr))
        self.assertEqual(ptr[0], 100)

    def test_tuple(self):
        tuple = r.Tupled()
        tuple.x0 = 100

        self.assertEqual(200, r.raw.tupled(tuple).x0)

    def test_complex(self):
        vec = r.Vec3f32()
        foreign = r.SomeForeignType()

        ptr = r.ffi.new("int64_t*", 100)

        self.assertEqual(r.FFIError.Ok, r.raw.complex_args_1(vec, null))
        self.assertEqual(null, r.raw.complex_args_2(foreign))

    def test_callback(self):
        @r.ffi.callback(r.callbacks.fn_u8_rval_u8)
        def my_callback(param):
            return param * 3

        self.assertEqual(9, r.raw.callback(my_callback, 3))

    def test_generic(self):
        uint32 = r.ffi.new("uint32_t *", 10)
        uint8 = r.ffi.new("uint8_t *", 10)

        genericu32 = r.Genericu32()
        genericu32.x = uint32

        genericu8 = r.Genericu8()
        genericu8.x = uint8

        phantom = r.Phantomu8()

        self.assertEqual(10, r.raw.generic_1(genericu32, phantom))
        self.assertEqual(10, r.raw.generic_2(genericu8, phantom))

    def test_documented(self):
        documented = r.StructDocumented()

        self.assertEqual(r.EnumDocumented.A, r.raw.documented(documented))


    def test_ambiguous(self):
        vec1 = r.Vec1()
        vec1.x = 10.0

        vec2 = r.Vec2()
        vec2.x = 10.0
        vec2.z = 11.0

        for i in range(1000):
            self.assertEqual(10.0, r.raw.ambiguous_1(vec1).x)
            self.assertEqual(11.0, r.raw.ambiguous_2(vec2).z)
            self.assertEqual(True, r.raw.ambiguous_3(vec1, vec2))


    def test_namespaces(self):
        vec = r.Vec()
        vec.x = 10

        self.assertEqual(10.0, r.raw.namespaced_type(vec).x)


    def test_panics(self):
        self.assertEqual(r.FFIError.Panic, r.raw.panics())


class TestPatterns(unittest.TestCase):

    def test_ascii_pointers(self):
        some_str_10 = r.ascii_string(b"0123456789")
        some_str_5 = r.ascii_string(b"01234")
        use_ascii = r.UseAsciiStringPattern()
        use_ascii.ascii_string = some_str_5

        self.assertEqual(0, r.raw.pattern_ascii_pointer_1(null))
        self.assertEqual(10, r.raw.pattern_ascii_pointer_1(some_str_10))

        self.assertEqual(10, r.raw.pattern_ascii_pointer_len(some_str_5, use_ascii))
        self.assertEqual(5, r.raw.pattern_ascii_pointer_len(null, use_ascii))

        # Test these really hard because we've had some spurious errors
        for i in range(1000):
            s = b"x" * random.randint(0, 100)
            astr = r.ascii_string(s)

            self.assertEqual(len(s), r.raw.pattern_ascii_pointer_1(astr))


    # def test_options(self):
    #     inner = r.Inner()
    #     option = r.Op

    def test_api_entry(self):
        api = r.raw.my_api_init_v1()
        print(api)


if __name__ == '__main__':
    unittest.main()
