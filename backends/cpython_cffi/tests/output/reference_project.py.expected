# Print usable error message if dependency is not installed.
try:
    from cffi import FFI
except ImportError:
    print("Please install package `cffi`, probably via `pip install cffi`.")
    print("")

api_definition = """
const uint8_t CFFI_U8 = 255;
const float CFFI_F32_MIN_POSITIVE = 0.000000000000000000000000000000000000011754944;
const int32_t CFFI_COMPUTED_I32 = -2147483647;

typedef enum cffi_enumdocumented
    {
    CFFI_ENUMDOCUMENTED_A = 0,
    CFFI_ENUMDOCUMENTED_B = 1,
    CFFI_ENUMDOCUMENTED_C = 2,
    } cffi_enumdocumented;

typedef enum cffi_enumrenamed
    {
    CFFI_ENUMRENAMED_X = 0,
    } cffi_enumrenamed;

typedef struct cffi_generic2u8 cffi_generic2u8;
typedef struct cffi_generic3 cffi_generic3;
typedef struct cffi_generic4 cffi_generic4;
typedef struct cffi_opaque cffi_opaque;
typedef struct cffi_simpleservice cffi_simpleservice;
typedef struct cffi_empty cffi_empty;

typedef enum cffi_ffierror
    {
    CFFI_FFIERROR_OK = 0,
    CFFI_FFIERROR_NULL = 100,
    CFFI_FFIERROR_PANIC = 200,
    CFFI_FFIERROR_FAIL = 300,
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
void pattern_ffi_slice_4(cffi_sliceu8 _slice, cffi_slicemutu8 _slice2);
void pattern_ffi_slice_5(cffi_sliceu8* slice, cffi_slicemutu8* slice2);
uint8_t pattern_ffi_slice_delegate(cffi_fptr_fn_Sliceu8_rval_u8 callback);
cffi_vec3f32 pattern_ffi_slice_delegate_huge(cffi_fptr_fn_SliceVec3f32_rval_Vec3f32 callback);
cffi_optioninner pattern_ffi_option_1(cffi_optioninner ffi_slice);
cffi_inner pattern_ffi_option_2(cffi_optioninner ffi_slice);
uint8_t pattern_ffi_bool(uint8_t ffi_bool);
void pattern_my_api_init_v1(cffi_myapiv1* api);
uint64_t pattern_api_guard();
uint32_t pattern_callback_1(cffi_fptr_fn_u32_rval_u32 callback, uint32_t x);
cffi_ffierror simple_service_destroy(cffi_simpleservice** context);
cffi_ffierror simple_service_new_with(cffi_simpleservice** context, uint32_t some_value);
cffi_ffierror simple_service_new_without(cffi_simpleservice** context);
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




class EnumDocumented:
    """ Documented enum."""
    A = 0
    B = 1
    C = 2


class EnumRenamed:
    """"""
    X = 0


class Array(CHeapAllocated):
    """"""
    def __init__(self, data=None):
        self._ctx = ffi.new("cffi_array[]", 1)
        if data is not None:
            self.data = data

    @staticmethod
    def c_array(n):
        return CArray("cffi_array", n)

    @property
    def data(self):
        """"""
        return self._ctx[0].data

    @data.setter
    def data(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].data = value


class Empty(CHeapAllocated):
    """"""
    def __init__(self, ):
        self._ctx = ffi.new("cffi_empty[]", 1)

    @staticmethod
    def c_array(n):
        return CArray("cffi_empty", n)


class ExtraTypef32(CHeapAllocated):
    """"""
    def __init__(self, x=None):
        self._ctx = ffi.new("cffi_extratypef32[]", 1)
        if x is not None:
            self.x = x

    @staticmethod
    def c_array(n):
        return CArray("cffi_extratypef32", n)

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


class Genericu32(CHeapAllocated):
    """"""
    def __init__(self, x=None):
        self._ctx = ffi.new("cffi_genericu32[]", 1)
        if x is not None:
            self.x = x

    @staticmethod
    def c_array(n):
        return CArray("cffi_genericu32", n)

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


class Genericu8(CHeapAllocated):
    """"""
    def __init__(self, x=None):
        self._ctx = ffi.new("cffi_genericu8[]", 1)
        if x is not None:
            self.x = x

    @staticmethod
    def c_array(n):
        return CArray("cffi_genericu8", n)

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


class Inner(CHeapAllocated):
    """"""
    def __init__(self, x=None):
        self._ctx = ffi.new("cffi_inner[]", 1)
        if x is not None:
            self.x = x

    @staticmethod
    def c_array(n):
        return CArray("cffi_inner", n)

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


class MyAPIv1(CHeapAllocated):
    """"""
    def __init__(self, ref_mut_option=None, tupled=None):
        self._ctx = ffi.new("cffi_myapiv1[]", 1)
        if ref_mut_option is not None:
            self.ref_mut_option = ref_mut_option
        if tupled is not None:
            self.tupled = tupled

    @staticmethod
    def c_array(n):
        return CArray("cffi_myapiv1", n)

    @property
    def ref_mut_option(self):
        """"""
        return self._ctx[0].ref_mut_option

    @ref_mut_option.setter
    def ref_mut_option(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].ref_mut_option = value

    @property
    def tupled(self):
        """"""
        return self._ctx[0].tupled

    @tupled.setter
    def tupled(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].tupled = value


class Phantomu8(CHeapAllocated):
    """"""
    def __init__(self, x=None):
        self._ctx = ffi.new("cffi_phantomu8[]", 1)
        if x is not None:
            self.x = x

    @staticmethod
    def c_array(n):
        return CArray("cffi_phantomu8", n)

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


class SomeForeignType(CHeapAllocated):
    """"""
    def __init__(self, x=None):
        self._ctx = ffi.new("cffi_someforeigntype[]", 1)
        if x is not None:
            self.x = x

    @staticmethod
    def c_array(n):
        return CArray("cffi_someforeigntype", n)

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


class StructDocumented(CHeapAllocated):
    """ Documented struct."""
    def __init__(self, x=None):
        self._ctx = ffi.new("cffi_structdocumented[]", 1)
        if x is not None:
            self.x = x

    @staticmethod
    def c_array(n):
        return CArray("cffi_structdocumented", n)

    @property
    def x(self):
        """ Documented field."""
        return self._ctx[0].x

    @x.setter
    def x(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].x = value


class StructRenamed(CHeapAllocated):
    """"""
    def __init__(self, e=None):
        self._ctx = ffi.new("cffi_structrenamed[]", 1)
        if e is not None:
            self.e = e

    @staticmethod
    def c_array(n):
        return CArray("cffi_structrenamed", n)

    @property
    def e(self):
        """"""
        return self._ctx[0].e

    @e.setter
    def e(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].e = value


class Tupled(CHeapAllocated):
    """"""
    def __init__(self, x0=None):
        self._ctx = ffi.new("cffi_tupled[]", 1)
        if x0 is not None:
            self.x0 = x0

    @staticmethod
    def c_array(n):
        return CArray("cffi_tupled", n)

    @property
    def x0(self):
        """"""
        return self._ctx[0].x0

    @x0.setter
    def x0(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].x0 = value


class UseAsciiStringPattern(CHeapAllocated):
    """"""
    def __init__(self, ascii_string=None):
        self._ctx = ffi.new("cffi_useasciistringpattern[]", 1)
        if ascii_string is not None:
            self.ascii_string = ascii_string

    @staticmethod
    def c_array(n):
        return CArray("cffi_useasciistringpattern", n)

    @property
    def ascii_string(self):
        """"""
        return self._ctx[0].ascii_string

    @ascii_string.setter
    def ascii_string(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].ascii_string = value


class Vec(CHeapAllocated):
    """"""
    def __init__(self, x=None, z=None):
        self._ctx = ffi.new("cffi_vec[]", 1)
        if x is not None:
            self.x = x
        if z is not None:
            self.z = z

    @staticmethod
    def c_array(n):
        return CArray("cffi_vec", n)

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
    def z(self):
        """"""
        return self._ctx[0].z

    @z.setter
    def z(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].z = value


class Vec1(CHeapAllocated):
    """"""
    def __init__(self, x=None, y=None):
        self._ctx = ffi.new("cffi_vec1[]", 1)
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y

    @staticmethod
    def c_array(n):
        return CArray("cffi_vec1", n)

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


class Vec2(CHeapAllocated):
    """"""
    def __init__(self, x=None, z=None):
        self._ctx = ffi.new("cffi_vec2[]", 1)
        if x is not None:
            self.x = x
        if z is not None:
            self.z = z

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
    def z(self):
        """"""
        return self._ctx[0].z

    @z.setter
    def z(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].z = value


class Vec3f32(CHeapAllocated):
    """"""
    def __init__(self, x=None, y=None, z=None):
        self._ctx = ffi.new("cffi_vec3f32[]", 1)
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y
        if z is not None:
            self.z = z

    @staticmethod
    def c_array(n):
        return CArray("cffi_vec3f32", n)

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

    @property
    def z(self):
        """"""
        return self._ctx[0].z

    @z.setter
    def z(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].z = value


class Visibility1(CHeapAllocated):
    """"""
    def __init__(self, pblc=None, prvt=None):
        self._ctx = ffi.new("cffi_visibility1[]", 1)
        if pblc is not None:
            self.pblc = pblc
        if prvt is not None:
            self.prvt = prvt

    @staticmethod
    def c_array(n):
        return CArray("cffi_visibility1", n)

    @property
    def pblc(self):
        """"""
        return self._ctx[0].pblc

    @pblc.setter
    def pblc(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].pblc = value

    @property
    def prvt(self):
        """"""
        return self._ctx[0].prvt

    @prvt.setter
    def prvt(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].prvt = value


class Visibility2(CHeapAllocated):
    """"""
    def __init__(self, pblc1=None, pblc2=None):
        self._ctx = ffi.new("cffi_visibility2[]", 1)
        if pblc1 is not None:
            self.pblc1 = pblc1
        if pblc2 is not None:
            self.pblc2 = pblc2

    @staticmethod
    def c_array(n):
        return CArray("cffi_visibility2", n)

    @property
    def pblc1(self):
        """"""
        return self._ctx[0].pblc1

    @pblc1.setter
    def pblc1(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].pblc1 = value

    @property
    def pblc2(self):
        """"""
        return self._ctx[0].pblc2

    @pblc2.setter
    def pblc2(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].pblc2 = value


class Weird1u32(CHeapAllocated):
    """"""
    def __init__(self, x=None):
        self._ctx = ffi.new("cffi_weird1u32[]", 1)
        if x is not None:
            self.x = x

    @staticmethod
    def c_array(n):
        return CArray("cffi_weird1u32", n)

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


class Weird2u8(CHeapAllocated):
    """"""
    def __init__(self, t=None, a=None, r=None):
        self._ctx = ffi.new("cffi_weird2u8[]", 1)
        if t is not None:
            self.t = t
        if a is not None:
            self.a = a
        if r is not None:
            self.r = r

    @staticmethod
    def c_array(n):
        return CArray("cffi_weird2u8", n)

    @property
    def t(self):
        """"""
        return self._ctx[0].t

    @t.setter
    def t(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].t = value

    @property
    def a(self):
        """"""
        return self._ctx[0].a

    @a.setter
    def a(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
        self._ctx[0].a = value

    @property
    def r(self):
        """"""
        return self._ctx[0].r

    @r.setter
    def r(self, value):
        if hasattr(value, "_ctx"):
            if hasattr(value, "_c_array"):
                value = value.c_ptr()
            else:
                value = value.c_value()
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


class api:
    """Raw access to all exported functions."""
    @staticmethod
    def primitive_void():
        """"""

        return _api.primitive_void()

    @staticmethod
    def primitive_void2():
        """"""

        return _api.primitive_void2()

    @staticmethod
    def primitive_bool(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.primitive_bool(x)

    @staticmethod
    def primitive_u8(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.primitive_u8(x)

    @staticmethod
    def primitive_u16(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.primitive_u16(x)

    @staticmethod
    def primitive_u32(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.primitive_u32(x)

    @staticmethod
    def primitive_u64(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.primitive_u64(x)

    @staticmethod
    def primitive_i8(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.primitive_i8(x)

    @staticmethod
    def primitive_i16(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.primitive_i16(x)

    @staticmethod
    def primitive_i32(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.primitive_i32(x)

    @staticmethod
    def primitive_i64(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.primitive_i64(x)

    @staticmethod
    def many_args_5(x0, x1, x2, x3, x4):
        """"""
        if hasattr(x0, "_ctx"):
            x0 = x0.c_value()
        if hasattr(x1, "_ctx"):
            x1 = x1.c_value()
        if hasattr(x2, "_ctx"):
            x2 = x2.c_value()
        if hasattr(x3, "_ctx"):
            x3 = x3.c_value()
        if hasattr(x4, "_ctx"):
            x4 = x4.c_value()

        return _api.many_args_5(x0, x1, x2, x3, x4)

    @staticmethod
    def many_args_10(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9):
        """"""
        if hasattr(x0, "_ctx"):
            x0 = x0.c_value()
        if hasattr(x1, "_ctx"):
            x1 = x1.c_value()
        if hasattr(x2, "_ctx"):
            x2 = x2.c_value()
        if hasattr(x3, "_ctx"):
            x3 = x3.c_value()
        if hasattr(x4, "_ctx"):
            x4 = x4.c_value()
        if hasattr(x5, "_ctx"):
            x5 = x5.c_value()
        if hasattr(x6, "_ctx"):
            x6 = x6.c_value()
        if hasattr(x7, "_ctx"):
            x7 = x7.c_value()
        if hasattr(x8, "_ctx"):
            x8 = x8.c_value()
        if hasattr(x9, "_ctx"):
            x9 = x9.c_value()

        return _api.many_args_10(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)

    @staticmethod
    def ptr(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.ptr(x)

    @staticmethod
    def ptr_mut(x):
        """ # Safety

 Parameter x must point to valid data."""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.ptr_mut(x)

    @staticmethod
    def ptr_ptr(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.ptr_ptr(x)

    @staticmethod
    def ref_simple(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.ref_simple(x)

    @staticmethod
    def ref_mut_simple(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.ref_mut_simple(x)

    @staticmethod
    def ref_option(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.ref_option(x)

    @staticmethod
    def ref_mut_option(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.ref_mut_option(x)

    @staticmethod
    def tupled(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.tupled(x)

    @staticmethod
    def complex_args_1(_a, _b):
        """"""
        if hasattr(_a, "_ctx"):
            _a = _a.c_value()
        if hasattr(_b, "_ctx"):
            _b = _b.c_ptr()

        _rval = _api.complex_args_1(_a, _b)
        if _rval == FFIError.Ok:
            return _rval
        else:
            raise Exception(f"Function returned error {_rval}")

    @staticmethod
    def complex_args_2(_cmplx):
        """"""
        if hasattr(_cmplx, "_ctx"):
            _cmplx = _cmplx.c_value()

        return _api.complex_args_2(_cmplx)

    @staticmethod
    def callback(callback, value):
        """"""
        _callback = callback

        @ffi.callback(callbacks.fn_u8_rval_u8)
        def _callback_callback(x0):
            return _callback(x0)

        callback = _callback_callback
        if hasattr(value, "_ctx"):
            value = value.c_value()

        return _api.callback(callback, value)

    @staticmethod
    def generic_1a(x, _y):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()
        if hasattr(_y, "_ctx"):
            _y = _y.c_value()

        return _api.generic_1a(x, _y)

    @staticmethod
    def generic_1b(x, _y):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()
        if hasattr(_y, "_ctx"):
            _y = _y.c_value()

        return _api.generic_1b(x, _y)

    @staticmethod
    def generic_1c(_x, y):
        """"""
        if hasattr(_x, "_ctx"):
            _x = _x.c_ptr()
        if hasattr(y, "_ctx"):
            y = y.c_ptr()

        return _api.generic_1c(_x, y)

    @staticmethod
    def generic_2(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.generic_2(x)

    @staticmethod
    def generic_3(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.generic_3(x)

    @staticmethod
    def generic_4(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_ptr()

        return _api.generic_4(x)

    @staticmethod
    def array_1(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.array_1(x)

    @staticmethod
    def documented(_x):
        """ This function has documentation."""
        if hasattr(_x, "_ctx"):
            _x = _x.c_value()

        return _api.documented(_x)

    @staticmethod
    def ambiguous_1(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.ambiguous_1(x)

    @staticmethod
    def ambiguous_2(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.ambiguous_2(x)

    @staticmethod
    def ambiguous_3(x, y):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()
        if hasattr(y, "_ctx"):
            y = y.c_value()

        return _api.ambiguous_3(x, y)

    @staticmethod
    def namespaced_type(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.namespaced_type(x)

    @staticmethod
    def panics():
        """"""

        _rval = _api.panics()
        if _rval == FFIError.Ok:
            return _rval
        else:
            raise Exception(f"Function returned error {_rval}")

    @staticmethod
    def renamed(x):
        """"""
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.renamed(x)

    @staticmethod
    def sleep(millis):
        """"""
        if hasattr(millis, "_ctx"):
            millis = millis.c_value()

        return _api.sleep(millis)

    @staticmethod
    def weird_1(_x, _y):
        """"""
        if hasattr(_x, "_ctx"):
            _x = _x.c_value()
        if hasattr(_y, "_ctx"):
            _y = _y.c_value()

        return _api.weird_1(_x, _y)

    @staticmethod
    def visibility(_x, _y):
        """"""
        if hasattr(_x, "_ctx"):
            _x = _x.c_value()
        if hasattr(_y, "_ctx"):
            _y = _y.c_value()

        return _api.visibility(_x, _y)

    @staticmethod
    def pattern_ascii_pointer_1(x):
        """"""
        if isinstance(x, bytes):
            x = ascii_string(x)

        return _api.pattern_ascii_pointer_1(x)

    @staticmethod
    def pattern_ascii_pointer_len(x, y):
        """"""
        if isinstance(x, bytes):
            x = ascii_string(x)
        if hasattr(y, "_ctx"):
            y = y.c_value()

        return _api.pattern_ascii_pointer_len(x, y)

    @staticmethod
    def pattern_ffi_slice_1(ffi_slice):
        """"""
        _ffi_slice = ffi.new("cffi_sliceu32[]", 1)
        _ffi_slice[0].data = ffi_slice.c_ptr()
        _ffi_slice[0].len = len(ffi_slice)
        ffi_slice = _ffi_slice[0]

        return _api.pattern_ffi_slice_1(ffi_slice)

    @staticmethod
    def pattern_ffi_slice_2(ffi_slice, i):
        """"""
        _ffi_slice = ffi.new("cffi_slicevec3f32[]", 1)
        _ffi_slice[0].data = ffi_slice.c_ptr()
        _ffi_slice[0].len = len(ffi_slice)
        ffi_slice = _ffi_slice[0]
        if hasattr(i, "_ctx"):
            i = i.c_value()

        return _api.pattern_ffi_slice_2(ffi_slice, i)

    @staticmethod
    def pattern_ffi_slice_3(slice, callback):
        """"""
        _slice = ffi.new("cffi_slicemutu8[]", 1)
        _slice[0].data = slice.c_ptr()
        _slice[0].len = len(slice)
        slice = _slice[0]
        _callback = callback

        @ffi.callback(callbacks.fn_SliceMutu8)
        def _callback_callback(slice):
            slice = CSlice(slice)
            return _callback(slice)

        callback = _callback_callback

        return _api.pattern_ffi_slice_3(slice, callback)

    @staticmethod
    def pattern_ffi_slice_4(_slice, _slice2):
        """"""
        __slice = ffi.new("cffi_sliceu8[]", 1)
        __slice[0].data = _slice.c_ptr()
        __slice[0].len = len(_slice)
        _slice = __slice[0]
        __slice2 = ffi.new("cffi_slicemutu8[]", 1)
        __slice2[0].data = _slice2.c_ptr()
        __slice2[0].len = len(_slice2)
        _slice2 = __slice2[0]

        return _api.pattern_ffi_slice_4(_slice, _slice2)

    @staticmethod
    def pattern_ffi_slice_5(slice, slice2):
        """"""
        if hasattr(slice, "_ctx"):
            slice = slice.c_ptr()
        if hasattr(slice2, "_ctx"):
            slice2 = slice2.c_ptr()

        return _api.pattern_ffi_slice_5(slice, slice2)

    @staticmethod
    def pattern_ffi_slice_delegate(callback):
        """"""
        _callback = callback

        @ffi.callback(callbacks.fn_Sliceu8_rval_u8)
        def _callback_callback(slice):
            slice = CSlice(slice)
            return _callback(slice)

        callback = _callback_callback

        return _api.pattern_ffi_slice_delegate(callback)

    @staticmethod
    def pattern_ffi_slice_delegate_huge(callback):
        """"""
        _callback = callback

        @ffi.callback(callbacks.fn_SliceVec3f32_rval_Vec3f32)
        def _callback_callback(slice):
            slice = CSlice(slice)
            return _callback(slice)

        callback = _callback_callback

        return _api.pattern_ffi_slice_delegate_huge(callback)

    @staticmethod
    def pattern_ffi_option_1(ffi_slice):
        """"""
        if hasattr(ffi_slice, "_ctx"):
            ffi_slice = ffi_slice.c_value()

        return _api.pattern_ffi_option_1(ffi_slice)

    @staticmethod
    def pattern_ffi_option_2(ffi_slice):
        """"""
        if hasattr(ffi_slice, "_ctx"):
            ffi_slice = ffi_slice.c_value()

        return _api.pattern_ffi_option_2(ffi_slice)

    @staticmethod
    def pattern_ffi_bool(ffi_bool):
        """"""
        if hasattr(ffi_bool, "_ctx"):
            ffi_bool = ffi_bool.c_value()

        return _api.pattern_ffi_bool(ffi_bool)

    @staticmethod
    def pattern_my_api_init_v1(api):
        """"""
        if hasattr(api, "_ctx"):
            api = api.c_ptr()

        return _api.pattern_my_api_init_v1(api)

    @staticmethod
    def pattern_api_guard():
        """"""

        return _api.pattern_api_guard()

    @staticmethod
    def pattern_callback_1(callback, x):
        """"""
        _callback = callback

        @ffi.callback(callbacks.fn_u32_rval_u32)
        def _callback_callback(x):
            return _callback(x)

        callback = _callback_callback
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.pattern_callback_1(callback, x)

    @staticmethod
    def simple_service_destroy(context):
        """ Destroys the given instance.

 # Safety

 The passed parameter MUST have been created with the corresponding init function;
 passing any other value results in undefined behavior."""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()

        _rval = _api.simple_service_destroy(context)
        if _rval == FFIError.Ok:
            return _rval
        else:
            raise Exception(f"Function returned error {_rval}")

    @staticmethod
    def simple_service_new_with(context, some_value):
        """ The constructor must return a `Result<Self, Error>`."""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()
        if hasattr(some_value, "_ctx"):
            some_value = some_value.c_value()

        _rval = _api.simple_service_new_with(context, some_value)
        if _rval == FFIError.Ok:
            return _rval
        else:
            raise Exception(f"Function returned error {_rval}")

    @staticmethod
    def simple_service_new_without(context):
        """"""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()

        _rval = _api.simple_service_new_without(context)
        if _rval == FFIError.Ok:
            return _rval
        else:
            raise Exception(f"Function returned error {_rval}")

    @staticmethod
    def simple_service_method_result(context, _anon1):
        """ Methods returning a Result<(), _> are the default and do not
 need annotations."""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()
        if hasattr(_anon1, "_ctx"):
            _anon1 = _anon1.c_value()

        _rval = _api.simple_service_method_result(context, _anon1)
        if _rval == FFIError.Ok:
            return _rval
        else:
            raise Exception(f"Function returned error {_rval}")

    @staticmethod
    def simple_service_method_value(context, x):
        """"""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()
        if hasattr(x, "_ctx"):
            x = x.c_value()

        return _api.simple_service_method_value(context, x)

    @staticmethod
    def simple_service_method_void(context):
        """ This method should be documented.

 Multiple lines."""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()

        return _api.simple_service_method_void(context)

    @staticmethod
    def simple_service_method_mut_self(context, slice):
        """"""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()
        _slice = ffi.new("cffi_sliceu8[]", 1)
        _slice[0].data = slice.c_ptr()
        _slice[0].len = len(slice)
        slice = _slice[0]

        return _api.simple_service_method_mut_self(context, slice)

    @staticmethod
    def simple_service_method_mut_self_void(context, _slice):
        """ Single line."""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()
        __slice = ffi.new("cffi_slicebool[]", 1)
        __slice[0].data = _slice.c_ptr()
        __slice[0].len = len(_slice)
        _slice = __slice[0]

        return _api.simple_service_method_mut_self_void(context, _slice)

    @staticmethod
    def simple_service_method_mut_self_ref(context, x, _y):
        """"""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()
        if hasattr(x, "_ctx"):
            x = x.c_ptr()
        if hasattr(_y, "_ctx"):
            _y = _y.c_ptr()

        return _api.simple_service_method_mut_self_ref(context, x, _y)

    @staticmethod
    def simple_service_method_mut_self_ref_slice(context, x, _y, _slice):
        """"""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()
        if hasattr(x, "_ctx"):
            x = x.c_ptr()
        if hasattr(_y, "_ctx"):
            _y = _y.c_ptr()
        __slice = ffi.new("cffi_sliceu8[]", 1)
        __slice[0].data = _slice.c_ptr()
        __slice[0].len = len(_slice)
        _slice = __slice[0]

        return _api.simple_service_method_mut_self_ref_slice(context, x, _y, _slice)

    @staticmethod
    def simple_service_method_mut_self_ref_slice_limited(context, x, _y, _slice, _slice2):
        """"""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()
        if hasattr(x, "_ctx"):
            x = x.c_ptr()
        if hasattr(_y, "_ctx"):
            _y = _y.c_ptr()
        __slice = ffi.new("cffi_sliceu8[]", 1)
        __slice[0].data = _slice.c_ptr()
        __slice[0].len = len(_slice)
        _slice = __slice[0]
        __slice2 = ffi.new("cffi_sliceu8[]", 1)
        __slice2[0].data = _slice2.c_ptr()
        __slice2[0].len = len(_slice2)
        _slice2 = __slice2[0]

        return _api.simple_service_method_mut_self_ref_slice_limited(context, x, _y, _slice, _slice2)

    @staticmethod
    def simple_service_method_mut_self_ffi_error(context, _slice):
        """"""
        if hasattr(context, "_ctx"):
            context = context.c_ptr()
        __slice = ffi.new("cffi_slicemutu8[]", 1)
        __slice[0].data = _slice.c_ptr()
        __slice[0].len = len(_slice)
        _slice = __slice[0]

        _rval = _api.simple_service_method_mut_self_ffi_error(context, _slice)
        if _rval == FFIError.Ok:
            return _rval
        else:
            raise Exception(f"Function returned error {_rval}")



class SimpleService(CHeapAllocated):
    def __init__(self, some_value):
        """ The constructor must return a `Result<Self, Error>`."""
        self._ctx = ffi.new("cffi_simpleservice**")
        if hasattr(some_value, "_ctx"):
            some_value = some_value.c_ptr()
        api.simple_service_new_with(self.c_ptr(), some_value)

    def __init__(self, ):
        """"""
        self._ctx = ffi.new("cffi_simpleservice**")
        api.simple_service_new_without(self.c_ptr(), )

    def __del__(self):
        api.simple_service_destroy(self.c_ptr(), )

    def method_result(self, _anon1):
        """ Methods returning a Result<(), _> are the default and do not
 need annotations."""
        return api.simple_service_method_result(self.c_value(), _anon1)

    def method_value(self, x):
        """"""
        return api.simple_service_method_value(self.c_value(), x)

    def method_void(self, ):
        """ This method should be documented.

 Multiple lines."""
        return api.simple_service_method_void(self.c_value(), )

    def method_mut_self(self, slice):
        """"""
        return api.simple_service_method_mut_self(self.c_value(), slice)

    def method_mut_self_void(self, _slice):
        """ Single line."""
        return api.simple_service_method_mut_self_void(self.c_value(), _slice)

    def method_mut_self_ref(self, x, _y):
        """"""
        return api.simple_service_method_mut_self_ref(self.c_value(), x, _y)

    def method_mut_self_ref_slice(self, x, _y, _slice):
        """"""
        return api.simple_service_method_mut_self_ref_slice(self.c_value(), x, _y, _slice)

    def method_mut_self_ref_slice_limited(self, x, _y, _slice, _slice2):
        """"""
        return api.simple_service_method_mut_self_ref_slice_limited(self.c_value(), x, _y, _slice, _slice2)

    def method_mut_self_ffi_error(self, _slice):
        """"""
        return api.simple_service_method_mut_self_ffi_error(self.c_value(), _slice)


