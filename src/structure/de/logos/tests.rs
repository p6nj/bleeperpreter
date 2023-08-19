use super::*;

fn assert_parses(this: &str, into: MaskAtom, notes: String) {
    assert_eq!(
        into,
        MaskAtom::lexer_with_extras(this, Extras::new(notes, TextPosition::new(this)))
            .next()
            .unwrap()
            .unwrap()
    );
}

#[test]
fn octave() {
    assert_parses(
        "@2",
        MaskAtom::Octave(NonZeroU8::new(2).unwrap()),
        String::default(),
    );
}

#[test]
fn length() {
    assert_parses("$4", MaskAtom::Length(4), String::default());
}

#[test]
fn volume() {
    assert_parses("!16", MaskAtom::Volume(16), String::default());
}

#[test]
fn note() {
    assert_parses("e", MaskAtom::Note(4), "abcde".to_string());
}
