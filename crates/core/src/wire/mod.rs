//! Transfer complex object hierarchies over FFI.

mod buffer;
mod error;
mod serde;

pub use buffer::WireBuffer;
pub use error::WireError;
pub use serde::{De, Ser};

use crate::lang::{Composite, Docs, Field, Meta, Type, TypeInfo, WireInfo};
use std::marker::PhantomData;

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

/// Blanket implementation to let any `Wire<T>` to be unwireable back to T.
impl<T> Unwireable for Wire<'_, T>
where
    T: Ser + De,
{
    type Base = T;

    fn unwire(&mut self) -> Result<Self::Base, WireError> {
        self.unwire()
    }
}

/// Wraps and transfers complex objects over FFI.
///
/// The backing storage uses a ptr+size representation that can safely cross FFI boundaries.
///
/// # Examples
///
/// ## Creating owned Wire (allocates new buffer):
/// ```rust
/// use interoptopus::wire::Wire;
///
/// // Pre-allocated owned buffer
/// let wire: Wire<String> = Wire::with_size(1024);
/// assert!(wire.is_owned());
/// ```
///
/// ## Creating borrowed Wire (uses external buffer):
/// ```rust
/// use interoptopus::wire::Wire;
///
/// let mut buffer = [0u8; 15];
/// let wire: Wire<String> = Wire::new_with_buffer(&mut buffer);
/// assert!(!wire.is_owned());
/// ```
///
/// ## FFI Usage Example:
/// ```rust
/// use interoptopus::wire::{Wire, Wireable};
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
    const RAW_SAFE: bool = false;

    fn type_info() -> Type {
        let fields = vec![Field::new("buf".to_string(), WireBuffer::type_info())];

        let docs = Docs::from_lines(vec!["Wired data FFI wrapper".to_string()]);

        let composite = Composite::with_meta(T::name().to_string(), fields, Meta::with_module_docs(T::wire_info().namespace().unwrap_or_default().to_string(), docs));

        // The root Wire<T> types are Wired, this makes backend generate WireOfT handling code.
        // All inner types are Domain types.
        Type::Wire(composite)
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

/// Emits helper functions used by [`Wire`](crate::wire::Wire).
#[macro_export]
macro_rules! builtins_wire {
    () => {{
        use ::interoptopus::lang::FunctionInfo;

        #[$crate::ffi_function]
        pub unsafe extern "C" fn interoptopus_wire_destroy(data: *mut u8, len: i32, capacity: i32) {
            if capacity <= 0 {
                // If the buffer was borrowed or allocated on the opposite FFI side, cannot deallocate it.
                return;
            }
            let _ = unsafe { Vec::from_raw_parts(data, usize::try_from(len).expect("Invalid vec length"), usize::try_from(capacity).expect("Invalid vec capacity")) };
        }

        // #[$crate::ffi_function]
        // pub fn interoptopus_string_destroy(utf8: $crate::pattern::string::String) -> i64 {
        //     0
        // }

        let items = vec![interoptopus_wire_destroy::function_info()];
        let builtins = $crate::pattern::builtins::Builtins::new(items);
        let pattern = $crate::pattern::LibraryPattern::Builtins(builtins);
        $crate::inventory::Symbol::Pattern(pattern)
    }};
}
