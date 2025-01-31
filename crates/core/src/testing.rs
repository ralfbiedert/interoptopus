//! Test generated bindings for various languages.

use std::fs::read_to_string;

/// Used by backends to verify a `file.ext` matches an existing `file.ext.expected`.
///
/// # Panics
/// Panics if the files don't match.
#[track_caller]
pub fn assert_file_matches_generated(file: &str) {
    let actual = read_to_string(file).unwrap_or_else(|_| panic!("Must be able to read file '{file}'"));
    let expected = read_to_string(format!("{file}.expected")).unwrap_or_else(|_| panic!("Must be able to read pre-generated file for '{file}'"));

    assert_eq!(expected, actual);
}
