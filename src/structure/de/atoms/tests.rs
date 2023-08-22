use super::*;
#[test]
fn octave() {
    assert_eq!(
        Ok(("", MaskAtom::Octave(NonZeroU8::new(2).unwrap()))),
        super::octave(format!("{OCTAVE}2").as_str())
    );
}
#[test]
fn length() {
    assert_eq!(
        Ok(("", MaskAtom::Length(4))),
        super::length(format!("{LENGTH}4").as_str())
    );
}
#[test]
fn volume() {
    assert_eq!(
        Ok(("", MaskAtom::Volume(100))),
        super::volume(format!("{VOLUME}100").as_str())
    );
}
#[test]
fn note() {
    assert_eq!(Ok(("", MaskAtom::Note(2))), super::note("abcde")("c"));
}
#[test]
fn rest() {
    assert_eq!(
        Ok(("", MaskAtom::Rest)),
        super::rest(format!("{REST}").as_str())
    );
}
#[test]
fn octave_incr() {
    assert_eq!(
        Ok(("", MaskAtom::OctaveIncr)),
        super::octaveincr(format!("{OCTAVEINCR}").as_str())
    );
}
#[test]
fn octave_decr() {
    assert_eq!(
        Ok(("", MaskAtom::OctaveDecr)),
        super::octavedecr(format!("{OCTAVEDECR}").as_str())
    );
}
#[test]
fn length_incr() {
    assert_eq!(
        Ok(("", MaskAtom::LengthIncr)),
        super::lengthincr(format!("{LENGTHINCR}").as_str())
    );
}
#[test]
fn length_decr() {
    assert_eq!(
        Ok(("", MaskAtom::LengthDecr)),
        super::lengthdecr(format!("{LENGTHDECR}").as_str())
    );
}
#[test]
fn volume_incr() {
    assert_eq!(
        Ok(("", MaskAtom::VolumeIncr)),
        super::volumeincr(format!("{VOLUMEINCR}").as_str())
    );
}
#[test]
fn volume_decr() {
    assert_eq!(
        Ok(("", MaskAtom::VolumeDecr)),
        super::volumedecr(format!("{VOLUMEDECR}").as_str())
    );
}
// #[test]
// fn loop_() {
//     assert_parses("(ccc)", MaskAtom::Loop(vec![]), None);
// }
// #[test]
// fn tuplet() {
//     assert_parses("[]", MaskAtom::Tuplet(vec![]), None);
// }
