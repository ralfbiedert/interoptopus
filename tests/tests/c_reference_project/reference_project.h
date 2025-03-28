

#ifndef interoptopus_generated
#define interoptopus_generated

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>



const uint8_t U8 = 255;
const float F32_MIN_POSITIVE = 0.000000000000000000000000000000000000011754944;
const int32_t COMPUTED_I32 = -2147483647;

///  Documented enum.
typedef enum ENUMDOCUMENTED
    {
    ///  Variant A.
    ENUMDOCUMENTED_A = 0,
    ///  Variant B.
    ENUMDOCUMENTED_B = 1,
    ///  Variant B.
    ENUMDOCUMENTED_C = 2,
    } ENUMDOCUMENTED;

typedef enum ENUMPAYLOAD
    {
    ENUMPAYLOAD_A = 0,
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    } ENUMPAYLOAD;

typedef enum ENUMRENAMED
    {
    ENUMRENAMED_X = 0,
    } ENUMRENAMED;

typedef enum ERROR
    {
    ERROR_FAIL = 0,
    } ERROR;

typedef struct GENERIC2U8 GENERIC2U8;

typedef struct GENERIC3 GENERIC3;

typedef struct GENERIC4 GENERIC4;

typedef struct SERVICEASYNC SERVICEASYNC;

typedef struct SERVICEBASIC SERVICEBASIC;

///  Some struct we want to expose as a class.
typedef struct SERVICECALLBACKS SERVICECALLBACKS;

typedef struct SERVICEDEPENDENT SERVICEDEPENDENT;

typedef struct SERVICEIGNORINGMETHODS SERVICEIGNORINGMETHODS;

typedef struct SERVICEMAIN SERVICEMAIN;

///  Some struct we want to expose as a class.
typedef struct SERVICEMULTIPLECTORS SERVICEMULTIPLECTORS;

///  Some struct we want to expose as a class.
typedef struct SERVICEONPANIC SERVICEONPANIC;

typedef struct SERVICERESULT SERVICERESULT;

///  Some struct we want to expose as a class.
typedef struct SERVICESTRINGS SERVICESTRINGS;

///  Some struct we want to expose as a class.
typedef struct SERVICEVARIOUSSLICES SERVICEVARIOUSSLICES;

///  UTF-8 string marshalling helper.
///  A highly dangerous 'use once type' that has ownership semantics!
///  Once passed over an FFI boundary 'the other side' is meant to own
///  (and free) it. Rust handles that fine, but if in C# you put this
///  in a struct and then call Rust multiple times with that struct 
///  you'll free the same pointer multiple times, and get UB!
typedef struct UTF8STRING
    {
    uint8_t* ptr;
    uint64_t len;
    uint64_t capacity;
    } UTF8STRING;

typedef struct BOOLFIELD
    {
    bool val;
    } BOOLFIELD;

typedef struct EXTRATYPEF32
    {
    float x;
    } EXTRATYPEF32;

typedef struct INNER
    {
    float x;
    } INNER;

typedef struct LOCAL
    {
    uint32_t x;
    } LOCAL;

typedef struct PACKED1
    {
    uint8_t x;
    uint16_t y;
    } PACKED1;

typedef struct PACKED2
    {
    uint16_t y;
    uint8_t x;
    } PACKED2;

typedef struct PHANTOMU8
    {
    uint32_t x;
    } PHANTOMU8;

///  Documented struct.
typedef struct STRUCTDOCUMENTED
    {
    ///  Documented field.
    float x;
    } STRUCTDOCUMENTED;

typedef struct STRUCTRENAMED
    {
    ENUMRENAMED e;
    } STRUCTRENAMED;

typedef struct TUPLED
    {
    uint8_t x0;
    } TUPLED;

typedef struct USECSTRPTR
    {
    const char* ascii_string;
    } USECSTRPTR;

typedef struct USESTRING
    {
    UTF8STRING s1;
    UTF8STRING s2;
    } USESTRING;

typedef struct VEC
    {
    double x;
    double z;
    } VEC;

typedef struct VEC1
    {
    float x;
    float y;
    } VEC1;

typedef struct VEC2
    {
    double x;
    double z;
    } VEC2;

typedef struct VEC3F32
    {
    float x;
    float y;
    float z;
    } VEC3F32;

typedef struct VISIBILITY1
    {
    uint8_t pblc;
    uint8_t prvt;
    } VISIBILITY1;

typedef struct VISIBILITY2
    {
    uint8_t pblc1;
    uint8_t pblc2;
    } VISIBILITY2;

typedef struct WEIRD1U32
    {
    uint32_t x;
    } WEIRD1U32;

typedef uint8_t (*fptr_fn_u8_rval_u8)(uint8_t x0);

/// A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEBOOL
    {
    /// Pointer to start of immutable data.
    const uint8_t* data;
    /// Number of elements.
    uint64_t len;
    } SLICEBOOL;

/// A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEI32
    {
    /// Pointer to start of immutable data.
    const int32_t* data;
    /// Number of elements.
    uint64_t len;
    } SLICEI32;

/// A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEU32
    {
    /// Pointer to start of immutable data.
    const uint32_t* data;
    /// Number of elements.
    uint64_t len;
    } SLICEU32;

/// A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEU8
    {
    /// Pointer to start of immutable data.
    const uint8_t* data;
    /// Number of elements.
    uint64_t len;
    } SLICEU8;

/// A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEUTF8STRING
    {
    /// Pointer to start of immutable data.
    const UTF8STRING* data;
    /// Number of elements.
    uint64_t len;
    } SLICEUTF8STRING;

/// A pointer to an array of data someone else owns which may be modified.
typedef struct SLICEMUTU32
    {
    /// Pointer to start of mutable data.
    const uint32_t* data;
    /// Number of elements.
    uint64_t len;
    } SLICEMUTU32;

/// A pointer to an array of data someone else owns which may be modified.
typedef struct SLICEMUTU8
    {
    /// Pointer to start of mutable data.
    const uint8_t* data;
    /// Number of elements.
    uint64_t len;
    } SLICEMUTU8;

/// Option that contains Some(value) or None.
typedef enum OPTIONENUMPAYLOAD
    {
    /// Element if Some().
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    OPTIONENUMPAYLOAD_NONE = 1,
    } OPTIONENUMPAYLOAD;

/// Option that contains Some(value) or None.
typedef enum OPTIONUTF8STRING
    {
    /// Element if Some().
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    OPTIONUTF8STRING_NONE = 1,
    } OPTIONUTF8STRING;

/// Result that contains value or an error.
typedef enum RESULTERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTERROR_PANIC = 2,
    RESULTERROR_NULL = 3,
    } RESULTERROR;

/// Result that contains value or an error.
typedef enum RESULTU32ERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTU32ERROR_PANIC = 2,
    RESULTU32ERROR_NULL = 3,
    } RESULTU32ERROR;

/// Result that contains value or an error.
typedef enum RESULTU64ERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTU64ERROR_PANIC = 2,
    RESULTU64ERROR_NULL = 3,
    } RESULTU64ERROR;

/// Result that contains value or an error.
typedef enum RESULTUTF8STRINGERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTUTF8STRINGERROR_PANIC = 2,
    RESULTUTF8STRINGERROR_NULL = 3,
    } RESULTUTF8STRINGERROR;

///  Vec marshalling helper.
///  A highly dangerous 'use once type' that has ownership semantics!
///  Once passed over an FFI boundary 'the other side' is meant to own
///  (and free) it. Rust handles that fine, but if in C# you put this
///  in a struct and then call Rust multiple times with that struct 
///  you'll free the same pointer multiple times, and get UB!
typedef struct VECU8
    {
    uint8_t* ptr;
    uint64_t len;
    uint64_t capacity;
    } VECU8;

///  Vec marshalling helper.
///  A highly dangerous 'use once type' that has ownership semantics!
///  Once passed over an FFI boundary 'the other side' is meant to own
///  (and free) it. Rust handles that fine, but if in C# you put this
///  in a struct and then call Rust multiple times with that struct 
///  you'll free the same pointer multiple times, and get UB!
typedef struct VECUTF8STRING
    {
    UTF8STRING* ptr;
    uint64_t len;
    uint64_t capacity;
    } VECUTF8STRING;

typedef struct ARRAY
    {
    uint8_t data[16];
    } ARRAY;

typedef struct CONTAINER
    {
    LOCAL foreign;
    } CONTAINER;

typedef struct FIXEDSTRING
    {
    uint8_t data[32];
    } FIXEDSTRING;

typedef struct GENERICU32
    {
    const uint32_t* x;
    } GENERICU32;

typedef struct GENERICU8
    {
    const uint8_t* x;
    } GENERICU8;

typedef struct USESLICEANDVEC
    {
    SLICEUTF8STRING s1;
    VECUTF8STRING s2;
    } USESLICEANDVEC;

typedef struct WEIRD2U8
    {
    uint8_t t;
    uint8_t a[5];
    const uint8_t* r;
    } WEIRD2U8;

/// A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEUSECSTRPTR
    {
    /// Pointer to start of immutable data.
    const USECSTRPTR* data;
    /// Number of elements.
    uint64_t len;
    } SLICEUSECSTRPTR;

/// A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEUSESTRING
    {
    /// Pointer to start of immutable data.
    const USESTRING* data;
    /// Number of elements.
    uint64_t len;
    } SLICEUSESTRING;

/// A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEVEC
    {
    /// Pointer to start of immutable data.
    const VEC* data;
    /// Number of elements.
    uint64_t len;
    } SLICEVEC;

/// A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEVEC3F32
    {
    /// Pointer to start of immutable data.
    const VEC3F32* data;
    /// Number of elements.
    uint64_t len;
    } SLICEVEC3F32;

/// A pointer to an array of data someone else owns which may be modified.
typedef struct SLICEMUTVEC
    {
    /// Pointer to start of mutable data.
    const VEC* data;
    /// Number of elements.
    uint64_t len;
    } SLICEMUTVEC;

/// Option that contains Some(value) or None.
typedef enum OPTIONINNER
    {
    /// Element if Some().
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    OPTIONINNER_NONE = 1,
    } OPTIONINNER;

/// Option that contains Some(value) or None.
typedef enum OPTIONVEC
    {
    /// Element if Some().
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    OPTIONVEC_NONE = 1,
    } OPTIONVEC;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEASYNCERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEASYNCERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEASYNCERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEASYNCERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEBASICERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEBASICERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEBASICERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEBASICERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICECALLBACKSERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICECALLBACKSERROR_PANIC = 2,
    RESULTCONSTPTRSERVICECALLBACKSERROR_NULL = 3,
    } RESULTCONSTPTRSERVICECALLBACKSERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEDEPENDENTERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEDEPENDENTERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEDEPENDENTERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEDEPENDENTERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEIGNORINGMETHODSERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEIGNORINGMETHODSERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEIGNORINGMETHODSERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEIGNORINGMETHODSERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEMAINERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEMAINERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEMAINERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEMAINERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEMULTIPLECTORSERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEMULTIPLECTORSERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEMULTIPLECTORSERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEMULTIPLECTORSERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEONPANICERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEONPANICERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEONPANICERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEONPANICERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICERESULTERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICERESULTERROR_PANIC = 2,
    RESULTCONSTPTRSERVICERESULTERROR_NULL = 3,
    } RESULTCONSTPTRSERVICERESULTERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICESTRINGSERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICESTRINGSERROR_PANIC = 2,
    RESULTCONSTPTRSERVICESTRINGSERROR_NULL = 3,
    } RESULTCONSTPTRSERVICESTRINGSERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEVARIOUSSLICESERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEVARIOUSSLICESERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEVARIOUSSLICESERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEVARIOUSSLICESERROR;

/// Result that contains value or an error.
typedef enum RESULTOPTIONENUMPAYLOADERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTOPTIONENUMPAYLOADERROR_PANIC = 2,
    RESULTOPTIONENUMPAYLOADERROR_NULL = 3,
    } RESULTOPTIONENUMPAYLOADERROR;

/// Result that contains value or an error.
typedef enum RESULTOPTIONUTF8STRINGERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTOPTIONUTF8STRINGERROR_PANIC = 2,
    RESULTOPTIONUTF8STRINGERROR_NULL = 3,
    } RESULTOPTIONUTF8STRINGERROR;

/// Result that contains value or an error.
typedef enum RESULTUSESTRINGERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTUSESTRINGERROR_PANIC = 2,
    RESULTUSESTRINGERROR_NULL = 3,
    } RESULTUSESTRINGERROR;

typedef uint8_t (*CALLBACKFFISLICE)(SLICEU8 SLICE, const void* CALLBACK_DATA);

typedef void (*CALLBACKSLICEMUT)(SLICEMUTU8 SLICE, const void* CALLBACK_DATA);

typedef uint8_t (*CALLBACKU8)(uint8_t VALUE, const void* CALLBACK_DATA);

typedef uint32_t (*MYCALLBACK)(uint32_t VALUE, const void* CALLBACK_DATA);

typedef void (*MYCALLBACKCONTEXTUAL)(const void* CONTEXT, uint32_t VALUE, const void* CALLBACK_DATA);

typedef uint32_t (*MYCALLBACKNAMESPACED)(uint32_t VALUE, const void* CALLBACK_DATA);

typedef void (*MYCALLBACKVOID)(const void* PTR, const void* CALLBACK_DATA);

typedef void (*NESTEDSTRINGCALLBACK)(USESTRING S, const void* CALLBACK_DATA);

typedef void (*STRINGCALLBACK)(UTF8STRING S, const void* CALLBACK_DATA);

typedef void (*SUMDELEGATE1)(const void* CALLBACK_DATA);

typedef int32_t (*SUMDELEGATE2)(int32_t X, int32_t Y, const void* CALLBACK_DATA);

typedef RESULTERROR (*SUMDELEGATERETURN)(int32_t X, int32_t Y, const void* CALLBACK_DATA);

typedef void (*SUMDELEGATERETURN2)(int32_t X, int32_t Y, const void* CALLBACK_DATA);

///  Vec marshalling helper.
///  A highly dangerous 'use once type' that has ownership semantics!
///  Once passed over an FFI boundary 'the other side' is meant to own
///  (and free) it. Rust handles that fine, but if in C# you put this
///  in a struct and then call Rust multiple times with that struct 
///  you'll free the same pointer multiple times, and get UB!
typedef struct VECVEC3F32
    {
    VEC3F32* ptr;
    uint64_t len;
    uint64_t capacity;
    } VECVEC3F32;

typedef struct CALLBACKTABLE
    {
    MYCALLBACK my_callback;
    MYCALLBACKNAMESPACED my_callback_namespaced;
    MYCALLBACKVOID my_callback_void;
    MYCALLBACKCONTEXTUAL my_callback_contextual;
    SUMDELEGATE1 sum_delegate_1;
    SUMDELEGATE2 sum_delegate_2;
    SUMDELEGATERETURN sum_delegate_return;
    SUMDELEGATERETURN2 sum_delegate_return_2;
    } CALLBACKTABLE;

typedef struct CHARARRAY
    {
    FIXEDSTRING str;
    FIXEDSTRING str_2;
    } CHARARRAY;

typedef struct NESTEDARRAY
    {
    ENUMRENAMED field_enum;
    VEC3F32 field_vec;
    bool field_bool;
    int32_t field_int;
    uint16_t field_array[5];
    uint16_t field_array_2[5];
    ARRAY field_struct;
    } NESTEDARRAY;

/// Option that contains Some(value) or None.
typedef enum OPTIONRESULTOPTIONUTF8STRINGERROR
    {
    /// Element if Some().
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    OPTIONRESULTOPTIONUTF8STRINGERROR_NONE = 1,
    } OPTIONRESULTOPTIONUTF8STRINGERROR;

typedef VEC3F32 (*CALLBACKHUGEVECSLICE)(SLICEVEC3F32 SLICE, const void* CALLBACK_DATA);

typedef void (*fptr_fn_ConstPtrResultError_ConstPtr)(const RESULTERROR* x0, const void* x1);

typedef void (*fptr_fn_ConstPtrResultU64Error_ConstPtr)(const RESULTU64ERROR* x0, const void* x1);

typedef void (*fptr_fn_ConstPtrResultUtf8StringError_ConstPtr)(const RESULTUTF8STRINGERROR* x0, const void* x1);

typedef void (*fptr_fn_CharArray)(CHARARRAY x0);

/// A pointer to an array of data someone else owns which may be modified.
typedef struct SLICEMUTCHARARRAY
    {
    /// Pointer to start of mutable data.
    const CHARARRAY* data;
    /// Number of elements.
    uint64_t len;
    } SLICEMUTCHARARRAY;

/// Option that contains Some(value) or None.
typedef enum OPTIONOPTIONRESULTOPTIONUTF8STRINGERROR
    {
    /// Element if Some().
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    OPTIONOPTIONRESULTOPTIONUTF8STRINGERROR_NONE = 1,
    } OPTIONOPTIONRESULTOPTIONUTF8STRINGERROR;

/// Result that contains value or an error.
typedef enum RESULTNESTEDARRAYERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTNESTEDARRAYERROR_PANIC = 2,
    RESULTNESTEDARRAYERROR_NULL = 3,
    } RESULTNESTEDARRAYERROR;

typedef void (*CALLBACKCHARARRAY2)(CHARARRAY VALUE, const void* CALLBACK_DATA);

typedef void (*fptr_fn_ConstPtrResultUseStringError_ConstPtr)(const RESULTUSESTRINGERROR* x0, const void* x1);

typedef void (*fptr_fn_ConstPtrResultNestedArrayError_ConstPtr)(const RESULTNESTEDARRAYERROR* x0, const void* x1);


int64_t interoptopus_string_create(const void* UTF8, uint64_t LEN, UTF8STRING* RVAL);

int64_t interoptopus_string_destroy(UTF8STRING UTF8);

int64_t interoptopus_vec_create_18289942533122229086(const void* DATA, uint64_t LEN, VECU8* RVAL);

int64_t interoptopus_vec_destroy_17895994407320212994(VECU8 IGNORED);

int64_t interoptopus_vec_create_1491625606766217421(const void* DATA, uint64_t LEN, VECUTF8STRING* RVAL);

int64_t interoptopus_vec_destroy_2831836161306219799(VECUTF8STRING IGNORED);

int64_t interoptopus_vec_create_8489828321293410959(const void* DATA, uint64_t LEN, VECVEC3F32* RVAL);

int64_t interoptopus_vec_destroy_18428593021019987507(VECVEC3F32 IGNORED);

PACKED2 alignment_1(PACKED1 A);

uint8_t array_1(ARRAY X);

ARRAY array_2();

void array_3(ARRAY* ARR);

CHARARRAY char_array_1();

CHARARRAY char_array_2(CHARARRAY ARR);

uint8_t char_array_3(const CHARARRAY* ARR);

NESTEDARRAY nested_array_1();

void nested_array_2(NESTEDARRAY* RESULT);

uint8_t nested_array_3(NESTEDARRAY INPUT);

void behavior_sleep(uint64_t MILLIS);

void behavior_panics();

void enums_1(ENUMPAYLOAD IGNORED);

ENUMPAYLOAD enums_2(ENUMPAYLOAD X);

const ENUMPAYLOAD* enums_3(ENUMPAYLOAD* X);

uint8_t fnptr_1(fptr_fn_u8_rval_u8 CALLBACK, uint8_t VALUE);

void fnptr_2(fptr_fn_CharArray CALLBACK, CHARARRAY VALUE);

uint32_t generic_1a(GENERICU32 X, PHANTOMU8 Y);

uint8_t generic_1b(GENERICU8 X, PHANTOMU8 Y);

uint8_t generic_1c(const GENERICU8* X, const GENERICU8* Y);

uint8_t generic_2(const GENERIC2U8* X);

uint8_t generic_3(const GENERIC3* X);

uint8_t generic_4(const GENERIC4* X);

bool generic_5(WEIRD1U32 X, WEIRD2U8 Y);

VEC1 meta_ambiguous_1(VEC1 X);

VEC2 meta_ambiguous_2(VEC2 X);

bool meta_ambiguous_3(VEC1 X, VEC2 Y);

///  This function has documentation.
ENUMDOCUMENTED meta_documented(STRUCTDOCUMENTED X);

void meta_visibility1(VISIBILITY1 X, VISIBILITY2 Y);

ENUMRENAMED meta_renamed(STRUCTRENAMED X);

OPTIONVEC namespaced_inner_option(OPTIONVEC X);

SLICEVEC namespaced_inner_slice(SLICEVEC X);

SLICEMUTVEC namespaced_inner_slice_mut(SLICEMUTVEC X);

VEC namespaced_type(VEC X);

int64_t primitive_args_5(int64_t X0, int64_t X1, int64_t X2, int64_t X3, int64_t X4);

int64_t primitive_args_10(int64_t X0, int64_t X1, int64_t X2, int64_t X3, int64_t X4, int64_t X5, int64_t X6, int64_t X7, int64_t X8, int64_t X9);

bool primitive_bool(bool X);

float primitive_f32(float X);

double primitive_f64(double X);

int16_t primitive_i16(int16_t X);

int32_t primitive_i32(int32_t X);

int64_t primitive_i64(int64_t X);

int8_t primitive_i8(int8_t X);

uint16_t primitive_u16(uint16_t X);

uint32_t primitive_u32(uint32_t X);

uint64_t primitive_u64(uint64_t X);

uint8_t primitive_u8(uint8_t X);

void primitive_void();

void primitive_void2();

const int64_t* ptr1(const int64_t* X);

const const int64_t** ptr2(const const int64_t** X);

///  # Safety
/// 
///  Parameter x must point to valid data.
int64_t* ptr3(int64_t* X);

const int64_t* ref1(const int64_t* X);

int64_t* ref2(int64_t* X);

bool ref3(const int64_t* X);

bool ref4(int64_t* X);

TUPLED struct1(TUPLED X);

RESULTERROR struct2(VEC3F32 A, const TUPLED* B);

bool struct3(BOOLFIELD X);

uint32_t pattern_ascii_pointer_1(const char* X);

const char* pattern_ascii_pointer_2();

const char* pattern_ascii_pointer_3(const char* X);

const char* pattern_ascii_pointer_4(const char* X, uint32_t L);

uint8_t pattern_ascii_pointer_5(const char* X, uint32_t I);

SLICEUSECSTRPTR pattern_ascii_pointer_return_slice();

UTF8STRING pattern_string_1(UTF8STRING X);

uint32_t pattern_string_2(UTF8STRING X);

UTF8STRING pattern_string_3();

USESTRING pattern_string_4(USESTRING X);

RESULTUSESTRINGERROR pattern_string_5(USESTRING X);

RESULTERROR pattern_string_6a(const USESTRING* IGNORED);

RESULTERROR pattern_string_6b(USESTRING* Y);

RESULTUTF8STRINGERROR pattern_string_7(SLICEUTF8STRING X, uint64_t I);

RESULTUSESTRINGERROR pattern_string_8(SLICEUSESTRING X, uint64_t I);

RESULTUTF8STRINGERROR pattern_string_9();

uint32_t pattern_ffi_slice_1(SLICEU32 FFI_SLICE);

uint32_t pattern_ffi_slice_1b(SLICEMUTU32 FFI_SLICE);

VEC3F32 pattern_ffi_slice_2(SLICEVEC3F32 FFI_SLICE, int32_t I);

void pattern_ffi_slice_3(SLICEMUTU8 SLICE, CALLBACKSLICEMUT CALLBACK);

void pattern_ffi_slice_4(SLICEU8 SLICE, SLICEMUTU8 SLICE2);

///  It is (probably?) UB to call this function with the same FFI slice data at the same time.
void pattern_ffi_slice_5(const SLICEU8* SLICE, SLICEMUTU8* SLICE2);

void pattern_ffi_slice_6(const SLICEMUTU8* SLICE, CALLBACKU8 CALLBACK);

void pattern_ffi_slice_8(const SLICEMUTCHARARRAY* SLICE, CALLBACKCHARARRAY2 CALLBACK);

uint8_t pattern_ffi_slice_delegate(CALLBACKFFISLICE CALLBACK);

VEC3F32 pattern_ffi_slice_delegate_huge(CALLBACKHUGEVECSLICE CALLBACK);

OPTIONINNER pattern_ffi_option_1(OPTIONINNER X);

INNER pattern_ffi_option_2(OPTIONINNER X);

OPTIONOPTIONRESULTOPTIONUTF8STRINGERROR pattern_ffi_option_3(OPTIONOPTIONRESULTOPTIONUTF8STRINGERROR X);

uint8_t pattern_ffi_bool(uint8_t FFI_BOOL);

char pattern_ffi_cchar(char FFI_CCHAR);

const char* pattern_ffi_cchar_const_pointer(const char* FFI_CCHAR);

char* pattern_ffi_cchar_mut_pointer(char* FFI_CCHAR);

RESULTU32ERROR pattern_result_1(RESULTU32ERROR X);

RESULTERROR pattern_result_2();

RESULTERROR pattern_result_3(RESULTERROR X);

uint64_t pattern_api_guard();

uint32_t pattern_callback_1(MYCALLBACK CALLBACK, uint32_t X);

MYCALLBACKVOID pattern_callback_2(MYCALLBACKVOID CALLBACK);

uint32_t pattern_callback_4(MYCALLBACKNAMESPACED CALLBACK, uint32_t X);

SUMDELEGATE1 pattern_callback_5();

SUMDELEGATE2 pattern_callback_6();

RESULTERROR pattern_callback_7(SUMDELEGATERETURN C1, SUMDELEGATERETURN2 C2, int32_t X, int32_t I, int32_t* O);

void pattern_callback_8(STRINGCALLBACK CB, NESTEDSTRINGCALLBACK CB2, UTF8STRING S);

void pattern_surrogates_1(LOCAL S, CONTAINER* C);

VECU8 pattern_vec_1();

void pattern_vec_2(VECU8 IGNORED);

VECU8 pattern_vec_3(VECU8 V);

VECU8 pattern_vec_4(const VECU8* V);

VECUTF8STRING pattern_vec_5(VECUTF8STRING V);

VECVEC3F32 pattern_vec_6(VECVEC3F32 V);

void pattern_vec_7(USESLICEANDVEC IGNORED);

USESLICEANDVEC pattern_vec_8(USESLICEANDVEC V);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICEASYNCERROR service_async_destroy(const SERVICEASYNC* _CONTEXT);

RESULTCONSTPTRSERVICEASYNCERROR service_async_new();

RESULTERROR service_async_return_after_ms(const SERVICEASYNC* _CONTEXT, uint64_t X, uint64_t MS, fptr_fn_ConstPtrResultU64Error_ConstPtr _ASYNC_CALLBACK);

RESULTERROR service_async_process_struct(const SERVICEASYNC* _CONTEXT, NESTEDARRAY X, fptr_fn_ConstPtrResultNestedArrayError_ConstPtr _ASYNC_CALLBACK);

RESULTERROR service_async_handle_string(const SERVICEASYNC* _CONTEXT, UTF8STRING S, fptr_fn_ConstPtrResultUtf8StringError_ConstPtr _ASYNC_CALLBACK);

RESULTERROR service_async_handle_nested_string(const SERVICEASYNC* _CONTEXT, UTF8STRING S, fptr_fn_ConstPtrResultUseStringError_ConstPtr _ASYNC_CALLBACK);

void service_async_callback_string(const SERVICEASYNC* _CONTEXT, UTF8STRING S, STRINGCALLBACK CB);

RESULTERROR service_async_success(const SERVICEASYNC* _CONTEXT, fptr_fn_ConstPtrResultError_ConstPtr _ASYNC_CALLBACK);

RESULTERROR service_async_fail(const SERVICEASYNC* _CONTEXT, fptr_fn_ConstPtrResultError_ConstPtr _ASYNC_CALLBACK);

void service_async_bad(SERVICEASYNC* _CONTEXT);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICEBASICERROR service_basic_destroy(SERVICEBASIC* _CONTEXT);

RESULTCONSTPTRSERVICEBASICERROR service_basic_new();

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICEMAINERROR service_main_destroy(SERVICEMAIN* _CONTEXT);

RESULTCONSTPTRSERVICEMAINERROR service_main_new(uint32_t VALUE);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICEDEPENDENTERROR service_dependent_destroy(SERVICEDEPENDENT* _CONTEXT);

RESULTCONSTPTRSERVICEDEPENDENTERROR service_dependent_from_main(const SERVICEMAIN* MAIN);

uint32_t service_dependent_get(const SERVICEDEPENDENT* _CONTEXT);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICERESULTERROR service_result_destroy(SERVICERESULT* _CONTEXT);

RESULTCONSTPTRSERVICERESULTERROR service_result_new();

RESULTERROR service_result_test(const SERVICERESULT* _CONTEXT);

RESULTU32ERROR service_result_result_u32(const SERVICERESULT* _CONTEXT);

RESULTUTF8STRINGERROR service_result_result_string(const SERVICERESULT* _CONTEXT);

RESULTOPTIONENUMPAYLOADERROR service_result_result_option_enum(const SERVICERESULT* _CONTEXT);

RESULTU32ERROR service_result_result_slice(const SERVICERESULT* _CONTEXT, SLICEU32 SLICE, uint64_t I);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICEONPANICERROR service_on_panic_destroy(SERVICEONPANIC* _CONTEXT);

RESULTCONSTPTRSERVICEONPANICERROR service_on_panic_new();

///  Methods returning a Result<(), _> are the default and do not
///  need annotations.
RESULTERROR service_on_panic_return_result(const SERVICEONPANIC* _CONTEXT, uint32_t ANON1);

///  Methods returning a value need an `on_panic` annotation.
uint32_t service_on_panic_return_default_value(const SERVICEONPANIC* _CONTEXT, uint32_t X);

///  This function has no panic safeguards. It will be a bit faster to
///  call, but if it panics your host app will abort.
const char* service_on_panic_return_ub_on_panic(SERVICEONPANIC* _CONTEXT);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICECALLBACKSERROR service_callbacks_destroy(SERVICECALLBACKS* _CONTEXT);

RESULTCONSTPTRSERVICECALLBACKSERROR service_callbacks_new();

RESULTERROR service_callbacks_callback_simple(SERVICECALLBACKS* _CONTEXT, MYCALLBACK CALLBACK);

RESULTERROR service_callbacks_callback_ffi_return(SERVICECALLBACKS* _CONTEXT, SUMDELEGATERETURN CALLBACK);

RESULTERROR service_callbacks_callback_with_slice(SERVICECALLBACKS* _CONTEXT, SUMDELEGATERETURN CALLBACK, SLICEI32 INPUT);

void service_callbacks_set_delegate_table(SERVICECALLBACKS* _CONTEXT, CALLBACKTABLE TABLE);

RESULTERROR service_callbacks_invoke_delegates(const SERVICECALLBACKS* _CONTEXT);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICEIGNORINGMETHODSERROR service_ignoring_methods_destroy(SERVICEIGNORINGMETHODS* _CONTEXT);

RESULTCONSTPTRSERVICEIGNORINGMETHODSERROR service_ignoring_methods_new();

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICEMULTIPLECTORSERROR service_multiple_ctors_destroy(SERVICEMULTIPLECTORS* _CONTEXT);

RESULTCONSTPTRSERVICEMULTIPLECTORSERROR service_multiple_ctors_new_with(uint32_t SOME_VALUE);

RESULTCONSTPTRSERVICEMULTIPLECTORSERROR service_multiple_ctors_new_without();

RESULTCONSTPTRSERVICEMULTIPLECTORSERROR service_multiple_ctors_new_with_string(const char* ANON0);

RESULTCONSTPTRSERVICEMULTIPLECTORSERROR service_multiple_ctors_new_failing(uint8_t SOME_VALUE);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICEVARIOUSSLICESERROR service_various_slices_destroy(SERVICEVARIOUSSLICES* _CONTEXT);

RESULTCONSTPTRSERVICEVARIOUSSLICESERROR service_various_slices_new();

uint8_t service_various_slices_mut_self(SERVICEVARIOUSSLICES* _CONTEXT, SLICEU8 SLICE);

///  Single line.
void service_various_slices_mut_self_void(SERVICEVARIOUSSLICES* _CONTEXT, SLICEBOOL SLICE);

uint8_t service_various_slices_mut_self_ref(SERVICEVARIOUSSLICES* _CONTEXT, const uint8_t* X, uint8_t* Y);

uint8_t service_various_slices_mut_self_ref_slice(SERVICEVARIOUSSLICES* _CONTEXT, const uint8_t* X, uint8_t* Y, SLICEU8 SLICE);

uint8_t service_various_slices_mut_self_ref_slice_limited(SERVICEVARIOUSSLICES* _CONTEXT, const uint8_t* X, uint8_t* Y, SLICEU8 SLICE, SLICEU8 SLICE2);

RESULTERROR service_various_slices_mut_self_ffi_error(SERVICEVARIOUSSLICES* _CONTEXT, SLICEMUTU8 SLICE);

RESULTERROR service_various_slices_mut_self_no_error(SERVICEVARIOUSSLICES* _CONTEXT, SLICEMUTU8 SLICE);

///  Warning, you _must_ discard the returned slice object before calling into this service
///  again, as otherwise undefined behavior might happen.
SLICEU32 service_various_slices_return_slice(SERVICEVARIOUSSLICES* _CONTEXT);

///  Warning, you _must_ discard the returned slice object before calling into this service
///  again, as otherwise undefined behavior might happen.
SLICEMUTU32 service_various_slices_return_slice_mut(SERVICEVARIOUSSLICES* _CONTEXT);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
RESULTCONSTPTRSERVICESTRINGSERROR service_strings_destroy(SERVICESTRINGS* _CONTEXT);

RESULTCONSTPTRSERVICESTRINGSERROR service_strings_new();

RESULTCONSTPTRSERVICESTRINGSERROR service_strings_new_string(UTF8STRING X);

void service_strings_pass_cstr(SERVICESTRINGS* _CONTEXT, const char* ANON1);

const char* service_strings_return_cstr(SERVICESTRINGS* _CONTEXT);

void service_strings_callback_string(const SERVICESTRINGS* _CONTEXT, UTF8STRING S, STRINGCALLBACK CB);


#ifdef __cplusplus
}
#endif

#endif /* interoptopus_generated */
