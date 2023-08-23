use super::*;
#[test]
fn octave() {
    assert_eq!(
        Ok(("", Atom::Octave(NonZeroU8::new(2).unwrap()))),
        super::octave(format!("{OCTAVE}2").as_str())
    );
}
#[test]
fn length() {
    assert_eq!(
        Ok(("", Atom::Length(std::num::NonZeroU8::new(4).unwrap()))),
        super::length(format!("{LENGTH}4").as_str())
    );
}
#[test]
fn volume() {
    assert_eq!(
        Ok(("", Atom::Volume(100))),
        super::volume(format!("{VOLUME}100").as_str())
    );
}
#[test]
fn note() {
    assert_eq!(
        Ok(("", Atom::Note(2, NonZeroUsize::new(1).unwrap()))),
        super::note("abcde")("c")
    );
}
#[test]
fn rest() {
    assert_eq!(
        Ok(("", Atom::Rest)),
        super::rest(format!("{REST}").as_str())
    );
}
#[test]
fn octave_incr() {
    assert_eq!(
        Ok(("", Atom::OctaveIncr)),
        super::octaveincr(format!("{OCTAVEINCR}").as_str())
    );
}
#[test]
fn octave_decr() {
    assert_eq!(
        Ok(("", Atom::OctaveDecr)),
        super::octavedecr(format!("{OCTAVEDECR}").as_str())
    );
}
#[test]
fn length_incr() {
    assert_eq!(
        Ok(("", Atom::LengthIncr)),
        super::lengthincr(format!("{LENGTHINCR}").as_str())
    );
}
#[test]
fn length_decr() {
    assert_eq!(
        Ok(("", Atom::LengthDecr)),
        super::lengthdecr(format!("{LENGTHDECR}").as_str())
    );
}
#[test]
fn volume_incr() {
    assert_eq!(
        Ok(("", Atom::VolumeIncr)),
        super::volumeincr(format!("{VOLUMEINCR}").as_str())
    );
}
#[test]
fn volume_decr() {
    assert_eq!(
        Ok(("", Atom::VolumeDecr)),
        super::volumedecr(format!("{VOLUMEDECR}").as_str())
    );
}
#[test]
fn loop_() {
    assert_eq!(
        Ok((
            "",
            Atom::Loop(vec![Atom::Note(2, NonZeroUsize::new(1).unwrap()); 3])
        )),
        super::loop_("abcde")("(ccc)")
    );
}
// #[test]
// fn tuplet() {
//     assert_parses("[]", Atom::Tuplet(vec![]), None);
// }
