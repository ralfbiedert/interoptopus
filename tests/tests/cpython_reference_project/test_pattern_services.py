import common
import reference_project as r
import unittest
import sys

r.init_lib(common.DLL)

class TestFunctions(unittest.TestCase):
    def test_services(self):
        service = r.SimpleService.new_with(123)
        slice = service.return_slice_mut()

        self.assertEqual(10, service.method_value(10))
        self.assertEqual(123, slice[0])


if __name__ == '__main__':
    unittest.main()
    sys.exit(0)
