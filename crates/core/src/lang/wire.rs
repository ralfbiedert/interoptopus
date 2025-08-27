//! A protobuf-like marshaller across the rust-ffi border.<sup>🚧</sup>
//! Wire<T> helpers to de-/serialize built-in types. <sup>:🚧</sup>
//
// ✅ String -> serialize as Vec<u8> but maybe Vec<u16> - see which is faster
// ✅ Vec<T> - usize len + this many T's
// ✅ HashMap<T,U> - usize len + this many (T,U)'s
// ✅ (), (T,...)
// ✅ Option<T> - bool + maybe T
// ✅ bool - 1u8 or 0u8
// ✅ arbitrary Structs - all fields in order of declaration
//
// Additionally, support serializing externally provided buffer (hopefully from C#).
//
// Generate serialization code on both sides, Rust and backend's language, to transfer
// type T over the FFI border in a byte array package.

use crate::lang::{Composite, Docs, DomainType, Field, Meta, Primitive, Type, TypeInfo};
use std::marker::PhantomData;
use std::{
    collections::HashMap,
    io::{Read, Write},
};

// @todo play with implementing it as a struct?
#[derive(thiserror::Error, Debug)]
pub enum WireError {
    #[error("I/O error {0}")]
    Io(#[from] std::io::Error),
    #[error("(De-)serialization error {0}")]
    InvalidData(String),
    #[error("Invalid discriminant {1} while deserializing {0}")]
    InvalidDiscriminant(String, usize),
}

pub trait WireInfo {
    /// Provide a rust-side name of the type. Backends can convert it to their corresponding types.
    fn name() -> &'static str;
    /// Is this type layout fixed, or has variable elements. Useful for optimizing collection size calculations.
    fn is_fixed_size_element() -> bool;
    /// Provide a Type description, this is used to collect type hierarchy of wire types.
    fn wire_info() -> Type;
}

impl WireInfo for bool {
    fn name() -> &'static str {
        "bool"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::Bool)
    }
}

impl WireInfo for i8 {
    fn name() -> &'static str {
        "i8"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::I8)
    }
}
impl WireInfo for i16 {
    fn name() -> &'static str {
        "i16"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::I16)
    }
}
impl WireInfo for i32 {
    fn name() -> &'static str {
        "i32"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::I32)
    }
}
impl WireInfo for i64 {
    fn name() -> &'static str {
        "i64"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::I64)
    }
}

impl WireInfo for u8 {
    fn name() -> &'static str {
        "u8"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::U8)
    }
}
impl WireInfo for u16 {
    fn name() -> &'static str {
        "u16"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::U16)
    }
}
impl WireInfo for u32 {
    fn name() -> &'static str {
        "u32"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::U32)
    }
}
impl WireInfo for u64 {
    fn name() -> &'static str {
        "u64"
    }
    fn is_fixed_size_element() -> bool {
        true
    }
    fn wire_info() -> Type {
        Type::Primitive(Primitive::U64)
    }
}

impl<T> WireInfo for Vec<T>
where
    T: WireInfo,
{
    fn name() -> &'static str {
        "Vec<T>" // @todo
    }
    fn is_fixed_size_element() -> bool {
        false
    }
    fn wire_info() -> Type {
        Type::Domain(DomainType::Vec(Box::new(T::wire_info())))
    }
}

impl<T> WireInfo for Option<T>
where
    T: WireInfo,
{
    fn name() -> &'static str {
        "Option<T>" // @todo
    }
    fn is_fixed_size_element() -> bool {
        false
    }
    fn wire_info() -> Type {
        Type::Domain(DomainType::Option(Box::new(T::wire_info())))
    }
}

impl<T, U, S> WireInfo for HashMap<T, U, S>
where
    T: WireInfo,
    U: WireInfo,
{
    fn name() -> &'static str {
        "HashMap<T,U>" // @todo
    }
    fn is_fixed_size_element() -> bool {
        false
    }
    fn wire_info() -> Type {
        Type::Domain(DomainType::Map(Box::new(T::wire_info()), Box::new(U::wire_info())))
    }
}

impl WireInfo for String {
    fn name() -> &'static str {
        "String"
    }
    fn is_fixed_size_element() -> bool {
        false
    }
    fn wire_info() -> Type {
        Type::Domain(DomainType::String)
    }
}

/// FFI buffer that can represent both owned and borrowed data
#[repr(C)]
pub struct WireBuffer<'a> {
    data: *mut u8,
    len: i32,
    capacity: i32,
    _phantom: PhantomData<&'a [u8]>,
}

unsafe impl Send for WireBuffer<'_> {}
unsafe impl Sync for WireBuffer<'_> {}

impl<'a> WireBuffer<'a> {
    /// Create a new owned buffer from a Vec
    #[must_use]
    pub fn from_vec(mut vec: Vec<u8>) -> WireBuffer<'static> {
        let data = vec.as_mut_ptr();
        let len = i32::try_from(vec.len()).expect("Too large Wire buffer!");
        let capacity = i32::try_from(vec.capacity()).expect("Too large Wire buffer!");

        std::mem::forget(vec); // LEAKS the vec here, must use deallocate_wire_buffer_storage() to free it

        WireBuffer { data, len, capacity, _phantom: PhantomData }
    }

    /// Create a new borrowed buffer from a slice
    #[allow(clippy::use_self, reason = "We want to keep the explicit lifetime")]
    #[must_use]
    pub fn from_slice(slice: &'a mut [u8]) -> WireBuffer<'a> {
        WireBuffer {
            data: slice.as_mut_ptr(),
            len: i32::try_from(slice.len()).expect("Too large Wire buffer!"),
            capacity: 0, // indicates borrowed
            _phantom: PhantomData,
        }
    }

    /// Create an empty owned buffer with capacity
    #[must_use]
    pub fn with_size(size: usize) -> WireBuffer<'static> {
        WireBuffer::from_vec(vec![0u8; size])
    }

    /// Get length of the buffer
    #[must_use]
    pub fn len(&self) -> usize {
        usize::try_from(self.len).expect("Invalid Wire buffer len")
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Check if this buffer owns its data
    #[must_use]
    pub const fn is_owned(&self) -> bool {
        self.capacity != 0
    }

    /// Get a slice view of the buffer
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.data.cast_const(), self.len()) }
        }
    }

    /// Get a mutable slice access to the buffer
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.data, self.len()) }
        }
    }

    #[must_use]
    pub fn reader(&self) -> impl Read {
        WireBufferReader::new(self)
    }

    #[must_use]
    pub fn writer(&mut self) -> impl Write + '_ {
        WireBufferWriter::new(self)
    }
}

/// Allows serializing types into a wire buffer storage.
struct WireBufferWriter<'a, 'b> {
    buf: &'a mut WireBuffer<'b>,
    pos: usize,
}

impl<'a, 'b> WireBufferWriter<'a, 'b> {
    pub fn new(buf: &'a mut WireBuffer<'b>) -> Self {
        Self { buf, pos: 0 }
    }
}

impl std::io::Write for WireBufferWriter<'_, '_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let data = self.buf.as_slice_mut();
        let remaining = data.len().saturating_sub(self.pos);
        let to_copy = std::cmp::min(remaining, buf.len());

        if to_copy > 0 {
            data[self.pos..self.pos + to_copy].copy_from_slice(&buf[..to_copy]);
            self.pos += to_copy;
        }

        Ok(to_copy)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Allows deserializing types from a wire buffer storage.
/// This is NOT a zerocopy implementation.
struct WireBufferReader<'a> {
    buf: &'a WireBuffer<'a>,
    pos: usize,
}

impl<'a> WireBufferReader<'a> {
    pub fn new(buf: &'a WireBuffer) -> Self {
        Self { buf, pos: 0 }
    }
}

impl std::io::Read for WireBufferReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let data = self.buf.as_slice();
        let remaining = data.len().saturating_sub(self.pos);
        let to_copy = std::cmp::min(remaining, buf.len());

        if to_copy > 0 {
            buf[..to_copy].copy_from_slice(&data[self.pos..self.pos + to_copy]);
            self.pos += to_copy;
        }

        Ok(to_copy)
    }
}

impl Drop for WireBuffer<'_> {
    fn drop(&mut self) {
        // Explicitly do nothing here as the wire buffer must be deallocated on the other side.
    }
}

/// This is exposed to C# to deallocate memory from C# side (in `WireOfT`'s Dispose).
///
/// # Safety
///
/// If you pass wrong values here, everything will blow up. For use only by generated `WireOfT` wrappers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn deallocate_wire_buffer_storage(data: *mut u8, len: i32, capacity: i32) {
    if capacity <= 0 {
        // If the buffer was borrowed or allocated on the opposite FFI side, cannot deallocate it.
        return;
    }
    let _ = unsafe { Vec::from_raw_parts(data, usize::try_from(len).expect("Invalid vec length"), usize::try_from(capacity).expect("Invalid vec capacity")) };
}

unsafe impl TypeInfo for WireBuffer<'_> {
    fn type_info() -> Type {
        let fields = vec![
            Field::new("data".to_string(), Type::ReadPointer(Box::new(Type::Primitive(Primitive::U8)))),
            Field::new("len".to_string(), Type::Primitive(Primitive::I32)),
            Field::new("capacity".to_string(), Type::Primitive(Primitive::I32)),
        ];

        let docs = Docs::from_lines(vec!["FFI buffer for Wire data transfer".to_string()]);
        let composite = Composite::with_meta("WireBuffer".to_string(), fields, Meta::with_docs(docs));

        Type::Composite(composite)
    }
}

/// Create a Wire from a type, either by allocating or by taking an external buffer.
pub trait Wireable
where
    Self: Ser + De,
{
    fn wire<'my>(&self) -> Wire<'my, Self>;

    fn wire_with_buffer<'a>(&self, buf: &'a mut [u8]) -> Wire<'a, Self>;
}

/// Unwire into the original Base type.
pub trait Unwireable {
    type Base;
    fn unwire(&mut self) -> Result<Self::Base, WireError>;
}

/// Blanket implementation to let any Wire<T> to be unwireable back to T.
impl<T> Unwireable for Wire<'_, T>
where
    T: Ser + De,
{
    type Base = T;

    fn unwire(&mut self) -> Result<Self::Base, WireError> {
        self.unwire()
    }
}

/// A struct that wraps a byte buffer passed through FFI boundary, allows serialization of a value of type T.
///
/// The backing storage uses a ptr+size representation that can safely cross FFI boundaries.
///
/// # Examples
///
/// ## Creating owned Wire (allocates new buffer):
/// ```rust
/// use interoptopus::lang::Wire;
///
/// // Pre-allocated owned buffer
/// let wire: Wire<String> = Wire::with_size(1024);
/// assert!(wire.is_owned());
/// ```
///
/// ## Creating borrowed Wire (uses external buffer):
/// ```rust
/// use interoptopus::lang::Wire;
///
/// let mut buffer = [0u8; 15];
/// let wire: Wire<String> = Wire::new_with_buffer(&mut buffer);
/// assert!(!wire.is_owned());
/// ```
///
/// ## FFI Usage Example:
/// ```rust
/// use interoptopus::lang::{Wire, Wireable};
///
/// extern "C" fn process_data(mut input: Wire<String>) -> Wire<String> {
///     let string = input.unwire();
///
///     // Process and return new Wire
///     String::from("reply").wire()
/// }
/// ```
///
/// ## Complete C# Integration Example
///
/// When using the C# backend, the following Rust code:
/// ```rust,ignore
/// #[ffi_type(wired)]
/// pub struct UserData {
///     pub name: String,
///     pub age: u32,
///     pub active: bool,
/// }
///
/// #[ffi_function]
/// pub fn process_user(user: Wire<UserData>) -> Wire<UserData> {
///     let mut data = user.unwire().unwrap();
///     data.age += 1;
///     data.active = true;
///     data.wire()
/// }
/// ```
///
/// Can be used from C# code:
/// ```csharp
/// public static UserData ProcessUser(UserData user)
/// {
///     int bufferSize = user.WireSize();
///     Span<byte> buffer = stackalloc byte[bufferSize];
///
///     // Pin the GC memory so it is not moved while calling Rust.
///     fixed (byte* bufferPtr = buffer)
///     {
///         var wireInput = WireOfUserData.From(user, bufferPtr, bufferSize);
///         var wireResult = process_user(wireInput);
///
///         try
///         {
///             return wireResult.Unwire();
///         }
///         finally
///         {
///             wireResult.Dispose();
///         }
///     }
/// }
/// ```
///
#[repr(C)]
pub struct Wire<'my, T>
where
    T: Ser + De + ?Sized,
{
    buf: WireBuffer<'my>,          // FFI-safe storage either owned or borrowed
    _phantom: PhantomData<&'my T>, // behaves like a lifetimed reference
}

impl<'a, T: Ser + De> Wire<'a, T> {
    /// Creates a new Wire with owned storage pre-allocated to the given capacity
    #[must_use]
    pub fn with_size(capacity: usize) -> Wire<'static, T> {
        Wire { buf: WireBuffer::with_size(capacity), _phantom: PhantomData }
    }

    /// Creates a new Wire with borrowed storage from the provided buffer
    #[allow(clippy::use_self)]
    #[must_use]
    pub fn new_with_buffer(buffer: &'a mut [u8]) -> Wire<'a, T> {
        Wire { buf: WireBuffer::from_slice(buffer), _phantom: PhantomData }
    }

    pub fn serialize(&mut self, value: &T) -> Result<(), WireError> {
        value.ser(&mut self.buf.writer())
    }

    // FIXME: Consume self?
    pub fn unwire(&mut self) -> Result<T, WireError> {
        T::de(&mut self.buf.reader())
    }

    // /// Get a pointer to the buffer data
    // pub fn as_ptr(&self) -> *const u8 {
    //     self.buf.data as *const u8
    // }

    // /// Get the length of the buffer
    // pub fn len(&self) -> u64 {
    //     self.buf.len
    // }

    // /// Get the capacity of the buffer
    // pub fn capacity(&self) -> u64 {
    //     self.buf.capacity
    // }

    /// Check if this Wire owns its buffer data
    #[must_use]
    pub fn is_owned(&self) -> bool {
        self.buf.is_owned()
    }

    /// Get a slice view of the buffer data
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.buf.as_slice()
    }
}

impl<T> Wireable for T
where
    T: Ser + De + 'static,
{
    fn wire<'my>(&self) -> Wire<'my, Self> {
        let size = self.storage_size();
        let mut wire = Wire::with_size(size);
        wire.serialize(self).expect("Failed to serialize"); // TODO: return Result here
        wire
    }

    fn wire_with_buffer<'a>(&self, buf: &'a mut [u8]) -> Wire<'a, Self> {
        let mut wire = Wire::new_with_buffer(buf);
        wire.serialize(self).expect("Failed to serialize"); // TODO: return Result here
        wire
    }
}

unsafe impl<T> TypeInfo for Wire<'_, T>
where
    T: Ser + De + WireInfo,
{
    fn type_info() -> Type {
        let fields = vec![Field::new("buf".to_string(), WireBuffer::type_info())];

        let docs = Docs::from_lines(vec!["Wired data FFI wrapper".to_string()]);

        let composite = Composite::with_meta(T::name().to_string(), fields, Meta::with_module_docs(T::wire_info().namespace().unwrap_or_default().to_string(), docs));

        // The root Wire<T> types are Wired, this makes backend generate WireOfT handling code.
        // All inner types are Domain types.
        Type::Wired(composite)
    }
}

// Wire means:
// On CSharp side:
// - allocate and pin buffer
// - serialize WireOfInput to that buffer
// - pass it over to the fn
// On Rust side:
// - deserializ from Wire<Input>'s buffer into Input
// - do stuff with input
// - allocate or borrow Wire<Output>'s buffer
// - serialize Output into Wire buffer
// - pass Wire<Output> over to C#
// On CSharp side:
// - deserialize from WireOfOutput to Output
// - drop rust buffer
// - unpin and drop CSharp buffer for WireOfInput
//
// WireOfInput takes Input and writes it into a pinned buf
// Wire<Input> takes buf SLICE and deserializes Input from there
// Wire<Output> takes owned buf and serializes Output to it
// WireOfOutput takes buf over ffi and deserializes Output from it

// // for fn service_name(input: Wire<Input>, input2: Wire<Input>) -> Wire<Output>;
// fixed (var buf = new byte[input.estimated_size()+input2.estimated_size()]) {
//     WireOfInput.serialize(input, buf);
//     WireOfInput.serialize(input2, buf+input.estimated_size());
//     var out = service_name(buf);
//     var output = WireOfOutput.deserialize(out);
// }

// Wire<Input>::de(buf_slice)->Input

/// A trait that domain types should implement (via ffi wired attribute) to be able to send itself in a Wire.
pub trait Ser {
    /// Write self into the buffer addressed by `out`
    fn ser(&self, out: &mut impl Write) -> Result<(), WireError>;

    /// Calculate amount of storage needed for writing self
    fn storage_size(&self) -> usize;
}

/// A trait that domain types should implement (via ffi wired attribute) to be able to receive itself from a Wire.
/// This is not zerocopy!
pub trait De {
    /// Read contents of type Self from the reader `input`
    fn de(input: &mut impl Read) -> Result<Self, WireError>
    where
        Self: Sized;
}

/// Implement Ser and De for all primitive types
macro_rules! impl_primitive_wire {
    ($($ty:ty),+) => {
        $(
        impl $crate::lang::wire::Ser for $ty {
            fn ser(&self, out: &mut impl Write) -> Result<(), WireError> {
                out.write_all(&self.to_le_bytes()).map_err(WireError::Io)
            }

            fn storage_size(&self) -> usize {
                std::mem::size_of::<$ty>()
            }
        }

        impl $crate::lang::wire::De for $ty {
            fn de(input: &mut impl Read) -> Result<Self, WireError> {
                let mut bytes = [0; std::mem::size_of::<$ty>()];
                input.read_exact(&mut bytes)?;
                Ok(<$ty>::from_le_bytes(bytes))
            }
        }
        )*
    };
}

impl_primitive_wire!(i8, i16, i32, i64, i128, isize);
impl_primitive_wire!(u8, u16, u32, u64, u128, usize);
impl_primitive_wire!(f32, f64);

impl Ser for bool {
    fn ser(&self, out: &mut impl Write) -> Result<(), WireError> {
        out.write_all(&u8::from(*self).to_le_bytes()).map_err(WireError::Io)
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl De for bool {
    fn de(input: &mut impl Read) -> Result<Self, WireError> {
        let mut bytes = [0; 1];
        input.read_exact(&mut bytes)?;
        match bytes[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(WireError::InvalidData("Invalid boolean value".into())),
        }
    }
}

impl<T: Ser> Ser for Option<T> {
    fn ser(&self, out: &mut impl Write) -> Result<(), WireError> {
        match self {
            None => false.ser(out),
            Some(t) => {
                true.ser(out)?;
                t.ser(out)
            }
        }
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<bool>() + self.as_ref().map_or(0, Ser::storage_size)
    }
}

impl<T: De> De for Option<T> {
    #[allow(clippy::match_bool)]
    fn de(input: &mut impl Read) -> Result<Self, WireError> {
        let t = bool::de(input)?;
        match t {
            false => Ok(None),
            true => Ok(Some(T::de(input)?)),
        }
    }
}

impl<T: Ser> Ser for Vec<T> {
    fn ser(&self, out: &mut impl Write) -> Result<(), WireError> {
        self.len().ser(out)?;
        for item in self {
            item.ser(out)?;
        }
        Ok(())
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<usize>() + self.iter().map(Ser::storage_size).sum::<usize>()
    }
}

impl<T: De> De for Vec<T> {
    fn de(input: &mut impl Read) -> Result<Self, WireError> {
        let len = usize::de(input)?;
        let mut me = Self::with_capacity(len);
        for _ in 0..len {
            me.push(T::de(input)?);
        }
        Ok(me)
    }
}

impl<K: Ser, V: Ser, S> Ser for HashMap<K, V, S> {
    fn ser(&self, out: &mut impl Write) -> Result<(), WireError> {
        self.len().ser(out)?;
        for item in self {
            item.0.ser(out)?;
            item.1.ser(out)?;
        }
        Ok(())
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<usize>() + self.iter().map(|item| item.0.storage_size() + item.1.storage_size()).sum::<usize>()
    }
}

impl<K: De + Eq + core::hash::Hash, V: De, S: ::std::hash::BuildHasher + Default> De for HashMap<K, V, S> {
    fn de(input: &mut impl Read) -> Result<Self, WireError> {
        let len = usize::de(input)?;
        let mut me = Self::with_capacity_and_hasher(len, Default::default());
        for _ in 0..len {
            let k = K::de(input)?;
            let v = V::de(input)?;
            me.insert(k, v);
        }
        Ok(me)
    }
}

impl Ser for String {
    fn ser(&self, out: &mut impl Write) -> Result<(), WireError> {
        self.len().ser(out)?;
        out.write_all(self.as_bytes()).map_err(WireError::Io)
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<usize>() + self.len()
    }
}

// don't need a Read but a Cursor - we need to make sure a sufficient sized slice exist and create string from it directly
// i.e. ensure_readable(len); String::from_utf8(&buf[..len])
impl De for String {
    fn de(input: &mut impl Read) -> Result<Self, WireError> {
        let len = usize::de(input)?;
        let mut s = Self::with_capacity(len);
        input.take(len as u64).read_to_string(&mut s)?; // TODO: ensure read result equals len
        Ok(s)
    }
}

macro_rules! impl_tuple_wire {
    ( $( $name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($name: Ser),+> crate::lang::wire::Ser for ($($name,)+)
        {
            fn ser(&self, output: &mut impl Write) -> Result<(), WireError> {
                let ($($name,)+) = self;
                $(
                    $name.ser(output)?;
                )+
                Ok(())
            }

            fn storage_size(&self) -> usize {
                let ($($name,)+) = self;
                0 $(
                    + $name.storage_size()
                )+
            }
        }

        #[allow(non_snake_case)]
        impl<$($name: De),+> crate::lang::wire::De for ($($name,)+)
        {
            fn de(input: &mut impl Read) -> Result<Self, WireError> {
                Ok((
                $(
                    $name::de(input)?,
                )+
                ))
            }
        }
    };
}

impl_tuple_wire! { A }
impl_tuple_wire! { A B }
impl_tuple_wire! { A B C }
impl_tuple_wire! { A B C D }
impl_tuple_wire! { A B C D E }
impl_tuple_wire! { A B C D E F }
impl_tuple_wire! { A B C D E F G }
impl_tuple_wire! { A B C D E F G H }
impl_tuple_wire! { A B C D E F G H I }
impl_tuple_wire! { A B C D E F G H I J }
impl_tuple_wire! { A B C D E F G H I J K }
impl_tuple_wire! { A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom};

    use super::*;

    macro_rules! assert_seq_eq {
        ($container:expr, $($seq:expr),+) => {
            #[allow(unused_assignments)]
            {
                let mut counter = 0;
                $(
                    assert_eq!($container[counter], $seq, "mismatch in byte {counter}");
                    counter += 1;
                )+
            }
        };
    }

    #[test]
    fn u_roundtrip() -> Result<(), WireError> {
        let x = 144u8;
        let y = 61233u16;
        let z = 3253534345u32;
        let u = 18442244000709551615u64;
        let w = 78999999999328478187456873456352387456u128;

        let mut cursor = std::io::Cursor::new(Vec::new());
        x.ser(&mut cursor)?;
        y.ser(&mut cursor)?;
        z.ser(&mut cursor)?;
        u.ser(&mut cursor)?;
        w.ser(&mut cursor)?;

        // Check byte repr in the buffer.
        assert_eq!(x.storage_size(), 1);
        assert_eq!(y.storage_size(), 2);
        assert_eq!(z.storage_size(), 4);
        assert_eq!(u.storage_size(), 8);
        assert_eq!(w.storage_size(), 16);

        cursor.seek(SeekFrom::Start(0))?;
        let mut x_repr = [0u8; 1];
        let mut y_repr = [0u8; 2];
        let mut z_repr = [0u8; 4];
        let mut u_repr = [0u8; 8];
        let mut w_repr = [0u8; 16];

        cursor.read_exact(&mut x_repr)?;
        cursor.read_exact(&mut y_repr)?;
        cursor.read_exact(&mut z_repr)?;
        cursor.read_exact(&mut u_repr)?;
        cursor.read_exact(&mut w_repr)?;

        assert_seq_eq!(x_repr, 0x90);

        assert_seq_eq!(y_repr, 0x31, 0xef);

        assert_seq_eq!(z_repr, 0x89, 0xfe, 0xec, 0xc1);

        assert_seq_eq!(u_repr, 0xff, 0x25, 0x5f, 0x1b, 0x35, 0x03, 0xf0, 0xff);

        assert_seq_eq!(w_repr, 0x80, 0x61, 0xfc, 0x3d, 0xd7, 0x36, 0x8b, 0xed, 0x6b, 0xb7, 0xdd, 0x30, 0xb8, 0xd8, 0x6e, 0x3b);

        // Deserialize back.
        cursor.seek(SeekFrom::Start(0))?;

        let nx = u8::de(&mut cursor)?;
        let ny = u16::de(&mut cursor)?;
        let nz = u32::de(&mut cursor)?;
        let nu = u64::de(&mut cursor)?;
        let nw = u128::de(&mut cursor)?;

        assert_eq!(nx, x);
        assert_eq!(ny, y);
        assert_eq!(nz, z);
        assert_eq!(nu, u);
        assert_eq!(nw, w);
        Ok(())
    }

    #[test]
    fn i_roundtrip() -> Result<(), WireError> {
        let x = -128i8;
        let y = -32000i16;
        let z = -2100500900i32;
        let u = -9200072000054775808i64;
        let w = -328478187456873456352387456i128;

        let mut cursor = std::io::Cursor::new(Vec::new());
        x.ser(&mut cursor)?;
        y.ser(&mut cursor)?;
        z.ser(&mut cursor)?;
        u.ser(&mut cursor)?;
        w.ser(&mut cursor)?;

        // Check byte repr in the buffer.
        assert_eq!(x.storage_size(), 1);
        assert_eq!(y.storage_size(), 2);
        assert_eq!(z.storage_size(), 4);
        assert_eq!(u.storage_size(), 8);
        assert_eq!(w.storage_size(), 16);

        cursor.seek(SeekFrom::Start(0))?;
        let mut x_repr = [0u8; 1];
        let mut y_repr = [0u8; 2];
        let mut z_repr = [0u8; 4];
        let mut u_repr = [0u8; 8];
        let mut w_repr = [0u8; 16];

        cursor.read_exact(&mut x_repr)?;
        cursor.read_exact(&mut y_repr)?;
        cursor.read_exact(&mut z_repr)?;
        cursor.read_exact(&mut u_repr)?;
        cursor.read_exact(&mut w_repr)?;

        assert_seq_eq!(x_repr, 0x80);

        assert_seq_eq!(y_repr, 0x00, 0x83);

        assert_seq_eq!(z_repr, 0x5c, 0xe6, 0xcc, 0x82);

        assert_seq_eq!(u_repr, 0x00, 0xb0, 0xb7, 0x90, 0x42, 0xc7, 0x52, 0x80);

        assert_seq_eq!(w_repr, 0x80, 0x9e, 0x03, 0xda, 0xdb, 0x5e, 0xfa, 0xc6, 0x09, 0x4a, 0xf0, 0xfe, 0xff, 0xff, 0xff, 0xff);

        // Deserialize back.
        cursor.seek(SeekFrom::Start(0))?;

        let nx = i8::de(&mut cursor)?;
        let ny = i16::de(&mut cursor)?;
        let nz = i32::de(&mut cursor)?;
        let nu = i64::de(&mut cursor)?;
        let nw = i128::de(&mut cursor)?;

        assert_eq!(nx, x);
        assert_eq!(ny, y);
        assert_eq!(nz, z);
        assert_eq!(nu, u);
        assert_eq!(nw, w);
        Ok(())
    }

    #[test]
    fn option_roundtrip() -> Result<(), WireError> {
        let none = None;
        let some = Some(13u8);

        let mut cursor = std::io::Cursor::new(Vec::new());
        none.ser(&mut cursor)?;
        some.ser(&mut cursor)?;

        // Check byte repr in the buffer.
        cursor.seek(SeekFrom::Start(0))?;

        assert_eq!(none.storage_size(), 1);
        assert_eq!(some.storage_size(), 2);

        let mut none_repr = [0u8; 1];
        let mut some_repr = [0u8; 2];
        cursor.read_exact(&mut none_repr)?;
        cursor.read_exact(&mut some_repr)?;

        assert_seq_eq!(none_repr, 0x00);

        assert_seq_eq!(some_repr, 0x01, 13);

        // Deserialize back.
        cursor.seek(SeekFrom::Start(0))?;

        let deserialized_none = Option::<u8>::de(&mut cursor)?;
        let deserialized_some = Option::<u8>::de(&mut cursor)?;

        assert_eq!(deserialized_none, none);
        assert_eq!(deserialized_some, some);
        Ok(())
    }

    #[test]
    fn vec_roundtrip() -> Result<(), WireError> {
        let v1 = vec![0x1u8, 0x2, 0x3];
        let v2 = Vec::<u8>::new();

        let mut cursor = std::io::Cursor::new(Vec::new());
        v1.ser(&mut cursor)?;
        v2.ser(&mut cursor)?;

        // Check byte repr in the buffer.
        cursor.seek(SeekFrom::Start(0))?;

        match core::mem::size_of::<usize>() {
            8 => {
                assert_eq!(v1.storage_size(), 8 + 3);
                assert_eq!(v2.storage_size(), 8);

                let mut v1_repr = [0u8; 8 + 3];
                let mut v2_repr = [0u8; 8];
                cursor.read_exact(&mut v1_repr)?;
                cursor.read_exact(&mut v2_repr)?;

                #[rustfmt::skip]
                assert_seq_eq!(v1_repr,
                    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x01, 0x02, 0x03);

                #[rustfmt::skip]
                assert_seq_eq!(v2_repr,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
            }
            4 => {
                assert_eq!(v1.storage_size(), 4 + 3);
                assert_eq!(v2.storage_size(), 4);

                let mut v1_repr = [0u8; 4 + 3];
                let mut v2_repr = [0u8; 4];
                cursor.read_exact(&mut v1_repr)?;
                cursor.read_exact(&mut v2_repr)?;

                assert_seq_eq!(v1_repr, 0x03, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03);

                assert_seq_eq!(v2_repr, 0x00, 0x00, 0x00, 0x00);
            }
            _ => {
                unimplemented!("We don't know how to test this weird size of usize")
            }
        }

        // Deserialize back.
        cursor.seek(SeekFrom::Start(0))?;

        let deserialized_v1 = Vec::<u8>::de(&mut cursor)?;
        let deserialized_v2 = Vec::<u8>::de(&mut cursor)?;

        assert_eq!(deserialized_v1, v1);
        assert_eq!(deserialized_v2, v2);
        Ok(())
    }

    #[test]
    fn string_roundtrip() -> Result<(), WireError> {
        let s1 = String::from("Hello world");
        let s2 = String::from("selâm aleyküm dünya");

        let mut cursor = std::io::Cursor::new(Vec::new());
        s1.ser(&mut cursor)?;
        s2.ser(&mut cursor)?;

        // Check byte repr in the buffer.
        cursor.seek(SeekFrom::Start(0))?;

        match core::mem::size_of::<usize>() {
            8 => {
                assert_eq!(s1.storage_size(), 8 + 11);
                assert_eq!(s2.storage_size(), 8 + 22);

                let mut s1_repr = [0u8; 8 + 11];
                let mut s2_repr = [0u8; 8 + 22];

                cursor.read_exact(&mut s1_repr)?;
                cursor.read_exact(&mut s2_repr)?;

                #[rustfmt::skip]
                assert_seq_eq!(s1_repr,
                    0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100);

                #[rustfmt::skip]
                assert_seq_eq!(
                    s2_repr,
                    0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    115, 101, 108, 195, 162, 109, 32, 97, 108, 101, 121, 107,
                    195, 188, 109, 32, 100, 195, 188, 110, 121, 97
                );
            }
            4 => {
                assert_eq!(s1.storage_size(), 4 + 11);
                assert_eq!(s2.storage_size(), 4 + 22);

                let mut s1_repr = [0u8; 4 + 11];
                let mut s2_repr = [0u8; 4 + 22];

                cursor.read_exact(&mut s1_repr)?;
                cursor.read_exact(&mut s2_repr)?;

                assert_seq_eq!(s1_repr, 0x0b, 0x00, 0x00, 0x00, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100);

                assert_seq_eq!(s2_repr, 0x16, 0x00, 0x00, 0x00, 115, 101, 108, 195, 162, 109, 32, 97, 108, 101, 121, 107, 195, 188, 109, 32, 100, 195, 188, 110, 121, 97);
            }
            _ => {
                unimplemented!("We don't know how to test this weird size of usize")
            }
        }

        // Deserialize back.
        cursor.seek(SeekFrom::Start(0))?;

        let deserialized_s1 = String::de(&mut cursor)?;
        let deserialized_s2 = String::de(&mut cursor)?;

        assert_eq!(deserialized_s1, s1);
        assert_eq!(deserialized_s2, s2);
        Ok(())
    }

    #[test]
    fn hashmap_roundtrip() -> Result<(), WireError> {
        use rustc_hash::FxSeededState;

        // Create maps with fixed seed so they keep ordering for serialization tests.
        let mut h1 = HashMap::<String, u16, FxSeededState>::with_hasher(FxSeededState::with_seed(123));
        let mut h2 = HashMap::<u16, Vec<bool>, FxSeededState>::with_hasher(FxSeededState::with_seed(123));

        h1.insert("First".into(), 0x11aa);
        h1.insert("Second".into(), 0x22bb);
        h2.insert(0x22bb, vec![true, true, false]);
        h2.insert(0x11aa, vec![false, true, true]);

        let mut cursor = std::io::Cursor::new(Vec::new());
        h1.ser(&mut cursor)?;
        h2.ser(&mut cursor)?;

        // Check byte repr in the buffer.
        cursor.seek(SeekFrom::Start(0))?;

        match core::mem::size_of::<usize>() {
            8 => {
                assert_eq!(h1.storage_size(), 8 + 8 + 5 + 2 + 8 + 6 + 2);
                assert_eq!(h2.storage_size(), 8 + 2 + 8 + 3 + 2 + 8 + 3);

                let mut h1_repr = [0u8; 8 + 8 + 5 + 2 + 8 + 6 + 2];
                let mut h2_repr = [0u8; 8 + 2 + 8 + 3 + 2 + 8 + 3];

                cursor.read_exact(&mut h1_repr)?;
                cursor.read_exact(&mut h2_repr)?;

                #[rustfmt::skip]
                assert_seq_eq!(
                    h1_repr,
                    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    70, 105, 114, 115, 116,
                    0xaa, 0x11,
                    0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    83, 101, 99, 111, 110, 100,
                    0xbb, 0x22
                );

                #[rustfmt::skip]
                assert_seq_eq!(
                    h2_repr,
                    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0xaa, 0x11,
                    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0, 1, 1,
                    0xbb, 0x22,
                    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    1, 1, 0
                );
            }
            4 => {
                assert_eq!(h1.storage_size(), 4 + 4 + 5 + 2 + 4 + 6 + 2);
                assert_eq!(h2.storage_size(), 4 + 2 + 4 + 3 + 2 + 4 + 3);

                let mut h1_repr = [0u8; 4 + 4 + 5 + 2];
                let mut h2_repr = [0u8; 4 + 2 + 4 + 3];

                cursor.read_exact(&mut h1_repr)?;
                cursor.read_exact(&mut h2_repr)?;

                #[rustfmt::skip]
                assert_seq_eq!(
                    h1_repr,
                    0x02, 0x00, 0x00, 0x00,
                    0x05, 0x00, 0x00, 0x00,
                    70, 105, 114, 115, 116,
                    0xaa, 0x11,
                    0x06, 0x00, 0x00, 0x00,
                    83, 101, 99, 111, 110, 100,
                    0xbb, 0x22
                );

                #[rustfmt::skip]
                assert_seq_eq!(
                    h2_repr,
                    0x02, 0x00, 0x00, 0x00,
                    0xaa, 0x11,
                    0x03, 0x00, 0x00, 0x00,
                    0, 1, 1,
                    0xbb, 0x22,
                    0x03, 0x00, 0x00, 0x00,
                    1, 1, 0
                );
            }
            _ => {
                unimplemented!("We don't know how to test this weird size of usize")
            }
        }

        // Deserialize back.
        cursor.seek(SeekFrom::Start(0))?;

        let deserialized_h1 = HashMap::<String, u16>::de(&mut cursor)?;
        let mut comparable_h1 = HashMap::<String, u16, FxSeededState>::with_hasher(FxSeededState::with_seed(123));
        comparable_h1.extend(deserialized_h1);

        let deserialized_h2 = HashMap::<u16, Vec<bool>>::de(&mut cursor)?;
        let mut comparable_h2 = HashMap::<u16, Vec<bool>, FxSeededState>::with_hasher(FxSeededState::with_seed(123));
        comparable_h2.extend(deserialized_h2);

        assert_eq!(comparable_h1, h1);
        assert_eq!(comparable_h2, h2);
        Ok(())
    }

    #[test]
    fn tuple_roundtrip() -> Result<(), WireError> {
        let a = (8u32, "Hello world".to_string(), vec![1, 2, 3]);

        let mut cursor = std::io::Cursor::new(Vec::new());
        a.ser(&mut cursor)?;

        // Check byte repr in the buffer.
        cursor.seek(SeekFrom::Start(0))?;
        let mut a_repr = [0u8; 43];

        cursor.read_exact(&mut a_repr)?;

        match core::mem::size_of::<usize>() {
            8 => {
                assert_eq!(a.storage_size(), 4 + 8 + 11 + 8 + 4 + 4 + 4);

                #[rustfmt::skip]
                assert_seq_eq!(a_repr,
                    0x08, 0x00, 0x00, 0x00,
                    0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100,
                    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x01, 0x00, 0x00, 0x00,
                    0x02, 0x00, 0x00, 0x00,
                    0x03, 0x00, 0x00, 0x00);
            }
            4 => {
                assert_eq!(a.storage_size(), 4 + 4 + 11 + 4 + 4 + 4 + 4);

                #[rustfmt::skip]
                assert_seq_eq!(a_repr,
                    0x08, 0x00, 0x00, 0x00,
                    0x0b, 0x00, 0x00, 0x00,
                    72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100,
                    0x03, 0x00, 0x00, 0x00,
                    0x01, 0x00, 0x00, 0x00,
                    0x02, 0x00, 0x00, 0x00,
                    0x03, 0x00, 0x00, 0x00);
            }
            _ => {
                unimplemented!("We don't know how to test this weird size of usize")
            }
        }

        // Deserialize back.
        cursor.seek(SeekFrom::Start(0))?;

        let deserialized_a = <(u32, String, Vec<u32>)>::de(&mut cursor)?;

        assert_eq!(deserialized_a, a);

        Ok(())
    }

    #[test]
    fn wire_ownership() {
        // Create Wire with owned data
        let owned_wire: Wire<String> = Wire::with_size(64);
        assert!(owned_wire.is_owned());

        // Create Wire with borrowed data
        let mut buffer = vec![0; 64];
        let borrowed_wire: Wire<String> = Wire::new_with_buffer(&mut buffer);
        assert!(!borrowed_wire.is_owned());
    }

    #[test]
    fn simple_wire_roundtrip() {
        extern "C" fn ffi_function(mut wire: Wire<String>) -> Wire<String> {
            let s = wire.unwire().unwrap();
            s.wire()
        }

        // The function can be called with our Wire types
        let test_wire = "hello world".to_string().wire();
        assert_eq!(ffi_function(test_wire).unwire().unwrap(), "hello world".to_string());
    }

    // TODO: move to real project perhaps, this needs deps
    // #[test]
    // fn wire_type_name_generation() {
    //     // Test that Wire<T> generates correct type names for C# binding

    //     // Create a test struct
    //     use crate::{ffi_type, lang::Type};

    //     #[ffi_type(wired)]
    //     struct TestStruct {
    //         field: u32,
    //     }

    //     // Get type info for Wire<TestStruct>
    //     let wire_type_info = <Wire<TestStruct> as TypeInfo>::type_info();

    //     // Should be Type::Wired containing TestStruct's composite info
    //     match wire_type_info {
    //         Type::Wired(composite) => {
    //             // The composite name should be "TestStruct", not "WireTestStruct"
    //             assert_eq!(composite.rust_name(), "TestStruct");
    //         }
    //         _ => panic!("Expected Type::Wired for Wire<TestStruct>"),
    //     }

    //     // This ensures C# backend will generate "WireOfTestStruct" not "WireOfWireTestStruct"
    // }

    #[test]
    fn wire_buffer_reader_test() {
        use std::io::Read;

        const BUF_SIZE: usize = 64;

        // Test with borrowed data
        let mut data = vec![0; BUF_SIZE];
        let buffer = WireBuffer::from_slice(&mut data);
        let mut reader = buffer.reader();

        // Read full buffer
        let mut output = vec![0u8; BUF_SIZE];
        let bytes_read = reader.read(&mut output).unwrap();
        assert_eq!(bytes_read, BUF_SIZE);
        // assert_eq!(output, data);

        // Read again should return 0 (EOF)
        let mut output2 = vec![0u8; 10];
        let bytes_read2 = reader.read(&mut output2).unwrap();
        assert_eq!(bytes_read2, 0);

        // Test with owned data
        let owned_data = vec![1, 2, 3, 4, 5];
        let owned_buffer = WireBuffer::from_vec(owned_data.clone());
        let mut owned_reader = owned_buffer.reader();

        // Read partial data
        let mut partial_output = vec![0u8; 3];
        let partial_bytes_read = owned_reader.read(&mut partial_output).unwrap();
        assert_eq!(partial_bytes_read, 3);
        assert_eq!(partial_output, &owned_data[0..3]);

        // Read remaining data
        let mut remaining_output = vec![0u8; 5];
        let remaining_bytes_read = owned_reader.read(&mut remaining_output).unwrap();
        assert_eq!(remaining_bytes_read, 2);
        assert_eq!(&remaining_output[0..2], &owned_data[3..5]);
    }
}
