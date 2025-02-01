

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

/// Documented enum.
typedef enum ENUMDOCUMENTED
    {
    /// Variant A.
    ENUMDOCUMENTED_A = 0,
    /// Variant B.
    ENUMDOCUMENTED_B = 1,
    /// Variant B.
    ENUMDOCUMENTED_C = 2,
    } ENUMDOCUMENTED;

typedef enum ENUMRENAMED
    {
    ENUMRENAMED_X = 0,
    } ENUMRENAMED;

typedef struct BASICSERVICE BASICSERVICE;

typedef struct GENERIC2U8 GENERIC2U8;

typedef struct GENERIC3 GENERIC3;

typedef struct GENERIC4 GENERIC4;

/// Some struct we want to expose as a class.
typedef struct SERVICECALLBACKS SERVICECALLBACKS;

typedef struct SERVICEIGNORINGMETHODS SERVICEIGNORINGMETHODS;

/// Some struct we want to expose as a class.
typedef struct SERVICEMULTIPLECTORS SERVICEMULTIPLECTORS;

/// Some struct we want to expose as a class.
typedef struct SERVICEONPANIC SERVICEONPANIC;

/// Some struct we want to expose as a class.
typedef struct SERVICESTRINGS SERVICESTRINGS;

/// Services can use lifetimes. However, they are more dangerous to use
/// via FFI, since you will not get any help tracking lifetimes there.
typedef struct SERVICEUSINGLIFETIMES SERVICEUSINGLIFETIMES;

/// Some struct we want to expose as a class.
typedef struct SERVICEVARIOUSSLICES SERVICEVARIOUSSLICES;

typedef enum FFIERROR
    {
    FFIERROR_OK = 0,
    FFIERROR_NULL = 100,
    FFIERROR_PANIC = 200,
    FFIERROR_DELEGATE = 300,
    FFIERROR_FAIL = 400,
    } FFIERROR;

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

/// Documented struct.
typedef struct STRUCTDOCUMENTED
    {
    /// Documented field.
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

typedef struct USEASCIISTRINGPATTERN
    {
    const char* ascii_string;
    } USEASCIISTRINGPATTERN;

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

typedef uint8_t (*CALLBACKU8)(uint8_t VALUE);

typedef uint32_t (*MYCALLBACK)(uint32_t VALUE);

typedef uint32_t (*MYCALLBACKNAMESPACED)(uint32_t VALUE);

typedef void (*SUMDELEGATE1)();

typedef int32_t (*SUMDELEGATE2)(int32_t X, int32_t Y);

typedef FFIERROR (*SUMDELEGATERETURN)(int32_t X, int32_t Y);

typedef void (*SUMDELEGATERETURN2)(int32_t X, int32_t Y);

typedef struct ARRAY
    {
    uint8_t data[16];
    } ARRAY;

typedef struct CHARARRAY
    {
    char str[32];
    char str_2[32];
    } CHARARRAY;

typedef struct CONTAINER
    {
    LOCAL foreign;
    } CONTAINER;

typedef struct GENERICU32
    {
    const uint32_t* x;
    } GENERICU32;

typedef struct GENERICU8
    {
    const uint8_t* x;
    } GENERICU8;

typedef struct WEIRD2U8
    {
    uint8_t t;
    uint8_t a[5];
    const uint8_t* r;
    } WEIRD2U8;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEBOOL
    {
    ///Pointer to start of immutable data.
    const uint8_t* data;
    ///Number of elements.
    uint64_t len;
    } SLICEBOOL;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEI32
    {
    ///Pointer to start of immutable data.
    const int32_t* data;
    ///Number of elements.
    uint64_t len;
    } SLICEI32;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEU32
    {
    ///Pointer to start of immutable data.
    const uint32_t* data;
    ///Number of elements.
    uint64_t len;
    } SLICEU32;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEU8
    {
    ///Pointer to start of immutable data.
    const uint8_t* data;
    ///Number of elements.
    uint64_t len;
    } SLICEU8;

///A pointer to an array of data someone else owns which may be modified.
typedef struct SLICEMUTU32
    {
    ///Pointer to start of mutable data.
    const uint32_t* data;
    ///Number of elements.
    uint64_t len;
    } SLICEMUTU32;

///A pointer to an array of data someone else owns which may be modified.
typedef struct SLICEMUTU8
    {
    ///Pointer to start of mutable data.
    const uint8_t* data;
    ///Number of elements.
    uint64_t len;
    } SLICEMUTU8;

///Option type containing boolean flag and maybe valid data.
typedef struct OPTIONINNER
    {
    ///Element that is maybe valid.
    INNER t;
    ///Byte where `1` means element `t` is valid.
    uint8_t is_some;
    } OPTIONINNER;

///Option type containing boolean flag and maybe valid data.
typedef struct OPTIONVEC
    {
    ///Element that is maybe valid.
    VEC t;
    ///Byte where `1` means element `t` is valid.
    uint8_t is_some;
    } OPTIONVEC;

typedef void (*MYCALLBACKVOID)(const void* PTR);

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

///A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEUSEASCIISTRINGPATTERN
    {
    ///Pointer to start of immutable data.
    const USEASCIISTRINGPATTERN* data;
    ///Number of elements.
    uint64_t len;
    } SLICEUSEASCIISTRINGPATTERN;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEVEC
    {
    ///Pointer to start of immutable data.
    const VEC* data;
    ///Number of elements.
    uint64_t len;
    } SLICEVEC;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct SLICEVEC3F32
    {
    ///Pointer to start of immutable data.
    const VEC3F32* data;
    ///Number of elements.
    uint64_t len;
    } SLICEVEC3F32;

///A pointer to an array of data someone else owns which may be modified.
typedef struct SLICEMUTVEC
    {
    ///Pointer to start of mutable data.
    const VEC* data;
    ///Number of elements.
    uint64_t len;
    } SLICEMUTVEC;

typedef uint8_t (*CALLBACKFFISLICE)(SLICEU8 SLICE);

typedef void (*CALLBACKSLICEMUT)(SLICEMUTU8 SLICE);

typedef VEC3F32 (*CALLBACKHUGEVECSLICE)(SLICEVEC3F32 SLICE);


void primitive_void();

void primitive_void2();

bool primitive_bool(bool X);

uint8_t primitive_u8(uint8_t X);

uint16_t primitive_u16(uint16_t X);

uint32_t primitive_u32(uint32_t X);

uint64_t primitive_u64(uint64_t X);

int8_t primitive_i8(int8_t X);

int16_t primitive_i16(int16_t X);

int32_t primitive_i32(int32_t X);

int64_t primitive_i64(int64_t X);

PACKED2 packed_to_packed1(PACKED1 A);

int64_t many_args_5(int64_t X0, int64_t X1, int64_t X2, int64_t X3, int64_t X4);

int64_t many_args_10(int64_t X0, int64_t X1, int64_t X2, int64_t X3, int64_t X4, int64_t X5, int64_t X6, int64_t X7, int64_t X8, int64_t X9);

const int64_t* ptr(const int64_t* X);

/// # Safety
///
/// Parameter x must point to valid data.
int64_t* ptr_mut(int64_t* X);

const const int64_t** ptr_ptr(const const int64_t** X);

const int64_t* ref_simple(const int64_t* X);

int64_t* ref_mut_simple(int64_t* X);

bool ref_option(const int64_t* X);

bool ref_mut_option(int64_t* X);

TUPLED call_tupled(TUPLED X);

FFIERROR complex_args_1(VEC3F32 A, const TUPLED* B);

uint8_t callback(fptr_fn_u8_rval_u8 CALLBACK, uint8_t VALUE);

uint32_t generic_1a(GENERICU32 X, PHANTOMU8 Y);

uint8_t generic_1b(GENERICU8 X, PHANTOMU8 Y);

uint8_t generic_1c(const GENERICU8* X, const GENERICU8* Y);

uint8_t generic_2(const GENERIC2U8* X);

uint8_t generic_3(const GENERIC3* X);

uint8_t generic_4(const GENERIC4* X);

uint8_t array_1(ARRAY X);

ARRAY array_2();

void array_3(ARRAY* ARR);

NESTEDARRAY nested_array_1();

void nested_array_2(NESTEDARRAY* RESULT);

uint8_t nested_array_3(NESTEDARRAY INPUT);

CHARARRAY char_array_1();

CHARARRAY char_array_2(CHARARRAY ARR);

uint8_t char_array_3(const CHARARRAY* ARR);

bool bool_field(BOOLFIELD X);

/// This function has documentation.
ENUMDOCUMENTED documented(STRUCTDOCUMENTED X);

VEC1 ambiguous_1(VEC1 X);

VEC2 ambiguous_2(VEC2 X);

bool ambiguous_3(VEC1 X, VEC2 Y);

VEC namespaced_type(VEC X);

OPTIONVEC namespaced_inner_option(OPTIONVEC X);

SLICEVEC namespaced_inner_slice(SLICEVEC X);

SLICEMUTVEC namespaced_inner_slice_mut(SLICEMUTVEC X);

FFIERROR panics();

ENUMRENAMED renamed(STRUCTRENAMED X);

void sleep(uint64_t MILLIS);

bool weird_1(WEIRD1U32 X, WEIRD2U8 Y);

void visibility(VISIBILITY1 X, VISIBILITY2 Y);

TUPLED repr_transparent(TUPLED X, const TUPLED* R);

uint32_t pattern_ascii_pointer_1(const char* X);

const char* pattern_ascii_pointer_2();

const char* pattern_ascii_pointer_3(const char* X);

const char* pattern_ascii_pointer_4(const char* X, uint32_t L);

uint8_t pattern_ascii_pointer_5(const char* X, uint32_t I);

SLICEUSEASCIISTRINGPATTERN pattern_ascii_pointer_return_slice();

uint32_t pattern_ffi_slice_1(SLICEU32 FFI_SLICE);

uint32_t pattern_ffi_slice_1b(SLICEMUTU32 FFI_SLICE);

VEC3F32 pattern_ffi_slice_2(SLICEVEC3F32 FFI_SLICE, int32_t I);

void pattern_ffi_slice_3(SLICEMUTU8 SLICE, CALLBACKSLICEMUT CALLBACK);

void pattern_ffi_slice_4(SLICEU8 SLICE, SLICEMUTU8 SLICE2);

void pattern_ffi_slice_5(const SLICEU8* SLICE, SLICEMUTU8* SLICE2);

void pattern_ffi_slice_6(const SLICEMUTU8* SLICE, CALLBACKU8 CALLBACK);

uint8_t pattern_ffi_slice_delegate(CALLBACKFFISLICE CALLBACK);

VEC3F32 pattern_ffi_slice_delegate_huge(CALLBACKHUGEVECSLICE CALLBACK);

OPTIONINNER pattern_ffi_option_1(OPTIONINNER FFI_SLICE);

INNER pattern_ffi_option_2(OPTIONINNER FFI_SLICE);

uint8_t pattern_ffi_bool(uint8_t FFI_BOOL);

char pattern_ffi_cchar(char FFI_CCHAR);

const char* pattern_ffi_cchar_const_pointer(const char* FFI_CCHAR);

char* pattern_ffi_cchar_mut_pointer(char* FFI_CCHAR);

uint64_t pattern_api_guard();

uint32_t pattern_callback_1(MYCALLBACK CALLBACK, uint32_t X);

MYCALLBACKVOID pattern_callback_2(MYCALLBACKVOID CALLBACK);

uint32_t pattern_callback_4(MYCALLBACKNAMESPACED CALLBACK, uint32_t X);

SUMDELEGATE1 pattern_callback_5();

SUMDELEGATE2 pattern_callback_6();

FFIERROR pattern_callback_7(SUMDELEGATERETURN C1, SUMDELEGATERETURN2 C2, int32_t X, int32_t I, int32_t* O);

void pattern_surrogates_1(LOCAL S, CONTAINER* C);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
FFIERROR basic_service_destroy(BASICSERVICE** CONTEXT);

FFIERROR basic_service_new(BASICSERVICE** CONTEXT);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
FFIERROR service_on_panic_destroy(SERVICEONPANIC** CONTEXT);

FFIERROR service_on_panic_new(SERVICEONPANIC** CONTEXT);

/// Methods returning a Result<(), _> are the default and do not
/// need annotations.
FFIERROR service_on_panic_return_result(const SERVICEONPANIC* CONTEXT, uint32_t ANON1);

/// Methods returning a value need an `on_panic` annotation.
uint32_t service_on_panic_return_default_value(const SERVICEONPANIC* CONTEXT, uint32_t X);

/// This function has no panic safeguards. It will be a bit faster to
/// call, but if it panics your host app will be in an undefined state.
const char* service_on_panic_return_ub_on_panic(SERVICEONPANIC* CONTEXT);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
FFIERROR service_callbacks_destroy(SERVICECALLBACKS** CONTEXT);

FFIERROR service_callbacks_new(SERVICECALLBACKS** CONTEXT);

FFIERROR service_callbacks_callback_simple(SERVICECALLBACKS* CONTEXT, MYCALLBACK CALLBACK);

FFIERROR service_callbacks_callback_ffi_return(SERVICECALLBACKS* CONTEXT, SUMDELEGATERETURN CALLBACK);

FFIERROR service_callbacks_callback_with_slice(SERVICECALLBACKS* CONTEXT, SUMDELEGATERETURN CALLBACK, SLICEI32 INPUT);

FFIERROR service_callbacks_invoke_delegates(const SERVICECALLBACKS* CONTEXT);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
FFIERROR service_ignoring_methods_destroy(SERVICEIGNORINGMETHODS** CONTEXT);

FFIERROR service_ignoring_methods_new(SERVICEIGNORINGMETHODS** CONTEXT);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
FFIERROR service_multiple_ctors_destroy(SERVICEMULTIPLECTORS** CONTEXT);

FFIERROR service_multiple_ctors_new_with(SERVICEMULTIPLECTORS** CONTEXT, uint32_t SOME_VALUE);

FFIERROR service_multiple_ctors_new_without(SERVICEMULTIPLECTORS** CONTEXT);

FFIERROR service_multiple_ctors_new_with_string(SERVICEMULTIPLECTORS** CONTEXT, const char* ANON0);

FFIERROR service_multiple_ctors_new_failing(SERVICEMULTIPLECTORS** CONTEXT, uint8_t SOME_VALUE);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
FFIERROR service_using_lifetimes_destroy(SERVICEUSINGLIFETIMES** CONTEXT);

FFIERROR service_using_lifetimes_new_with(SERVICEUSINGLIFETIMES** CONTEXT, const uint32_t* SOME_VALUE);

void service_using_lifetimes_lifetime_1(SERVICEUSINGLIFETIMES* CONTEXT, SLICEBOOL SLICE);

void service_using_lifetimes_lifetime_2(SERVICEUSINGLIFETIMES* CONTEXT, SLICEBOOL SLICE);

const char* service_using_lifetimes_return_string_accept_slice(SERVICEUSINGLIFETIMES* ANON0, SLICEU8 ANON1);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
FFIERROR service_various_slices_destroy(SERVICEVARIOUSSLICES** CONTEXT);

FFIERROR service_various_slices_new(SERVICEVARIOUSSLICES** CONTEXT);

uint8_t service_various_slices_mut_self(SERVICEVARIOUSSLICES* CONTEXT, SLICEU8 SLICE);

/// Single line.
void service_various_slices_mut_self_void(SERVICEVARIOUSSLICES* CONTEXT, SLICEBOOL SLICE);

uint8_t service_various_slices_mut_self_ref(SERVICEVARIOUSSLICES* CONTEXT, const uint8_t* X, uint8_t* Y);

uint8_t service_various_slices_mut_self_ref_slice(SERVICEVARIOUSSLICES* CONTEXT, const uint8_t* X, uint8_t* Y, SLICEU8 SLICE);

uint8_t service_various_slices_mut_self_ref_slice_limited(SERVICEVARIOUSSLICES* CONTEXT, const uint8_t* X, uint8_t* Y, SLICEU8 SLICE, SLICEU8 SLICE2);

FFIERROR service_various_slices_mut_self_ffi_error(SERVICEVARIOUSSLICES* CONTEXT, SLICEMUTU8 SLICE);

FFIERROR service_various_slices_mut_self_no_error(SERVICEVARIOUSSLICES* CONTEXT, SLICEMUTU8 SLICE);

/// Warning, you _must_ discard the returned slice object before calling into this service
/// again, as otherwise undefined behavior might happen.
SLICEU32 service_various_slices_return_slice(SERVICEVARIOUSSLICES* CONTEXT);

/// Warning, you _must_ discard the returned slice object before calling into this service
/// again, as otherwise undefined behavior might happen.
SLICEMUTU32 service_various_slices_return_slice_mut(SERVICEVARIOUSSLICES* CONTEXT);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
FFIERROR service_strings_destroy(SERVICESTRINGS** CONTEXT);

FFIERROR service_strings_new(SERVICESTRINGS** CONTEXT);

void service_strings_pass_string(SERVICESTRINGS* CONTEXT, const char* ANON1);

const char* service_strings_return_string(SERVICESTRINGS* CONTEXT);


#ifdef __cplusplus
}
#endif

#endif /* interoptopus_generated */
