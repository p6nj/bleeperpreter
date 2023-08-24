use crate::{mixing::tests::helpers::custom_mask, structure::Root};
use serde_json::from_str;
mod helpers;
use helpers::mixed_root;

#[test]
fn note_length() {
    assert_eq!(48_000, mixed_root(60, (4, 4)), "quarter notes");
    assert_eq!(96_000, mixed_root(60, (2, 2)), "half notes");
    assert_eq!(24_000, mixed_root(60, (8, 8)), "eighth notes");
}

#[test]
fn tempo() {
    assert_eq!(48_000, mixed_root(60, (4, 4)), "60 bpm");
    assert_eq!(96_000, mixed_root(30, (4, 4)), "30 bpm");
    assert_eq!(24_000, mixed_root(120, (4, 4)), "120 bpm");
}

#[test]
fn note_loss() {
    assert_eq!(96_000, mixed_root(60, (2, 4)), "one half, one quarter");
    assert_eq!(96_000, mixed_root(60, (4, 2)), "one quarter, one half");
    assert_eq!(48_000, mixed_root(60, (8, 4)), "one eighth, one quarter");
    assert_eq!(48_000, mixed_root(60, (4, 8)), "one quarter, one eighth");
    assert_eq!(96_000, mixed_root(60, (2, 8)), "one half, one eighth");
    assert_eq!(96_000, mixed_root(60, (8, 2)), "one eighth, one half");
}

#[test]
fn loop_precision() {
    assert_eq!(48_000, custom_mask("[abc]"), "just a lil triplet");
    assert_eq!(48_000, custom_mask("[(12a)]"), "a dodecaplet");
    assert_eq!(
        48_000,
        custom_mask("[(312a)]"),
        "a fucking triakosiadodecaplet"
    );
}
