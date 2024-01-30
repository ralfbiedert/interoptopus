from __future__ import annotations
import ctypes
import typing

T = typing.TypeVar("T")
c_lib = None

def init_lib(path):
    """Initializes the native library. Must be called at least once before anything else."""
    global c_lib
    c_lib = ctypes.cdll.LoadLibrary(path)

    c_lib.example_api_version.argtypes = []
    c_lib.example_always_fails.argtypes = []
    c_lib.example_create_context.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    c_lib.example_destroy_context.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    c_lib.example_print_score.argtypes = [ctypes.c_void_p]
    c_lib.example_return_score.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint32)]
    c_lib.example_update_score_by_callback.argtypes = [ctypes.c_void_p, ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32)]
    c_lib.example_write_foreign_type.argtypes = [ctypes.c_void_p, ctypes.POINTER(ThirdPartyVecF32)]
    c_lib.example_double_super_complex_entity.argtypes = [ctypes.c_void_p, ctypes.POINTER(SuperComplexEntity), ctypes.POINTER(SuperComplexEntity)]

    c_lib.example_api_version.restype = ctypes.c_uint32
    c_lib.example_always_fails.restype = ctypes.c_int
    c_lib.example_create_context.restype = ctypes.c_int
    c_lib.example_destroy_context.restype = ctypes.c_int
    c_lib.example_print_score.restype = ctypes.c_int
    c_lib.example_return_score.restype = ctypes.c_int
    c_lib.example_update_score_by_callback.restype = ctypes.c_int
    c_lib.example_write_foreign_type.restype = ctypes.c_int
    c_lib.example_double_super_complex_entity.restype = ctypes.c_int



def example_api_version() -> int:
    """ Returns the version of this API."""
    return c_lib.example_api_version()

def example_always_fails() -> ctypes.c_int:
    """ A function that always fails."""
    return c_lib.example_always_fails()

def example_create_context(context_ptr: ctypes.POINTER(ctypes.c_void_p)) -> ctypes.c_int:
    """ Creates a new instance of this library."""
    return c_lib.example_create_context(context_ptr)

def example_destroy_context(context_ptr: ctypes.POINTER(ctypes.c_void_p)) -> ctypes.c_int:
    """ Deletes an existing instance of this library.

 You **must** ensure that `context_ptr` is being called with the context produced by
 `example_create_context`, otherwise bad things will happen."""
    return c_lib.example_destroy_context(context_ptr)

def example_print_score(context: ctypes.c_void_p) -> ctypes.c_int:
    """ Prints the current player score."""
    return c_lib.example_print_score(context)

def example_return_score(context: ctypes.c_void_p, score: ctypes.POINTER(ctypes.c_uint32)) -> ctypes.c_int:
    """ Updates the score."""
    return c_lib.example_return_score(context, score)

def example_update_score_by_callback(context: ctypes.c_void_p, update) -> ctypes.c_int:
    """ Updates the score."""
    if not hasattr(update, "__ctypes_from_outparam__"):
        update = callbacks.fn_u32_rval_u32(update)

    return c_lib.example_update_score_by_callback(context, update)

def example_write_foreign_type(context: ctypes.c_void_p, foreign: ctypes.POINTER(ThirdPartyVecF32)) -> ctypes.c_int:
    """ Accepts some foreign types."""
    return c_lib.example_write_foreign_type(context, foreign)

def example_double_super_complex_entity(context: ctypes.c_void_p, incoming: ctypes.POINTER(SuperComplexEntity), outgoing: ctypes.POINTER(SuperComplexEntity)) -> ctypes.c_int:
    return c_lib.example_double_super_complex_entity(context, incoming, outgoing)



THE_MAGIC_CONSTANT = 666


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


class FFIError:
    """ Possible errors in our library."""
    #  All went fine.
    Ok = 0
    #  Naughty API call detected.
    NullPointerPassed = 10


class ThirdPartyVecF32(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
        ("z", ctypes.c_float),
        ("w", ctypes.c_float),
    ]

    def __init__(self, x: float = None, y: float = None, z: float = None, w: float = None):
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y
        if z is not None:
            self.z = z
        if w is not None:
            self.w = w

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

    @property
    def z(self) -> float:
        return ctypes.Structure.__get__(self, "z")

    @z.setter
    def z(self, value: float):
        return ctypes.Structure.__set__(self, "z", value)

    @property
    def w(self) -> float:
        return ctypes.Structure.__get__(self, "w")

    @w.setter
    def w(self, value: float):
        return ctypes.Structure.__set__(self, "w", value)


class Vec3(ctypes.Structure):
    """ A vector used in our game engine."""

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
        ("z", ctypes.c_float),
    ]

    def __init__(self, x: float = None, y: float = None, z: float = None):
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y
        if z is not None:
            self.z = z

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

    @property
    def z(self) -> float:
        return ctypes.Structure.__get__(self, "z")

    @z.setter
    def z(self, value: float):
        return ctypes.Structure.__set__(self, "z", value)


class SuperComplexEntity(ctypes.Structure):
    """ A vector used in our game engine."""

    # These fields represent the underlying C data layout
    _fields_ = [
        ("player_1", Vec3),
        ("player_2", Vec3),
        ("ammo", ctypes.c_uint64),
        ("some_str", ctypes.POINTER(ctypes.c_uint8)),
        ("str_len", ctypes.c_uint32),
    ]

    def __init__(self, player_1: Vec3 = None, player_2: Vec3 = None, ammo: int = None, some_str: ctypes.POINTER(ctypes.c_uint8) = None, str_len: int = None):
        if player_1 is not None:
            self.player_1 = player_1
        if player_2 is not None:
            self.player_2 = player_2
        if ammo is not None:
            self.ammo = ammo
        if some_str is not None:
            self.some_str = some_str
        if str_len is not None:
            self.str_len = str_len

    @property
    def player_1(self) -> Vec3:
        return ctypes.Structure.__get__(self, "player_1")

    @player_1.setter
    def player_1(self, value: Vec3):
        return ctypes.Structure.__set__(self, "player_1", value)

    @property
    def player_2(self) -> Vec3:
        return ctypes.Structure.__get__(self, "player_2")

    @player_2.setter
    def player_2(self, value: Vec3):
        return ctypes.Structure.__set__(self, "player_2", value)

    @property
    def ammo(self) -> int:
        return ctypes.Structure.__get__(self, "ammo")

    @ammo.setter
    def ammo(self, value: int):
        return ctypes.Structure.__set__(self, "ammo", value)

    @property
    def some_str(self) -> ctypes.POINTER(ctypes.c_uint8):
        """ Point to an ASCII encoded whatnot."""
        return ctypes.Structure.__get__(self, "some_str")

    @some_str.setter
    def some_str(self, value: ctypes.POINTER(ctypes.c_uint8)):
        """ Point to an ASCII encoded whatnot."""
        return ctypes.Structure.__set__(self, "some_str", value)

    @property
    def str_len(self) -> int:
        return ctypes.Structure.__get__(self, "str_len")

    @str_len.setter
    def str_len(self, value: int):
        return ctypes.Structure.__set__(self, "str_len", value)




class callbacks:
    """Helpers to define callbacks."""
    fn_u32_rval_u32 = ctypes.CFUNCTYPE(ctypes.c_uint32, ctypes.c_uint32)


