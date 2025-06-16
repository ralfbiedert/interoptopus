//! A protobuf-like marshaller across the rust-ffi border.<sup>🚧</sup>

use std::io::{Error, ErrorKind, Read, Result, Write};

pub trait Ser {
    fn ser(&self, out: &mut impl Write) -> Result<()>;
}

pub trait De {
    fn de(input: &mut impl Read) -> Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_primitive_wire {
    ($($ty:ty),+) => {
        $(
        impl Ser for $ty {
            fn ser(&self, out: &mut impl Write) -> Result<()> {
                out.write_all(&self.to_le_bytes())
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

impl Ser for bool {
    fn ser(&self, out: &mut impl Write) -> Result<()> {
        out.write_all(&u8::from(*self).to_le_bytes())
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

impl Ser for String {
    fn ser(&self, out: &mut impl Write) -> Result<()> {
        self.len().ser(out)?;
        out.write_all(self.as_bytes())
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

// String -> serialize as Vec<u8> but maybe Vec<u16> - see which is faster
// ✅ Vec<T> - u32 len + this many T's
// HashMap<T,U> - u32 len + this many (T,U)'s
// (), (T,...)
// ✅ Option<T> - bool + maybe T
// ✅ bool - 1u8 or 0u8
// arbitrary Structs - all fields in order of declaration
//
// Additionally, support serializing into C#-provided buffer.

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
                    assert_eq!($container[counter], $seq);
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

        assert_eq!(u_repr[0], 0x00);
        assert_eq!(u_repr[1], 0xb0);
        assert_eq!(u_repr[2], 0xb7);
        assert_eq!(u_repr[3], 0x90);
        assert_eq!(u_repr[4], 0x42);
        assert_eq!(u_repr[5], 0xc7);
        assert_eq!(u_repr[6], 0x52);
        assert_eq!(u_repr[7], 0x80);

        assert_eq!(w_repr[0], 0x80);
        assert_eq!(w_repr[1], 0x9e);
        assert_eq!(w_repr[2], 0x03);
        assert_eq!(w_repr[3], 0xda);
        assert_eq!(w_repr[4], 0xdb);
        assert_eq!(w_repr[5], 0x5e);
        assert_eq!(w_repr[6], 0xfa);
        assert_eq!(w_repr[7], 0xc6);
        assert_eq!(w_repr[8], 0x09);
        assert_eq!(w_repr[9], 0x4a);
        assert_eq!(w_repr[10], 0xf0);
        assert_eq!(w_repr[11], 0xfe);
        assert_eq!(w_repr[12], 0xff);
        assert_eq!(w_repr[13], 0xff);
        assert_eq!(w_repr[14], 0xff);
        assert_eq!(w_repr[15], 0xff);

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

        cursor.seek(SeekFrom::Start(0))?;

        let mut none_repr = [0u8; 1];
        let mut some_repr = [0u8; 2];
        cursor.read_exact(&mut none_repr)?;
        cursor.read_exact(&mut some_repr)?;

        assert_eq!(none_repr[0], 0x00);

        assert_seq_eq!(some_repr, 0x01, 13);

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

        cursor.seek(SeekFrom::Start(0))?;

        match core::mem::size_of::<usize>() {
            8 => {
                let mut v1_repr = [0u8; 8 + 3];
                let mut v2_repr = [0u8; 8];
                cursor.read_exact(&mut v1_repr)?;
                cursor.read_exact(&mut v2_repr)?;

                assert_seq_eq!(v1_repr, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03);

                assert_seq_eq!(v2_repr, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
            }
            4 => {
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

        cursor.seek(SeekFrom::Start(0))?;

        match core::mem::size_of::<usize>() {
            8 => {
                let mut s1_repr = [0u8; 8 + 11];
                let mut s2_repr = [0u8; 8 + 22];

                cursor.read_exact(&mut s1_repr)?;
                cursor.read_exact(&mut s2_repr)?;

                assert_seq_eq!(s1_repr, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100);

                assert_seq_eq!(
                    s2_repr, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 115, 101, 108, 195, 162, 109, 32, 97, 108, 101, 121, 107, 195, 188, 109, 32, 100, 195, 188,
                    110, 121, 97
                );
            }
            4 => {
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

        cursor.seek(SeekFrom::Start(0))?;

        let deserialized_s1 = String::de(&mut cursor)?;
        let deserialized_s2 = String::de(&mut cursor)?;

        assert_eq!(deserialized_s1, s1);
        assert_eq!(deserialized_s2, s2);
        Ok(())
    }
}

// Generate serialization code on both sides, Rust and backend's language, to transfer
// type T over the FFI border in a byte array package.
// struct Wire<T> {}

// unsafe impl<T> TypeInfo for Slice<'_, T>
// where
//     T: TypeInfo,
// {
//     #[rustfmt::skip]
//     fn type_info() -> Type {
//     }
// }
