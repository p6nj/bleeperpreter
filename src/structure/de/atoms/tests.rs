use super::*;
#[test]
fn octave() {
    assert_eq!(
        Ok(("", Atom::Octave(NonZeroU8::new(2).unwrap()))),
        super::octave(&format!("{OCTAVE}2"))
    );
}
#[test]
fn length() {
    assert_eq!(
        Ok(("", Atom::Length(std::num::NonZeroU8::new(4).unwrap()))),
        super::length(&format!("{LENGTH}4"))
    );
}
#[test]
fn volume() {
    assert_eq!(
        Ok(("", Atom::Volume(100))),
        super::volume(&format!("{VOLUME}100"))
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
    assert_eq!(Ok(("", Atom::Rest)), super::rest(&format!("{REST}")));
}
#[test]
fn octave_incr() {
    assert_eq!(
        Ok(("", Atom::OctaveIncr)),
        super::octaveincr(&format!("{OCTAVEINCR}"))
    );
}
#[test]
fn octave_decr() {
    assert_eq!(
        Ok(("", Atom::OctaveDecr)),
        super::octavedecr(&format!("{OCTAVEDECR}"))
    );
}
#[test]
fn length_incr() {
    assert_eq!(
        Ok(("", Atom::LengthIncr)),
        super::lengthincr(&format!("{LENGTHINCR}"))
    );
}
#[test]
fn length_decr() {
    assert_eq!(
        Ok(("", Atom::LengthDecr)),
        super::lengthdecr(&format!("{LENGTHDECR}"))
    );
}
#[test]
fn volume_incr() {
    assert_eq!(
        Ok(("", Atom::VolumeIncr)),
        super::volumeincr(&format!("{VOLUMEINCR}"))
    );
}
#[test]
fn volume_decr() {
    assert_eq!(
        Ok(("", Atom::VolumeDecr)),
        super::volumedecr(&format!("{VOLUMEDECR}"))
    );
}
#[test]
fn loop_() {
    let input = format!("{LOOP_IN}ccc{LOOP_OUT}");
    assert_eq!(
        Ok((
            "",
            Atom::Loop(
                NonZeroU16::new(2).unwrap(),
                vec![Atom::Note(2, NonZeroUsize::new(1).unwrap()); 3]
            )
        )),
        super::loop_("abcde")(&input)
    );
    let input = format!("{LOOP_IN}45ccc{LOOP_OUT}");
    assert_eq!(
        Ok((
            "",
            Atom::Loop(
                NonZeroU16::new(45).unwrap(),
                vec![Atom::Note(2, NonZeroUsize::new(1).unwrap()); 3]
            )
        )),
        super::loop_("abcde")(&input)
    );
}
// #[test]
// fn tuplet() {
//     assert_parses("[]", Atom::Tuplet(vec![]), None);
// }
