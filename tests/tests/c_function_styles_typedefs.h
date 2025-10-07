

#ifndef interoptopus_generated
#define interoptopus_generated

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>
#include <sys/types.h>



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

typedef struct SERVICEASYNCBASIC SERVICEASYNCBASIC;

typedef struct SERVICEASYNCRESULT SERVICEASYNCRESULT;

typedef struct SERVICEASYNCSLEEP SERVICEASYNCSLEEP;

typedef struct SERVICEASYNCSTRUCTS SERVICEASYNCSTRUCTS;

typedef struct SERVICEASYNCVECSTRING SERVICEASYNCVECSTRING;

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

typedef enum ENUMNUM
    {
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    ENUMNUM_B = 1,
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    } ENUMNUM;

typedef struct BOOLFIELD
    {
    bool val;
    } BOOLFIELD;

typedef struct EXTRATYPEF32
    {
    float x;
    } EXTRATYPEF32;

typedef struct IVEC3
    {
    ptrdiff_t x;
    ptrdiff_t y;
    ptrdiff_t z;
    } IVEC3;

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

/// Result that contains value or an error.
typedef enum RESULTVOID
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTVOID_PANIC = 2,
    RESULTVOID_NULL = 3,
    } RESULTVOID;

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

typedef enum ENUMPAYLOAD
    {
    ENUMPAYLOAD_A = 0,
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    } ENUMPAYLOAD;

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

typedef struct LAYER1UTF8STRING
    {
    OPTIONUTF8STRING maybe_1;
    VECUTF8STRING maybe_2;
    UTF8STRING maybe_3;
    } LAYER1UTF8STRING;

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

/// FFI buffer for Wire data transfer
typedef struct WIREBUFFER
    {
    const uint8_t* data;
    int32_t len;
    int32_t capacity;
    } WIREBUFFER;

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
typedef enum RESULTCONSTPTRSERVICEASYNCBASICERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEASYNCBASICERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEASYNCBASICERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEASYNCBASICERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEASYNCRESULTERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEASYNCRESULTERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEASYNCRESULTERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEASYNCRESULTERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEASYNCSLEEPERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEASYNCSLEEPERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEASYNCSLEEPERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEASYNCSLEEPERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEASYNCSTRUCTSERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEASYNCSTRUCTSERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEASYNCSTRUCTSERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEASYNCSTRUCTSERROR;

/// Result that contains value or an error.
typedef enum RESULTCONSTPTRSERVICEASYNCVECSTRINGERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTCONSTPTRSERVICEASYNCVECSTRINGERROR_PANIC = 2,
    RESULTCONSTPTRSERVICEASYNCVECSTRINGERROR_NULL = 3,
    } RESULTCONSTPTRSERVICEASYNCVECSTRINGERROR;

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

/// Result that contains value or an error.
typedef enum RESULTVECUTF8STRINGERROR
    {
    /// Element if err is `Ok`.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    /// Error value.
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    RESULTVECUTF8STRINGERROR_PANIC = 2,
    RESULTVECUTF8STRINGERROR_NULL = 3,
    } RESULTVECUTF8STRINGERROR;

typedef uint8_t (*CALLBACKFFISLICE)(SLICEU8 SLICE, const void* CALLBACK_DATA);

typedef void (*CALLBACKSLICEMUT)(SLICEMUTU8 SLICE, const void* CALLBACK_DATA);

typedef uint8_t (*CALLBACKU8)(uint8_t VALUE, const void* CALLBACK_DATA);

typedef uint32_t (*MYCALLBACK)(uint32_t VALUE, const void* CALLBACK_DATA);

typedef void (*MYCALLBACKCONTEXTUAL)(const void* CONTEXT, uint32_t VALUE, const void* CALLBACK_DATA);

typedef uint32_t (*MYCALLBACKNAMESPACED)(uint32_t VALUE, const void* CALLBACK_DATA);

typedef void (*MYCALLBACKVOID)(const void* PTR, const void* CALLBACK_DATA);

typedef void (*NESTEDSTRINGCALLBACK)(USESTRING S, const void* CALLBACK_DATA);

typedef void (*POINTERS)(const int32_t* X, int32_t* Y, const void* CALLBACK_DATA);

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

typedef struct LAYER2UTF8STRING
    {
    LAYER1UTF8STRING layer_1;
    VEC3F32 vec;
    ENUMPAYLOAD the_enum;
    VECUTF8STRING strings;
    } LAYER2UTF8STRING;

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
typedef enum OPTIONENUMPAYLOAD
    {
    /// Element if Some().
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    OPTIONENUMPAYLOAD_NONE = 1,
    } OPTIONENUMPAYLOAD;

/// Option that contains Some(value) or None.
typedef enum OPTIONRESULTOPTIONUTF8STRINGERROR
    {
    /// Element if Some().
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    OPTIONRESULTOPTIONUTF8STRINGERROR_NONE = 1,
    } OPTIONRESULTOPTIONUTF8STRINGERROR;

typedef VEC3F32 (*CALLBACKHUGEVECSLICE)(SLICEVEC3F32 SLICE, const void* CALLBACK_DATA);

typedef void (*fptr_fn_ConstPtrResultError_ConstPtrVoid_rval_void)(const RESULTERROR* x0, const void* x1);

typedef void (*fptr_fn_ConstPtrResultU64Error_ConstPtrVoid_rval_void)(const RESULTU64ERROR* x0, const void* x1);

typedef void (*fptr_fn_ConstPtrResultUtf8StringError_ConstPtrVoid_rval_void)(const RESULTUTF8STRINGERROR* x0, const void* x1);

///  Vec marshalling helper.
///  A highly dangerous 'use once type' that has ownership semantics!
///  Once passed over an FFI boundary 'the other side' is meant to own
///  (and free) it. Rust handles that fine, but if in C# you put this
///  in a struct and then call Rust multiple times with that struct 
///  you'll free the same pointer multiple times, and get UB!
typedef struct VECENUMPAYLOAD
    {
    ENUMPAYLOAD* ptr;
    uint64_t len;
    uint64_t capacity;
    } VECENUMPAYLOAD;

typedef enum LAYER3
    {
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    // TODO - OMITTED DATA VARIANT - BINDINGS ARE BROKEN
    } LAYER3;

typedef void (*fptr_fn_CharArray_rval_void)(CHARARRAY x0);

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

typedef void (*CALLBACKCHARARRAY2)(CHARARRAY VALUE, const void* CALLBACK_DATA);

typedef void (*fptr_fn_ConstPtrResultUseStringError_ConstPtrVoid_rval_void)(const RESULTUSESTRINGERROR* x0, const void* x1);

typedef void (*fptr_fn_ConstPtrResultVecUtf8StringError_ConstPtrVoid_rval_void)(const RESULTVECUTF8STRINGERROR* x0, const void* x1);

typedef void (*fptr_fn_ConstPtrResultNestedArrayError_ConstPtrVoid_rval_void)(const RESULTNESTEDARRAYERROR* x0, const void* x1);


typedef int64_t (*interoptopus_string_create)(const void*, uint64_t, UTF8STRING*);

typedef int64_t (*interoptopus_string_destroy)(UTF8STRING);

typedef int64_t (*interoptopus_string_clone)(const UTF8STRING*, UTF8STRING*);

typedef void (*interoptopus_wire_destroy)(uint8_t*, int32_t, int32_t);

typedef int64_t (*interoptopus_vec_create_6849152863081469284)(const void*, uint64_t, VECU8*);

typedef int64_t (*interoptopus_vec_destroy_9353420175730321243)(VECU8);

typedef int64_t (*interoptopus_vec_create_17163327104750939811)(const void*, uint64_t, VECUTF8STRING*);

typedef int64_t (*interoptopus_vec_destroy_13145557392013674812)(VECUTF8STRING);

typedef int64_t (*interoptopus_vec_create_1781563852271199874)(const void*, uint64_t, VECVEC3F32*);

typedef int64_t (*interoptopus_vec_destroy_12107015372467426116)(VECVEC3F32);

typedef int64_t (*interoptopus_vec_create_12058709700293828778)(const void*, uint64_t, VECENUMPAYLOAD*);

typedef int64_t (*interoptopus_vec_destroy_4854562680055612543)(VECENUMPAYLOAD);

typedef PACKED2 (*alignment_1)(PACKED1);

typedef uint8_t (*array_1)(ARRAY);

typedef ARRAY (*array_2)();

typedef void (*array_3)(ARRAY*);

typedef CHARARRAY (*char_array_1)();

typedef CHARARRAY (*char_array_2)(CHARARRAY);

typedef uint8_t (*char_array_3)(const CHARARRAY*);

typedef NESTEDARRAY (*nested_array_1)();

typedef void (*nested_array_2)(NESTEDARRAY*);

typedef uint8_t (*nested_array_3)(NESTEDARRAY);

typedef void (*behavior_sleep)(uint64_t);

typedef void (*behavior_panics)();

typedef RESULTERROR (*behavior_panics_via_result)();

typedef void (*enums_1)(ENUMPAYLOAD);

typedef ENUMPAYLOAD (*enums_2)(ENUMPAYLOAD);

typedef const ENUMPAYLOAD* (*enums_3)(ENUMPAYLOAD*);

typedef UTF8STRING (*enums_4)(LAYER3);

typedef uint8_t (*fnptr_1)(fptr_fn_u8_rval_u8, uint8_t);

typedef void (*fnptr_2)(fptr_fn_CharArray_rval_void, CHARARRAY);

typedef uint32_t (*generic_1a)(GENERICU32, PHANTOMU8);

typedef uint8_t (*generic_1b)(GENERICU8, PHANTOMU8);

typedef uint8_t (*generic_1c)(const GENERICU8*, const GENERICU8*);

typedef uint8_t (*generic_2)(const GENERIC2U8*);

typedef uint8_t (*generic_3)(const GENERIC3*);

typedef uint8_t (*generic_4)(const GENERIC4*);

typedef bool (*generic_5)(WEIRD1U32, WEIRD2U8);

typedef VEC1 (*meta_ambiguous_1)(VEC1);

typedef VEC2 (*meta_ambiguous_2)(VEC2);

typedef bool (*meta_ambiguous_3)(VEC1, VEC2);

///  This function has documentation.
typedef ENUMDOCUMENTED (*meta_documented)(STRUCTDOCUMENTED);

typedef void (*meta_visibility1)(VISIBILITY1, VISIBILITY2);

typedef ENUMRENAMED (*meta_renamed)(STRUCTRENAMED);

typedef OPTIONVEC (*namespaced_inner_option)(OPTIONVEC);

typedef SLICEVEC (*namespaced_inner_slice)(SLICEVEC);

typedef SLICEMUTVEC (*namespaced_inner_slice_mut)(SLICEMUTVEC);

typedef VEC (*namespaced_type)(VEC);

typedef int64_t (*primitive_args_5)(int64_t, int64_t, int64_t, int64_t, int64_t);

typedef int64_t (*primitive_args_10)(int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t);

typedef bool (*primitive_bool)(bool);

typedef float (*primitive_f32)(float);

typedef double (*primitive_f64)(double);

typedef int16_t (*primitive_i16)(int16_t);

typedef int32_t (*primitive_i32)(int32_t);

typedef int64_t (*primitive_i64)(int64_t);

typedef int8_t (*primitive_i8)(int8_t);

typedef uint16_t (*primitive_u16)(uint16_t);

typedef uint32_t (*primitive_u32)(uint32_t);

typedef uint64_t (*primitive_u64)(uint64_t);

typedef size_t (*primitive_usize)(size_t);

typedef ptrdiff_t (*primitive_isize)(ptrdiff_t);

typedef uint8_t (*primitive_u8)(uint8_t);

typedef void (*primitive_void)();

typedef void (*primitive_void2)();

typedef const int64_t* (*ptr1)(const int64_t*);

typedef const const int64_t** (*ptr2)(const const int64_t**);

///  # Safety
/// 
///  Parameter x must point to valid data.
typedef int64_t* (*ptr3)(int64_t*);

typedef const int64_t* (*ref1)(const int64_t*);

typedef int64_t* (*ref2)(int64_t*);

typedef bool (*ref3)(const int64_t*);

typedef bool (*ref4)(int64_t*);

typedef void (*ref5)(ENUMPAYLOAD*);

typedef void (*ref6)(OPTIONENUMPAYLOAD*);

typedef void (*ref7)(VECUTF8STRING*);

typedef TUPLED (*struct1)(TUPLED);

typedef RESULTERROR (*struct2)(VEC3F32, const TUPLED*);

typedef bool (*struct3)(BOOLFIELD);

typedef uint32_t (*pattern_ascii_pointer_1)(const char*);

typedef const char* (*pattern_ascii_pointer_2)();

typedef const char* (*pattern_ascii_pointer_3)(const char*);

typedef const char* (*pattern_ascii_pointer_4)(const char*, uint32_t);

typedef uint8_t (*pattern_ascii_pointer_5)(const char*, uint32_t);

typedef SLICEUSECSTRPTR (*pattern_ascii_pointer_return_slice)();

typedef UTF8STRING (*pattern_string_1)(UTF8STRING);

typedef uint32_t (*pattern_string_2)(UTF8STRING);

typedef UTF8STRING (*pattern_string_3)();

typedef USESTRING (*pattern_string_4)(USESTRING);

typedef RESULTUSESTRINGERROR (*pattern_string_5)(USESTRING);

typedef RESULTERROR (*pattern_string_6a)(const USESTRING*);

typedef RESULTERROR (*pattern_string_6b)(USESTRING*);

typedef RESULTUTF8STRINGERROR (*pattern_string_7)(SLICEUTF8STRING, uint64_t);

typedef RESULTUSESTRINGERROR (*pattern_string_8)(SLICEUSESTRING, uint64_t);

typedef RESULTUTF8STRINGERROR (*pattern_string_9)();

typedef void (*pattern_string_10)(UTF8STRING);

typedef void (*pattern_string_11)(const UTF8STRING*);

typedef uint32_t (*pattern_ffi_slice_1)(SLICEU32);

typedef uint32_t (*pattern_ffi_slice_1b)(SLICEMUTU32);

typedef VEC3F32 (*pattern_ffi_slice_2)(SLICEVEC3F32, int32_t);

typedef void (*pattern_ffi_slice_3)(SLICEMUTU8, CALLBACKSLICEMUT);

typedef void (*pattern_ffi_slice_4)(SLICEU8, SLICEMUTU8);

///  It is (probably?) UB to call this function with the same FFI slice data at the same time.
typedef void (*pattern_ffi_slice_5)(const SLICEU8*, SLICEMUTU8*);

typedef void (*pattern_ffi_slice_6)(const SLICEMUTU8*, CALLBACKU8);

typedef void (*pattern_ffi_slice_8)(const SLICEMUTCHARARRAY*, CALLBACKCHARARRAY2);

typedef UTF8STRING (*pattern_ffi_slice_9)(SLICEUSESTRING);

typedef uint8_t (*pattern_ffi_slice_delegate)(CALLBACKFFISLICE);

typedef VEC3F32 (*pattern_ffi_slice_delegate_huge)(CALLBACKHUGEVECSLICE);

typedef OPTIONINNER (*pattern_ffi_option_1)(OPTIONINNER);

typedef INNER (*pattern_ffi_option_2)(OPTIONINNER);

typedef OPTIONOPTIONRESULTOPTIONUTF8STRINGERROR (*pattern_ffi_option_3)(OPTIONOPTIONRESULTOPTIONUTF8STRINGERROR);

typedef uint8_t (*pattern_ffi_bool)(uint8_t);

typedef char (*pattern_ffi_cchar)(char);

typedef const char* (*pattern_ffi_cchar_const_pointer)(const char*);

typedef char* (*pattern_ffi_cchar_mut_pointer)(char*);

typedef RESULTU32ERROR (*pattern_result_1)(RESULTU32ERROR);

typedef RESULTERROR (*pattern_result_2)();

typedef RESULTERROR (*pattern_result_3)(RESULTERROR);

typedef RESULTVOID (*pattern_result_4)(RESULTVOID);

typedef uint64_t (*pattern_api_guard)();

typedef uint32_t (*pattern_callback_1)(MYCALLBACK, uint32_t);

typedef MYCALLBACKVOID (*pattern_callback_2)(MYCALLBACKVOID);

typedef uint32_t (*pattern_callback_4)(MYCALLBACKNAMESPACED, uint32_t);

typedef SUMDELEGATE1 (*pattern_callback_5)();

typedef SUMDELEGATE2 (*pattern_callback_6)();

typedef RESULTERROR (*pattern_callback_7)(SUMDELEGATERETURN, SUMDELEGATERETURN2, int32_t, int32_t, int32_t*);

typedef void (*pattern_callback_8)(STRINGCALLBACK, NESTEDSTRINGCALLBACK, UTF8STRING);

typedef int32_t (*pattern_callback_9)(POINTERS);

typedef void (*pattern_surrogates_1)(LOCAL, CONTAINER*);

typedef VECU8 (*pattern_vec_1)();

typedef void (*pattern_vec_2)(VECU8);

typedef VECU8 (*pattern_vec_3)(VECU8);

typedef VECU8 (*pattern_vec_4)(const VECU8*);

typedef VECUTF8STRING (*pattern_vec_5)(VECUTF8STRING);

typedef VECVEC3F32 (*pattern_vec_6)(VECVEC3F32);

typedef void (*pattern_vec_7)(USESLICEANDVEC);

typedef USESLICEANDVEC (*pattern_vec_8)(USESLICEANDVEC);

typedef void (*wire_accept_string_2)(void *);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEASYNCBASICERROR (*service_async_basic_destroy)(const SERVICEASYNCBASIC*);

typedef RESULTCONSTPTRSERVICEASYNCBASICERROR (*service_async_basic_new)();

typedef RESULTERROR (*service_async_basic_call)(const SERVICEASYNCBASIC*, fptr_fn_ConstPtrResultError_ConstPtrVoid_rval_void);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEASYNCSLEEPERROR (*service_async_sleep_destroy)(const SERVICEASYNCSLEEP*);

typedef RESULTCONSTPTRSERVICEASYNCSLEEPERROR (*service_async_sleep_new)();

typedef RESULTERROR (*service_async_sleep_return_after_ms)(const SERVICEASYNCSLEEP*, uint64_t, uint64_t, fptr_fn_ConstPtrResultU64Error_ConstPtrVoid_rval_void);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEASYNCVECSTRINGERROR (*service_async_vec_string_destroy)(const SERVICEASYNCVECSTRING*);

typedef RESULTCONSTPTRSERVICEASYNCVECSTRINGERROR (*service_async_vec_string_new)();

typedef RESULTERROR (*service_async_vec_string_handle_string)(const SERVICEASYNCVECSTRING*, UTF8STRING, fptr_fn_ConstPtrResultUtf8StringError_ConstPtrVoid_rval_void);

typedef RESULTERROR (*service_async_vec_string_handle_vec_string)(const SERVICEASYNCVECSTRING*, VECUTF8STRING, fptr_fn_ConstPtrResultVecUtf8StringError_ConstPtrVoid_rval_void);

typedef RESULTERROR (*service_async_vec_string_handle_nested_string)(const SERVICEASYNCVECSTRING*, UTF8STRING, fptr_fn_ConstPtrResultUseStringError_ConstPtrVoid_rval_void);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEASYNCRESULTERROR (*service_async_result_destroy)(const SERVICEASYNCRESULT*);

typedef RESULTCONSTPTRSERVICEASYNCRESULTERROR (*service_async_result_new)();

typedef RESULTERROR (*service_async_result_success)(const SERVICEASYNCRESULT*, fptr_fn_ConstPtrResultError_ConstPtrVoid_rval_void);

typedef RESULTERROR (*service_async_result_fail)(const SERVICEASYNCRESULT*, fptr_fn_ConstPtrResultError_ConstPtrVoid_rval_void);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEASYNCSTRUCTSERROR (*service_async_structs_destroy)(const SERVICEASYNCSTRUCTS*);

typedef RESULTCONSTPTRSERVICEASYNCSTRUCTSERROR (*service_async_structs_new)();

typedef RESULTERROR (*service_async_structs_process_struct)(const SERVICEASYNCSTRUCTS*, NESTEDARRAY, fptr_fn_ConstPtrResultNestedArrayError_ConstPtrVoid_rval_void);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEBASICERROR (*service_basic_destroy)(const SERVICEBASIC*);

typedef RESULTCONSTPTRSERVICEBASICERROR (*service_basic_new)();

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEMAINERROR (*service_main_destroy)(const SERVICEMAIN*);

typedef RESULTCONSTPTRSERVICEMAINERROR (*service_main_new)(uint32_t);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEDEPENDENTERROR (*service_dependent_destroy)(const SERVICEDEPENDENT*);

typedef RESULTCONSTPTRSERVICEDEPENDENTERROR (*service_dependent_from_main)(const SERVICEMAIN*);

typedef uint32_t (*service_dependent_get)(const SERVICEDEPENDENT*);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICERESULTERROR (*service_result_destroy)(const SERVICERESULT*);

typedef RESULTCONSTPTRSERVICERESULTERROR (*service_result_new)();

typedef RESULTERROR (*service_result_test)(const SERVICERESULT*);

typedef RESULTU32ERROR (*service_result_result_u32)(const SERVICERESULT*);

typedef RESULTUTF8STRINGERROR (*service_result_result_string)(const SERVICERESULT*);

typedef RESULTOPTIONENUMPAYLOADERROR (*service_result_result_option_enum)(const SERVICERESULT*);

typedef RESULTU32ERROR (*service_result_result_slice)(const SERVICERESULT*, SLICEU32, uint64_t);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEONPANICERROR (*service_on_panic_destroy)(const SERVICEONPANIC*);

typedef RESULTCONSTPTRSERVICEONPANICERROR (*service_on_panic_new)();

///  Methods returning a Result<(), _> are the default and do not
///  need annotations.
typedef RESULTERROR (*service_on_panic_return_result)(const SERVICEONPANIC*, uint32_t);

///  Methods returning a value need an `on_panic` annotation.
typedef uint32_t (*service_on_panic_return_default_value)(const SERVICEONPANIC*, uint32_t);

///  This function has no panic safeguards. It will be a bit faster to
///  call, but if it panics your host app will abort.
typedef const char* (*service_on_panic_return_ub_on_panic)(SERVICEONPANIC*);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICECALLBACKSERROR (*service_callbacks_destroy)(const SERVICECALLBACKS*);

typedef RESULTCONSTPTRSERVICECALLBACKSERROR (*service_callbacks_new)();

typedef RESULTERROR (*service_callbacks_callback_simple)(SERVICECALLBACKS*, MYCALLBACK);

typedef RESULTERROR (*service_callbacks_callback_ffi_return)(SERVICECALLBACKS*, SUMDELEGATERETURN);

typedef RESULTERROR (*service_callbacks_callback_with_slice)(SERVICECALLBACKS*, SUMDELEGATERETURN, SLICEI32);

typedef void (*service_callbacks_set_delegate_table)(SERVICECALLBACKS*, CALLBACKTABLE);

typedef RESULTERROR (*service_callbacks_invoke_delegates)(const SERVICECALLBACKS*);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEIGNORINGMETHODSERROR (*service_ignoring_methods_destroy)(const SERVICEIGNORINGMETHODS*);

typedef RESULTCONSTPTRSERVICEIGNORINGMETHODSERROR (*service_ignoring_methods_new)();

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEMULTIPLECTORSERROR (*service_multiple_ctors_destroy)(const SERVICEMULTIPLECTORS*);

typedef RESULTCONSTPTRSERVICEMULTIPLECTORSERROR (*service_multiple_ctors_new_with)(uint32_t);

typedef RESULTCONSTPTRSERVICEMULTIPLECTORSERROR (*service_multiple_ctors_new_without)();

typedef RESULTCONSTPTRSERVICEMULTIPLECTORSERROR (*service_multiple_ctors_new_with_string)(const char*);

typedef RESULTCONSTPTRSERVICEMULTIPLECTORSERROR (*service_multiple_ctors_new_failing)(uint8_t);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICEVARIOUSSLICESERROR (*service_various_slices_destroy)(const SERVICEVARIOUSSLICES*);

typedef RESULTCONSTPTRSERVICEVARIOUSSLICESERROR (*service_various_slices_new)();

typedef uint8_t (*service_various_slices_mut_self)(SERVICEVARIOUSSLICES*, SLICEU8);

///  Single line.
typedef void (*service_various_slices_mut_self_void)(SERVICEVARIOUSSLICES*, SLICEBOOL);

typedef uint8_t (*service_various_slices_mut_self_ref)(SERVICEVARIOUSSLICES*, const uint8_t*, uint8_t*);

typedef uint8_t (*service_various_slices_mut_self_ref_slice)(SERVICEVARIOUSSLICES*, const uint8_t*, uint8_t*, SLICEU8);

typedef uint8_t (*service_various_slices_mut_self_ref_slice_limited)(SERVICEVARIOUSSLICES*, const uint8_t*, uint8_t*, SLICEU8, SLICEU8);

typedef RESULTERROR (*service_various_slices_mut_self_ffi_error)(SERVICEVARIOUSSLICES*, SLICEMUTU8);

typedef RESULTERROR (*service_various_slices_mut_self_no_error)(SERVICEVARIOUSSLICES*, SLICEMUTU8);

///  Warning, you _must_ discard the returned slice object before calling into this service
///  again, as otherwise undefined behavior might happen.
typedef SLICEU32 (*service_various_slices_return_slice)(SERVICEVARIOUSSLICES*);

///  Warning, you _must_ discard the returned slice object before calling into this service
///  again, as otherwise undefined behavior might happen.
typedef SLICEMUTU32 (*service_various_slices_return_slice_mut)(SERVICEVARIOUSSLICES*);

///  Destroys the given instance.
/// 
///  # Safety
/// 
///  The passed parameter MUST have been created with the corresponding init function;
///  passing any other value results in undefined behavior.
typedef RESULTCONSTPTRSERVICESTRINGSERROR (*service_strings_destroy)(const SERVICESTRINGS*);

typedef RESULTCONSTPTRSERVICESTRINGSERROR (*service_strings_new)();

typedef RESULTCONSTPTRSERVICESTRINGSERROR (*service_strings_new_string)(UTF8STRING);

typedef void (*service_strings_pass_cstr)(SERVICESTRINGS*, const char*);

typedef const char* (*service_strings_return_cstr)(SERVICESTRINGS*);

typedef void (*service_strings_callback_string)(const SERVICESTRINGS*, UTF8STRING, STRINGCALLBACK);


#ifdef __cplusplus
}
#endif

#endif /* interoptopus_generated */
