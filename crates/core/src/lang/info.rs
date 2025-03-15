//! Helpers to introspect Rust code when generating bindings, mostly derived by the `#[ffi_...]` macros.

use crate::lang::enums::Variant;
use crate::lang::function::{FunctionSignature, Parameter};
use crate::lang::{Array, Constant, FnPointer, Function, Primitive, PrimitiveValue, Type};
use std::mem::MaybeUninit;
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
pub unsafe trait TypeInfo {
    fn type_info() -> Type;
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
        unsafe impl crate::lang::TypeInfo for $rust_type {
            fn type_info() -> Type {
                Type::Primitive($primitive)
            }
        }
    };
}

macro_rules! impl_const_value_primitive {
    (
        $rust_type:ty,
        $x:path
    ) => {
        impl From<$rust_type> for crate::lang::ConstantValue {
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

impl_ctype_primitive!(std::ffi::c_void, Primitive::Void);
impl_ctype_primitive!((), Primitive::Void);
impl_ctype_primitive!(u8, Primitive::U8);
impl_ctype_primitive!(u16, Primitive::U16);
impl_ctype_primitive!(u32, Primitive::U32);
impl_ctype_primitive!(u64, Primitive::U64);
impl_ctype_primitive!(i8, Primitive::I8);
impl_ctype_primitive!(i16, Primitive::I16);
impl_ctype_primitive!(i32, Primitive::I32);
impl_ctype_primitive!(i64, Primitive::I64);
impl_ctype_primitive!(f32, Primitive::F32);
impl_ctype_primitive!(f64, Primitive::F64);
impl_ctype_primitive!(bool, Primitive::Bool);
impl_ctype_primitive!(std::num::NonZeroU8, Primitive::U8);
impl_ctype_primitive!(std::num::NonZeroU16, Primitive::U16);
impl_ctype_primitive!(std::num::NonZeroU32, Primitive::U32);
impl_ctype_primitive!(std::num::NonZeroU64, Primitive::U64);
impl_ctype_primitive!(std::num::NonZeroI8, Primitive::I8);
impl_ctype_primitive!(std::num::NonZeroI16, Primitive::I16);
impl_ctype_primitive!(std::num::NonZeroI32, Primitive::I32);
impl_ctype_primitive!(std::num::NonZeroI64, Primitive::I64);
impl_ctype_primitive!(Option<std::num::NonZeroU8>, Primitive::U8);
impl_ctype_primitive!(Option<std::num::NonZeroU16>, Primitive::U16);
impl_ctype_primitive!(Option<std::num::NonZeroU32>, Primitive::U32);
impl_ctype_primitive!(Option<std::num::NonZeroU64>, Primitive::U64);
impl_ctype_primitive!(Option<std::num::NonZeroI8>, Primitive::I8);
impl_ctype_primitive!(Option<std::num::NonZeroI16>, Primitive::I16);
impl_ctype_primitive!(Option<std::num::NonZeroI32>, Primitive::I32);
impl_ctype_primitive!(Option<std::num::NonZeroI64>, Primitive::I64);

unsafe impl<T> TypeInfo for NonNull<T>
where
    T: TypeInfo,
{
    fn type_info() -> Type {
        Type::ReadWritePointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> TypeInfo for &'_ T
where
    T: TypeInfo + Sized + 'static,
{
    fn type_info() -> Type {
        Type::ReadPointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> TypeInfo for &'_ mut T
where
    T: TypeInfo + Sized + 'static,
{
    fn type_info() -> Type {
        Type::ReadWritePointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> TypeInfo for *const T
where
    T: TypeInfo + Sized + 'static,
{
    fn type_info() -> Type {
        Type::ReadPointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> TypeInfo for *mut T
where
    T: TypeInfo + Sized + 'static,
{
    fn type_info() -> Type {
        Type::ReadWritePointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> TypeInfo for Option<&'_ T>
where
    T: TypeInfo + Sized + 'static,
{
    fn type_info() -> Type {
        Type::ReadPointer(Box::new(T::type_info()))
    }
}

unsafe impl<T> TypeInfo for Option<&'_ mut T>
where
    T: TypeInfo + Sized + 'static,
{
    fn type_info() -> Type {
        Type::ReadWritePointer(Box::new(T::type_info()))
    }
}

unsafe impl<R> TypeInfo for extern "C" fn() -> R
where
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(vec![], R::type_info());
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<R> TypeInfo for Option<extern "C" fn() -> R>
where
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(vec![], R::type_info());
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, R> TypeInfo for extern "C" fn(T1) -> R
where
    T1: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(vec![Parameter::new("x0".to_string(), T1::type_info())], R::type_info());
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, R> TypeInfo for Option<extern "C" fn(T1) -> R>
where
    T1: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(vec![Parameter::new("x0".to_string(), T1::type_info())], R::type_info());
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, T2, R> TypeInfo for extern "C" fn(T1, T2) -> R
where
    T1: TypeInfo,
    T2: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(vec![Parameter::new("x0".to_string(), T1::type_info()), Parameter::new("x1".to_string(), T2::type_info())], R::type_info());
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, T2, R> TypeInfo for Option<extern "C" fn(T1, T2) -> R>
where
    T1: TypeInfo,
    T2: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(vec![Parameter::new("x0".to_string(), T1::type_info()), Parameter::new("x1".to_string(), T2::type_info())], R::type_info());
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, T2, T3, R> TypeInfo for extern "C" fn(T1, T2, T3) -> R
where
    T1: TypeInfo,
    T2: TypeInfo,
    T3: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
            ],
            R::type_info(),
        );
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, T2, T3, R> TypeInfo for Option<extern "C" fn(T1, T2, T3) -> R>
where
    T1: TypeInfo,
    T2: TypeInfo,
    T3: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
            ],
            R::type_info(),
        );
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, T2, T3, T4, R> TypeInfo for extern "C" fn(T1, T2, T3, T4) -> R
where
    T1: TypeInfo,
    T2: TypeInfo,
    T3: TypeInfo,
    T4: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
                Parameter::new("x3".to_string(), T4::type_info()),
            ],
            R::type_info(),
        );
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, T2, T3, T4, R> TypeInfo for Option<extern "C" fn(T1, T2, T3, T4) -> R>
where
    T1: TypeInfo,
    T2: TypeInfo,
    T3: TypeInfo,
    T4: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
        let sig = FunctionSignature::new(
            vec![
                Parameter::new("x0".to_string(), T1::type_info()),
                Parameter::new("x1".to_string(), T2::type_info()),
                Parameter::new("x2".to_string(), T3::type_info()),
                Parameter::new("x3".to_string(), T4::type_info()),
            ],
            R::type_info(),
        );
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, T2, T3, T4, T5, R> TypeInfo for extern "C" fn(T1, T2, T3, T4, T5) -> R
where
    T1: TypeInfo,
    T2: TypeInfo,
    T3: TypeInfo,
    T4: TypeInfo,
    T5: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
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
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T1, T2, T3, T4, T5, R> TypeInfo for Option<extern "C" fn(T1, T2, T3, T4, T5) -> R>
where
    T1: TypeInfo,
    T2: TypeInfo,
    T3: TypeInfo,
    T4: TypeInfo,
    T5: TypeInfo,
    R: TypeInfo,
{
    fn type_info() -> Type {
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
        Type::FnPointer(FnPointer::new(sig))
    }
}

unsafe impl<T, const N: usize> TypeInfo for [T; N]
where
    T: TypeInfo,
{
    fn type_info() -> Type {
        Type::Array(Array::new(T::type_info(), N))
    }
}

unsafe impl<T> TypeInfo for MaybeUninit<T>
where
    T: TypeInfo,
{
    fn type_info() -> Type {
        T::type_info()
    }
}
