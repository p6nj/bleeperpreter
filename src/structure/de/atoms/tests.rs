use super::*;
#[test]
fn octave() {
    assert_eq!(
        Ok(("", MaskAtom::Octave(NonZeroU8::new(2).unwrap()))),
        super::octave("@2")
    );
}
// #[test]
// fn length() {
//     assert_parses("$4", MaskAtom::Length(4), None);
// }
// #[test]
// fn volume() {
//     assert_parses("!16", MaskAtom::Volume(16), None);
// }
// #[test]
// fn note() {
//     assert_parses("e", MaskAtom::Note(4), Some("abcde"));
// }
// #[test]
// fn rest() {
//     assert_parses(".", MaskAtom::Rest, None);
// }
// #[test]
// fn octave_incr() {
//     assert_parses(">", MaskAtom::OctaveIncr, None);
// }
// #[test]
// fn octave_decr() {
//     assert_parses("<", MaskAtom::OctaveDecr, None);
// }
// #[test]
// fn length_incr() {
//     assert_parses("\\", MaskAtom::LengthIncr, None);
// }
// #[test]
// fn length_decr() {
//     assert_parses("/", MaskAtom::LengthDecr, None);
// }
// #[test]
// fn volume_incr() {
//     assert_parses("^", MaskAtom::VolumeIncr, None);
// }
// #[test]
// fn volume_decr() {
//     assert_parses("_", MaskAtom::VolumeDecr, None);
// }
// #[test]
// fn loop_() {
//     assert_parses("(ccc)", MaskAtom::Loop(vec![]), None);
// }
// #[test]
// fn tuplet() {
//     assert_parses("[]", MaskAtom::Tuplet(vec![]), None);
// }
