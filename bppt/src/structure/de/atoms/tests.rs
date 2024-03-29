use super::*;
use crate::structure::Atom;

#[test]
fn octave() {
    assert_eq!(
        Ok(("", Atom::O(NonZeroU8::new(2).unwrap()))),
        super::octave(&format!("{OCTAVE}2"))
    );
}
#[test]
fn length() {
    assert_eq!(
        Ok(("", Atom::L(std::num::NonZeroU8::new(4).unwrap()))),
        super::length(&format!("{LENGTH}4"))
    );
}
#[test]
fn volume() {
    assert_eq!(
        Ok(("", Atom::V(100))),
        super::volume(&format!("{VOLUME}100"))
    );
}
#[test]
fn note() {
    assert_eq!(
        Ok(("", Atom::N(2, NonZeroUsize::new(1).unwrap()))),
        super::note("abcde")("c")
    );
}
#[test]
fn rest() {
    assert_eq!(
        Ok(("", Atom::Rest(NonZeroUsize::new(1).unwrap()))),
        super::rest(&format!("{REST}"))
    );
}
#[test]
fn octave_incr() {
    assert_eq!(
        Ok(("", Atom::OIncr)),
        super::octaveincr(&format!("{OCTAVEINCR}"))
    );
}
#[test]
fn octave_decr() {
    assert_eq!(
        Ok(("", Atom::ODecr)),
        super::octavedecr(&format!("{OCTAVEDECR}"))
    );
}
#[test]
fn length_incr() {
    assert_eq!(
        Ok(("", Atom::LIncr)),
        super::lengthincr(&format!("{LENGTHINCR}"))
    );
}
#[test]
fn length_decr() {
    assert_eq!(
        Ok(("", Atom::LDecr)),
        super::lengthdecr(&format!("{LENGTHDECR}"))
    );
}
#[test]
fn volume_incr() {
    assert_eq!(
        Ok(("", Atom::VIncr)),
        super::volumeincr(&format!("{VOLUMEINCR}"))
    );
}
#[test]
fn volume_decr() {
    assert_eq!(
        Ok(("", Atom::VDecr)),
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
                vec![Atom::N(2, NonZeroUsize::new(1).unwrap()); 3]
            )
        )),
        super::r#loop("abcde")(&input)
    );
    let input = format!("{LOOP_IN}45ccc{LOOP_OUT}");
    assert_eq!(
        Ok((
            "",
            Atom::Loop(
                NonZeroU16::new(45).unwrap(),
                vec![Atom::N(2, NonZeroUsize::new(1).unwrap()); 3]
            )
        )),
        super::r#loop("abcde")(&input)
    );
}
#[test]
fn tuplet() {
    let input = format!("{TUP_IN}ccc{TUP_OUT}");
    assert_eq!(
        Ok((
            "",
            Atom::Tuplet(vec![Atom::N(2, NonZeroUsize::new(1).unwrap()); 3])
        )),
        super::tuplet("abcde")(&input)
    );
}
