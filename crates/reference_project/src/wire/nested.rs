use interoptopus::ffi;
use interoptopus::lang::types::WireIO;
use interoptopus::wire::Wire;
use std::collections::HashMap;

#[ffi]
pub struct DeeplyNestedWire3 {
    x: HashMap<u32, u32>,
    y: String,
}

#[ffi]
pub struct DeeplyNestedWire2 {
    values: Vec<DeeplyNestedWire3>,
}

#[ffi]
pub struct DeeplyNestedWire1 {
    name: String,
    values: HashMap<u32, DeeplyNestedWire2>,
}

#[ffi]
pub fn wire_deeply_nested(_: Wire<DeeplyNestedWire1>) {}
