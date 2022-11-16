
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
 - **[pattern_ffi_slice_1b](#pattern_ffi_slice_1b)** - 
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
     - **[NewWith](#SimpleService.NewWith)** <sup>**ctor**</sup> -  The constructor must return a `Result<Self, Error>`.
     - **[NewWithout](#SimpleService.NewWithout)** <sup>**ctor**</sup> - 
     - **[NewWithString](#SimpleService.NewWithString)** <sup>**ctor**</sup> - 
     - **[NewFailing](#SimpleService.NewFailing)** <sup>**ctor**</sup> - 
     - **[MethodResult](#SimpleService.MethodResult)** -  Methods returning a Result<(), _> are the default and do not
     - **[MethodValue](#SimpleService.MethodValue)** - 
     - **[MethodVoid](#SimpleService.MethodVoid)** -  This method should be documented.
     - **[MethodMutSelf](#SimpleService.MethodMutSelf)** - 
     - **[MethodMutSelfVoid](#SimpleService.MethodMutSelfVoid)** -  Single line.
     - **[MethodMutSelfRef](#SimpleService.MethodMutSelfRef)** - 
     - **[MethodMutSelfRefSlice](#SimpleService.MethodMutSelfRefSlice)** - 
     - **[MethodMutSelfRefSliceLimited](#SimpleService.MethodMutSelfRefSliceLimited)** - 
     - **[MethodMutSelfFfiError](#SimpleService.MethodMutSelfFfiError)** - 
     - **[MethodMutSelfNoError](#SimpleService.MethodMutSelfNoError)** - 
     - **[ReturnSlice](#SimpleService.ReturnSlice)** -  Warning, you _must_ discard the returned slice object before calling into this service
     - **[ReturnSliceMut](#SimpleService.ReturnSliceMut)** -  Warning, you _must_ discard the returned slice object before calling into this service
     - **[ReturnString](#SimpleService.ReturnString)** -  This function has no panic safeguards. If it panics your host app will be in an undefined state.
     - **[MethodVoidFfiError](#SimpleService.MethodVoidFfiError)** - 
     - **[MethodCallback](#SimpleService.MethodCallback)** - 
 - **[SimpleServiceLifetime](#SimpleServiceLifetime)** - 
     - **[NewWith](#SimpleServiceLifetime.NewWith)** <sup>**ctor**</sup> - 
     - **[MethodLt](#SimpleServiceLifetime.MethodLt)** - 
     - **[MethodLt2](#SimpleServiceLifetime.MethodLt2)** - 
     - **[ReturnStringAcceptSlice](#SimpleServiceLifetime.ReturnStringAcceptSlice)** - 
     - **[MethodVoidFfiError](#SimpleServiceLifetime.MethodVoidFfiError)** - 

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

---

# Types 


 ### <a name="Array">**Array**</a>


#### Fields 
- **data** -  
#### Definition 
```csharp
public partial struct Array
{
    public byte data0;
    public byte data1;
    public byte data2;
    public byte data3;
    public byte data4;
    public byte data5;
    public byte data6;
    public byte data7;
    public byte data8;
    public byte data9;
    public byte data10;
    public byte data11;
    public byte data12;
    public byte data13;
    public byte data14;
    public byte data15;
}
```

---



 ### <a name="ExtraTypef32">**ExtraTypef32**</a>


#### Fields 
- **x** -  
#### Definition 
```csharp
public partial struct ExtraTypef32
{
    public float x;
}
```

---



 ### <a name="Genericu32">**Genericu32**</a>


#### Fields 
- **x** -  
#### Definition 
```csharp
public partial struct Genericu32
{
    public IntPtr x;
}
```

---



 ### <a name="Genericu8">**Genericu8**</a>


#### Fields 
- **x** -  
#### Definition 
```csharp
public partial struct Genericu8
{
    public IntPtr x;
}
```

---



 ### <a name="Inner">**Inner**</a>


#### Fields 
- **x** -  
#### Definition 
```csharp
public partial struct Inner
{
    float x;
}
```

---



 ### <a name="Phantomu8">**Phantomu8**</a>


#### Fields 
- **x** -  
#### Definition 
```csharp
public partial struct Phantomu8
{
    public uint x;
}
```

---



 ### <a name="SomeForeignType">**SomeForeignType**</a>


#### Fields 
- **x** -  
#### Definition 
```csharp
public partial struct SomeForeignType
{
    public uint x;
}
```

---



 ### <a name="StructDocumented">**StructDocumented**</a>

Documented struct.

#### Fields 
- **x** -  Documented field. 
#### Definition 
```csharp
public partial struct StructDocumented
{
    public float x;
}
```

---



 ### <a name="StructRenamed">**StructRenamed**</a>


#### Fields 
- **e** -  
#### Definition 
```csharp
public partial struct StructRenamed
{
    public EnumRenamed e;
}
```

---



 ### <a name="Tupled">**Tupled**</a>


#### Fields 
- **x0** -  
#### Definition 
```csharp
public partial struct Tupled
{
    public byte x0;
}
```

---



 ### <a name="UseAsciiStringPattern">**UseAsciiStringPattern**</a>


#### Fields 
- **ascii_string** -  
#### Definition 
```csharp
public partial struct UseAsciiStringPattern
{
    public string ascii_string;
}
```

---



 ### <a name="Vec">**Vec**</a>


#### Fields 
- **x** -  
- **z** -  
#### Definition 
```csharp
public partial struct Vec
{
    public double x;
    public double z;
}
```

---



 ### <a name="Vec1">**Vec1**</a>


#### Fields 
- **x** -  
- **y** -  
#### Definition 
```csharp
public partial struct Vec1
{
    public float x;
    public float y;
}
```

---



 ### <a name="Vec2">**Vec2**</a>


#### Fields 
- **x** -  
- **z** -  
#### Definition 
```csharp
public partial struct Vec2
{
    public double x;
    public double z;
}
```

---



 ### <a name="Vec3f32">**Vec3f32**</a>


#### Fields 
- **x** -  
- **y** -  
- **z** -  
#### Definition 
```csharp
public partial struct Vec3f32
{
    public float x;
    public float y;
    public float z;
}
```

---



 ### <a name="Visibility1">**Visibility1**</a>


#### Fields 
- **pblc** -  
- **prvt** -  
#### Definition 
```csharp
public partial struct Visibility1
{
    public byte pblc;
    byte prvt;
}
```

---



 ### <a name="Visibility2">**Visibility2**</a>


#### Fields 
- **pblc1** -  
- **pblc2** -  
#### Definition 
```csharp
public partial struct Visibility2
{
    public byte pblc1;
    public byte pblc2;
}
```

---



 ### <a name="Weird1u32">**Weird1u32**</a>


#### Fields 
- **x** -  
#### Definition 
```csharp
public partial struct Weird1u32
{
    uint x;
}
```

---



 ### <a name="Weird2u8">**Weird2u8**</a>


#### Fields 
- **t** -  
- **a** -  
- **r** -  
#### Definition 
```csharp
public partial struct Weird2u8
{
    byte t;
    byte a0;
    byte a1;
    byte a2;
    byte a3;
    byte a4;
    IntPtr r;
}
```

---



 ### <a name="SliceBool">**SliceBool**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```csharp
public partial struct SliceBool
{
    IntPtr data;
    ulong len;
}
```

---



 ### <a name="SliceUseAsciiStringPattern">**SliceUseAsciiStringPattern**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```csharp
public partial struct SliceUseAsciiStringPattern
{
    IntPtr data;
    ulong len;
}
```

---



 ### <a name="SliceVec">**SliceVec**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```csharp
public partial struct SliceVec
{
    IntPtr data;
    ulong len;
}
```

---



 ### <a name="SliceVec3f32">**SliceVec3f32**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```csharp
public partial struct SliceVec3f32
{
    IntPtr data;
    ulong len;
}
```

---



 ### <a name="Sliceu32">**Sliceu32**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```csharp
public partial struct Sliceu32
{
    IntPtr data;
    ulong len;
}
```

---



 ### <a name="Sliceu8">**Sliceu8**</a>

A pointer to an array of data someone else owns which may not be modified.

#### Fields 
- **data** - Pointer to start of immutable data. 
- **len** - Number of elements. 
#### Definition 
```csharp
public partial struct Sliceu8
{
    IntPtr data;
    ulong len;
}
```

---



 ### <a name="OptionInner">**OptionInner**</a>

Option type containing boolean flag and maybe valid data.

#### Fields 
- **t** - Element that is maybe valid. 
- **is_some** - Byte where `1` means element `t` is valid. 
#### Definition 
```csharp
public partial struct OptionInner
{
    Inner t;
    byte is_some;
}
```

---



 ### <a name="OptionVec">**OptionVec**</a>

Option type containing boolean flag and maybe valid data.

#### Fields 
- **t** - Element that is maybe valid. 
- **is_some** - Byte where `1` means element `t` is valid. 
#### Definition 
```csharp
public partial struct OptionVec
{
    Vec t;
    byte is_some;
}
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
```csharp
public enum EnumDocumented
{
    A = 0,
    B = 1,
    C = 2,
}
```

---



 ### <a name="EnumRenamed">**EnumRenamed**</a>


#### Variants 
- **X** -  
#### Definition 
```csharp
public enum EnumRenamed
{
    X = 0,
}
```

---

# Functions
### <a name="primitive_void">**primitive_void**</a>
#### Definition 
```csharp
public static extern void primitive_void();
```

---

### <a name="primitive_void2">**primitive_void2**</a>
#### Definition 
```csharp
public static extern void primitive_void2();
```

---

### <a name="primitive_bool">**primitive_bool**</a>
#### Definition 
```csharp
public static extern bool primitive_bool(bool x);
```

---

### <a name="primitive_u8">**primitive_u8**</a>
#### Definition 
```csharp
public static extern byte primitive_u8(byte x);
```

---

### <a name="primitive_u16">**primitive_u16**</a>
#### Definition 
```csharp
public static extern ushort primitive_u16(ushort x);
```

---

### <a name="primitive_u32">**primitive_u32**</a>
#### Definition 
```csharp
public static extern uint primitive_u32(uint x);
```

---

### <a name="primitive_u64">**primitive_u64**</a>
#### Definition 
```csharp
public static extern ulong primitive_u64(ulong x);
```

---

### <a name="primitive_i8">**primitive_i8**</a>
#### Definition 
```csharp
public static extern sbyte primitive_i8(sbyte x);
```

---

### <a name="primitive_i16">**primitive_i16**</a>
#### Definition 
```csharp
public static extern short primitive_i16(short x);
```

---

### <a name="primitive_i32">**primitive_i32**</a>
#### Definition 
```csharp
public static extern int primitive_i32(int x);
```

---

### <a name="primitive_i64">**primitive_i64**</a>
#### Definition 
```csharp
public static extern long primitive_i64(long x);
```

---

### <a name="many_args_5">**many_args_5**</a>
#### Definition 
```csharp
public static extern long many_args_5(long x0, long x1, long x2, long x3, long x4);
```

---

### <a name="many_args_10">**many_args_10**</a>
#### Definition 
```csharp
public static extern long many_args_10(long x0, long x1, long x2, long x3, long x4, long x5, long x6, long x7, long x8, long x9);
```

---

### <a name="ptr">**ptr**</a>
#### Definition 
```csharp
public static extern IntPtr ptr(ref long x);
```

---

### <a name="ptr_mut">**ptr_mut**</a>
### Safety

Parameter x must point to valid data.
#### Definition 
```csharp
public static extern IntPtr ptr_mut(out long x);
```

---

### <a name="ptr_ptr">**ptr_ptr**</a>
#### Definition 
```csharp
public static extern IntPtr ptr_ptr(ref IntPtr x);
```

---

### <a name="ref_simple">**ref_simple**</a>
#### Definition 
```csharp
public static extern IntPtr ref_simple(ref long x);
```

---

### <a name="ref_mut_simple">**ref_mut_simple**</a>
#### Definition 
```csharp
public static extern IntPtr ref_mut_simple(out long x);
```

---

### <a name="ref_option">**ref_option**</a>
#### Definition 
```csharp
public static extern bool ref_option(ref long x);
```

---

### <a name="ref_mut_option">**ref_mut_option**</a>
#### Definition 
```csharp
public static extern bool ref_mut_option(out long x);
```

---

### <a name="tupled">**tupled**</a>
#### Definition 
```csharp
public static extern Tupled tupled(Tupled x);
```

---

### <a name="complex_args_1">**complex_args_1**</a>
#### Definition 
```csharp
public static extern FFIError complex_args_1(Vec3f32 a, ref Tupled b);
public static void complex_args_1_checked(Vec3f32 a, ref Tupled b);
```

---

### <a name="complex_args_2">**complex_args_2**</a>
#### Definition 
```csharp
public static extern IntPtr complex_args_2(SomeForeignType cmplx);
```

---

### <a name="callback">**callback**</a>
#### Definition 
```csharp
public static extern byte callback(InteropDelegate_fn_u8_rval_u8 callback, byte value);
public static extern byte callback(IntPtr callback, byte value);
```

---

### <a name="generic_1a">**generic_1a**</a>
#### Definition 
```csharp
public static extern uint generic_1a(Genericu32 x, Phantomu8 y);
```

---

### <a name="generic_1b">**generic_1b**</a>
#### Definition 
```csharp
public static extern byte generic_1b(Genericu8 x, Phantomu8 y);
```

---

### <a name="generic_1c">**generic_1c**</a>
#### Definition 
```csharp
public static extern byte generic_1c(ref Genericu8 x, ref Genericu8 y);
```

---

### <a name="generic_2">**generic_2**</a>
#### Definition 
```csharp
public static extern byte generic_2(IntPtr x);
```

---

### <a name="generic_3">**generic_3**</a>
#### Definition 
```csharp
public static extern byte generic_3(IntPtr x);
```

---

### <a name="generic_4">**generic_4**</a>
#### Definition 
```csharp
public static extern byte generic_4(IntPtr x);
```

---

### <a name="array_1">**array_1**</a>
#### Definition 
```csharp
public static extern byte array_1(Array x);
```

---

### <a name="documented">**documented**</a>
This function has documentation.
#### Definition 
```csharp
public static extern EnumDocumented documented(StructDocumented x);
```

---

### <a name="ambiguous_1">**ambiguous_1**</a>
#### Definition 
```csharp
public static extern Vec1 ambiguous_1(Vec1 x);
```

---

### <a name="ambiguous_2">**ambiguous_2**</a>
#### Definition 
```csharp
public static extern Vec2 ambiguous_2(Vec2 x);
```

---

### <a name="ambiguous_3">**ambiguous_3**</a>
#### Definition 
```csharp
public static extern bool ambiguous_3(Vec1 x, Vec2 y);
```

---

### <a name="namespaced_type">**namespaced_type**</a>
#### Definition 
```csharp
public static extern Vec namespaced_type(Vec x);
```

---

### <a name="namespaced_inner_option">**namespaced_inner_option**</a>
#### Definition 
```csharp
public static extern OptionVec namespaced_inner_option(OptionVec x);
```

---

### <a name="namespaced_inner_slice">**namespaced_inner_slice**</a>
#### Definition 
```csharp
public static extern SliceVec namespaced_inner_slice(SliceVec x);
public static SliceVec namespaced_inner_slice(Vec[] x);
#if UNITY_2018_1_OR_NEWER
public static SliceVec namespaced_inner_slice(NativeArray<Vec> x);
#endif
```

---

### <a name="namespaced_inner_slice_mut">**namespaced_inner_slice_mut**</a>
#### Definition 
```csharp
public static extern SliceMutVec namespaced_inner_slice_mut(SliceMutVec x);
public static SliceMutVec namespaced_inner_slice_mut(Vec[] x);
#if UNITY_2018_1_OR_NEWER
public static SliceMutVec namespaced_inner_slice_mut(NativeArray<Vec> x);
#endif
```

---

### <a name="panics">**panics**</a>
#### Definition 
```csharp
public static extern FFIError panics();
public static void panics_checked();
```

---

### <a name="renamed">**renamed**</a>
#### Definition 
```csharp
public static extern EnumRenamed renamed(StructRenamed x);
```

---

### <a name="sleep">**sleep**</a>
#### Definition 
```csharp
public static extern void sleep(ulong millis);
```

---

### <a name="weird_1">**weird_1**</a>
#### Definition 
```csharp
public static extern bool weird_1(Weird1u32 x, Weird2u8 y);
```

---

### <a name="visibility">**visibility**</a>
#### Definition 
```csharp
public static extern void visibility(Visibility1 x, Visibility2 y);
```

---

### <a name="repr_transparent">**repr_transparent**</a>
#### Definition 
```csharp
public static extern Tupled repr_transparent(Tupled x, ref Tupled r);
```

---

### <a name="pattern_ascii_pointer_1">**pattern_ascii_pointer_1**</a>
#### Definition 
```csharp
public static extern uint pattern_ascii_pointer_1(string x);
```

---

### <a name="pattern_ascii_pointer_2">**pattern_ascii_pointer_2**</a>
#### Definition 
```csharp
public static extern IntPtr pattern_ascii_pointer_2();
```

---

### <a name="pattern_ascii_pointer_len">**pattern_ascii_pointer_len**</a>
#### Definition 
```csharp
public static extern uint pattern_ascii_pointer_len(string x, UseAsciiStringPattern y);
```

---

### <a name="pattern_ascii_pointer_return_slice">**pattern_ascii_pointer_return_slice**</a>
#### Definition 
```csharp
public static extern SliceUseAsciiStringPattern pattern_ascii_pointer_return_slice();
```

---

### <a name="pattern_ffi_slice_1">**pattern_ffi_slice_1**</a>
#### Definition 
```csharp
public static extern uint pattern_ffi_slice_1(Sliceu32 ffi_slice);
public static uint pattern_ffi_slice_1(uint[] ffi_slice);
#if UNITY_2018_1_OR_NEWER
public static uint pattern_ffi_slice_1(NativeArray<uint> ffi_slice);
#endif
```

---

### <a name="pattern_ffi_slice_1b">**pattern_ffi_slice_1b**</a>
#### Definition 
```csharp
public static extern uint pattern_ffi_slice_1b(SliceMutu32 ffi_slice);
public static uint pattern_ffi_slice_1b(uint[] ffi_slice);
#if UNITY_2018_1_OR_NEWER
public static uint pattern_ffi_slice_1b(NativeArray<uint> ffi_slice);
#endif
```

---

### <a name="pattern_ffi_slice_2">**pattern_ffi_slice_2**</a>
#### Definition 
```csharp
public static extern Vec3f32 pattern_ffi_slice_2(SliceVec3f32 ffi_slice, int i);
public static Vec3f32 pattern_ffi_slice_2(Vec3f32[] ffi_slice, int i);
#if UNITY_2018_1_OR_NEWER
public static Vec3f32 pattern_ffi_slice_2(NativeArray<Vec3f32> ffi_slice, int i);
#endif
```

---

### <a name="pattern_ffi_slice_3">**pattern_ffi_slice_3**</a>
#### Definition 
```csharp
public static extern void pattern_ffi_slice_3(SliceMutu8 slice, CallbackSliceMut callback);
public static void pattern_ffi_slice_3(byte[] slice, CallbackSliceMut callback);
public static extern void pattern_ffi_slice_3(SliceMutu8 slice, IntPtr callback);
#if UNITY_2018_1_OR_NEWER
public static void pattern_ffi_slice_3(NativeArray<byte> slice, IntPtr callback);
#endif
```

---

### <a name="pattern_ffi_slice_4">**pattern_ffi_slice_4**</a>
#### Definition 
```csharp
public static extern void pattern_ffi_slice_4(Sliceu8 slice, SliceMutu8 slice2);
public static void pattern_ffi_slice_4(byte[] slice, byte[] slice2);
#if UNITY_2018_1_OR_NEWER
public static void pattern_ffi_slice_4(NativeArray<byte> slice, NativeArray<byte> slice2);
#endif
```

---

### <a name="pattern_ffi_slice_5">**pattern_ffi_slice_5**</a>
#### Definition 
```csharp
public static extern void pattern_ffi_slice_5(ref Sliceu8 slice, ref SliceMutu8 slice2);
public static void pattern_ffi_slice_5(byte[] slice, byte[] slice2);
#if UNITY_2018_1_OR_NEWER
public static void pattern_ffi_slice_5(NativeArray<byte> slice, NativeArray<byte> slice2);
#endif
```

---

### <a name="pattern_ffi_slice_6">**pattern_ffi_slice_6**</a>
#### Definition 
```csharp
public static extern void pattern_ffi_slice_6(ref SliceMutu8 slice, CallbackU8 callback);
public static void pattern_ffi_slice_6(byte[] slice, CallbackU8 callback);
public static extern void pattern_ffi_slice_6(ref SliceMutu8 slice, IntPtr callback);
#if UNITY_2018_1_OR_NEWER
public static void pattern_ffi_slice_6(NativeArray<byte> slice, IntPtr callback);
#endif
```

---

### <a name="pattern_ffi_slice_delegate">**pattern_ffi_slice_delegate**</a>
#### Definition 
```csharp
public static extern byte pattern_ffi_slice_delegate(CallbackFFISlice callback);
public static extern byte pattern_ffi_slice_delegate(IntPtr callback);
```

---

### <a name="pattern_ffi_slice_delegate_huge">**pattern_ffi_slice_delegate_huge**</a>
#### Definition 
```csharp
public static extern Vec3f32 pattern_ffi_slice_delegate_huge(CallbackHugeVecSlice callback);
public static extern Vec3f32 pattern_ffi_slice_delegate_huge(IntPtr callback);
```

---

### <a name="pattern_ffi_option_1">**pattern_ffi_option_1**</a>
#### Definition 
```csharp
public static extern OptionInner pattern_ffi_option_1(OptionInner ffi_slice);
```

---

### <a name="pattern_ffi_option_2">**pattern_ffi_option_2**</a>
#### Definition 
```csharp
public static extern Inner pattern_ffi_option_2(OptionInner ffi_slice);
```

---

### <a name="pattern_ffi_bool">**pattern_ffi_bool**</a>
#### Definition 
```csharp
public static extern Bool pattern_ffi_bool(Bool ffi_bool);
```

---

### <a name="pattern_ffi_cchar">**pattern_ffi_cchar**</a>
#### Definition 
```csharp
public static extern sbyte pattern_ffi_cchar(sbyte ffi_cchar);
```

---

### <a name="pattern_ffi_cchar_const_pointer">**pattern_ffi_cchar_const_pointer**</a>
#### Definition 
```csharp
public static extern IntPtr pattern_ffi_cchar_const_pointer(IntPtr ffi_cchar);
```

---

### <a name="pattern_ffi_cchar_mut_pointer">**pattern_ffi_cchar_mut_pointer**</a>
#### Definition 
```csharp
public static extern IntPtr pattern_ffi_cchar_mut_pointer(IntPtr ffi_cchar);
```

---

### <a name="pattern_api_guard">**pattern_api_guard**</a>
#### Definition 
```csharp
public static extern ulong pattern_api_guard();
```

---

### <a name="pattern_callback_1">**pattern_callback_1**</a>
#### Definition 
```csharp
public static extern uint pattern_callback_1(MyCallback callback, uint x);
public static extern uint pattern_callback_1(IntPtr callback, uint x);
```

---

### <a name="pattern_callback_2">**pattern_callback_2**</a>
#### Definition 
```csharp
public static extern MyCallbackVoid pattern_callback_2(MyCallbackVoid callback);
public static extern MyCallbackVoid pattern_callback_2(IntPtr callback);
```

---

# Classes
## <a name="SimpleService">**SimpleService**</a>
 Some struct we want to expose as a class.
### <a name="NewWith">**NewWith**</a> <sup>ctor</sup>
 The constructor must return a `Result<Self, Error>`.

#### Definition 
```csharp
public SimpleService NewWith(uint some_value);
```

---

### <a name="NewWithout">**NewWithout**</a> <sup>ctor</sup>

#### Definition 
```csharp
public SimpleService NewWithout();
```

---

### <a name="NewWithString">**NewWithString**</a> <sup>ctor</sup>

#### Definition 
```csharp
public SimpleService NewWithString(string ascii);
```

---

### <a name="NewFailing">**NewFailing**</a> <sup>ctor</sup>

#### Definition 
```csharp
public SimpleService NewFailing(byte some_value);
```

---

### <a name="MethodResult">**MethodResult**</a>
 Methods returning a Result<(), _> are the default and do not
 need annotations.

#### Definition 
```csharp
public class SimpleService {
    public void MethodResult(uint anon1);
}
```

---

### <a name="MethodValue">**MethodValue**</a>

#### Definition 
```csharp
public class SimpleService {
    public uint MethodValue(uint x);
}
```

---

### <a name="MethodVoid">**MethodVoid**</a>
 This method should be documented.

 Multiple lines.

#### Definition 
```csharp
public class SimpleService {
    public void MethodVoid();
}
```

---

### <a name="MethodMutSelf">**MethodMutSelf**</a>

#### Definition 
```csharp
public class SimpleService {
    public byte MethodMutSelf(Sliceu8 slice);
    public byte MethodMutSelf(byte[] slice);
#if UNITY_2018_1_OR_NEWER
    public byte MethodMutSelf(NativeArray<byte> slice);
#endif
}
```

---

### <a name="MethodMutSelfVoid">**MethodMutSelfVoid**</a>
 Single line.

#### Definition 
```csharp
public class SimpleService {
    public void MethodMutSelfVoid(SliceBool slice);
    public void MethodMutSelfVoid(Bool[] slice);
#if UNITY_2018_1_OR_NEWER
    public void MethodMutSelfVoid(NativeArray<Bool> slice);
#endif
}
```

---

### <a name="MethodMutSelfRef">**MethodMutSelfRef**</a>

#### Definition 
```csharp
public class SimpleService {
    public byte MethodMutSelfRef(ref byte x, out byte y);
}
```

---

### <a name="MethodMutSelfRefSlice">**MethodMutSelfRefSlice**</a>

#### Definition 
```csharp
public class SimpleService {
    public byte MethodMutSelfRefSlice(ref byte x, out byte y, Sliceu8 slice);
    public byte MethodMutSelfRefSlice(ref byte x, out byte y, byte[] slice);
#if UNITY_2018_1_OR_NEWER
    public byte MethodMutSelfRefSlice(ref byte x, out byte y, NativeArray<byte> slice);
#endif
}
```

---

### <a name="MethodMutSelfRefSliceLimited">**MethodMutSelfRefSliceLimited**</a>

#### Definition 
```csharp
public class SimpleService {
    public byte MethodMutSelfRefSliceLimited(ref byte x, out byte y, Sliceu8 slice, Sliceu8 slice2);
    public byte MethodMutSelfRefSliceLimited(ref byte x, out byte y, byte[] slice, byte[] slice2);
#if UNITY_2018_1_OR_NEWER
    public byte MethodMutSelfRefSliceLimited(ref byte x, out byte y, NativeArray<byte> slice, NativeArray<byte> slice2);
#endif
}
```

---

### <a name="MethodMutSelfFfiError">**MethodMutSelfFfiError**</a>

#### Definition 
```csharp
public class SimpleService {
    public void MethodMutSelfFfiError(SliceMutu8 slice);
    public void MethodMutSelfFfiError(byte[] slice);
#if UNITY_2018_1_OR_NEWER
    public void MethodMutSelfFfiError(NativeArray<byte> slice);
#endif
}
```

---

### <a name="MethodMutSelfNoError">**MethodMutSelfNoError**</a>

#### Definition 
```csharp
public class SimpleService {
    public void MethodMutSelfNoError(SliceMutu8 slice);
    public void MethodMutSelfNoError(byte[] slice);
#if UNITY_2018_1_OR_NEWER
    public void MethodMutSelfNoError(NativeArray<byte> slice);
#endif
}
```

---

### <a name="ReturnSlice">**ReturnSlice**</a>
 Warning, you _must_ discard the returned slice object before calling into this service
 again, as otherwise undefined behavior might happen.

#### Definition 
```csharp
public class SimpleService {
    public Sliceu32 ReturnSlice();
}
```

---

### <a name="ReturnSliceMut">**ReturnSliceMut**</a>
 Warning, you _must_ discard the returned slice object before calling into this service
 again, as otherwise undefined behavior might happen.

#### Definition 
```csharp
public class SimpleService {
    public SliceMutu32 ReturnSliceMut();
}
```

---

### <a name="ReturnString">**ReturnString**</a>
 This function has no panic safeguards. If it panics your host app will be in an undefined state.

#### Definition 
```csharp
public class SimpleService {
    public string ReturnString();
}
```

---

### <a name="MethodVoidFfiError">**MethodVoidFfiError**</a>

#### Definition 
```csharp
public class SimpleService {
    public void MethodVoidFfiError();
}
```

---

### <a name="MethodCallback">**MethodCallback**</a>

#### Definition 
```csharp
public class SimpleService {
    public void MethodCallback(MyCallback callback);
#if UNITY_2018_1_OR_NEWER
    public void MethodCallback(IntPtr callback);
#endif
}
```

---



## <a name="SimpleServiceLifetime">**SimpleServiceLifetime**</a>
### <a name="NewWith">**NewWith**</a> <sup>ctor</sup>

#### Definition 
```csharp
public SimpleServiceLifetime NewWith(ref uint some_value);
```

---

### <a name="MethodLt">**MethodLt**</a>

#### Definition 
```csharp
public class SimpleServiceLifetime {
    public void MethodLt(SliceBool slice);
    public void MethodLt(Bool[] slice);
#if UNITY_2018_1_OR_NEWER
    public void MethodLt(NativeArray<Bool> slice);
#endif
}
```

---

### <a name="MethodLt2">**MethodLt2**</a>

#### Definition 
```csharp
public class SimpleServiceLifetime {
    public void MethodLt2(SliceBool slice);
    public void MethodLt2(Bool[] slice);
#if UNITY_2018_1_OR_NEWER
    public void MethodLt2(NativeArray<Bool> slice);
#endif
}
```

---

### <a name="ReturnStringAcceptSlice">**ReturnStringAcceptSlice**</a>

#### Definition 
```csharp
public class SimpleServiceLifetime {
    public string ReturnStringAcceptSlice(Sliceu8 anon1);
    public string ReturnStringAcceptSlice(byte[] anon1);
#if UNITY_2018_1_OR_NEWER
    public string ReturnStringAcceptSlice(NativeArray<byte> anon1);
#endif
}
```

---

### <a name="MethodVoidFfiError">**MethodVoidFfiError**</a>

#### Definition 
```csharp
public class SimpleServiceLifetime {
    public void MethodVoidFfiError();
}
```

---




