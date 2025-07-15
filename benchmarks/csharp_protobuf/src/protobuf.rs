use crate::protobuf_models;
use prost::Message;
use std::{ffi::c_void, slice};

// Rust client to benchmark Protobuf based serialization/deserialization speed.
// # Safety
/// This function is unsafe because it dereferences raw pointers.
/// The caller must ensure that the pointers are valid and the lengths are correct.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ProtoRustClient(
    serialized_input: *const u8,
    serialized_input_length: usize,
    serialized_output: *mut *mut c_void,
    serialized_output_length: *mut u32,
) {
    // SAFETY: Ensure the pointer is valid and the length is correct
    let byte_slice = unsafe { slice::from_raw_parts(serialized_input, serialized_input_length) };

    // Deserialize the byte array argument into Input
    let _input = match protobuf_models::Input::decode(byte_slice) {
        Ok(input) => input,
        Err(e) => {
            let output = protobuf_models::Outputs {
                response: None,
                data: Some(protobuf_models::Data { items: None, errors: Some(protobuf_models::Error { error_messages: vec![format!("Failed to deserialize input: {e}")] }) }),
            };
            unsafe {
                write_output(output, serialized_output, serialized_output_length);
            }
            return;
        }
    };

    // ==========================================================================================
    // TODO: Fill in the response payload based on some fields in the Input payload (i.e. some N)
    // ==========================================================================================
    let response = protobuf_models::Response { results: vec![protobuf_models::Result { item_id: 1.to_string(), item_value: 42 }] };
    let data = protobuf_models::Data { items: None, errors: Some(protobuf_models::Error { error_messages: vec![format!("Failed to deserialize input")] }) };

    // Create the output object
    let output = protobuf_models::Outputs { response: Some(response), data: Some(data) };

    // Serialize the response
    unsafe {
        write_output(output, serialized_output, serialized_output_length);
    }
}

unsafe fn write_output(output: protobuf_models::Outputs, serialized_output: *mut *mut c_void, serialized_output_length: *mut u32) {
    let mut encoded_response = Vec::new();
    if let Err(e) = output.encode(&mut encoded_response) {
        // If we can't encode the response, try to encode an error response
        let error_output = protobuf_models::Outputs {
            response: None,
            data: Some(protobuf_models::Data { items: None, errors: Some(protobuf_models::Error { error_messages: vec![format!("Failed to serialize response: {e}")] }) }),
        };

        if error_output.encode(&mut encoded_response).is_err() {
            // If we still can't encode, give up
            return; // !! No error indication here in protobuf case, we can do betterer.
        }
    }

    // SAFETY: Ensure the pointer is valid and the length is correct
    unsafe {
        *serialized_output_length = u32::try_from(encoded_response.len()).expect("Length exceeds u32");
    }
    // SAFETY: Ensure the pointer is valid and the length is correct
    unsafe {
        let result_boxed = encoded_response.into_boxed_slice();
        *serialized_output = Box::into_raw(result_boxed).cast::<c_void>();
    }
}

// TODO: this should not be necessary to Wire<T> implementantion, only needed for Protobuf
/// # Safety
/// This function is unsafe because it dereferences raw pointers.
/// The caller must ensure that the pointers are valid and the lengths are correct.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRustResultMemory(ptr: *mut u8, len: usize) {
    // SAFETY: Reclaim ownership
    let slice_ptr = unsafe { slice::from_raw_parts_mut(ptr, len) };
    // SAFETY: Deallocate memory
    _ = unsafe { Box::from_raw(slice_ptr) };
}
