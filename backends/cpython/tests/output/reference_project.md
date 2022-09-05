
## API Overview

### Functions
Freestanding callables inside the module.
 - **[primitive_void](#primitive_void)** - 
 - **[primitive_void2](#primitive_void2)** - 
 - **[primitive_bool](#primitive_bool)** - 
 - **[primitive_u8](#primitive_u8)** - 
 - **[primitive_u16](#primitive_u16)** - 
 - **[primitive_u32](#primitive_u32)** - 
 - **[primitive_u64](#primitive_u64)** - 
 - **[primitive_i8](#primitive_i8)** - 
 - **[primitive_i16](#primitive_i16)** - 
 - **[primitive_i32](#primitive_i32)** - 
 - **[primitive_i64](#primitive_i64)** - 
 - **[many_args_5](#many_args_5)** - 
 - **[many_args_10](#many_args_10)** - 
 - **[ptr](#ptr)** - 
 - **[ptr_mut](#ptr_mut)** -  # Safety
 - **[ptr_ptr](#ptr_ptr)** - 
 - **[ref_simple](#ref_simple)** - 
 - **[ref_mut_simple](#ref_mut_simple)** - 
 - **[ref_option](#ref_option)** - 
 - **[ref_mut_option](#ref_mut_option)** - 
 - **[tupled](#tupled)** - 
 - **[complex_args_1](#complex_args_1)** - 
 - **[complex_args_2](#complex_args_2)** - 
 - **[callback](#callback)** - 
 - **[generic_1a](#generic_1a)** - 
 - **[generic_1b](#generic_1b)** - 
 - **[generic_1c](#generic_1c)** - 
 - **[generic_2](#generic_2)** - 
 - **[generic_3](#generic_3)** - 
 - **[generic_4](#generic_4)** - 
 - **[array_1](#array_1)** - 
 - **[documented](#documented)** -  This function has documentation.
 - **[ambiguous_1](#ambiguous_1)** - 
 - **[ambiguous_2](#ambiguous_2)** - 
 - **[ambiguous_3](#ambiguous_3)** - 
 - **[namespaced_type](#namespaced_type)** - 
 - **[namespaced_inner_option](#namespaced_inner_option)** - 
 - **[namespaced_inner_slice](#namespaced_inner_slice)** - 
 - **[namespaced_inner_slice_mut](#namespaced_inner_slice_mut)** - 
 - **[panics](#panics)** - 
 - **[renamed](#renamed)** - 
 - **[sleep](#sleep)** - 
 - **[weird_1](#weird_1)** - 
 - **[visibility](#visibility)** - 
 - **[repr_transparent](#repr_transparent)** - 
 - **[pattern_ascii_pointer_1](#pattern_ascii_pointer_1)** - 
 - **[pattern_ascii_pointer_2](#pattern_ascii_pointer_2)** - 
 - **[pattern_ascii_pointer_len](#pattern_ascii_pointer_len)** - 
 - **[pattern_ascii_pointer_return_slice](#pattern_ascii_pointer_return_slice)** - 
 - **[pattern_ffi_slice_1](#pattern_ffi_slice_1)** - 
 - **[pattern_ffi_slice_2](#pattern_ffi_slice_2)** - 
 - **[pattern_ffi_slice_3](#pattern_ffi_slice_3)** - 
 - **[pattern_ffi_slice_4](#pattern_ffi_slice_4)** - 
 - **[pattern_ffi_slice_5](#pattern_ffi_slice_5)** - 
 - **[pattern_ffi_slice_6](#pattern_ffi_slice_6)** - 
 - **[pattern_ffi_slice_delegate](#pattern_ffi_slice_delegate)** - 
 - **[pattern_ffi_slice_delegate_huge](#pattern_ffi_slice_delegate_huge)** - 
 - **[pattern_ffi_option_1](#pattern_ffi_option_1)** - 
 - **[pattern_ffi_option_2](#pattern_ffi_option_2)** - 
 - **[pattern_ffi_bool](#pattern_ffi_bool)** - 
 - **[pattern_ffi_cchar](#pattern_ffi_cchar)** - 
 - **[pattern_ffi_cchar_const_pointer](#pattern_ffi_cchar_const_pointer)** - 
 - **[pattern_ffi_cchar_mut_pointer](#pattern_ffi_cchar_mut_pointer)** - 
 - **[pattern_api_guard](#pattern_api_guard)** - 
 - **[pattern_callback_1](#pattern_callback_1)** - 
 - **[pattern_callback_2](#pattern_callback_2)** - 

### Classes
Methods operating on common state.
 - **[SimpleService](#SimpleService)** -  Some struct we want to expose as a class.
     - **[new_with](#SimpleService.new_with)** <sup>**ctor**</sup> -  The constructor must return a `Result<Self, Error>`.
     - **[new_without](#SimpleService.new_without)** <sup>**ctor**</sup> - 
     - **[new_with_string](#SimpleService.new_with_string)** <sup>**ctor**</sup> - 
     - **[new_failing](#SimpleService.new_failing)** <sup>**ctor**</sup> - 
     - **[method_result](#SimpleService.method_result)** -  Methods returning a Result<(), _> are the default and do not
     - **[method_value](#SimpleService.method_value)** - 
     - **[method_void](#SimpleService.method_void)** -  This method should be documented.
     - **[method_mut_self](#SimpleService.method_mut_self)** - 
     - **[method_mut_self_void](#SimpleService.method_mut_self_void)** -  Single line.
     - **[method_mut_self_ref](#SimpleService.method_mut_self_ref)** - 
     - **[method_mut_self_ref_slice](#SimpleService.method_mut_self_ref_slice)** - 
     - **[method_mut_self_ref_slice_limited](#SimpleService.method_mut_self_ref_slice_limited)** - 
     - **[method_mut_self_ffi_error](#SimpleService.method_mut_self_ffi_error)** - 
     - **[method_mut_self_no_error](#SimpleService.method_mut_self_no_error)** - 
     - **[return_slice](#SimpleService.return_slice)** -  Warning, you _must_ discard the returned slice object before calling into this service
     - **[return_slice_mut](#SimpleService.return_slice_mut)** -  Warning, you _must_ discard the returned slice object before calling into this service
     - **[return_string](#SimpleService.return_string)** -  This function has no panic safeguards. If it panics your host app will be in an undefined state.
     - **[method_void_ffi_error](#SimpleService.method_void_ffi_error)** - 
     - **[method_callback](#SimpleService.method_callback)** - 
 - **[SimpleServiceLifetime](#SimpleServiceLifetime)** - 
     - **[new_with](#SimpleServiceLifetime.new_with)** <sup>**ctor**</sup> - 
     - **[method_lt](#SimpleServiceLifetime.method_lt)** - 
     - **[method_lt2](#SimpleServiceLifetime.method_lt2)** - 
     - **[return_string_accept_slice](#SimpleServiceLifetime.return_string_accept_slice)** - 
     - **[method_void_ffi_error](#SimpleServiceLifetime.method_void_ffi_error)** - 

### Enums
Groups of related constants.
 - **[EnumDocumented](#EnumDocumented)** -  Documented enum.
 - **[EnumRenamed](#EnumRenamed)** - 

### Data Structs
Composite data used by functions and methods.
 - **[Array](#Array)** - 
 - **[ExtraTypef32](#ExtraTypef32)** - 
 - **[Genericu32](#Genericu32)** - 
 - **[Genericu8](#Genericu8)** - 
 - **[Inner](#Inner)** - 
 - **[Phantomu8](#Phantomu8)** - 
 - **[SomeForeignType](#SomeForeignType)** - 
 - **[StructDocumented](#StructDocumented)** -  Documented struct.
 - **[StructRenamed](#StructRenamed)** - 
 - **[Tupled](#Tupled)** - 
 - **[UseAsciiStringPattern](#UseAsciiStringPattern)** - 
 - **[Vec](#Vec)** - 
 - **[Vec1](#Vec1)** - 
 - **[Vec2](#Vec2)** - 
 - **[Vec3f32](#Vec3f32)** - 
 - **[Visibility1](#Visibility1)** - 
 - **[Visibility2](#Visibility2)** - 
 - **[Weird1u32](#Weird1u32)** - 
 - **[Weird2u8](#Weird2u8)** - 
 - **[SliceBool](#SliceBool)** - A pointer and length of un-owned elements.
 - **[SliceUseAsciiStringPattern](#SliceUseAsciiStringPattern)** - A pointer and length of un-owned elements.
 - **[SliceVec](#SliceVec)** - A pointer and length of un-owned elements.
 - **[SliceVec3f32](#SliceVec3f32)** - A pointer and length of un-owned elements.
 - **[Sliceu32](#Sliceu32)** - A pointer and length of un-owned elements.
 - **[Sliceu8](#Sliceu8)** - A pointer and length of un-owned elements.
 - **[OptionInner](#OptionInner)** - A boolean flag and optionally data.
 - **[OptionVec](#OptionVec)** - A boolean flag and optionally data.
# Types 


 ### <a name="Array">**Array**</a>


#### Fields 
- **data** -  
#### Definition 
```python
class Array(ctypes.Structure):

    _fields_ = [
        ("data", ctypes.c_uint8 * 16),
    ]

    def __init__(self, data = None):
        ...
```

---



 ### <a name="ExtraTypef32">**ExtraTypef32**</a>


#### Fields 
- **x** -  
#### Definition 
```python
class ExtraTypef32(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_float),
    ]

    def __init__(self, x: float = None):
        ...
```

---



 ### <a name="Genericu32">**Genericu32**</a>


#### Fields 
- **x** -  
#### Definition 
```python
class Genericu32(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.POINTER(ctypes.c_uint32)),
    ]

    def __init__(self, x: ctypes.POINTER(ctypes.c_uint32) = None):
        ...
```

---



 ### <a name="Genericu8">**Genericu8**</a>


#### Fields 
- **x** -  
#### Definition 
```python
class Genericu8(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.POINTER(ctypes.c_uint8)),
    ]

    def __init__(self, x: ctypes.POINTER(ctypes.c_uint8) = None):
        ...
```

---



 ### <a name="Inner">**Inner**</a>


#### Fields 
- **x** -  
#### Definition 
```python
class Inner(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_float),
    ]

    def __init__(self, x: float = None):
        ...
```

---



 ### <a name="Phantomu8">**Phantomu8**</a>


#### Fields 
- **x** -  
#### Definition 
```python
class Phantomu8(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_uint32),
    ]

    def __init__(self, x: int = None):
        ...
```

---



 ### <a name="SomeForeignType">**SomeForeignType**</a>


#### Fields 
- **x** -  
#### Definition 
```python
class SomeForeignType(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_uint32),
    ]

    def __init__(self, x: int = None):
        ...
```

---



 ### <a name="StructDocumented">**StructDocumented**</a>

Documented struct.

#### Fields 
- **x** -  Documented field. 
#### Definition 
```python
class StructDocumented(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_float),
    ]

    def __init__(self, x: float = None):
        ...
```

---



 ### <a name="StructRenamed">**StructRenamed**</a>


#### Fields 
- **e** -  
#### Definition 
```python
class StructRenamed(ctypes.Structure):

    _fields_ = [
        ("e", ctypes.c_int),
    ]

    def __init__(self, e: ctypes.c_int = None):
        ...
```

---



 ### <a name="Tupled">**Tupled**</a>


#### Fields 
- **x0** -  
#### Definition 
```python
class Tupled(ctypes.Structure):

    _fields_ = [
        ("x0", ctypes.c_uint8),
    ]

    def __init__(self, x0: int = None):
        ...
```

---



 ### <a name="UseAsciiStringPattern">**UseAsciiStringPattern**</a>


#### Fields 
- **ascii_string** -  
#### Definition 
```python
class UseAsciiStringPattern(ctypes.Structure):

    _fields_ = [
        ("ascii_string", ctypes.POINTER(ctypes.c_char)),
    ]

    def __init__(self, ascii_string: str = None):
        ...
```

---



 ### <a name="Vec">**Vec**</a>


#### Fields 
- **x** -  
- **z** -  
#### Definition 
```python
class Vec(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_double),
        ("z", ctypes.c_double),
    ]

    def __init__(self, x: float = None, z: float = None):
        ...
```

---



 ### <a name="Vec1">**Vec1**</a>


#### Fields 
- **x** -  
- **y** -  
#### Definition 
```python
class Vec1(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
    ]

    def __init__(self, x: float = None, y: float = None):
        ...
```

---



 ### <a name="Vec2">**Vec2**</a>


#### Fields 
- **x** -  
- **z** -  
#### Definition 
```python
class Vec2(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_double),
        ("z", ctypes.c_double),
    ]

    def __init__(self, x: float = None, z: float = None):
        ...
```

---



 ### <a name="Vec3f32">**Vec3f32**</a>


#### Fields 
- **x** -  
- **y** -  
- **z** -  
#### Definition 
```python
class Vec3f32(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
        ("z", ctypes.c_float),
    ]

    def __init__(self, x: float = None, y: float = None, z: float = None):
        ...
```

---



 ### <a name="Visibility1">**Visibility1**</a>


#### Fields 
- **pblc** -  
- **prvt** -  
#### Definition 
```python
class Visibility1(ctypes.Structure):

    _fields_ = [
        ("pblc", ctypes.c_uint8),
        ("prvt", ctypes.c_uint8),
    ]

    def __init__(self, pblc: int = None, prvt: int = None):
        ...
```

---



 ### <a name="Visibility2">**Visibility2**</a>


#### Fields 
- **pblc1** -  
- **pblc2** -  
#### Definition 
```python
class Visibility2(ctypes.Structure):

    _fields_ = [
        ("pblc1", ctypes.c_uint8),
        ("pblc2", ctypes.c_uint8),
    ]

    def __init__(self, pblc1: int = None, pblc2: int = None):
        ...
```

---



 ### <a name="Weird1u32">**Weird1u32**</a>


#### Fields 
- **x** -  
#### Definition 
```python
class Weird1u32(ctypes.Structure):

    _fields_ = [
        ("x", ctypes.c_uint32),
    ]

    def __init__(self, x: int = None):
        ...
```

---



 ### <a name="Weird2u8">**Weird2u8**</a>


#### Fields 
- **t** -  
- **a** -  
- **r** -  
#### Definition 
```python
class Weird2u8(ctypes.Structure):

    _fields_ = [
        ("t", ctypes.c_uint8),
        ("a", ctypes.c_uint8 * 5),
        ("r", ctypes.POINTER(ctypes.c_uint8)),
    ]

    def __init__(self, t: int = None, a = None, r: ctypes.POINTER(ctypes.c_uint8) = None):
        ...
```

---



 ### <a name="SliceBool">**SliceBool**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```python
class SliceBool(ctypes.Structure):

    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_uint8)),
        ("len", ctypes.c_uint64),
    ]

    def __init__(self, data: ctypes.POINTER(ctypes.c_uint8) = None, len: int = None):
        ...
```

---



 ### <a name="SliceUseAsciiStringPattern">**SliceUseAsciiStringPattern**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```python
class SliceUseAsciiStringPattern(ctypes.Structure):

    _fields_ = [
        ("data", ctypes.POINTER(UseAsciiStringPattern)),
        ("len", ctypes.c_uint64),
    ]

    def __init__(self, data: ctypes.POINTER(UseAsciiStringPattern) = None, len: int = None):
        ...
```

---



 ### <a name="SliceVec">**SliceVec**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```python
class SliceVec(ctypes.Structure):

    _fields_ = [
        ("data", ctypes.POINTER(Vec)),
        ("len", ctypes.c_uint64),
    ]

    def __init__(self, data: ctypes.POINTER(Vec) = None, len: int = None):
        ...
```

---



 ### <a name="SliceVec3f32">**SliceVec3f32**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```python
class SliceVec3f32(ctypes.Structure):

    _fields_ = [
        ("data", ctypes.POINTER(Vec3f32)),
        ("len", ctypes.c_uint64),
    ]

    def __init__(self, data: ctypes.POINTER(Vec3f32) = None, len: int = None):
        ...
```

---



 ### <a name="Sliceu32">**Sliceu32**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```python
class Sliceu32(ctypes.Structure):

    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_uint32)),
        ("len", ctypes.c_uint64),
    ]

    def __init__(self, data: ctypes.POINTER(ctypes.c_uint32) = None, len: int = None):
        ...
```

---



 ### <a name="Sliceu8">**Sliceu8**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```python
class Sliceu8(ctypes.Structure):

    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_uint8)),
        ("len", ctypes.c_uint64),
    ]

    def __init__(self, data: ctypes.POINTER(ctypes.c_uint8) = None, len: int = None):
        ...
```

---



 ### <a name="OptionInner">**OptionInner**</a>

Option type containing boolean flag and maybe valid data.

#### Fields 
- **t** - Element that is maybe valid. 
- **is_some** - Byte where `1` means element `t` is valid. 
#### Definition 
```python
class OptionInner(ctypes.Structure):

    _fields_ = [
        ("t", Inner),
        ("is_some", ctypes.c_uint8),
    ]

    def __init__(self, t: Inner = None, is_some: int = None):
        ...
```

---



 ### <a name="OptionVec">**OptionVec**</a>

Option type containing boolean flag and maybe valid data.

#### Fields 
- **t** - Element that is maybe valid. 
- **is_some** - Byte where `1` means element `t` is valid. 
#### Definition 
```python
class OptionVec(ctypes.Structure):

    _fields_ = [
        ("t", Vec),
        ("is_some", ctypes.c_uint8),
    ]

    def __init__(self, t: Vec = None, is_some: int = None):
        ...
```

---

# Enums 


 ### <a name="EnumDocumented">**EnumDocumented**</a>

Documented enum.

#### Variants 
- **A** -  Variant A. 
- **B** -  Variant B. 
- **C** -  Variant B. 
#### Definition 
```python
class EnumDocumented:
    A = 0
    B = 1
    C = 2
```

---



 ### <a name="EnumRenamed">**EnumRenamed**</a>


#### Variants 
- **X** -  
#### Definition 
```python
class EnumRenamed:
    X = 0
```

---

# Functions
## primitive_void 
#### Definition 
```python
def primitive_void():
    ...
```

---

## primitive_void2 
#### Definition 
```python
def primitive_void2():
    ...
```

---

## primitive_bool 
#### Definition 
```python
def primitive_bool(x: bool) -> bool:
    ...
```

---

## primitive_u8 
#### Definition 
```python
def primitive_u8(x: int) -> int:
    ...
```

---

## primitive_u16 
#### Definition 
```python
def primitive_u16(x: int) -> int:
    ...
```

---

## primitive_u32 
#### Definition 
```python
def primitive_u32(x: int) -> int:
    ...
```

---

## primitive_u64 
#### Definition 
```python
def primitive_u64(x: int) -> int:
    ...
```

---

## primitive_i8 
#### Definition 
```python
def primitive_i8(x: int) -> int:
    ...
```

---

## primitive_i16 
#### Definition 
```python
def primitive_i16(x: int) -> int:
    ...
```

---

## primitive_i32 
#### Definition 
```python
def primitive_i32(x: int) -> int:
    ...
```

---

## primitive_i64 
#### Definition 
```python
def primitive_i64(x: int) -> int:
    ...
```

---

## many_args_5 
#### Definition 
```python
def many_args_5(x0: int, x1: int, x2: int, x3: int, x4: int) -> int:
    ...
```

---

## many_args_10 
#### Definition 
```python
def many_args_10(x0: int, x1: int, x2: int, x3: int, x4: int, x5: int, x6: int, x7: int, x8: int, x9: int) -> int:
    ...
```

---

## ptr 
#### Definition 
```python
def ptr(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    ...
```

---

## ptr_mut 
### Safety

Parameter x must point to valid data.
#### Definition 
```python
def ptr_mut(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    ...
```

---

## ptr_ptr 
#### Definition 
```python
def ptr_ptr(x: ctypes.POINTER(ctypes.POINTER(ctypes.c_int64))) -> ctypes.POINTER(ctypes.POINTER(ctypes.c_int64)):
    ...
```

---

## ref_simple 
#### Definition 
```python
def ref_simple(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    ...
```

---

## ref_mut_simple 
#### Definition 
```python
def ref_mut_simple(x: ctypes.POINTER(ctypes.c_int64)) -> ctypes.POINTER(ctypes.c_int64):
    ...
```

---

## ref_option 
#### Definition 
```python
def ref_option(x: ctypes.POINTER(ctypes.c_int64)) -> bool:
    ...
```

---

## ref_mut_option 
#### Definition 
```python
def ref_mut_option(x: ctypes.POINTER(ctypes.c_int64)) -> bool:
    ...
```

---

## tupled 
#### Definition 
```python
def tupled(x: Tupled) -> Tupled:
    ...
```

---

## complex_args_1 
#### Definition 
```python
def complex_args_1(a: Vec3f32, b: ctypes.POINTER(Tupled)):
    ...
```

---

## complex_args_2 
#### Definition 
```python
def complex_args_2(cmplx: SomeForeignType) -> ctypes.c_void_p:
    ...
```

---

## callback 
#### Definition 
```python
def callback(callback, value: int) -> int:
    ...
```

---

## generic_1a 
#### Definition 
```python
def generic_1a(x: Genericu32, y: Phantomu8) -> int:
    ...
```

---

## generic_1b 
#### Definition 
```python
def generic_1b(x: Genericu8, y: Phantomu8) -> int:
    ...
```

---

## generic_1c 
#### Definition 
```python
def generic_1c(x: ctypes.POINTER(Genericu8), y: ctypes.POINTER(Genericu8)) -> int:
    ...
```

---

## generic_2 
#### Definition 
```python
def generic_2(x: ctypes.c_void_p) -> int:
    ...
```

---

## generic_3 
#### Definition 
```python
def generic_3(x: ctypes.c_void_p) -> int:
    ...
```

---

## generic_4 
#### Definition 
```python
def generic_4(x: ctypes.c_void_p) -> int:
    ...
```

---

## array_1 
#### Definition 
```python
def array_1(x: Array) -> int:
    ...
```

---

## documented 
This function has documentation.
#### Definition 
```python
def documented(x: StructDocumented) -> ctypes.c_int:
    ...
```

---

## ambiguous_1 
#### Definition 
```python
def ambiguous_1(x: Vec1) -> Vec1:
    ...
```

---

## ambiguous_2 
#### Definition 
```python
def ambiguous_2(x: Vec2) -> Vec2:
    ...
```

---

## ambiguous_3 
#### Definition 
```python
def ambiguous_3(x: Vec1, y: Vec2) -> bool:
    ...
```

---

## namespaced_type 
#### Definition 
```python
def namespaced_type(x: Vec) -> Vec:
    ...
```

---

## namespaced_inner_option 
#### Definition 
```python
def namespaced_inner_option(x: OptionVec) -> OptionVec:
    ...
```

---

## namespaced_inner_slice 
#### Definition 
```python
def namespaced_inner_slice(x: SliceVec | ctypes.Array[Vec]) -> SliceVec:
    ...
```

---

## namespaced_inner_slice_mut 
#### Definition 
```python
def namespaced_inner_slice_mut(x: SliceMutVec | ctypes.Array[Vec]) -> SliceMutVec:
    ...
```

---

## panics 
#### Definition 
```python
def panics():
    ...
```

---

## renamed 
#### Definition 
```python
def renamed(x: StructRenamed) -> ctypes.c_int:
    ...
```

---

## sleep 
#### Definition 
```python
def sleep(millis: int):
    ...
```

---

## weird_1 
#### Definition 
```python
def weird_1(x: Weird1u32, y: Weird2u8) -> bool:
    ...
```

---

## visibility 
#### Definition 
```python
def visibility(x: Visibility1, y: Visibility2):
    ...
```

---

## repr_transparent 
#### Definition 
```python
def repr_transparent(x: Tupled, r: ctypes.POINTER(Tupled)) -> Tupled:
    ...
```

---

## pattern_ascii_pointer_1 
#### Definition 
```python
def pattern_ascii_pointer_1(x: str) -> int:
    ...
```

---

## pattern_ascii_pointer_2 
#### Definition 
```python
def pattern_ascii_pointer_2() -> str:
    ...
```

---

## pattern_ascii_pointer_len 
#### Definition 
```python
def pattern_ascii_pointer_len(x: str, y: UseAsciiStringPattern) -> int:
    ...
```

---

## pattern_ascii_pointer_return_slice 
#### Definition 
```python
def pattern_ascii_pointer_return_slice() -> SliceUseAsciiStringPattern:
    ...
```

---

## pattern_ffi_slice_1 
#### Definition 
```python
def pattern_ffi_slice_1(ffi_slice: Sliceu32 | ctypes.Array[ctypes.c_uint32]) -> int:
    ...
```

---

## pattern_ffi_slice_2 
#### Definition 
```python
def pattern_ffi_slice_2(ffi_slice: SliceVec3f32 | ctypes.Array[Vec3f32], i: int) -> Vec3f32:
    ...
```

---

## pattern_ffi_slice_3 
#### Definition 
```python
def pattern_ffi_slice_3(slice: SliceMutu8 | ctypes.Array[ctypes.c_uint8], callback):
    ...
```

---

## pattern_ffi_slice_4 
#### Definition 
```python
def pattern_ffi_slice_4(slice: Sliceu8 | ctypes.Array[ctypes.c_uint8], slice2: SliceMutu8 | ctypes.Array[ctypes.c_uint8]):
    ...
```

---

## pattern_ffi_slice_5 
#### Definition 
```python
def pattern_ffi_slice_5(slice: ctypes.POINTER(Sliceu8), slice2: ctypes.POINTER(SliceMutu8)):
    ...
```

---

## pattern_ffi_slice_6 
#### Definition 
```python
def pattern_ffi_slice_6(slice: ctypes.POINTER(SliceMutu8), callback):
    ...
```

---

## pattern_ffi_slice_delegate 
#### Definition 
```python
def pattern_ffi_slice_delegate(callback) -> int:
    ...
```

---

## pattern_ffi_slice_delegate_huge 
#### Definition 
```python
def pattern_ffi_slice_delegate_huge(callback) -> Vec3f32:
    ...
```

---

## pattern_ffi_option_1 
#### Definition 
```python
def pattern_ffi_option_1(ffi_slice: OptionInner) -> OptionInner:
    ...
```

---

## pattern_ffi_option_2 
#### Definition 
```python
def pattern_ffi_option_2(ffi_slice: OptionInner) -> Inner:
    ...
```

---

## pattern_ffi_bool 
#### Definition 
```python
def pattern_ffi_bool(ffi_bool):
    ...
```

---

## pattern_ffi_cchar 
#### Definition 
```python
def pattern_ffi_cchar(ffi_cchar: ctypes.c_char) -> ctypes.c_char:
    ...
```

---

## pattern_ffi_cchar_const_pointer 
#### Definition 
```python
def pattern_ffi_cchar_const_pointer(ffi_cchar: ctypes.POINTER(ctypes.c_char)) -> ctypes.POINTER(ctypes.c_char):
    ...
```

---

## pattern_ffi_cchar_mut_pointer 
#### Definition 
```python
def pattern_ffi_cchar_mut_pointer(ffi_cchar: ctypes.POINTER(ctypes.c_char)) -> ctypes.POINTER(ctypes.c_char):
    ...
```

---

## pattern_api_guard 
#### Definition 
```python
def pattern_api_guard():
    ...
```

---

## pattern_callback_1 
#### Definition 
```python
def pattern_callback_1(callback, x: int) -> int:
    ...
```

---

## pattern_callback_2 
#### Definition 
```python
def pattern_callback_2(callback):
    ...
```

---

# Services
## <a name="SimpleService">**SimpleService**</a> <sup>ctor</sup>
 Some struct we want to expose as a class.
### <a name="SimpleService.new_with">**new_with**</a> <sup>ctor</sup>
 The constructor must return a `Result<Self, Error>`.

#### Definition 
```python
class SimpleService:

    @staticmethod
    def new_with(some_value: int) -> SimpleService:
        ...
```

---

### <a name="SimpleService.new_without">**new_without**</a> <sup>ctor</sup>

#### Definition 
```python
class SimpleService:

    @staticmethod
    def new_without() -> SimpleService:
        ...
```

---

### <a name="SimpleService.new_with_string">**new_with_string**</a> <sup>ctor</sup>

#### Definition 
```python
class SimpleService:

    @staticmethod
    def new_with_string(ascii: str) -> SimpleService:
        ...
```

---

### <a name="SimpleService.new_failing">**new_failing**</a> <sup>ctor</sup>

#### Definition 
```python
class SimpleService:

    @staticmethod
    def new_failing(some_value: int) -> SimpleService:
        ...
```

---

### <a name="SimpleService.method_result">**method_result**</a>
 Methods returning a Result<(), _> are the default and do not
 need annotations.

#### Definition 
```python
class SimpleService:

    def method_result(self, anon1: int):
        ...
```

---

### <a name="SimpleService.method_value">**method_value**</a>

#### Definition 
```python
class SimpleService:

    def method_value(self, x: int) -> int:
        ...
```

---

### <a name="SimpleService.method_void">**method_void**</a>
 This method should be documented.

 Multiple lines.

#### Definition 
```python
class SimpleService:

    def method_void(self, ):
        ...
```

---

### <a name="SimpleService.method_mut_self">**method_mut_self**</a>

#### Definition 
```python
class SimpleService:

    def method_mut_self(self, slice: Sliceu8 | ctypes.Array[ctypes.c_uint8]) -> int:
        ...
```

---

### <a name="SimpleService.method_mut_self_void">**method_mut_self_void**</a>
 Single line.

#### Definition 
```python
class SimpleService:

    def method_mut_self_void(self, slice: SliceBool | ctypes.Array[ctypes.c_uint8]):
        ...
```

---

### <a name="SimpleService.method_mut_self_ref">**method_mut_self_ref**</a>

#### Definition 
```python
class SimpleService:

    def method_mut_self_ref(self, x: ctypes.POINTER(ctypes.c_uint8), y: ctypes.POINTER(ctypes.c_uint8)) -> int:
        ...
```

---

### <a name="SimpleService.method_mut_self_ref_slice">**method_mut_self_ref_slice**</a>

#### Definition 
```python
class SimpleService:

    def method_mut_self_ref_slice(self, x: ctypes.POINTER(ctypes.c_uint8), y: ctypes.POINTER(ctypes.c_uint8), slice: Sliceu8 | ctypes.Array[ctypes.c_uint8]) -> int:
        ...
```

---

### <a name="SimpleService.method_mut_self_ref_slice_limited">**method_mut_self_ref_slice_limited**</a>

#### Definition 
```python
class SimpleService:

    def method_mut_self_ref_slice_limited(self, x: ctypes.POINTER(ctypes.c_uint8), y: ctypes.POINTER(ctypes.c_uint8), slice: Sliceu8 | ctypes.Array[ctypes.c_uint8], slice2: Sliceu8 | ctypes.Array[ctypes.c_uint8]) -> int:
        ...
```

---

### <a name="SimpleService.method_mut_self_ffi_error">**method_mut_self_ffi_error**</a>

#### Definition 
```python
class SimpleService:

    def method_mut_self_ffi_error(self, slice: SliceMutu8 | ctypes.Array[ctypes.c_uint8]):
        ...
```

---

### <a name="SimpleService.method_mut_self_no_error">**method_mut_self_no_error**</a>

#### Definition 
```python
class SimpleService:

    def method_mut_self_no_error(self, slice: SliceMutu8 | ctypes.Array[ctypes.c_uint8]):
        ...
```

---

### <a name="SimpleService.return_slice">**return_slice**</a>
 Warning, you _must_ discard the returned slice object before calling into this service
 again, as otherwise undefined behavior might happen.

#### Definition 
```python
class SimpleService:

    def return_slice(self, ) -> Sliceu32:
        ...
```

---

### <a name="SimpleService.return_slice_mut">**return_slice_mut**</a>
 Warning, you _must_ discard the returned slice object before calling into this service
 again, as otherwise undefined behavior might happen.

#### Definition 
```python
class SimpleService:

    def return_slice_mut(self, ) -> SliceMutu32:
        ...
```

---

### <a name="SimpleService.return_string">**return_string**</a>
 This function has no panic safeguards. If it panics your host app will be in an undefined state.

#### Definition 
```python
class SimpleService:

    def return_string(self, ) -> str:
        ...
```

---

### <a name="SimpleService.method_void_ffi_error">**method_void_ffi_error**</a>

#### Definition 
```python
class SimpleService:

    def method_void_ffi_error(self, ):
        ...
```

---

### <a name="SimpleService.method_callback">**method_callback**</a>

#### Definition 
```python
class SimpleService:

    def method_callback(self, callback):
        ...
```

---



## <a name="SimpleServiceLifetime">**SimpleServiceLifetime**</a> <sup>ctor</sup>
### <a name="SimpleServiceLifetime.new_with">**new_with**</a> <sup>ctor</sup>

#### Definition 
```python
class SimpleServiceLifetime:

    @staticmethod
    def new_with(some_value: ctypes.POINTER(ctypes.c_uint32)) -> SimpleServiceLifetime:
        ...
```

---

### <a name="SimpleServiceLifetime.method_lt">**method_lt**</a>

#### Definition 
```python
class SimpleServiceLifetime:

    def method_lt(self, slice: SliceBool | ctypes.Array[ctypes.c_uint8]):
        ...
```

---

### <a name="SimpleServiceLifetime.method_lt2">**method_lt2**</a>

#### Definition 
```python
class SimpleServiceLifetime:

    def method_lt2(self, slice: SliceBool | ctypes.Array[ctypes.c_uint8]):
        ...
```

---

### <a name="SimpleServiceLifetime.return_string_accept_slice">**return_string_accept_slice**</a>

#### Definition 
```python
class SimpleServiceLifetime:

    def return_string_accept_slice(self, anon1: Sliceu8 | ctypes.Array[ctypes.c_uint8]) -> str:
        ...
```

---

### <a name="SimpleServiceLifetime.method_void_ffi_error">**method_void_ffi_error**</a>

#### Definition 
```python
class SimpleServiceLifetime:

    def method_void_ffi_error(self, ):
        ...
```

---




