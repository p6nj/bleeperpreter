use nom::character::complete::char;
use nom::character::streaming::u8;
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{Err, IResult};
use std::num::NonZeroU8;
use text_lines::TextLines as TextPosition;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub(crate) enum MaskAtom {
    Octave(NonZeroU8),
    Length(u8),
    Volume(u8),
    Note(u8),
    Rest,
    OctaveIncr,
    OctaveDecr,
    LengthIncr,
    LengthDecr,
    VolumeIncr,
    VolumeDecr,
}

fn octave<'a>(i: &'a str) -> IResult<&'a str, MaskAtom> {
    map_res(
        preceded(char('@'), u8),
        move |n| -> Result<MaskAtom, &'static str> {
            Ok(MaskAtom::Octave(NonZeroU8::new(n).ok_or("Invalid octave")?))
        },
    )(i)
}

fn atom<'a>(i: &'a str) -> IResult<&'a str, MaskAtom> {
    todo!();
    map_res(
        preceded(char('@'), u8),
        move |n| -> Result<MaskAtom, &'static str> {
            Ok(MaskAtom::Octave(NonZeroU8::new(n).ok_or("Invalid octave")?))
        },
    )(i)
}

impl MaskAtom {
    pub(crate) fn parse<'a, 'b>(
        input: &'a str,
        notes: &'b str,
    ) -> Result<Vec<MaskAtom>, Err<Error<&'static str>>> {
        Ok(many0(atom)(input)?.1)
    }
}
