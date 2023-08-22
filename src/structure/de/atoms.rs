use nom::branch::alt;
use nom::character::complete::{char, multispace0, one_of};
use nom::character::streaming::u8;
use nom::combinator::{map_opt, map_res, value, verify};
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{Err, IResult};
use std::num::NonZeroU8;
use text_lines::TextLines as TextPosition;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Clone)]
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

type R = Result<MaskAtom, ErrorKind>;
type LeResult<'a> = IResult<&'a str, MaskAtom>;

fn octave<'a>(i: &'a str) -> IResult<&'a str, MaskAtom> {
    map_res(
        map_opt(
            verify(preceded(char('@'), u8), |n| NonZeroU8::new(*n).is_some()),
            move |n| NonZeroU8::new(n),
        ),
        |n| R::Ok(MaskAtom::Octave(n)),
    )(i)
}

fn length<'a>(i: &'a str) -> IResult<&'a str, MaskAtom> {
    map_res(preceded(char('$'), u8), move |n| R::Ok(MaskAtom::Length(n)))(i)
}

fn volume<'a>(i: &'a str) -> IResult<&'a str, MaskAtom> {
    map_res(preceded(char('!'), u8), move |n| R::Ok(MaskAtom::Volume(n)))(i)
}

fn note<'a>(notes: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtom> {
    map_res(one_of(notes), move |c| {
        R::Ok(MaskAtom::Note(notes.find(c).unwrap() as u8))
    })
}

fn rest<'a>(i: &'a str) -> LeResult {
    value(MaskAtom::Rest, char('.'))(i)
}

fn octaveincr<'a>(i: &'a str) -> LeResult {
    value(MaskAtom::OctaveIncr, char('>'))(i)
}

fn octavedecr<'a>(i: &'a str) -> LeResult {
    value(MaskAtom::OctaveDecr, char('<'))(i)
}

fn lengthincr<'a>(i: &'a str) -> LeResult {
    value(MaskAtom::LengthIncr, char('\\'))(i)
}

fn lengthdecr<'a>(i: &'a str) -> LeResult {
    value(MaskAtom::LengthDecr, char('/'))(i)
}

fn volumeincr<'a>(i: &'a str) -> LeResult {
    value(MaskAtom::VolumeIncr, char('^'))(i)
}

fn volumedecr<'a>(i: &'a str) -> LeResult {
    value(MaskAtom::VolumeDecr, char('_'))(i)
}

fn junk<'a>(i: &'a str) -> IResult<&'a str, ()> {
    value((), multispace0)(i)
}

fn atom<'a>(noteset: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtom> + 'a {
    preceded(
        junk,
        alt((
            note(noteset),
            octave,
            length,
            volume,
            rest,
            octaveincr,
            octavedecr,
            lengthincr,
            lengthdecr,
            volumeincr,
            volumedecr,
        )),
    )
}

impl MaskAtom {
    pub(crate) fn parse<'a>(
        input: &'a str,
        noteset: &'a str,
    ) -> Result<Vec<MaskAtom>, Err<Error<&'a str>>> {
        Ok(many0(atom(noteset))(input)?.1)
    }
}
