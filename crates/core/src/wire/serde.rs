use crate::wire::WireError;
use std::collections::HashMap;
use std::io::{Read, Write};

/// Implemented via the ffi wired attribute to be usable inside [`Wire`](crate::wire::Wire).
///
/// This is not zero copy!
pub trait Ser {
    /// Write self into the buffer addressed by `out`
    fn ser(&self, out: &mut impl Write) -> Result<(), WireError>;

    /// Calculate amount of storage needed for writing self
    fn storage_size(&self) -> usize;
}

/// Implemented via the ffi wired attribute to be usable inside [`Wire`](crate::wire::Wire).
///
/// This is not zero copy!
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
        impl $crate::wire::Ser for $ty {
            fn ser(&self, out: &mut impl ::std::io::Write) -> ::std::result::Result<(), $crate::wire::WireError> {
                out.write_all(&self.to_le_bytes()).map_err($crate::wire::WireError::Io)
            }

            fn storage_size(&self) -> usize {
                ::std::mem::size_of::<$ty>()
            }
        }

        impl $crate::wire::De for $ty {
            fn de(input: &mut impl ::std::io::Read) -> ::std::result::Result<Self, $crate::wire::WireError> {
                let mut bytes = [0; ::std::mem::size_of::<$ty>()];
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
        size_of::<Self>()
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
        size_of::<bool>() + self.as_ref().map_or(0, Ser::storage_size)
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
        size_of::<usize>() + self.iter().map(Ser::storage_size).sum::<usize>()
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
        impl<$($name: Ser),+> crate::wire::Ser for ($($name,)+)
        {
            fn ser(&self, output: &mut impl ::std::io::Write) -> ::std::result::Result<(), $crate::wire::WireError> {
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
        impl<$($name: De),+> crate::wire::De for ($($name,)+)
        {
            fn de(input: &mut impl ::std::io::Read) -> ::std::result::Result<Self, $crate::wire::WireError> {
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
