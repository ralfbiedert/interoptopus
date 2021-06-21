//! Test generated bindings for various languages.

use std::fs::read_to_string;

pub mod c;
pub mod csharp;
pub mod python;

#[track_caller]
pub fn assert_file_matches_generated(file: &str) {
    let actual = read_to_string(file).unwrap_or_else(|_| panic!("Must be able to read file '{}'", file));
    let expected = read_to_string(format!("{}.expected", file)).unwrap_or_else(|_| panic!("Must be able to read pre-generated file for '{}'", file));

    assert_eq!(expected, actual);
}
