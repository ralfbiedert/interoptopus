from cffi import FFI

api_definition = """




#define C1 1
#define C2 1
#define C3 -100

typedef enum EnumDocumented
    {
    A = 0,
    B = 1,
    } EnumDocumented;

typedef struct Context Context;

typedef struct Opaque Opaque;

typedef struct SimpleService SimpleService;

typedef struct Empty Empty;

typedef enum FFIError
    {
    Ok = 0,
    Null = 100,
    Fail = 200,
    } FFIError;

typedef struct Inner
    {
    float x;
    } Inner;

typedef struct Phantomu8
    {
    uint32_t x;
    } Phantomu8;

typedef struct SomeForeignType
    {
    uint32_t x;
    } SomeForeignType;

typedef struct StructDocumented
    {
    float x;
    } StructDocumented;

typedef struct Tupled
    {
    uint8_t x0;
    } Tupled;

typedef struct UseAsciiStringPattern
    {
    uint8_t* ascii_string;
    } UseAsciiStringPattern;

typedef struct Vec
    {
    double x;
    double z;
    } Vec;

typedef struct Vec1
    {
    float x;
    float y;
    } Vec1;

typedef struct Vec2
    {
    double x;
    double z;
    } Vec2;

typedef struct Vec3f32
    {
    float x;
    float y;
    float z;
    } Vec3f32;

typedef uint8_t (*fptr_fn_u8_rval_u8)(uint8_t x0);

typedef struct Genericu32
    {
    uint32_t* x;
    } Genericu32;

typedef struct Genericu8
    {
    uint8_t* x;
    } Genericu8;

typedef struct FFISliceu32
    {
    uint32_t* data;
    uint64_t len;
    } FFISliceu32;

typedef struct FFISliceu8
    {
    uint8_t* data;
    uint64_t len;
    } FFISliceu8;

typedef struct FFIOptionInner
    {
    Inner t;
    uint8_t is_some;
    } FFIOptionInner;

typedef uint8_t (*fptr_fn_FFISliceu8_rval_u8)(FFISliceu8 x0);


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
int64_t* ptr_simple(int64_t* x);
int64_t* ptr_simple_mut(int64_t* x);
bool ptr_option(int64_t* x);
bool ptr_option_mut(int64_t* x);
Tupled tupled(Tupled x);
FFIError complex_1(Vec3f32 _a, Empty* _b);
Opaque* complex_2(SomeForeignType _cmplx);
uint8_t callback(fptr_fn_u8_rval_u8 callback, uint8_t value);
uint32_t generic_1(Genericu32 x, Phantomu8 _y);
uint8_t generic_2(Genericu8 x, Phantomu8 _y);
EnumDocumented documented(StructDocumented _x);
Vec1 ambiguous_1(Vec1 x);
Vec2 ambiguous_2(Vec2 x);
bool ambiguous_3(Vec1 x, Vec2 y);
Vec namespaced_type(Vec x);
uint32_t pattern_ascii_pointer(uint8_t* x, UseAsciiStringPattern y);
uint32_t pattern_ffi_slice(FFISliceu32 ffi_slice);
uint8_t pattern_ffi_slice_delegate(fptr_fn_FFISliceu8_rval_u8 callback);
FFIOptionInner pattern_ffi_option(FFIOptionInner ffi_slice);
FFIError pattern_service_create(Context** context_ptr, uint32_t value);
FFIError pattern_service_destroy(Context** context_ptr);
uint32_t pattern_service_method(Context* context);
FFIError pattern_service_method_success_enum_ok(Context* _context);
FFIError pattern_service_method_success_enum_fail(Context* _context);
FFIError simple_service_create(SimpleService** context_ptr, uint32_t x);
FFIError simple_service_destroy(SimpleService** context_ptr);
FFIError simple_service_result(SimpleService* context_ptr, uint32_t x);
uint32_t simple_service_value(SimpleService* context_ptr, uint32_t x);
uint32_t simple_service_mut_self(SimpleService* context_ptr, uint32_t x);
void simple_service_void(SimpleService* context_ptr);
uint32_t simple_service_extra_method(SimpleService* _context);
"""


ffi = FFI()
ffi.cdef(api_definition)
_api = None


def init_api(dll):
    """Initializes this library, call with path to DLL."""
    global _api
    _api = ffi.dlopen(dll)





C1 = 1

C2 = 1

C3 = -100


class EnumDocumented:
    """Documented enum."""
    A = 0
    B = 1


class Empty(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Empty[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Empty[]", n)

class Genericu32(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Genericu32[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Genericu32[]", n)

    @property
    def x(self):
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

class Genericu8(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Genericu8[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Genericu8[]", n)

    @property
    def x(self):
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

class Inner(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Inner[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Inner[]", n)

    @property
    def x(self):
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

class Phantomu8(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Phantomu8[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Phantomu8[]", n)

    @property
    def x(self):
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

class SomeForeignType(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("SomeForeignType[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("SomeForeignType[]", n)

    @property
    def x(self):
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

class StructDocumented(object):
    """Documented struct."""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("StructDocumented[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("StructDocumented[]", n)

    @property
    def x(self):
        """Documented field."""
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

class Tupled(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Tupled[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Tupled[]", n)

    @property
    def x0(self):
        return self._ctx.x0

    @x0.setter
    def x0(self, value):
        self._ptr_x0 = value
        self._ctx.x0 = value

class UseAsciiStringPattern(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("UseAsciiStringPattern[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("UseAsciiStringPattern[]", n)

    @property
    def ascii_string(self):
        return self._ctx.ascii_string

    @ascii_string.setter
    def ascii_string(self, value):
        self._ptr_ascii_string = value
        self._ctx.ascii_string = value

class Vec(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Vec[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Vec[]", n)

    @property
    def x(self):
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

    @property
    def z(self):
        return self._ctx.z

    @z.setter
    def z(self, value):
        self._ptr_z = value
        self._ctx.z = value

class Vec1(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Vec1[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Vec1[]", n)

    @property
    def x(self):
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

    @property
    def y(self):
        return self._ctx.y

    @y.setter
    def y(self, value):
        self._ptr_y = value
        self._ctx.y = value

class Vec2(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Vec2[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Vec2[]", n)

    @property
    def x(self):
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

    @property
    def z(self):
        return self._ctx.z

    @z.setter
    def z(self, value):
        self._ptr_z = value
        self._ctx.z = value

class Vec3f32(object):
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Vec3f32[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Vec3f32[]", n)

    @property
    def x(self):
        return self._ctx.x

    @x.setter
    def x(self, value):
        self._ptr_x = value
        self._ctx.x = value

    @property
    def y(self):
        return self._ctx.y

    @y.setter
    def y(self, value):
        self._ptr_y = value
        self._ctx.y = value

    @property
    def z(self):
        return self._ctx.z

    @z.setter
    def z(self, value):
        self._ptr_z = value
        self._ctx.z = value

class FFIError:
    """"""
    Ok = 0
    Null = 100
    Fail = 200




class callbacks:
    """Helpers to define `@ffi.callback`-style callbacks."""
    fn_u8_rval_u8 = "uint8_t(uint8_t)"
    fn_FFISliceu8_rval_u8 = "uint8_t(FFISliceu8)"




class raw:
    """Raw access to all exported functions."""
    def primitive_void():
        global _api
        return _api.primitive_void()

    def primitive_void2():
        global _api
        return _api.primitive_void2()

    def primitive_bool(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_bool(x)

    def primitive_u8(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_u8(x)

    def primitive_u16(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_u16(x)

    def primitive_u32(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_u32(x)

    def primitive_u64(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_u64(x)

    def primitive_i8(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_i8(x)

    def primitive_i16(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_i16(x)

    def primitive_i32(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_i32(x)

    def primitive_i64(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_i64(x)

    def ptr(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr(x)

    def ptr_mut(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_mut(x)

    def ptr_ptr(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_ptr(x)

    def ptr_simple(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_simple(x)

    def ptr_simple_mut(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_simple_mut(x)

    def ptr_option(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_option(x)

    def ptr_option_mut(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_option_mut(x)

    def tupled(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.tupled(x)

    def complex_1(_a, _b):
        global _api
        if hasattr(_a, "_ctx"):
            _a = _a._ctx
        if hasattr(_b, "_ctx"):
            _b = _b._ctx
        return _api.complex_1(_a, _b)

    def complex_2(_cmplx):
        global _api
        if hasattr(_cmplx, "_ctx"):
            _cmplx = _cmplx._ctx
        return _api.complex_2(_cmplx)

    def callback(callback, value):
        global _api
        if hasattr(callback, "_ctx"):
            callback = callback._ctx
        if hasattr(value, "_ctx"):
            value = value._ctx
        return _api.callback(callback, value)

    def generic_1(x, _y):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        if hasattr(_y, "_ctx"):
            _y = _y._ctx
        return _api.generic_1(x, _y)

    def generic_2(x, _y):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        if hasattr(_y, "_ctx"):
            _y = _y._ctx
        return _api.generic_2(x, _y)

    def documented(_x):
        """This function has documentation."""
        global _api
        if hasattr(_x, "_ctx"):
            _x = _x._ctx
        return _api.documented(_x)

    def ambiguous_1(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ambiguous_1(x)

    def ambiguous_2(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ambiguous_2(x)

    def ambiguous_3(x, y):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        if hasattr(y, "_ctx"):
            y = y._ctx
        return _api.ambiguous_3(x, y)

    def namespaced_type(x):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.namespaced_type(x)

    def pattern_ascii_pointer(x, y):
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        if hasattr(y, "_ctx"):
            y = y._ctx
        return _api.pattern_ascii_pointer(x, y)

    def pattern_ffi_slice(ffi_slice):
        global _api
        if hasattr(ffi_slice, "_ctx"):
            ffi_slice = ffi_slice._ctx
        return _api.pattern_ffi_slice(ffi_slice)

    def pattern_ffi_slice_delegate(callback):
        global _api
        if hasattr(callback, "_ctx"):
            callback = callback._ctx
        return _api.pattern_ffi_slice_delegate(callback)

    def pattern_ffi_option(ffi_slice):
        global _api
        if hasattr(ffi_slice, "_ctx"):
            ffi_slice = ffi_slice._ctx
        return _api.pattern_ffi_option(ffi_slice)

    def pattern_service_create(context_ptr, value):
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        if hasattr(value, "_ctx"):
            value = value._ctx
        return _api.pattern_service_create(context_ptr, value)

    def pattern_service_destroy(context_ptr):
        """# Safety

This function may only be called with a context returned by a succeeding `pattern_service_create`."""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        return _api.pattern_service_destroy(context_ptr)

    def pattern_service_method(context):
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx
        return _api.pattern_service_method(context)

    def pattern_service_method_success_enum_ok(_context):
        global _api
        if hasattr(_context, "_ctx"):
            _context = _context._ctx
        return _api.pattern_service_method_success_enum_ok(_context)

    def pattern_service_method_success_enum_fail(_context):
        global _api
        if hasattr(_context, "_ctx"):
            _context = _context._ctx
        return _api.pattern_service_method_success_enum_fail(_context)

    def simple_service_create(context_ptr, x):
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.simple_service_create(context_ptr, x)

    def simple_service_destroy(context_ptr):
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        return _api.simple_service_destroy(context_ptr)

    def simple_service_result(context_ptr, x):
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.simple_service_result(context_ptr, x)

    def simple_service_value(context_ptr, x):
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.simple_service_value(context_ptr, x)

    def simple_service_mut_self(context_ptr, x):
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.simple_service_mut_self(context_ptr, x)

    def simple_service_void(context_ptr):
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        return _api.simple_service_void(context_ptr)

    def simple_service_extra_method(_context):
        """An extra exposed method."""
        global _api
        if hasattr(_context, "_ctx"):
            _context = _context._ctx
        return _api.simple_service_extra_method(_context)





class Context(object):
    def __init__(self, value):
        if hasattr(value, "_ctx"):
            value = value._ctx
        global _api, ffi
        self.ctx = ffi.new("Context**")
        rval = raw.pattern_service_create(self.ctx, value)
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def __del__(self):
        """# Safety

This function may only be called with a context returned by a succeeding `pattern_service_create`."""
        global _api, ffi
        rval = raw.pattern_service_destroy(self.ctx, )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def method(self, ):
        global raw
        return _api.pattern_service_method(self.ctx[0], )

    def method_success_enum_ok(self, ):
        global raw
        rval = raw.pattern_service_method_success_enum_ok(self.ctx[0], )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def method_success_enum_fail(self, ):
        global raw
        rval = raw.pattern_service_method_success_enum_fail(self.ctx[0], )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")



class SimpleService(object):
    def __init__(self, x):
        if hasattr(x, "_ctx"):
            x = x._ctx
        global _api, ffi
        self.ctx = ffi.new("SimpleService**")
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
        global raw
        rval = raw.simple_service_result(self.ctx[0], x)
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def value(self, x):
        global raw
        return _api.simple_service_value(self.ctx[0], x)

    def mut_self(self, x):
        global raw
        return _api.simple_service_mut_self(self.ctx[0], x)

    def void(self, ):
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



