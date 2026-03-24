use interoptopus::ffi;
use interoptopus::wire::Wire;

#[ffi]
pub fn wire_accept_byte_array(mut input: Wire<[u8; 32]>) -> u8 {
    let arr = input.unwire();
    arr[0]
}

#[ffi]
pub fn wire_return_byte_array() -> Wire<[u8; 32]> {
    Wire::from([42u8; 32])
}
