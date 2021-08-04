from cffi import FFI

api_definition = """




const uint8_t CFFI_U8 = 255;
const float CFFI_F32_MIN_POSITIVE = 0.000000000000000000000000000000000000011754944;
const int32_t CFFI_COMPUTED_I32 = -2147483647;

typedef enum cffi_enumdocumented
    {
    CFFI_A = 0,
    CFFI_B = 1,
    CFFI_C = 2,
    } cffi_enumdocumented;

typedef enum cffi_enumrenamed
    {
    CFFI_X = 0,
    } cffi_enumrenamed;

typedef struct cffi_generic2u8 cffi_generic2u8;
typedef struct cffi_generic3 cffi_generic3;
typedef struct cffi_generic4 cffi_generic4;
typedef struct cffi_opaque cffi_opaque;
typedef struct cffi_simpleservice cffi_simpleservice;
typedef struct cffi_empty cffi_empty;

typedef enum cffi_ffierror
    {
    CFFI_OK = 0,
    CFFI_NULL = 100,
    CFFI_PANIC = 200,
    CFFI_FAIL = 300,
    } cffi_ffierror;

typedef struct cffi_extratypef32
    {
    float x;
    } cffi_extratypef32;

typedef struct cffi_inner
    {
    float x;
    } cffi_inner;

typedef struct cffi_phantomu8
    {
    uint32_t x;
    } cffi_phantomu8;

typedef struct cffi_someforeigntype
    {
    uint32_t x;
    } cffi_someforeigntype;

typedef struct cffi_structdocumented
    {
    float x;
    } cffi_structdocumented;

typedef struct cffi_structrenamed
    {
    cffi_enumrenamed e;
    } cffi_structrenamed;

typedef struct cffi_tupled
    {
    uint8_t x0;
    } cffi_tupled;

typedef struct cffi_useasciistringpattern
    {
    uint8_t* ascii_string;
    } cffi_useasciistringpattern;

typedef struct cffi_vec
    {
    double x;
    double z;
    } cffi_vec;

typedef struct cffi_vec1
    {
    float x;
    float y;
    } cffi_vec1;

typedef struct cffi_vec2
    {
    double x;
    double z;
    } cffi_vec2;

typedef struct cffi_vec3f32
    {
    float x;
    float y;
    float z;
    } cffi_vec3f32;

typedef struct cffi_visibility1
    {
    uint8_t pblc;
    uint8_t prvt;
    } cffi_visibility1;

typedef struct cffi_visibility2
    {
    uint8_t pblc1;
    uint8_t pblc2;
    } cffi_visibility2;

typedef struct cffi_weird1u32
    {
    uint32_t x;
    } cffi_weird1u32;

typedef uint8_t (*cffi_fptr_fn_u8_rval_u8)(uint8_t x0);

typedef uint32_t (*cffi_fptr_fn_u32_rval_u32)(uint32_t x0);

typedef struct cffi_array
    {
    uint8_t data[16];
    } cffi_array;

typedef struct cffi_genericu32
    {
    uint32_t* x;
    } cffi_genericu32;

typedef struct cffi_genericu8
    {
    uint8_t* x;
    } cffi_genericu8;

typedef struct cffi_weird2u8
    {
    uint8_t t;
    uint8_t a[5];
    uint8_t* r;
    } cffi_weird2u8;

typedef cffi_tupled (*cffi_fptr_fn_Tupled_rval_Tupled)(cffi_tupled x0);

typedef bool (*cffi_fptr_fn_pmut_i64_rval_bool)(int64_t* x0);

typedef struct cffi_slicebool
    {
    uint8_t* data;
    uint64_t len;
    } cffi_slicebool;

typedef struct cffi_sliceu32
    {
    uint32_t* data;
    uint64_t len;
    } cffi_sliceu32;

typedef struct cffi_sliceu8
    {
    uint8_t* data;
    uint64_t len;
    } cffi_sliceu8;

typedef struct cffi_slicemutu8
    {
    uint8_t* data;
    uint64_t len;
    } cffi_slicemutu8;

typedef struct cffi_optioninner
    {
    cffi_inner t;
    uint8_t is_some;
    } cffi_optioninner;

typedef struct cffi_myapiv1
    {
    cffi_fptr_fn_pmut_i64_rval_bool ref_mut_option;
    cffi_fptr_fn_Tupled_rval_Tupled tupled;
    } cffi_myapiv1;

typedef struct cffi_slicevec3f32
    {
    cffi_vec3f32* data;
    uint64_t len;
    } cffi_slicevec3f32;

typedef uint8_t (*cffi_fptr_fn_Sliceu8_rval_u8)(cffi_sliceu8 x0);

typedef void (*cffi_fptr_fn_SliceMutu8)(cffi_slicemutu8 x0);

typedef cffi_vec3f32 (*cffi_fptr_fn_SliceVec3f32_rval_Vec3f32)(cffi_slicevec3f32 x0);


void primitive_void();
void primitive_void2();
bool primitive_bool(bool x);
uint8_t primitive_u8(uint8_t x);
uint16_t primitive_u16(uint16_t x);
uint32_t primitive_u32(uint32_t x);
uint64_t primitive_u64(uint64_t x);
int8_t primitive_i8(int8_t x);
int16_t primitive_i16(int16_t x);
int32_t primitive_i32(int32_t x);
int64_t primitive_i64(int64_t x);
int64_t many_args_5(int64_t x0, int64_t x1, int64_t x2, int64_t x3, int64_t x4);
int64_t many_args_10(int64_t x0, int64_t x1, int64_t x2, int64_t x3, int64_t x4, int64_t x5, int64_t x6, int64_t x7, int64_t x8, int64_t x9);
int64_t* ptr(int64_t* x);
int64_t* ptr_mut(int64_t* x);
int64_t** ptr_ptr(int64_t** x);
int64_t* ref_simple(int64_t* x);
int64_t* ref_mut_simple(int64_t* x);
bool ref_option(int64_t* x);
bool ref_mut_option(int64_t* x);
cffi_tupled tupled(cffi_tupled x);
cffi_ffierror complex_args_1(cffi_vec3f32 _a, cffi_empty* _b);
cffi_opaque* complex_args_2(cffi_someforeigntype _cmplx);
uint8_t callback(cffi_fptr_fn_u8_rval_u8 callback, uint8_t value);
uint32_t generic_1a(cffi_genericu32 x, cffi_phantomu8 _y);
uint8_t generic_1b(cffi_genericu8 x, cffi_phantomu8 _y);
uint8_t generic_1c(cffi_genericu8* _x, cffi_genericu8* y);
uint8_t generic_2(cffi_generic2u8* x);
uint8_t generic_3(cffi_generic3* x);
uint8_t generic_4(cffi_generic4* x);
uint8_t array_1(cffi_array x);
cffi_enumdocumented documented(cffi_structdocumented _x);
cffi_vec1 ambiguous_1(cffi_vec1 x);
cffi_vec2 ambiguous_2(cffi_vec2 x);
bool ambiguous_3(cffi_vec1 x, cffi_vec2 y);
cffi_vec namespaced_type(cffi_vec x);
cffi_ffierror panics();
cffi_enumrenamed renamed(cffi_structrenamed x);
void sleep(uint64_t millis);
bool weird_1(cffi_weird1u32 _x, cffi_weird2u8 _y);
void visibility(cffi_visibility1 _x, cffi_visibility2 _y);
uint32_t pattern_ascii_pointer_1(uint8_t* x);
uint32_t pattern_ascii_pointer_len(uint8_t* x, cffi_useasciistringpattern y);
uint32_t pattern_ffi_slice_1(cffi_sliceu32 ffi_slice);
cffi_vec3f32 pattern_ffi_slice_2(cffi_slicevec3f32 ffi_slice, int32_t i);
void pattern_ffi_slice_3(cffi_slicemutu8 slice, cffi_fptr_fn_SliceMutu8 callback);
uint8_t pattern_ffi_slice_delegate(cffi_fptr_fn_Sliceu8_rval_u8 callback);
cffi_vec3f32 pattern_ffi_slice_delegate_huge(cffi_fptr_fn_SliceVec3f32_rval_Vec3f32 callback);
cffi_optioninner pattern_ffi_option_1(cffi_optioninner ffi_slice);
cffi_inner pattern_ffi_option_2(cffi_optioninner ffi_slice);
uint8_t pattern_ffi_bool(uint8_t ffi_bool);
void pattern_my_api_init_v1(cffi_myapiv1* api);
uint64_t pattern_api_guard();
uint32_t pattern_callback_1(cffi_fptr_fn_u32_rval_u32 callback, uint32_t x);
cffi_ffierror simple_service_new_with(cffi_simpleservice** context, uint32_t some_value);
cffi_ffierror simple_service_simple_service_destroy(cffi_simpleservice** context);
cffi_ffierror simple_service_method_result(cffi_simpleservice* context, uint32_t _anon1);
uint32_t simple_service_method_value(cffi_simpleservice* context, uint32_t x);
void simple_service_method_void(cffi_simpleservice* context);
uint8_t simple_service_method_mut_self(cffi_simpleservice* context, cffi_sliceu8 slice);
void simple_service_method_mut_self_void(cffi_simpleservice* context, cffi_slicebool _slice);
uint8_t simple_service_method_mut_self_ref(cffi_simpleservice* context, uint8_t* x, uint8_t* _y);
uint8_t simple_service_method_mut_self_ref_slice(cffi_simpleservice* context, uint8_t* x, uint8_t* _y, cffi_sliceu8 _slice);
uint8_t simple_service_method_mut_self_ref_slice_limited(cffi_simpleservice* context, uint8_t* x, uint8_t* _y, cffi_sliceu8 _slice, cffi_sliceu8 _slice2);
cffi_ffierror simple_service_method_mut_self_ffi_error(cffi_simpleservice* context, cffi_slicemutu8 _slice);
"""


ffi = FFI()
ffi.cdef(api_definition)
_api = None


def init_api(dll):
    """Initializes this library, call with path to DLL."""
    global _api
    _api = ffi.dlopen(dll)





U8 = 255

F32_MIN_POSITIVE = 0.000000000000000000000000000000000000011754944

COMPUTED_I32 = -2147483647


class EnumDocumented:
    """Documented enum."""
    A = 0
    B = 1
    C = 2


class EnumRenamed:
    """"""
    X = 0


class Array(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_array[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_array[]", n)

    def ptr(self):
        return self._ctx

    @property
    def data(self):
        """"""
        return self._ctx[0].data

    @data.setter
    def data(self, value):
        self._ptr_data = value
        self._ctx[0].data = value

class Empty(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_empty[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_empty[]", n)

    def ptr(self):
        return self._ctx

class ExtraTypef32(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_extratypef32[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_extratypef32[]", n)

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

class Genericu32(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_genericu32[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_genericu32[]", n)

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

class Genericu8(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_genericu8[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_genericu8[]", n)

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

class Inner(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_inner[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_inner[]", n)

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

class MyAPIv1(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_myapiv1[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_myapiv1[]", n)

    def ptr(self):
        return self._ctx

    @property
    def ref_mut_option(self):
        """"""
        return self._ctx[0].ref_mut_option

    @ref_mut_option.setter
    def ref_mut_option(self, value):
        self._ptr_ref_mut_option = value
        self._ctx[0].ref_mut_option = value

    @property
    def tupled(self):
        """"""
        return self._ctx[0].tupled

    @tupled.setter
    def tupled(self, value):
        self._ptr_tupled = value
        self._ctx[0].tupled = value

class Phantomu8(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_phantomu8[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_phantomu8[]", n)

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

class SomeForeignType(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_someforeigntype[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_someforeigntype[]", n)

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

class StructDocumented(object):
    """Documented struct."""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_structdocumented[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_structdocumented[]", n)

    def ptr(self):
        return self._ctx

    @property
    def x(self):
        """Documented field."""
        return self._ctx[0].x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx[0].x = value

class StructRenamed(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_structrenamed[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_structrenamed[]", n)

    def ptr(self):
        return self._ctx

    @property
    def e(self):
        """"""
        return self._ctx[0].e

    @e.setter
    def e(self, value):
        self._ptr_e = value
        self._ctx[0].e = value

class Tupled(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_tupled[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_tupled[]", n)

    def ptr(self):
        return self._ctx

    @property
    def x0(self):
        """"""
        return self._ctx[0].x0

    @x0.setter
    def x0(self, value):
        self._ptr_x0 = value
        self._ctx[0].x0 = value

class UseAsciiStringPattern(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_useasciistringpattern[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_useasciistringpattern[]", n)

    def ptr(self):
        return self._ctx

    @property
    def ascii_string(self):
        """"""
        return self._ctx[0].ascii_string

    @ascii_string.setter
    def ascii_string(self, value):
        self._ptr_ascii_string = value
        self._ctx[0].ascii_string = value

class Vec(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_vec[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_vec[]", n)

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
    def z(self):
        """"""
        return self._ctx[0].z

    @z.setter
    def z(self, value):
        self._ptr_z = value
        self._ctx[0].z = value

class Vec1(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_vec1[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_vec1[]", n)

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

class Vec2(object):
    """"""
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
    def z(self):
        """"""
        return self._ctx[0].z

    @z.setter
    def z(self, value):
        self._ptr_z = value
        self._ctx[0].z = value

class Vec3f32(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_vec3f32[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_vec3f32[]", n)

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

class Visibility1(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_visibility1[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_visibility1[]", n)

    def ptr(self):
        return self._ctx

    @property
    def pblc(self):
        """"""
        return self._ctx[0].pblc

    @pblc.setter
    def pblc(self, value):
        self._ptr_pblc = value
        self._ctx[0].pblc = value

    @property
    def prvt(self):
        """"""
        return self._ctx[0].prvt

    @prvt.setter
    def prvt(self, value):
        self._ptr_prvt = value
        self._ctx[0].prvt = value

class Visibility2(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_visibility2[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_visibility2[]", n)

    def ptr(self):
        return self._ctx

    @property
    def pblc1(self):
        """"""
        return self._ctx[0].pblc1

    @pblc1.setter
    def pblc1(self, value):
        self._ptr_pblc1 = value
        self._ctx[0].pblc1 = value

    @property
    def pblc2(self):
        """"""
        return self._ctx[0].pblc2

    @pblc2.setter
    def pblc2(self, value):
        self._ptr_pblc2 = value
        self._ctx[0].pblc2 = value

class Weird1u32(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_weird1u32[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_weird1u32[]", n)

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

class Weird2u8(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("cffi_weird2u8[]", 1)

    def array(n):
        global _api, ffi
        return ffi.new("cffi_weird2u8[]", n)

    def ptr(self):
        return self._ctx

    @property
    def t(self):
        """"""
        return self._ctx[0].t

    @t.setter
    def t(self, value):
        self._ptr_t = value
        self._ctx[0].t = value

    @property
    def a(self):
        """"""
        return self._ctx[0].a

    @a.setter
    def a(self, value):
        self._ptr_a = value
        self._ctx[0].a = value

    @property
    def r(self):
        """"""
        return self._ctx[0].r

    @r.setter
    def r(self, value):
        self._ptr_r = value
        self._ctx[0].r = value

class FFIError:
    """"""
    Ok = 0
    Null = 100
    Panic = 200
    Fail = 300




class callbacks:
    """Helpers to define `@ffi.callback`-style callbacks."""
    fn_Tupled_rval_Tupled = "cffi_tupled(cffi_tupled)"
    fn_pmut_i64_rval_bool = "bool(int64_t*)"
    fn_u8_rval_u8 = "uint8_t(uint8_t)"
    fn_Sliceu8_rval_u8 = "uint8_t(cffi_sliceu8)"
    fn_SliceVec3f32_rval_Vec3f32 = "cffi_vec3f32(cffi_slicevec3f32)"
    fn_SliceMutu8 = "void(cffi_slicemutu8)"
    fn_u32_rval_u32 = "uint32_t(uint32_t)"




class raw:
    """Raw access to all exported functions."""
    def primitive_void():
        """"""
        global _api
        return _api.primitive_void()

    def primitive_void2():
        """"""
        global _api
        return _api.primitive_void2()

    def primitive_bool(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.primitive_bool(x)

    def primitive_u8(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.primitive_u8(x)

    def primitive_u16(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.primitive_u16(x)

    def primitive_u32(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.primitive_u32(x)

    def primitive_u64(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.primitive_u64(x)

    def primitive_i8(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.primitive_i8(x)

    def primitive_i16(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.primitive_i16(x)

    def primitive_i32(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.primitive_i32(x)

    def primitive_i64(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.primitive_i64(x)

    def many_args_5(x0, x1, x2, x3, x4):
        """"""
        global _api
        if hasattr(x0, "_ctx"):
            x0 = x0._ctx[0]
        if hasattr(x1, "_ctx"):
            x1 = x1._ctx[0]
        if hasattr(x2, "_ctx"):
            x2 = x2._ctx[0]
        if hasattr(x3, "_ctx"):
            x3 = x3._ctx[0]
        if hasattr(x4, "_ctx"):
            x4 = x4._ctx[0]
        return _api.many_args_5(x0, x1, x2, x3, x4)

    def many_args_10(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9):
        """"""
        global _api
        if hasattr(x0, "_ctx"):
            x0 = x0._ctx[0]
        if hasattr(x1, "_ctx"):
            x1 = x1._ctx[0]
        if hasattr(x2, "_ctx"):
            x2 = x2._ctx[0]
        if hasattr(x3, "_ctx"):
            x3 = x3._ctx[0]
        if hasattr(x4, "_ctx"):
            x4 = x4._ctx[0]
        if hasattr(x5, "_ctx"):
            x5 = x5._ctx[0]
        if hasattr(x6, "_ctx"):
            x6 = x6._ctx[0]
        if hasattr(x7, "_ctx"):
            x7 = x7._ctx[0]
        if hasattr(x8, "_ctx"):
            x8 = x8._ctx[0]
        if hasattr(x9, "_ctx"):
            x9 = x9._ctx[0]
        return _api.many_args_10(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)

    def ptr(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.ptr(x)

    def ptr_mut(x):
        """# Safety

Parameter x must point to valid data."""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.ptr_mut(x)

    def ptr_ptr(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.ptr_ptr(x)

    def ref_simple(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.ref_simple(x)

    def ref_mut_simple(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.ref_mut_simple(x)

    def ref_option(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.ref_option(x)

    def ref_mut_option(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.ref_mut_option(x)

    def tupled(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.tupled(x)

    def complex_args_1(_a, _b):
        """"""
        global _api
        if hasattr(_a, "_ctx"):
            _a = _a._ctx[0]
        if hasattr(_b, "_ctx"):
            _b = _b._ctx[0]
        return _api.complex_args_1(_a, _b)

    def complex_args_2(_cmplx):
        """"""
        global _api
        if hasattr(_cmplx, "_ctx"):
            _cmplx = _cmplx._ctx[0]
        return _api.complex_args_2(_cmplx)

    def callback(callback, value):
        """"""
        global _api
        if hasattr(callback, "_ctx"):
            callback = callback._ctx[0]
        if hasattr(value, "_ctx"):
            value = value._ctx[0]
        return _api.callback(callback, value)

    def generic_1a(x, _y):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        if hasattr(_y, "_ctx"):
            _y = _y._ctx[0]
        return _api.generic_1a(x, _y)

    def generic_1b(x, _y):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        if hasattr(_y, "_ctx"):
            _y = _y._ctx[0]
        return _api.generic_1b(x, _y)

    def generic_1c(_x, y):
        """"""
        global _api
        if hasattr(_x, "_ctx"):
            _x = _x._ctx[0]
        if hasattr(y, "_ctx"):
            y = y._ctx[0]
        return _api.generic_1c(_x, y)

    def generic_2(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.generic_2(x)

    def generic_3(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.generic_3(x)

    def generic_4(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.generic_4(x)

    def array_1(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.array_1(x)

    def documented(_x):
        """This function has documentation."""
        global _api
        if hasattr(_x, "_ctx"):
            _x = _x._ctx[0]
        return _api.documented(_x)

    def ambiguous_1(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.ambiguous_1(x)

    def ambiguous_2(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.ambiguous_2(x)

    def ambiguous_3(x, y):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        if hasattr(y, "_ctx"):
            y = y._ctx[0]
        return _api.ambiguous_3(x, y)

    def namespaced_type(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.namespaced_type(x)

    def panics():
        """"""
        global _api
        return _api.panics()

    def renamed(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.renamed(x)

    def sleep(millis):
        """"""
        global _api
        if hasattr(millis, "_ctx"):
            millis = millis._ctx[0]
        return _api.sleep(millis)

    def weird_1(_x, _y):
        """"""
        global _api
        if hasattr(_x, "_ctx"):
            _x = _x._ctx[0]
        if hasattr(_y, "_ctx"):
            _y = _y._ctx[0]
        return _api.weird_1(_x, _y)

    def visibility(_x, _y):
        """"""
        global _api
        if hasattr(_x, "_ctx"):
            _x = _x._ctx[0]
        if hasattr(_y, "_ctx"):
            _y = _y._ctx[0]
        return _api.visibility(_x, _y)

    def pattern_ascii_pointer_1(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.pattern_ascii_pointer_1(x)

    def pattern_ascii_pointer_len(x, y):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        if hasattr(y, "_ctx"):
            y = y._ctx[0]
        return _api.pattern_ascii_pointer_len(x, y)

    def pattern_ffi_slice_1(ffi_slice):
        """"""
        global _api
        if hasattr(ffi_slice, "_ctx"):
            ffi_slice = ffi_slice._ctx[0]
        return _api.pattern_ffi_slice_1(ffi_slice)

    def pattern_ffi_slice_2(ffi_slice, i):
        """"""
        global _api
        if hasattr(ffi_slice, "_ctx"):
            ffi_slice = ffi_slice._ctx[0]
        if hasattr(i, "_ctx"):
            i = i._ctx[0]
        return _api.pattern_ffi_slice_2(ffi_slice, i)

    def pattern_ffi_slice_3(slice, callback):
        """"""
        global _api
        if hasattr(slice, "_ctx"):
            slice = slice._ctx[0]
        if hasattr(callback, "_ctx"):
            callback = callback._ctx[0]
        return _api.pattern_ffi_slice_3(slice, callback)

    def pattern_ffi_slice_delegate(callback):
        """"""
        global _api
        if hasattr(callback, "_ctx"):
            callback = callback._ctx[0]
        return _api.pattern_ffi_slice_delegate(callback)

    def pattern_ffi_slice_delegate_huge(callback):
        """"""
        global _api
        if hasattr(callback, "_ctx"):
            callback = callback._ctx[0]
        return _api.pattern_ffi_slice_delegate_huge(callback)

    def pattern_ffi_option_1(ffi_slice):
        """"""
        global _api
        if hasattr(ffi_slice, "_ctx"):
            ffi_slice = ffi_slice._ctx[0]
        return _api.pattern_ffi_option_1(ffi_slice)

    def pattern_ffi_option_2(ffi_slice):
        """"""
        global _api
        if hasattr(ffi_slice, "_ctx"):
            ffi_slice = ffi_slice._ctx[0]
        return _api.pattern_ffi_option_2(ffi_slice)

    def pattern_ffi_bool(ffi_bool):
        """"""
        global _api
        if hasattr(ffi_bool, "_ctx"):
            ffi_bool = ffi_bool._ctx[0]
        return _api.pattern_ffi_bool(ffi_bool)

    def pattern_my_api_init_v1(api):
        """"""
        global _api
        if hasattr(api, "_ctx"):
            api = api._ctx[0]
        return _api.pattern_my_api_init_v1(api)

    def pattern_api_guard():
        """"""
        global _api
        return _api.pattern_api_guard()

    def pattern_callback_1(callback, x):
        """"""
        global _api
        if hasattr(callback, "_ctx"):
            callback = callback._ctx[0]
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.pattern_callback_1(callback, x)

    def simple_service_new_with(context, some_value):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(some_value, "_ctx"):
            some_value = some_value._ctx[0]
        return _api.simple_service_new_with(context, some_value)

    def simple_service_simple_service_destroy(context):
        """Destroys the given instance.

# Safety

The passed parameter MUST have been created with the corresponding init function;
passing any other value results in undefined behavior."""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        return _api.simple_service_simple_service_destroy(context)

    def simple_service_method_result(context, _anon1):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(_anon1, "_ctx"):
            _anon1 = _anon1._ctx[0]
        return _api.simple_service_method_result(context, _anon1)

    def simple_service_method_value(context, x):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.simple_service_method_value(context, x)

    def simple_service_method_void(context):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        return _api.simple_service_method_void(context)

    def simple_service_method_mut_self(context, slice):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(slice, "_ctx"):
            slice = slice._ctx[0]
        return _api.simple_service_method_mut_self(context, slice)

    def simple_service_method_mut_self_void(context, _slice):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(_slice, "_ctx"):
            _slice = _slice._ctx[0]
        return _api.simple_service_method_mut_self_void(context, _slice)

    def simple_service_method_mut_self_ref(context, x, _y):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        if hasattr(_y, "_ctx"):
            _y = _y._ctx[0]
        return _api.simple_service_method_mut_self_ref(context, x, _y)

    def simple_service_method_mut_self_ref_slice(context, x, _y, _slice):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        if hasattr(_y, "_ctx"):
            _y = _y._ctx[0]
        if hasattr(_slice, "_ctx"):
            _slice = _slice._ctx[0]
        return _api.simple_service_method_mut_self_ref_slice(context, x, _y, _slice)

    def simple_service_method_mut_self_ref_slice_limited(context, x, _y, _slice, _slice2):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        if hasattr(_y, "_ctx"):
            _y = _y._ctx[0]
        if hasattr(_slice, "_ctx"):
            _slice = _slice._ctx[0]
        if hasattr(_slice2, "_ctx"):
            _slice2 = _slice2._ctx[0]
        return _api.simple_service_method_mut_self_ref_slice_limited(context, x, _y, _slice, _slice2)

    def simple_service_method_mut_self_ffi_error(context, _slice):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        if hasattr(_slice, "_ctx"):
            _slice = _slice._ctx[0]
        return _api.simple_service_method_mut_self_ffi_error(context, _slice)





class SimpleService(object):
    def __init__(self, some_value):
        """"""
        global _api, ffi
        if hasattr(some_value, "_ctx"):
            some_value = some_value._ctx
        self.ctx = ffi.new("cffi_simpleservice**")
        rval = raw.simple_service_new_with(self.ctx, some_value)
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def __del__(self):
        global _api, ffi
        rval = raw.simple_service_simple_service_destroy(self.ctx, )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def method_result(self, _anon1):
        """"""
        global raw
        rval = raw.simple_service_method_result(self.ctx[0], _anon1)
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def method_value(self, x):
        """"""
        global raw
        return _api.simple_service_method_value(self.ctx[0], x)

    def method_void(self, ):
        """"""
        global raw
        return _api.simple_service_method_void(self.ctx[0], )

    def method_mut_self(self, slice):
        """"""
        global raw
        return _api.simple_service_method_mut_self(self.ctx[0], slice)

    def method_mut_self_void(self, _slice):
        """"""
        global raw
        return _api.simple_service_method_mut_self_void(self.ctx[0], _slice)

    def method_mut_self_ref(self, x, _y):
        """"""
        global raw
        return _api.simple_service_method_mut_self_ref(self.ctx[0], x, _y)

    def method_mut_self_ref_slice(self, x, _y, _slice):
        """"""
        global raw
        return _api.simple_service_method_mut_self_ref_slice(self.ctx[0], x, _y, _slice)

    def method_mut_self_ref_slice_limited(self, x, _y, _slice, _slice2):
        """"""
        global raw
        return _api.simple_service_method_mut_self_ref_slice_limited(self.ctx[0], x, _y, _slice, _slice2)

    def method_mut_self_ffi_error(self, _slice):
        """"""
        global raw
        rval = raw.simple_service_method_mut_self_ffi_error(self.ctx[0], _slice)
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")





def ascii_string(x):
    """Must be called with a b"my_string"."""
    global ffi
    return ffi.new("char[]", x)



