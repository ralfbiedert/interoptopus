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
// Additionally, support serializing into C#-provided buffer.
//
// Generate serialization code on both sides, Rust and backend's language, to transfer
// type T over the FFI border in a byte array package.

use crate::ffi;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Read, Result, Write},
};

/// A struct that wraps a byte buffer passed through FFI boundary, assuming it
/// contains serialized representation of a value of type T.
///
/// The backing storage is either a slice of foreign buffer, or an owned ffi::Vec<u8>.
#[repr(C)]
pub struct Wire<'my, T>
where
    T: Ser + De,
{
    buf: Cow<'my, ffi::Slice<'my, u8>>, // storage received from wherever
    _phantom: PhantomData<&'my T>,      // behaves like a lifetimed reference
}

impl<T: Ser + De> Wire<'_, T> {
    pub fn serialize(&self, _value: &T) -> Result<()> {
        // let mut cursor = Cursor::new(self.buf);
        // t.ser(&mut cursor)
        Ok(())
    }

    pub fn deserialize(&mut self) -> Result<T> {
        use std::ops::Deref;
        T::de(&self.buf.deref())
    }
}

impl<T: Ser + De> From<T> for Wire<'_, T> {
    // serialize value?
    fn from(_value: T) -> Self {
        Wire {
            buf: ffi::Vec::from(vec![]), // TODO: serialize value into buf
            _type: PhantomData,
        }
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
// - unpin and drop CSharp buffer
//
// WireOfInput takes Input and writes it into a pinned buf
// Wire<Input> takes buf SLICE and deserializes Input from there
// Wire<Output> takes owned buf and serializes Output to it
// WireOfOutput takes buf over ffi and deserializes Output from it

// // @todo: make this an extension method?
// class WireOfInput {
//     static void serialize(Input input, byte[] buf) {
//         // have friend access to Input and write it into buf field by field
//         input.field1.ser(buf);
//         input.field2.ser(buf);
//     }
// }

// // for fn service_name(input: Wire<Input>, input2: Wire<Input>) -> Wire<Output>;
// fixed (var buf = new byte[input.estimated_size()+input2.estimated_size()]) {
//     WireOfInput.serialize(input, buf);
//     WireOfInput.serialize(input2, buf+input.estimated_size());
//     var out = service_name(buf);
//     var output = WireOfOutput.deserialize(out);
// }

// Wire<Input>::de(buf_slice)->Input

pub trait Ser {
    /// Write self into the buffer addressed by `out`
    fn ser(&self, out: &mut impl Write) -> Result<()>;

    /// Calculate amount of storage needed for writing self
    fn storage_size(&self) -> usize;
}

pub trait De {
    /// Read contents of type Self from the reader `input`
    fn de(input: &mut impl Read) -> Result<Self>
    where
        Self: Sized;
}

/// Implement Ser and De for all primitive types
macro_rules! impl_primitive_wire {
    ($($ty:ty),+) => {
        $(
        impl Ser for $ty {
            fn ser(&self, out: &mut impl Write) -> Result<()> {
                out.write_all(&self.to_le_bytes())
            }

            fn storage_size(&self) -> usize {
                std::mem::size_of::<$ty>()
            }
        }

        impl De for $ty {
            fn de(input: &mut impl Read) -> Result<Self> {
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
    fn ser(&self, out: &mut impl Write) -> Result<()> {
        out.write_all(&u8::from(*self).to_le_bytes())
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<bool>()
    }
}

impl De for bool {
    fn de(input: &mut impl Read) -> Result<Self> {
        let mut bytes = [0; 1];
        input.read_exact(&mut bytes)?;
        match bytes[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid boolean value")),
        }
    }
}

impl<T: Ser> Ser for Option<T> {
    fn ser(&self, out: &mut impl Write) -> Result<()> {
        match self {
            None => false.ser(out),
            Some(t) => {
                true.ser(out)?;
                t.ser(out)
            }
        }
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<bool>() + self.as_ref().map_or(0, |t| t.storage_size())
    }
}

impl<T: De> De for Option<T> {
    fn de(input: &mut impl Read) -> Result<Self> {
        let t = bool::de(input)?;
        match t {
            false => Ok(None),
            true => Ok(Some(T::de(input)?)),
        }
    }
}

impl<T: Ser> Ser for Vec<T> {
    fn ser(&self, out: &mut impl Write) -> Result<()> {
        self.len().ser(out)?;
        for item in self.iter() {
            item.ser(out)?;
        }
        Ok(())
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<usize>() + self.iter().map(|item| item.storage_size()).sum::<usize>()
    }
}

impl<T: De> De for Vec<T> {
    fn de(input: &mut impl Read) -> Result<Self> {
        let len = usize::de(input)?;
        let mut me = Vec::with_capacity(len);
        for _ in 0..len {
            me.push(T::de(input)?);
        }
        Ok(me)
    }
}

impl<K: Ser, V: Ser, S> Ser for HashMap<K, V, S> {
    fn ser(&self, out: &mut impl Write) -> Result<()> {
        self.len().ser(out)?;
        for item in self.iter() {
            item.0.ser(out)?;
            item.1.ser(out)?;
        }
        Ok(())
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<usize>() + self.iter().map(|item| item.0.storage_size() + item.1.storage_size()).sum::<usize>()
    }
}

impl<K: De + Eq + core::hash::Hash, V: De> De for HashMap<K, V> {
    fn de(input: &mut impl Read) -> Result<Self> {
        let len = usize::de(input)?;
        let mut me = HashMap::<K, V>::with_capacity(len);
        for _ in 0..len {
            let k = K::de(input)?;
            let v = V::de(input)?;
            me.insert(k, v);
        }
        Ok(me)
    }
}

impl Ser for String {
    fn ser(&self, out: &mut impl Write) -> Result<()> {
        self.len().ser(out)?;
        out.write_all(self.as_bytes())
    }

    fn storage_size(&self) -> usize {
        std::mem::size_of::<usize>() + self.len()
    }
}

// don't need a Read but a Cursor - we need to make sure a sufficient sized slice exist and create string from it directly
// i.e. ensure_readable(len); String::from_utf8(&buf[..len])
//read.take(len).read_to_string() ?
impl De for String {
    fn de(input: &mut impl Read) -> Result<Self> {
        let len = usize::de(input)?;
        let mut s = String::with_capacity(len);
        input.take(len as u64).read_to_string(&mut s)?; // ensure read result equals len
        Ok(s)
    }
}

macro_rules! impl_tuple_wire {
    ( $( $name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($name: Ser),+> Ser for ($($name,)+)
        {
            fn ser(&self, output: &mut impl Write) -> Result<()> {
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
        impl<$($name: De),+> De for ($($name,)+)
        {
            fn de(input: &mut impl Read) -> Result<Self> {
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
    fn u_roundtrip() -> Result<()> {
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
    fn i_roundtrip() -> Result<()> {
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
    fn option_roundtrip() -> Result<()> {
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
    fn vec_roundtrip() -> Result<()> {
        let v1 = vec![0x1u8, 0x2, 0x3];
        let v2 = vec![];

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
    fn string_roundtrip() -> Result<()> {
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
    fn hashmap_roundtrip() -> Result<()> {
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
    fn tuple_roundtrip() -> Result<()> {
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
}
