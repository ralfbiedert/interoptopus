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






class Vec2(object):
    """ A simple type in our FFI layer."""
    def __init__(self, x = None, y = None):
        global _api, ffi
        self._ctx = ffi.new("cffi_vec2[]", 1)
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y

    def c_array(n):
        global _api, ffi
        return CArray("cffi_vec2", n)

    def c_ptr(self):
        return self._ctx

    def c_value(self):
        return self._ctx[0]

    @property
    def x(self):
        """"""
        return self._ctx[0].x

    @x.setter
    def x(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value._ctx
            else:
                value = value._ctx[0]
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
                value = value._ctx
            else:
                value = value._ctx[0]
        self._ptr_y = value
        self._ctx[0].y = value



class callbacks:
    """Helpers to define `@ffi.callback`-style callbacks."""




class raw:
    """Raw access to all exported functions."""
    def my_function(input):
        """ Function using the type."""
        global _api
        if hasattr(input, "_ctx"):
            input = input._ctx[0]
        return _api.my_function(input)







class CArray(object):
    """Holds a native C array with a given length."""
    def __init__(self, type, n):
        self._ctx = ffi.new(f"{type}[{n}]")
        self._len = n
        self._c_array = True

    def __getitem__(self, key):
        return self._ctx[key]

    def __setitem__(self, key, value):
        self._ctx[key] = value


def ascii_string(x):
    """Must be called with a b"my_string"."""
    global ffi
    return ffi.new("char[]", x)



