use interoptopus::ffi;
use interoptopus::wire::Wire;
use std::collections::HashMap;

#[ffi]
pub struct DeeplyNestedWire4 {
    a: u32,
}

#[ffi]
pub struct DeeplyNestedWire3 {
    x: HashMap<u32, DeeplyNestedWire4>,
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
pub fn wire_deeply_nested_1(mut x: Wire<DeeplyNestedWire1>) -> u32 {
    let w = x.try_unwire().unwrap();
    let entry = w.values.values().next().unwrap();
    let inner = entry.values.first().unwrap();
    let nested = inner.x.values().next().unwrap();
    nested.a
}

#[ffi]
pub fn wire_deeply_nested_2(_: DeeplyNestedWire4) {}
