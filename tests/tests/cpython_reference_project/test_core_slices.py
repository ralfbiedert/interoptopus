import common
import reference_project as r
import unittest
import sys
import ctypes

r.init_lib(common.DLL)

class TestFunctions(unittest.TestCase):
    def test_namespaces(self):
        vec = r.Vec(x=10)
        self.assertEqual(10.0, r.namespaced_type(vec).x)

    def test_slice_from_ctypes_array(self):
        array = (ctypes.c_uint32 * 10)()
        returned_length = r.pattern_ffi_slice_1(array)
        self.assertEqual(len(array), returned_length)

        slice = r.SliceU32(data=ctypes.cast(array, ctypes.POINTER(ctypes.c_uint32)), len=len(array))
        returned_length = r.pattern_ffi_slice_1(slice)
        self.assertEqual(len(slice), returned_length)

    # Callbacks are (ptr_fn, ptr_data) now, code gen needs to be reworked
    # def test_slice_from_ctypes_array_callback(self):
    #     array = (ctypes.c_uint8 * 10)()
    #
    #     def callback(x):
    #         self.assertEqual(1, x[0])
    #         self.assertEqual(0, x[1])
    #
    #     r.pattern_ffi_slice_3(array, callback)


if __name__ == '__main__':
    unittest.main()
    sys.exit(0)
