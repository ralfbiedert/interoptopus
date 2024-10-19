import common
import reference_project as r
import unittest
import sys

r.init_lib(common.DLL)

class TestFunctions(unittest.TestCase):
    def test_namespaces(self):
        vec = r.Vec(x=10)
        self.assertEqual(10.0, r.namespaced_type(vec).x)



if __name__ == '__main__':
    unittest.main()
    sys.exit(0)
