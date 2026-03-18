use interoptopus::wire::WireBuffer;

#[test]
fn wire_ownership() {
    // Create Wire with owned data
    let owned_wire_buffer = WireBuffer::with_size(64);
    assert!(owned_wire_buffer.is_owned());

    // Create Wire with borrowed data
    let mut buffer = vec![0; 64];
    let borrowed_wire_buffer = WireBuffer::from_slice(&mut buffer);
    assert!(!borrowed_wire_buffer.is_owned());
}

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
