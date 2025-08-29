use interoptopus::wire::{De, Ser, Wire, WireBuffer, WireError, Wireable};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

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
#[expect(clippy::cognitive_complexity, clippy::many_single_char_names)]
fn u_roundtrip() -> Result<(), WireError> {
    let x = 144_u8;
    let y = 61_233_u16;
    let z = 3_253_534_345_u32;
    let u = 18_442_244_000_709_551_615_u64;
    let w = 78_999_999_999_328_478_187_456_873_456_352_387_456_u128;

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
#[expect(clippy::cognitive_complexity, clippy::many_single_char_names)]
fn i_roundtrip() -> Result<(), WireError> {
    let x = -128_i8;
    let y = -32_000_i16;
    let z = -2_100_500_900_i32;
    let u = -9_200_072_000_054_775_808_i64;
    let w = -328_478_187_456_873_456_352_387_456_i128;

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
#[expect(clippy::cognitive_complexity)]
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
#[expect(clippy::cognitive_complexity)]
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
#[expect(clippy::cognitive_complexity)]
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
#[expect(clippy::cognitive_complexity)]
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
