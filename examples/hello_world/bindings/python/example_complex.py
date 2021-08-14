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
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_vec2[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_vec2[]", n)

    def ptr(self):
        return self._ctx

    @property
    def x(self):
        """"""
        return self._ctx[0].x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx[0].x = value

    @property
    def y(self):
        """"""
        return self._ctx[0].y

    @y.setter
    def y(self, value):
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







def ascii_string(x):
    """Must be called with a b"my_string"."""
    global ffi
    return ffi.new("char[]", x)



