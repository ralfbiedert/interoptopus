import common
import reference_project as r
import unittest
import sys

r.init_lib(common.DLL)

class TestFunctions(unittest.TestCase):
    def test_panics(self):
        worked = False
        try:
            r.panics()
            worked = True
        except:
            pass

        self.assertFalse(worked)


if __name__ == '__main__':
    unittest.main()
    sys.exit(0)
