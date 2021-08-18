from cffi import FFI

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




class BaseStruct(object):
    """Base class from which all struct type wrappers are derived."""
    def __init__(self):
        pass

    def c_ptr(self):
        """Returns a C-level pointer to the native data structure."""
        return self._ctx

    def c_value(self):
        """From the underlying pointer returns the (first) entry as a value."""
        return self._ctx[0]


class CArray(BaseStruct):
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


def ascii_string(x):
    """Must be called with a b"my_string"."""
    return ffi.new("char[]", x)




class Vec2(BaseStruct):
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
        self._ptr_x = value
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
        self._ptr_y = value
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





