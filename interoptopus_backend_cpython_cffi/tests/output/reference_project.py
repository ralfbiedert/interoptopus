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

typedef struct Empty Empty;

typedef enum FFIError
    {
    Ok = 0,
    Null = 100,
    Fail = 200,
    } FFIError;

typedef struct Phantom
    {
    uint32_t x;
    } Phantom;

typedef struct SomeForeignType
    {
    uint32_t x;
    } SomeForeignType;

typedef struct StructDocumented
    {
    float x;
    } StructDocumented;

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

typedef struct Generic
    {
    uint32_t* x;
    } Generic;

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
FFIError complex_1(Vec3f32 _a, Empty* _b);
Opaque* complex_2(SomeForeignType _cmplx);
uint8_t callback(fptr_fn_u8_rval_u8 callback, uint8_t value);
uint32_t generic(Generic x, Phantom _y);
EnumDocumented documented(StructDocumented _x);
uint32_t pattern_ascii_pointer(uint8_t* x, UseAsciiStringPattern y);
FFIError pattern_class_create(Context** context_ptr, uint32_t value);
uint32_t pattern_class_method(Context* context);
FFIError pattern_class_destroy(Context** context_ptr);
FFIError pattern_class_method_success_enum_ok(Context* _context);
FFIError pattern_class_method_success_enum_fail(Context* _context);
uint32_t pattern_ffi_slice(FFISliceu32 ffi_slice);
uint8_t pattern_ffi_slice_delegate(fptr_fn_FFISliceu8_rval_u8 callback);
Vec1 ambiguous_1(Vec1 x);
Vec2 ambiguous_2(Vec2 x);
bool ambiguous_3(Vec1 x, Vec2 y);
Vec namespaced_type(Vec x);
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
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Empty[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Empty[]", n)

class FFISliceu32(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("FFISliceu32[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("FFISliceu32[]", n)

    @property
    def data(self):
        """"""
        return self._ctx.data

    @data.setter
    def data(self, value):
        """"""
        self._ctx.data = value

    @property
    def len(self):
        """"""
        return self._ctx.len

    @len.setter
    def len(self, value):
        """"""
        self._ctx.len = value

class FFISliceu8(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("FFISliceu8[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("FFISliceu8[]", n)

    @property
    def data(self):
        """"""
        return self._ctx.data

    @data.setter
    def data(self, value):
        """"""
        self._ctx.data = value

    @property
    def len(self):
        """"""
        return self._ctx.len

    @len.setter
    def len(self, value):
        """"""
        self._ctx.len = value

class Generic(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Generic[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Generic[]", n)

    @property
    def x(self):
        """"""
        return self._ctx.x

    @x.setter
    def x(self, value):
        """"""
        self._ctx.x = value

class Phantom(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Phantom[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Phantom[]", n)

    @property
    def x(self):
        """"""
        return self._ctx.x

    @x.setter
    def x(self, value):
        """"""
        self._ctx.x = value

class SomeForeignType(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("SomeForeignType[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("SomeForeignType[]", n)

    @property
    def x(self):
        """"""
        return self._ctx.x

    @x.setter
    def x(self, value):
        """"""
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
        """Documented field."""
        self._ctx.x = value

class UseAsciiStringPattern(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("UseAsciiStringPattern[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("UseAsciiStringPattern[]", n)

    @property
    def ascii_string(self):
        """"""
        return self._ctx.ascii_string

    @ascii_string.setter
    def ascii_string(self, value):
        """"""
        self._ctx.ascii_string = value

class Vec(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Vec[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Vec[]", n)

    @property
    def x(self):
        """"""
        return self._ctx.x

    @x.setter
    def x(self, value):
        """"""
        self._ctx.x = value

    @property
    def z(self):
        """"""
        return self._ctx.z

    @z.setter
    def z(self, value):
        """"""
        self._ctx.z = value

class Vec1(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Vec1[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Vec1[]", n)

    @property
    def x(self):
        """"""
        return self._ctx.x

    @x.setter
    def x(self, value):
        """"""
        self._ctx.x = value

    @property
    def y(self):
        """"""
        return self._ctx.y

    @y.setter
    def y(self, value):
        """"""
        self._ctx.y = value

class Vec2(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Vec2[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Vec2[]", n)

    @property
    def x(self):
        """"""
        return self._ctx.x

    @x.setter
    def x(self, value):
        """"""
        self._ctx.x = value

    @property
    def z(self):
        """"""
        return self._ctx.z

    @z.setter
    def z(self, value):
        """"""
        self._ctx.z = value

class Vec3f32(object):
    """"""
    def __init__(self):
        global _api, ffi
        self._ctx = ffi.new("Vec3f32[]", 1)[0]

    def array(n):
        global _api, ffi
        return ffi.new("Vec3f32[]", n)

    @property
    def x(self):
        """"""
        return self._ctx.x

    @x.setter
    def x(self, value):
        """"""
        self._ctx.x = value

    @property
    def y(self):
        """"""
        return self._ctx.y

    @y.setter
    def y(self, value):
        """"""
        self._ctx.y = value

    @property
    def z(self):
        """"""
        return self._ctx.z

    @z.setter
    def z(self, value):
        """"""
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
            x = x._ctx
        return _api.primitive_bool(x)

    def primitive_u8(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_u8(x)

    def primitive_u16(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_u16(x)

    def primitive_u32(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_u32(x)

    def primitive_u64(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_u64(x)

    def primitive_i8(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_i8(x)

    def primitive_i16(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_i16(x)

    def primitive_i32(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_i32(x)

    def primitive_i64(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.primitive_i64(x)

    def ptr(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr(x)

    def ptr_mut(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_mut(x)

    def ptr_ptr(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_ptr(x)

    def ptr_simple(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_simple(x)

    def ptr_simple_mut(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_simple_mut(x)

    def ptr_option(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_option(x)

    def ptr_option_mut(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ptr_option_mut(x)

    def complex_1(_a, _b):
        """"""
        global _api
        if hasattr(_a, "_ctx"):
            _a = _a._ctx
        if hasattr(_b, "_ctx"):
            _b = _b._ctx
        return _api.complex_1(_a, _b)

    def complex_2(_cmplx):
        """"""
        global _api
        if hasattr(_cmplx, "_ctx"):
            _cmplx = _cmplx._ctx
        return _api.complex_2(_cmplx)

    def callback(callback, value):
        """"""
        global _api
        if hasattr(callback, "_ctx"):
            callback = callback._ctx
        if hasattr(value, "_ctx"):
            value = value._ctx
        return _api.callback(callback, value)

    def generic(x, _y):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        if hasattr(_y, "_ctx"):
            _y = _y._ctx
        return _api.generic(x, _y)

    def documented(_x):
        """This function has documentation."""
        global _api
        if hasattr(_x, "_ctx"):
            _x = _x._ctx
        return _api.documented(_x)

    def pattern_ascii_pointer(x, y):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        if hasattr(y, "_ctx"):
            y = y._ctx
        return _api.pattern_ascii_pointer(x, y)

    def pattern_class_create(context_ptr, value):
        """"""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        if hasattr(value, "_ctx"):
            value = value._ctx
        return _api.pattern_class_create(context_ptr, value)

    def pattern_class_method(context):
        """"""
        global _api
        if hasattr(context, "_ctx"):
            context = context._ctx
        return _api.pattern_class_method(context)

    def pattern_class_destroy(context_ptr):
        """# Safety

This function may only be called with a context returned by a succeeding `pattern_class_create`."""
        global _api
        if hasattr(context_ptr, "_ctx"):
            context_ptr = context_ptr._ctx
        return _api.pattern_class_destroy(context_ptr)

    def pattern_class_method_success_enum_ok(_context):
        """"""
        global _api
        if hasattr(_context, "_ctx"):
            _context = _context._ctx
        return _api.pattern_class_method_success_enum_ok(_context)

    def pattern_class_method_success_enum_fail(_context):
        """"""
        global _api
        if hasattr(_context, "_ctx"):
            _context = _context._ctx
        return _api.pattern_class_method_success_enum_fail(_context)

    def pattern_ffi_slice(ffi_slice):
        """"""
        global _api
        if hasattr(ffi_slice, "_ctx"):
            ffi_slice = ffi_slice._ctx
        return _api.pattern_ffi_slice(ffi_slice)

    def pattern_ffi_slice_delegate(callback):
        """"""
        global _api
        if hasattr(callback, "_ctx"):
            callback = callback._ctx
        return _api.pattern_ffi_slice_delegate(callback)

    def ambiguous_1(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ambiguous_1(x)

    def ambiguous_2(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.ambiguous_2(x)

    def ambiguous_3(x, y):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        if hasattr(y, "_ctx"):
            y = y._ctx
        return _api.ambiguous_3(x, y)

    def namespaced_type(x):
        """"""
        global _api
        if hasattr(x, "_ctx"):
            x = x._ctx
        return _api.namespaced_type(x)





class Context(object):
    def __init__(self, value):
        """"""
        global _api, ffi
        self.ctx = ffi.new("Context**")
        rval = _api.pattern_class_create(self.ctx, value)
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def __del__(self):
        """# Safety

This function may only be called with a context returned by a succeeding `pattern_class_create`."""
        global _api, ffi
        rval = _api.pattern_class_destroy(self.ctx, )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def method(self, ):
        """"""
        global _api
        return _api.pattern_class_method(self.ctx[0], )

    def method_success_enum_ok(self, ):
        """"""
        global _api
        rval = _api.pattern_class_method_success_enum_ok(self.ctx[0], )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")

    def method_success_enum_fail(self, ):
        """"""
        global _api
        rval = _api.pattern_class_method_success_enum_fail(self.ctx[0], )
        if rval == FFIError.Ok:
            return None
        else:
            raise Exception(f"return value ${rval}")





def ascii_string(x):
    """Must be called with a b"my_string"."""
    global ffi
    return ffi.new("char[]", x)



