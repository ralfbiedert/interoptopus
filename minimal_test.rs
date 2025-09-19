// Test file to verify the #[ffi_type] macro implementation

use interoptopus::lang::types::TypeInfo;

#[interoptopus_proc::ffi_type]
struct Simple {
    x: u32,
}

#[interoptopus_proc::ffi_type(name = "Renamed")]
struct WithName {
    y: u32,
}

#[interoptopus_proc::ffi_type]
enum BasicEnum {
    A,
    B(u32),
}

#[interoptopus_proc::ffi_type(packed)]
struct Packed {
    x: u8,
    y: u16,
}

#[test]
fn test_type_info_works() {
    // Just verify that the TypeInfo trait is implemented
    let _id = Simple::id();
    let _kind = Simple::kind();
    let _ty = Simple::ty();

    let _id2 = WithName::id();
    let _id3 = BasicEnum::id();
    let _id4 = Packed::id();
}