//! A protobuf-like marshaller across the rust<->ffi border.<sup>🚧</sup>

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
    use std::io::Seek;

    use super::*;

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
        cursor.seek(std::io::SeekFrom::Start(0))?;
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

        assert_eq!(x_repr[0], 0x90);

        assert_eq!(y_repr[0], 0x31);
        assert_eq!(y_repr[1], 0xef);

        assert_eq!(z_repr[0], 0x89);
        assert_eq!(z_repr[1], 0xfe);
        assert_eq!(z_repr[2], 0xec);
        assert_eq!(z_repr[3], 0xc1);

        assert_eq!(u_repr[0], 0xff);
        assert_eq!(u_repr[1], 0x25);
        assert_eq!(u_repr[2], 0x5f);
        assert_eq!(u_repr[3], 0x1b);
        assert_eq!(u_repr[4], 0x35);
        assert_eq!(u_repr[5], 0x03);
        assert_eq!(u_repr[6], 0xf0);
        assert_eq!(u_repr[7], 0xff);

        assert_eq!(w_repr[0], 0x80);
        assert_eq!(w_repr[1], 0x61);
        assert_eq!(w_repr[2], 0xfc);
        assert_eq!(w_repr[3], 0x3d);
        assert_eq!(w_repr[4], 0xd7);
        assert_eq!(w_repr[5], 0x36);
        assert_eq!(w_repr[6], 0x8b);
        assert_eq!(w_repr[7], 0xed);
        assert_eq!(w_repr[8], 0x6b);
        assert_eq!(w_repr[9], 0xb7);
        assert_eq!(w_repr[10], 0xdd);
        assert_eq!(w_repr[11], 0x30);
        assert_eq!(w_repr[12], 0xb8);
        assert_eq!(w_repr[13], 0xd8);
        assert_eq!(w_repr[14], 0x6e);
        assert_eq!(w_repr[15], 0x3b);

        // Deserialize back.
        cursor.seek(std::io::SeekFrom::Start(0))?;

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
        cursor.seek(std::io::SeekFrom::Start(0))?;
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

        assert_eq!(x_repr[0], 0x80);

        assert_eq!(y_repr[0], 0x00);
        assert_eq!(y_repr[1], 0x83);

        assert_eq!(z_repr[0], 0x5c);
        assert_eq!(z_repr[1], 0xe6);
        assert_eq!(z_repr[2], 0xcc);
        assert_eq!(z_repr[3], 0x82);

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
        cursor.seek(std::io::SeekFrom::Start(0))?;

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
