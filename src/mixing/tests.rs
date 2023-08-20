use super::*;
use crate::structure::Root;
use serde_json::from_str;
mod helpers;
use helpers::mixed_root_length;

#[test]
fn note_length() {
    assert_eq!(48_000, mixed_root_length(60, (4, 4)).unwrap());
    assert_eq!(48_000 * 2, mixed_root_length(60, (2, 2)).unwrap());
    assert_eq!(24_000, mixed_root_length(60, (8, 8)).unwrap());
}

#[test]
fn tempo() {
    assert_eq!(48_000, mixed_root_length(60, (4, 4)).unwrap());
    assert_eq!(48_000 * 2, mixed_root_length(30, (4, 4)).unwrap());
    assert_eq!(24_000, mixed_root_length(120, (4, 4)).unwrap());
}
