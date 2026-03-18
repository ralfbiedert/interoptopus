use prost::Message;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/deeply_nested.rs"));
}

#[unsafe(no_mangle)]
pub extern "C" fn protobuf_deeply_nested_1(data: *const u8, len: usize) -> u32 {
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    let msg = proto::DeeplyNestedWire1::decode(slice).unwrap();
    let entry = msg.values.values().next().unwrap();
    let inner = entry.values.first().unwrap();
    let nested = inner.x.values().next().unwrap();
    nested.a
}
