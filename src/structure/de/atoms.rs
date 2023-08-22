use nom::branch::alt;
use nom::character::complete::u8;
use nom::character::complete::{char, multispace0, one_of};
use nom::combinator::{map_opt, map_res, value, verify};
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{Err, IResult};
use std::num::{NonZeroU8, NonZeroUsize};

#[cfg(test)]
mod tests;

const OCTAVE: char = '@';
const LENGTH: char = '$';
const VOLUME: char = '!';
const REST: char = '.';
const OCTAVEINCR: char = '>';
const OCTAVEDECR: char = '<';
const LENGTHINCR: char = '/';
const LENGTHDECR: char = '\\';
const VOLUMEINCR: char = '^';
const VOLUMEDECR: char = '_';

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Atom {
    Octave(NonZeroU8),
    Length(NonZeroU8),
    Volume(u8),
    Note(u8, NonZeroUsize),
    Rest,
    OctaveIncr,
    OctaveDecr,
    LengthIncr,
    LengthDecr,
    VolumeIncr,
    VolumeDecr,
}

type R = Result<Atom, ErrorKind>;
type LeResult<'a> = IResult<&'a str, Atom>;

fn octave(i: &str) -> LeResult {
    map_res(
        map_opt(
            verify(preceded(char(OCTAVE), u8), |n| NonZeroU8::new(*n).is_some()),
            NonZeroU8::new,
        ),
        |n| R::Ok(Atom::Octave(n)),
    )(i)
}

fn length(i: &str) -> LeResult {
    map_res(
        map_opt(
            verify(preceded(char(LENGTH), u8), |n| NonZeroU8::new(*n).is_some()),
            NonZeroU8::new,
        ),
        |n| R::Ok(Atom::Length(n)),
    )(i)
}

fn volume(i: &str) -> LeResult {
    map_res(preceded(char(VOLUME), u8), move |n| R::Ok(Atom::Volume(n)))(i)
}

fn note<'a>(notes: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, Atom> {
    map_res(one_of(notes), move |c| {
        R::Ok(Atom::Note(
            notes.find(c).unwrap() as u8,
            NonZeroUsize::new(1).unwrap(),
        ))
    })
}

fn rest(i: &str) -> LeResult {
    value(Atom::Rest, char(REST))(i)
}

fn octaveincr(i: &str) -> LeResult {
    value(Atom::OctaveIncr, char(OCTAVEINCR))(i)
}

fn octavedecr(i: &str) -> LeResult {
    value(Atom::OctaveDecr, char(OCTAVEDECR))(i)
}

fn lengthincr(i: &str) -> LeResult {
    value(Atom::LengthIncr, char(LENGTHINCR))(i)
}

fn lengthdecr(i: &str) -> LeResult {
    value(Atom::LengthDecr, char(LENGTHDECR))(i)
}

fn volumeincr(i: &str) -> LeResult {
    value(Atom::VolumeIncr, char(VOLUMEINCR))(i)
}

fn volumedecr(i: &str) -> LeResult {
    value(Atom::VolumeDecr, char(VOLUMEDECR))(i)
}

fn junk(i: &str) -> IResult<&str, ()> {
    value((), multispace0)(i)
}

fn atom<'a>(noteset: &'a str) -> impl FnMut(&'a str) -> LeResult + 'a {
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

impl Atom {
    pub(crate) fn parse<'a>(
        input: &'a str,
        noteset: &'a str,
    ) -> Result<Vec<Atom>, Err<Error<&'a str>>> {
        Ok(many0(atom(noteset))(input)?.1)
    }
}
