from cffi import FFI

api_definition = """




#define THE_MAGIC_CONSTANT 666

typedef enum FFIError
    {
    Ok = 0,
    NullPointerPassed = 10,
    } FFIError;

typedef struct Context Context;

typedef struct ThirdPartyVecF32
    {
    float x;
    float y;
    float z;
    float w;
    } ThirdPartyVecF32;

typedef struct Vec2f32
    {
    float x;
    float y;
    float z;
    } Vec2f32;

typedef uint32_t (*fptr_fn_u32_rval_u32)(uint32_t x0);

typedef struct SuperComplexEntity
    {
    Vec2f32 player_1;
    Vec2f32 player_2;
    uint64_t ammo;
    uint8_t* some_str;
    uint32_t str_len;
    } SuperComplexEntity;

typedef struct WithForeignType
    {
    uint64_t secret_number;
    ThirdPartyVecF32* third_party;
    } WithForeignType;


uint32_t example_api_version();
FFIError example_always_fails();
FFIError example_create_context(Context** context_ptr);
FFIError example_destroy_context(Context** context_ptr);
FFIError example_print_score(Context* context);
FFIError example_return_score(Context* context, uint32_t* score);
FFIError example_update_score_by_callback(Context* context, fptr_fn_u32_rval_u32 update);
FFIError example_write_foreign_type(Context* context, WithForeignType* foreign);
FFIError example_double_super_complex_entity(Context* context, SuperComplexEntity* incoming, SuperComplexEntity* outgoing);
"""


ffi = FFI()
ffi.cdef(api_definition)
_api = None


def init_api(dll):
    """Initializes this library, call with path to DLL."""
    global _api
    _api = ffi.dlopen(dll)




# Call for a friend.
THE_MAGIC_CONSTANT = 666


class FFIError:
    """Possible errors in our library."""
    Ok = 0
    NullPointerPassed = 10




class callbacks:
    """Helpers to define `@ffi.callback`-style callbacks."""
    fn_u32_rval_u32 = "uint32_t(uint32_t)"




class raw:
    """Raw access to all exported functions."""
    def example_api_version():
        """Returns the version of this API."""
        global _api
        return _api.example_api_version()

    def example_always_fails():
        """A function that always fails."""
        global _api
        return _api.example_always_fails()

    def example_create_context(context_ptr):
        """Creates a new instance of this library."""
        global _api
        return _api.example_create_context(context_ptr)

    def example_destroy_context(context_ptr):
        """Deletes an existing instance of this library.

You **must** ensure that `context_ptr` is being called with the context produced by
`example_create_context`, otherwise bad things will happen."""
        global _api
        return _api.example_destroy_context(context_ptr)

    def example_print_score(context):
        """Prints the current player score."""
        global _api
        return _api.example_print_score(context)

    def example_return_score(context, score):
        """Updates the score."""
        global _api
        return _api.example_return_score(context, score)

    def example_update_score_by_callback(context, update):
        """Updates the score."""
        global _api
        return _api.example_update_score_by_callback(context, update)

    def example_write_foreign_type(context, foreign):
        """Accepts some foreign types."""
        global _api
        return _api.example_write_foreign_type(context, foreign)

    def example_double_super_complex_entity(context, incoming, outgoing):
        """"""
        global _api
        return _api.example_double_super_complex_entity(context, incoming, outgoing)







