import common
import reference_project as r
import unittest
import sys

r.init_lib(common.DLL)

class TestFunctions(unittest.TestCase):
    def test_service_new_string(self):
        service = r.SimpleService.new_with_string(b"abc")

    def test_ascii_pointer(self):
        self.assertEqual(3, r.pattern_ascii_pointer_1(b"111"))
        self.assertEqual(b'', r.pattern_ascii_pointer_2())

    def test_c_char(self):
        self.assertEqual(b'X', r.pattern_ffi_cchar(b'X'))


if __name__ == '__main__':
    unittest.main()
    sys.exit(0)
