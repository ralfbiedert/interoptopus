use interoptopus::{ffi_constant, ffi_function, ffi_type};

/// Something something circle.
#[ffi_constant]
const MY_CONST: u32 = 314;

/// A vector used in our game.
#[ffi_type]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A function which does something with the vector.
#[ffi_function]
#[no_mangle]
pub extern "C" fn my_function(input: Vec3) -> Vec3 {
    Vec3 { x: 2.0, y: 4.0, z: input.z }
}

// This will create a function `ffi_inventory` which can produce
// an abstract FFI representation (called `Library`) of this crate.
interoptopus::inventory!(ffi_inventory, [MY_CONST], [my_function], [], []);

// Small hack, we use a unit test to invoke `ffi_inventory` and produce our backend.
#[test]
fn generate_bindings() -> Result<(), interoptopus::Error> {
    use interoptopus::writer::IndentWriter;
    use interoptopus::Interop;
    use interoptopus_backend_csharp::{Config, Generator};

    let generator = Generator::new(Config::default(), ffi_inventory());

    let mut tmp: Vec<u8> = Vec::new();
    let mut writer = IndentWriter::new(&mut tmp);

    // Only writes to memory, replace with `write_file` for real interop.
    generator.write_to(&mut writer)?;

    Ok(())
}
