use crate::common::{GameEngine, SuperComplexEntity, Vec3};
use interoptopus::{ffi_constant, ffi_function, ffi_type};
use std::ptr::null_mut;

type SomeType = u32;

pub type UpdateScore = extern "C" fn(score: u32) -> u32;

const fn number_of_the_yeast() -> u32 {
    667
}

/// Call for a friend.
#[ffi_constant]
pub const THE_MAGIC_CONSTANT: SomeType = number_of_the_yeast() - 1;

#[ffi_type]
#[derive(Debug)]
#[allow(unused)]
pub struct Input {
    x: u8,
    z: u8,
    y: *mut u8,
    v: Vec3,
}

#[ffi_type(opaque)]
pub struct Context {
    my_game_engine: GameEngine,
}

/// Possible errors in our library.
// #[ffi_type]
// // pub enum FFIError {
//     /// All went fine.
//     Ok,
//
//     /// Naughty API call detected.
//     NullPointerPassed = 10,
// }

#[ffi_type]
pub enum FFIError {
    /// All went fine.
    Ok,
    /// Naughty API call detected.
    NullPointerPassed = 10,
}

/// Returns the version of this API.
#[no_mangle]
#[ffi_function]
pub extern "C" fn example_api_version() -> u32 {
    0x00_01_00_00
}

/// A function that always fails.
#[no_mangle]
#[ffi_function]
pub extern "C" fn example_always_fails() -> FFIError {
    FFIError::NullPointerPassed
}

/// Creates a new instance of this library.
#[no_mangle]
#[ffi_function]
pub extern "C" fn example_create_context(context_ptr: Option<&mut *mut Context>) -> FFIError {
    // DON'T DO THAT IN REAL WORLD ... Instead, define a helper that nicely maps `Result`
    // to `FFIError` and catch any panic unwind.
    let context = context_ptr.unwrap();

    let engine = Box::new(Context {
        my_game_engine: GameEngine::default(),
    });

    *context = Box::into_raw(engine);

    FFIError::Ok
}

/// Deletes an existing instance of this library.
///
/// You **must** ensure that `context_ptr` is being called with the context produced by
/// `example_create_context`, otherwise bad things will happen.
#[no_mangle]
#[ffi_function]
#[allow(unused_unsafe)]
pub unsafe extern "C" fn example_destroy_context(context_ptr: Option<&mut *mut Context>) -> FFIError {
    if context_ptr.is_none() {
        return FFIError::NullPointerPassed;
    }

    let ctx = context_ptr.unwrap();

    {
        unsafe { drop(Box::from_raw(*ctx)) };
    }

    *ctx = null_mut();

    FFIError::Ok
}

/// Prints the current player score.
#[no_mangle]
#[ffi_function]
pub extern "C" fn example_print_score(context: Option<&mut Context>) -> FFIError {
    let context = context.unwrap();

    dbg!(context.my_game_engine.player_score);

    FFIError::Ok
}

/// Updates the score.
#[no_mangle]
#[ffi_function]
pub extern "C" fn example_update_score_by_callback(context: Option<&mut Context>, update: UpdateScore) -> FFIError {
    let context = context.unwrap();

    context.my_game_engine.player_score = update(context.my_game_engine.player_score);

    FFIError::Ok
}

/// Updates the score.
#[no_mangle]
#[ffi_function]
pub extern "C" fn example_return_score(context: Option<&mut Context>, score: Option<&mut u32>) -> FFIError {
    let context = context.unwrap();
    let score = score.unwrap();

    *score = context.my_game_engine.player_score;

    FFIError::Ok
}

#[no_mangle]
#[ffi_function]
pub extern "C" fn example_double_super_complex_entity(
    context: Option<&mut Context>,
    incoming: Option<&SuperComplexEntity>,
    outgoing: Option<&mut SuperComplexEntity>,
) -> FFIError {
    let _context = context.unwrap();
    let incoming = incoming.unwrap();
    let outgoing = outgoing.unwrap();

    *outgoing = *incoming;

    outgoing.ammo *= 2;
    outgoing.player_1.x *= 2.0;
    outgoing.player_1.y *= 2.0;
    outgoing.player_1.z *= 2.0;
    outgoing.player_2.x *= 2.0;
    outgoing.player_2.y *= 2.0;
    outgoing.player_2.z *= 2.0;

    FFIError::Ok
}
