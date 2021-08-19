# Print usable error message if dependency is not installed.
try:
    from cffi import FFI
except ImportError:
    print("Please install package `cffi`, probably via `pip install cffi`.")
    print("")

api_definition = """
typedef struct cffi_vec2
    {
    float x;
    float y;
    } cffi_vec2;


cffi_vec2 my_function(cffi_vec2 input);
"""


ffi = FFI()
ffi.cdef(api_definition)
_api = None


def init_api(dll):
    """Initializes this library, call with path to DLL."""
    global _api
    _api = ffi.dlopen(dll)




class CHeapAllocated(object):
    """Base class from which all struct type wrappers are derived."""
    def __init__(self):
        pass

    def c_ptr(self):
        """Returns a C-level pointer to the native data structure."""
        return self._ctx

    def c_value(self):
        """From the underlying pointer returns the (first) entry as a value."""
        return self._ctx[0]


class int8_t(CHeapAllocated):
    """One or more heap allocated primitive `int8_t` values."""
    def __init__(self, x=None):
        self._ctx = ffi.new(f"int8_t[1]", [0])
        if x is not None:
            self._ctx[0] = x


class int16_t(CHeapAllocated):
    """One or more heap allocated primitive `int16_t` values."""
    def __init__(self, x=None):
        self._ctx = ffi.new(f"int16_t[1]", [0])
        if x is not None:
            self._ctx[0] = x


class int32_t(CHeapAllocated):
    """One or more heap allocated primitive `int32_t` values."""
    def __init__(self, x=None):
        self._ctx = ffi.new(f"int32_t[1]", [0])
        if x is not None:
            self._ctx[0] = x


class int64_t(CHeapAllocated):
    """One or more heap allocated primitive `int64_t` values."""
    def __init__(self, x=None):
        self._ctx = ffi.new(f"int64_t[1]", [0])
        if x is not None:
            self._ctx[0] = x


class uint8_t(CHeapAllocated):
    """One or more heap allocated primitive `uint8_t` values."""
    def __init__(self, x=None):
        self._ctx = ffi.new(f"uint8_t[1]", [0])
        if x is not None:
            self._ctx[0] = x


class uint16_t(CHeapAllocated):
    """One or more heap allocated primitive `uint16_t` values."""
    def __init__(self, x=None):
        self._ctx = ffi.new(f"uint16_t[1]", [0])
        if x is not None:
            self._ctx[0] = x


class uint32_t(CHeapAllocated):
    """One or more heap allocated primitive `uint32_t` values."""
    def __init__(self, x=None):
        self._ctx = ffi.new(f"uint32_t[1]", [0])
        if x is not None:
            self._ctx[0] = x


class uint64_t(CHeapAllocated):
    """One or more heap allocated primitive `uint64_t` values."""
    def __init__(self, x=None):
        self._ctx = ffi.new(f"uint64_t[1]", [0])
        if x is not None:
            self._ctx[0] = x


class CArray(CHeapAllocated):
    """Holds a native C array with a given length."""
    def __init__(self, type, n):
        self._ctx = ffi.new(f"{type}[{n}]")
        self._c_array = True
        self._len = n

    def __getitem__(self, key):
        return self._ctx[key]

    def __setitem__(self, key, value):
        self._ctx[key] = value

    def __len__(self):
        return self._len


class CSlice(CArray):
    """Holds a native C array with a given length."""
    def __init__(self, c_slice):
        self._ctx = c_slice
        self._c_slice = True
        self._len = c_slice.len

    def __getitem__(self, key):
        return self._ctx.data[key]

    def __setitem__(self, key, value):
        self._ctx.data[key] = value

    def __len__(self):
        return self._ctx.len


def ascii_string(x):
    """Must be called with a b"my_string"."""
    return ffi.new("char[]", x)




class Vec2(CHeapAllocated):
    """ A simple type in our FFI layer."""
    def __init__(self, x=None, y=None):
        self._ctx = ffi.new("cffi_vec2[]", 1)
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y

    @staticmethod
    def c_array(n):
        return CArray("cffi_vec2", n)

    @property
    def x(self):
        """"""
        return self._ctx[0].x

    @x.setter
    def x(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].x = value

    @property
    def y(self):
        """"""
        return self._ctx[0].y

    @y.setter
    def y(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].y = value


class callbacks:
    """Helpers to define `@ffi.callback`-style callbacks."""


class api:
    """Raw access to all exported functions."""
    @staticmethod
    def my_function(input):
        """ Function using the type."""
        if hasattr(input, "_ctx"):
            input = input.c_value()

        return _api.my_function(input)





