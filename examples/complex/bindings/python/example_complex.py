from cffi import FFI

api_definition = """




const uint32_t CFFI_THE_MAGIC_CONSTANT = 666;

typedef enum cffi_ffierror
    {
    CFFI_FFIERROR_OK = 0,
    CFFI_FFIERROR_NULLPOINTERPASSED = 10,
    } cffi_ffierror;

typedef struct cffi_context cffi_context;
typedef struct cffi_thirdpartyvecf32
    {
    float x;
    float y;
    float z;
    float w;
    } cffi_thirdpartyvecf32;

typedef struct cffi_vec3
    {
    float x;
    float y;
    float z;
    } cffi_vec3;

typedef uint32_t (*cffi_fptr_fn_u32_rval_u32)(uint32_t x0);

typedef struct cffi_supercomplexentity
    {
    cffi_vec3 player_1;
    cffi_vec3 player_2;
    uint64_t ammo;
    uint8_t* some_str;
    uint32_t str_len;
    } cffi_supercomplexentity;

typedef struct cffi_withforeigntype
    {
    uint64_t secret_number;
    cffi_thirdpartyvecf32* third_party;
    } cffi_withforeigntype;


uint32_t example_api_version();
cffi_ffierror example_always_fails();
cffi_ffierror example_create_context(cffi_context** context_ptr);
cffi_ffierror example_destroy_context(cffi_context** context_ptr);
cffi_ffierror example_print_score(cffi_context* context);
cffi_ffierror example_return_score(cffi_context* context, uint32_t* score);
cffi_ffierror example_update_score_by_callback(cffi_context* context, cffi_fptr_fn_u32_rval_u32 update);
cffi_ffierror example_write_foreign_type(cffi_context* context, cffi_withforeigntype* foreign);
cffi_ffierror example_double_super_complex_entity(cffi_context* context, cffi_supercomplexentity* incoming, cffi_supercomplexentity* outgoing);
"""


ffi = FFI()
ffi.cdef(api_definition)
_api = None


def init_api(dll):
    """Initializes this library, call with path to DLL."""
    global _api
    _api = ffi.dlopen(dll)




#  Call for a friend.
THE_MAGIC_CONSTANT = 666


class FFIError:
    """ Possible errors in our library."""
    Ok = 0
    NullPointerPassed = 10


class SuperComplexEntity(object):
    """ A vector used in our game engine."""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_supercomplexentity[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_supercomplexentity[]", n)

    def ptr(self):
        return self._ctx

    @property
    def player_1(self):
        """"""
        return self._ctx[0].player_1

    @player_1.setter
    def player_1(self, value):
        self._ptr_player_1 = value
        self._ctx[0].player_1 = value

    @property
    def player_2(self):
        """"""
        return self._ctx[0].player_2

    @player_2.setter
    def player_2(self, value):
        self._ptr_player_2 = value
        self._ctx[0].player_2 = value

    @property
    def ammo(self):
        """"""
        return self._ctx[0].ammo

    @ammo.setter
    def ammo(self, value):
        self._ptr_ammo = value
        self._ctx[0].ammo = value

    @property
    def some_str(self):
        """ Point to an ASCII encoded whatnot."""
        return self._ctx[0].some_str

    @some_str.setter
    def some_str(self, value):
        self._ptr_some_str = value
        self._ctx[0].some_str = value

    @property
    def str_len(self):
        """"""
        return self._ctx[0].str_len

    @str_len.setter
    def str_len(self, value):
        self._ptr_str_len = value
        self._ctx[0].str_len = value

class ThirdPartyVecF32(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_thirdpartyvecf32[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_thirdpartyvecf32[]", n)

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

    @property
    def z(self):
        """"""
        return self._ctx[0].z

    @z.setter
    def z(self, value):
        self._ptr_z = value
        self._ctx[0].z = value

    @property
    def w(self):
        """"""
        return self._ctx[0].w

    @w.setter
    def w(self, value):
        self._ptr_w = value
        self._ctx[0].w = value

class Vec3(object):
    """ A vector used in our game engine."""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_vec3[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_vec3[]", n)

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

    @property
    def z(self):
        """"""
        return self._ctx[0].z

    @z.setter
    def z(self, value):
        self._ptr_z = value
        self._ctx[0].z = value

class WithForeignType(object):
    """ A type containing a third-party type."""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_withforeigntype[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_withforeigntype[]", n)

    def ptr(self):
        return self._ctx

    @property
    def secret_number(self):
        """"""
        return self._ctx[0].secret_number

    @secret_number.setter
    def secret_number(self, value):
        self._ptr_secret_number = value
        self._ctx[0].secret_number = value

    @property
    def third_party(self):
        """"""
        return self._ctx[0].third_party

    @third_party.setter
    def third_party(self, value):
        self._ptr_third_party = value
        self._ctx[0].third_party = value



class callbacks:
    """Helpers to define `@ffi.callback`-style callbacks."""
    fn_u32_rval_u32 = "uint32_t(uint32_t)"




class raw:
    """Raw access to all exported functions."""
    def example_api_version():
        """ Returns the version of this API."""
        global _api
        return _api.example_api_version()

    def example_always_fails():
        """ A function that always fails."""
        global _api
        return _api.example_always_fails()

    def example_create_context(context_ptr):
        """ Creates a new instance of this library."""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        return _api.example_create_context(context_ptr)

    def example_destroy_context(context_ptr):
        """ Deletes an existing instance of this library.

 You **must** ensure that `context_ptr` is being called with the context produced by
 `example_create_context`, otherwise bad things will happen."""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        return _api.example_destroy_context(context_ptr)

    def example_print_score(context):
        """ Prints the current player score."""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        return _api.example_print_score(context)

    def example_return_score(context, score):
        """ Updates the score."""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(score, "_ctx"):
            score = score._ctx[0]
        return _api.example_return_score(context, score)

    def example_update_score_by_callback(context, update):
        """ Updates the score."""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(update, "_ctx"):
            update = update._ctx[0]
        return _api.example_update_score_by_callback(context, update)

    def example_write_foreign_type(context, foreign):
        """ Accepts some foreign types."""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(foreign, "_ctx"):
            foreign = foreign._ctx[0]
        return _api.example_write_foreign_type(context, foreign)

    def example_double_super_complex_entity(context, incoming, outgoing):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(incoming, "_ctx"):
            incoming = incoming._ctx[0]
        if hasattr(outgoing, "_ctx"):
            outgoing = outgoing._ctx[0]
        return _api.example_double_super_complex_entity(context, incoming, outgoing)







def ascii_string(x):
    """Must be called with a b"my_string"."""
    global ffi
    return ffi.new("char[]", x)



