use interoptopus::{ffi_constant, ffi_function, ffi_type};

/// Something something circle.
#[ffi_constant]
const MY_CONST: u32 = 314;

/// A vector used in our game.
#[ffi_type]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec2f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A function which does something with the vector.
#[ffi_function]
#[no_mangle]
pub extern "C" fn my_game_function(input: Option<&Vec2f32>) -> Vec2f32 {
    dbg!(input);

    Vec2f32 { x: 2.0, y: 4.0, z: 6.0 }
}

// This will create a function `ffi_inventory` which can produce
// an abstract FFI representation (called `Library`) of this crate.
//
// All functions you want to export via FFI have to be explicitly
// listed ("rooted") here, but all types relevant to these function
// can be automatically derived.
//
// Once you have that `Library` you pass it to a backend (e.g., csharp)
// which will then produce the interop bindings.
interoptopus::inventory_function!(ffi_inventory, [MY_CONST], [my_game_function]);

// This is a small hack, we use a unit test to invoke `ffi_inventory`
// and produce our backend. You could alternatively use another crate
// to do that job.
#[test]
fn generate_bindings() -> Result<(), interoptopus::Error> {
    use interoptopus_backend_csharp::{Config, Generator};
    use interoptopus::generators::Interop;
    use interoptopus::writer::IndentWriter;

    let library = ffi_inventory();

    let config = Config {
        namespace: "My.Company".to_string(),
        class: "InteropClass".to_string(),
        dll_name: "hello_world".to_string(),
        ..Config::default()
    };

    let generator = Generator::new(config, library);

    let mut tmp: Vec<u8> = Vec::new();
    let mut writer = IndentWriter::new(&mut tmp);

    generator.write_to(&mut writer)?;

    Ok(())
}
