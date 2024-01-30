from __future__ import annotations
import ctypes
import typing

T = typing.TypeVar("T")
c_lib = None

def init_lib(path):
    """Initializes the native library. Must be called at least once before anything else."""
    global c_lib
    c_lib = ctypes.cdll.LoadLibrary(path)

    c_lib.my_function.argtypes = [Vec2]
    c_lib.my_function2.argtypes = [Vec2]
    c_lib.my_function3.argtypes = [Vec2, ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(ctypes.c_int32)]

    c_lib.my_function.restype = Vec2



def my_function(input: Vec2) -> Vec2:
    """ Function using the type."""
    return c_lib.my_function(input)

def my_function2(input: Vec2):
    """ Function using the type."""
    return c_lib.my_function2(input)

def my_function3(input: Vec2, out_param_0: ctypes.POINTER(ctypes.c_int32), out_param_1: ctypes.POINTER(ctypes.c_int32)):
    return c_lib.my_function3(input, out_param_0, out_param_1)





TRUE = ctypes.c_uint8(1)
FALSE = ctypes.c_uint8(0)


def _errcheck(returned, success):
    """Checks for FFIErrors and converts them to an exception."""
    if returned == success: return
    else: raise Exception(f"Function returned error: {returned}")


class CallbackVars(object):
    """Helper to be used `lambda x: setattr(cv, "x", x)` when getting values from callbacks."""
    def __str__(self):
        rval = ""
        for var in  filter(lambda x: "__" not in x, dir(self)):
            rval += f"{var}: {getattr(self, var)}"
        return rval


class _Iter(object):
    """Helper for slice iterators."""
    def __init__(self, target):
        self.i = 0
        self.target = target

    def __iter__(self):
        self.i = 0
        return self

    def __next__(self):
        if self.i >= self.target.len:
            raise StopIteration()
        rval = self.target[self.i]
        self.i += 1
        return rval


class Vec2(ctypes.Structure):
    """ A simple type in our FFI layer."""

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
    ]

    def __init__(self, x: float = None, y: float = None):
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y

    @property
    def x(self) -> float:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: float):
        return ctypes.Structure.__set__(self, "x", value)

    @property
    def y(self) -> float:
        return ctypes.Structure.__get__(self, "y")

    @y.setter
    def y(self, value: float):
        return ctypes.Structure.__set__(self, "y", value)




class callbacks:
    """Helpers to define callbacks."""


