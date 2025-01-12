//! Helpers to introspect Rust code when generating bindings, mostly derived by the `#[ffi_...]` macros.

use crate::lang::c::{ArrayType, CType, Constant, ConstantValue, FnPointerType, Function, FunctionSignature, Parameter, PrimitiveType, PrimitiveValue, Variant};
use std::ptr::NonNull;

/// Implemented for a constant-helper produced by [`ffi_constant`](crate::ffi_constant), gives meta info for a constant.
///
/// # Safety
///
/// This trait must be implemented correctly, or else the generated bindings and this constant will
/// disagree in type or value, causing UB.
pub unsafe trait ConstantInfo {
    fn constant_info() -> Constant;
}

/// Implemented for a type via [`ffi_type`](crate::ffi_type), gives meta info for a type.
///
/// # Safety
///
/// This trait must be implemented correctly, or else the generated bindings disagree in
/// their type layout from the actual Rust type, leading to immediate UB upon function invocation.
pub unsafe trait CTypeInfo {
    fn type_info() -> CType;
}

/// Implemented for a function-helper produced by [`ffi_function`](crate::ffi_function), gives meta info for a function.
///
/// # Safety
///
/// This trait must be implemented correctly, or else the generated bindings signatures disagree from
/// their Rust counterparts, leading to immediate UB upon function invocation.
pub unsafe trait FunctionInfo {
    /// The function as a `fn` type.
    type Signature;

    // TODO: Code gen for this is total mess, ignore.
    // The Callback<> surrogate type to obtain proper CTypes when our extern "C" fn() can't be inferred
    // properly because of lifetimes.
    // type Surrogate;

    fn function_info() -> Function;
}

/// Implemented for an enum via [`ffi_type`](crate::ffi_type) allows us to translate a variant into its meta information.
///
/// # Safety
///
/// This trait must be implemented correctly, or else the generated bindings disagree on variant values
/// with their Rust counterparts, leading to UB when invoked with non-existent numbers.
pub unsafe trait VariantInfo {
    fn variant_info(&self) -> Variant;
}

macro_rules! impl_ctype_primitive {
    (
        $rust_type:ty,
        $primitive:expr
    ) => {
        unsafe impl CTypeInfo for $rust_type {
            fn type_info() -> CType {
                CType::Primitive($primitive)
            }
        }
    };
}

macro_rules! impl_const_value_primitive {
    (
        $rust_type:ty,
        $x:path
    ) => {
        impl From<$rust_type> for ConstantValue {
            fn from(x: $rust_type) -> Self {
                Self::Primitive($x(x))
            }
        }
    };
}

impl_const_value_primitive!(u8, PrimitiveValue::U8);
impl_const_value_primitive!(u16, PrimitiveValue::U16);
impl_const_value_primitive!(u32, PrimitiveValue::U32);
impl_const_value_primitive!(u64, PrimitiveValue::U64);
impl_const_value_primitive!(i8, PrimitiveValue::I8);
impl_const_value_primitive!(i16, PrimitiveValue::I16);
impl_const_value_primitive!(i32, PrimitiveValue::I32);
impl_const_value_primitive!(i64, PrimitiveValue::I64);
impl_const_value_primitive!(f32, PrimitiveValue::F32);
impl_const_value_primitive!(f64, PrimitiveValue::F64);
impl_const_value_primitive!(bool, PrimitiveValue::Bool);

impl_ctype_primitive!(std::ffi::c_void, PrimitiveType::Void);
impl_ctype_primitive!((), PrimitiveType::Void);
impl_ctype_primitive!(u8, PrimitiveType::U8);
impl_ctype_primitive!(u16, PrimitiveType::U16);
impl_ctype_primitive!(u32, PrimitiveType::U32);
impl_ctype_primitive!(u64, PrimitiveType::U64);
impl_ctype_primitive!(i8, PrimitiveType::I8);
impl_ctype_primitive!(i16, PrimitiveType::I16);
impl_ctype_primitive!(i32, PrimitiveType::I32);
impl_ctype_primitive!(i64, PrimitiveType::I64);
impl_ctype_primitive!(f32, PrimitiveType::F32);
impl_ctype_primitive!(f64, PrimitiveType::F64);
impl_ctype_primitive!(bool, PrimitiveType::Bool);
impl_ctype_primitive!(std::num::NonZeroU8, PrimitiveType::U8);
impl_ctype_primitive!(std::num::NonZeroU16, PrimitiveType::U16);
impl_ctype_primitive!(std::num::NonZeroU32, PrimitiveType::U32);
impl_ctype_primitive!(std::num::NonZeroU64, PrimitiveType::U64);
impl_ctype_primitive!(std::num::NonZeroI8, PrimitiveType::I8);
impl_ctype_primitive!(std::num::NonZeroI16, PrimitiveType::I16);
impl_ctype_primitive!(std::num::NonZeroI32, PrimitiveType::I32);
impl_ctype_primitive!(std::num::NonZeroI64, PrimitiveType::I64);
impl_ctype_primitive!(Option<std::num::NonZeroU8>, PrimitiveType::U8);
impl_ctype_primitive!(Option<std::num::NonZeroU16>, PrimitiveType::U16);
impl_ctype_primitive!(Option<std::num::NonZeroU32>, PrimitiveType::U32);
impl_ctype_primitive!(Option<std::num::NonZeroU64>, PrimitiveType::U64);
impl_ctype_primitive!(Option<std::num::NonZeroI8>, PrimitiveType::I8);
impl_ctype_primitive!(Option<std::num::NonZeroI16>, PrimitiveType::I16);
impl_ctype_primitive!(Option<std::num::NonZeroI32>, PrimitiveType::I32);
impl_ctype_primitive!(Option<std::num::NonZeroI64>, PrimitiveType::I64);

unsafe impl<T> CTypeInfo for NonNull<T>
where
    T: CTypeInfo,
{
    fn type_info() -> CType {
        CType::ReadWritePointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> CTypeInfo for &'_ T
where
    T: CTypeInfo + Sized + 'static,
{
    fn type_info() -> CType {
        CType::ReadPointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> CTypeInfo for &'_ mut T
where
    T: CTypeInfo + Sized + 'static,
{
    fn type_info() -> CType {
        CType::ReadWritePointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> CTypeInfo for *const T
where
    T: CTypeInfo + Sized + 'static,
{
    fn type_info() -> CType {
        CType::ReadPointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> CTypeInfo for *mut T
where
    T: CTypeInfo + Sized + 'static,
{
    fn type_info() -> CType {
        CType::ReadWritePointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> CTypeInfo for Option<&'_ T>
where
    T: CTypeInfo + Sized + 'static,
{
    fn type_info() -> CType {
        CType::ReadPointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> CTypeInfo for Option<&'_ mut T>
where
    T: CTypeInfo + Sized + 'static,
{
    fn type_info() -> CType {
        CType::ReadWritePointer(Box::new(T::type_info()))
    }
}

unsafe impl<R> CTypeInfo for extern "C" fn() -> R
where
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(vec![], R::type_info());
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<R> CTypeInfo for Option<extern "C" fn() -> R>
where
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(vec![], R::type_info());
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, R> CTypeInfo for extern "C" fn(T1) -> R
where
    T1: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(vec![Parameter::new("x0".to_string(), T1::type_info())], R::type_info());
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, R> CTypeInfo for Option<extern "C" fn(T1) -> R>
where
    T1: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(vec![Parameter::new("x0".to_string(), T1::type_info())], R::type_info());
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, T2, R> CTypeInfo for extern "C" fn(T1, T2) -> R
where
    T1: CTypeInfo,
    T2: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![Parameter::new("x0".to_string(), T1::type_info()), Parameter::new("x1".to_string(), T2::type_info())],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, T2, R> CTypeInfo for Option<extern "C" fn(T1, T2) -> R>
where
    T1: CTypeInfo,
    T2: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![Parameter::new("x0".to_string(), T1::type_info()), Parameter::new("x1".to_string(), T2::type_info())],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, T2, T3, R> CTypeInfo for extern "C" fn(T1, T2, T3) -> R
where
    T1: CTypeInfo,
    T2: CTypeInfo,
    T3: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
            ],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, T2, T3, R> CTypeInfo for Option<extern "C" fn(T1, T2, T3) -> R>
where
    T1: CTypeInfo,
    T2: CTypeInfo,
    T3: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
            ],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, T2, T3, T4, R> CTypeInfo for extern "C" fn(T1, T2, T3, T4) -> R
where
    T1: CTypeInfo,
    T2: CTypeInfo,
    T3: CTypeInfo,
    T4: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
                Parameter::new("x3".to_string(), T4::type_info()),
            ],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, T2, T3, T4, R> CTypeInfo for Option<extern "C" fn(T1, T2, T3, T4) -> R>
where
    T1: CTypeInfo,
    T2: CTypeInfo,
    T3: CTypeInfo,
    T4: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
                Parameter::new("x3".to_string(), T4::type_info()),
            ],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, T2, T3, T4, T5, R> CTypeInfo for extern "C" fn(T1, T2, T3, T4, T5) -> R
where
    T1: CTypeInfo,
    T2: CTypeInfo,
    T3: CTypeInfo,
    T4: CTypeInfo,
    T5: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
                Parameter::new("x3".to_string(), T4::type_info()),
                Parameter::new("x4".to_string(), T5::type_info()),
            ],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T1, T2, T3, T4, T5, R> CTypeInfo for Option<extern "C" fn(T1, T2, T3, T4, T5) -> R>
where
    T1: CTypeInfo,
    T2: CTypeInfo,
    T3: CTypeInfo,
    T4: CTypeInfo,
    T5: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
                Parameter::new("x3".to_string(), T4::type_info()),
                Parameter::new("x4".to_string(), T5::type_info()),
            ],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<T, const N: usize> CTypeInfo for [T; N]
where
    T: CTypeInfo,
{
    fn type_info() -> CType {
        CType::Array(ArrayType::new(T::type_info(), N))
    }
}
