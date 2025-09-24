use interoptopus::ffi;

#[ffi(packed)]
pub struct Packed1 {
    pub x: u8,
    pub y: u16,
}

#[ffi(packed)]
pub struct Packed2 {
    pub y: u16,
    pub x: u8,
}

// UNSUPPORTED FOR NOW - At least C# and Python seem to have issues doing this correctly.
// #[ffi_type(align = 2)]
// pub struct Aligned1 {
//     pub x: u8,
//     pub y: u16,
// }
//
// #[ffi_type(align = 64)]
// pub struct Aligned2 {
//     pub x: u8,
//     pub y: u16,
// }
