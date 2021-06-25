from cffi import FFI

api_definition = """




const uint8_t CFFI_U8 = 255;
const float CFFI_F32_MIN_POSITIVE = 0.000000000000000000000000000000000000011754944;
const int32_t CFFI_COMPUTED_I32 = -2147483647;

typedef enum cffi_enumdocumented
    {
    CFFI_A = 0,
    CFFI_B = 1,
    } cffi_enumdocumented;

typedef struct cffi_context cffi_context;
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

typedef uint8_t (*cffi_fptr_fn_u8_rval_u8)(uint8_t x0);

typedef struct cffi_genericu32
    {
    uint32_t* x;
    } cffi_genericu32;

typedef struct cffi_genericu8
    {
    uint8_t* x;
    } cffi_genericu8;

typedef struct cffi_ffisliceu32
    {
    uint32_t* data;
    uint64_t len;
    } cffi_ffisliceu32;

typedef struct cffi_ffisliceu8
    {
    uint8_t* data;
    uint64_t len;
    } cffi_ffisliceu8;

typedef struct cffi_ffioptioninner
    {
    cffi_inner t;
    uint8_t is_some;
    } cffi_ffioptioninner;

typedef uint8_t (*cffi_fptr_fn_FFISliceu8_rval_u8)(cffi_ffisliceu8 x0);

typedef struct cffi_ffislicevec3f32
    {
    cffi_vec3f32* data;
    uint64_t len;
    } cffi_ffislicevec3f32;


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
uint32_t generic_1(cffi_genericu32 x, cffi_phantomu8 _y);
uint8_t generic_2(cffi_genericu8 x, cffi_phantomu8 _y);
cffi_enumdocumented documented(cffi_structdocumented _x);
cffi_vec1 ambiguous_1(cffi_vec1 x);
cffi_vec2 ambiguous_2(cffi_vec2 x);
bool ambiguous_3(cffi_vec1 x, cffi_vec2 y);
cffi_vec namespaced_type(cffi_vec x);
uint32_t pattern_ascii_pointer_1(uint8_t* x);
uint32_t pattern_ascii_pointer_len(uint8_t* x, cffi_useasciistringpattern y);
uint32_t pattern_ffi_slice_1(cffi_ffisliceu32 ffi_slice);
cffi_vec3f32 pattern_ffi_slice_2(cffi_ffislicevec3f32 ffi_slice, int32_t i);
uint8_t pattern_ffi_slice_delegate(cffi_fptr_fn_FFISliceu8_rval_u8 callback);
cffi_ffioptioninner pattern_ffi_option_1(cffi_ffioptioninner ffi_slice);
cffi_inner pattern_ffi_option_2(cffi_ffioptioninner ffi_slice);
cffi_ffierror pattern_service_create(cffi_context** context_ptr, uint32_t value);
cffi_ffierror pattern_service_destroy(cffi_context** context_ptr);
uint32_t pattern_service_method(cffi_context* context);
cffi_ffierror pattern_service_method_success_enum_ok(cffi_context* _context);
cffi_ffierror pattern_service_method_success_enum_fail(cffi_context* _context);
cffi_ffierror simple_service_create(cffi_simpleservice** context_ptr, uint32_t x);
cffi_ffierror simple_service_destroy(cffi_simpleservice** context_ptr);
cffi_ffierror simple_service_result(cffi_simpleservice* context_ptr, uint32_t x);
uint32_t simple_service_value(cffi_simpleservice* context_ptr, uint32_t x);
uint32_t simple_service_mut_self(cffi_simpleservice* context_ptr, uint32_t x);
void simple_service_void(cffi_simpleservice* context_ptr);
uint32_t simple_service_extra_method(cffi_simpleservice* _context);
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

class FFIError:
    """"""
    Ok = 0
    Null = 100
    Panic = 200
    Fail = 300




class callbacks:
    """Helpers to define `@ffi.callback`-style callbacks."""
    fn_u8_rval_u8 = "uint8_t(uint8_t)"
    fn_FFISliceu8_rval_u8 = "uint8_t(cffi_ffisliceu8)"




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

    def generic_1(x, _y):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        if hasattr(_y, "_ctx"):
            _y = _y._ctx[0]
        return _api.generic_1(x, _y)

    def generic_2(x, _y):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        if hasattr(_y, "_ctx"):
            _y = _y._ctx[0]
        return _api.generic_2(x, _y)

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

    def pattern_ffi_slice_delegate(callback):
        """"""
        global _api
        if hasattr(callback, "_ctx"):
            callback = callback._ctx[0]
        return _api.pattern_ffi_slice_delegate(callback)

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

    def pattern_service_create(context_ptr, value):
        """"""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        if hasattr(value, "_ctx"):
            value = value._ctx[0]
        return _api.pattern_service_create(context_ptr, value)

    def pattern_service_destroy(context_ptr):
        """# Safety

This function may only be called with a context returned by a succeeding `pattern_service_create`."""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        return _api.pattern_service_destroy(context_ptr)

    def pattern_service_method(context):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx[0]
        return _api.pattern_service_method(context)

    def pattern_service_method_success_enum_ok(_context):
        """"""
        global _api
        if hasattr(_context, "_ctx"):
            _context = _context._ctx[0]
        return _api.pattern_service_method_success_enum_ok(_context)

    def pattern_service_method_success_enum_fail(_context):
        """"""
        global _api
        if hasattr(_context, "_ctx"):
            _context = _context._ctx[0]
        return _api.pattern_service_method_success_enum_fail(_context)

    def simple_service_create(context_ptr, x):
        """"""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.simple_service_create(context_ptr, x)

    def simple_service_destroy(context_ptr):
        """"""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        return _api.simple_service_destroy(context_ptr)

    def simple_service_result(context_ptr, x):
        """"""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.simple_service_result(context_ptr, x)

    def simple_service_value(context_ptr, x):
        """"""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.simple_service_value(context_ptr, x)

    def simple_service_mut_self(context_ptr, x):
        """"""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        if hasattr(x, "_ctx"):
            x = x._ctx[0]
        return _api.simple_service_mut_self(context_ptr, x)

    def simple_service_void(context_ptr):
        """"""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx[0]
        return _api.simple_service_void(context_ptr)

    def simple_service_extra_method(_context):
        """An extra exposed method."""
        global _api
        if hasattr(_context, "_ctx"):
            _context = _context._ctx[0]
        return _api.simple_service_extra_method(_context)





class Context(object):
    def __init__(self, value):
        """"""
        global _api, ffi
        if hasattr(value, "_ctx"):
            value = value._ctx
        self.ctx = ffi.new("cffi_context**")
        rval = raw.pattern_service_create(self.ctx, value)
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def __del__(self):
        global _api, ffi
        rval = raw.pattern_service_destroy(self.ctx, )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def method(self, ):
        """"""
        global raw
        return _api.pattern_service_method(self.ctx[0], )

    def method_success_enum_ok(self, ):
        """"""
        global raw
        rval = raw.pattern_service_method_success_enum_ok(self.ctx[0], )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def method_success_enum_fail(self, ):
        """"""
        global raw
        rval = raw.pattern_service_method_success_enum_fail(self.ctx[0], )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")



class SimpleService(object):
    def __init__(self, x):
        """"""
        global _api, ffi
        if hasattr(x, "_ctx"):
            x = x._ctx
        self.ctx = ffi.new("cffi_simpleservice**")
        rval = raw.simple_service_create(self.ctx, x)
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def __del__(self):
        global _api, ffi
        rval = raw.simple_service_destroy(self.ctx, )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def result(self, x):
        """"""
        global raw
        rval = raw.simple_service_result(self.ctx[0], x)
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def value(self, x):
        """"""
        global raw
        return _api.simple_service_value(self.ctx[0], x)

    def mut_self(self, x):
        """"""
        global raw
        return _api.simple_service_mut_self(self.ctx[0], x)

    def void(self, ):
        """"""
        global raw
        return _api.simple_service_void(self.ctx[0], )

    def extra_method(self, ):
        """An extra exposed method."""
        global raw
        return _api.simple_service_extra_method(self.ctx[0], )





def ascii_string(x):
    """Must be called with a b"my_string"."""
    global ffi
    return ffi.new("char[]", x)



