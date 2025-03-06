import common
import reference_project as r
import unittest
import sys

r.init_lib(common.DLL)

class TestFunctions(unittest.TestCase):
    def test_slice_callback(self):
        pass

        # Callbacks are (ptr_fn, ptr_data) now, code gen needs to be reworked

        # def callback(x):
        #     self.assertEqual(9, x[-1])
        #     self.assertEqual(9, x.last())
        #     self.assertEqual(9, x.bytearray()[-1])
        #     try:
        #         i = x[10]
        #         self.assertFalse(True, "Index out of error should throw exception")
        #     except IndexError:
        #         pass
        #
        #     return 0
        #
        # r.pattern_ffi_slice_delegate(callback)


if __name__ == '__main__':
    unittest.main()
    sys.exit(0)
