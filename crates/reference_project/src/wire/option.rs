use interoptopus::ffi;
use interoptopus::wire::Wire;
use std::collections::HashMap;

#[ffi]
pub struct OptionLeaf {
    pub tags: HashMap<String, String>,
    pub score_1: ffi::Option<u32>,
    pub score_2: Option<u32>,
    pub name_1: Option<String>,
    pub name_2: ffi::Option<String>,
}

#[ffi]
pub struct OptionMiddle {
    pub label: String,
    pub leaf_1: Option<OptionLeaf>,
    pub leaf_2: ffi::Option<OptionLeaf>,
}

#[ffi]
pub struct OptionRoot {
    pub id: u32,
    pub middle_1: Option<OptionMiddle>,
    pub middle_2: ffi::Option<OptionMiddle>,
    pub items: Vec<OptionLeaf>,
}

/// Round-trips the root, proving serialize/deserialize works for all option variants.
#[ffi]
pub fn wire_option_1(x: Wire<OptionRoot>) -> Wire<OptionRoot> {
    x
}

/// Returns the sum of all non-None scores across all items.
#[ffi]
pub fn wire_option_2(mut x: Wire<OptionRoot>) -> u32 {
    let root = x.try_unwire().unwrap();
    let mut sum = 0u32;
    for leaf in &root.items {
        if let ffi::Option::Some(s) = leaf.score_1 {
            sum += s;
        }
        if let Some(s) = leaf.score_2 {
            sum += s;
        }
    }
    sum
}

/// Returns how many `name_1` fields are `Some` across all items.
#[ffi]
pub fn wire_option_3(mut x: Wire<OptionRoot>) -> u32 {
    let root = x.try_unwire().unwrap();
    root.items.iter().filter(|leaf| leaf.name_1.is_some()).count() as u32
}

/// Reads `middle_1.leaf_1.score_2` if all layers are present, else returns 0.
#[ffi]
pub fn wire_option_4(mut x: Wire<OptionRoot>) -> u32 {
    let root = x.try_unwire().unwrap();
    root.middle_1
        .and_then(|m| m.leaf_1)
        .and_then(|l| l.score_2)
        .unwrap_or(0)
}
