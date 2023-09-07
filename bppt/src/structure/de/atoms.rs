use nom::branch::alt;
use nom::bytes::complete::take_till1;
use nom::character::complete::{char, multispace0, one_of};
use nom::character::complete::{u16, u8};
use nom::combinator::{consumed, map_opt, map_res, opt, value, verify};
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::sequence::{pair, preceded};
use nom::{Err, IResult};
use std::num::{NonZeroU16, NonZeroU8, NonZeroUsize};

use crate::structure::Atom;

#[cfg(test)]
mod tests;

const OCTAVE: char = '@';
const LENGTH: char = '$';
const VOLUME: char = '!';
const REST: char = '.';
const OCTAVEINCR: char = '>';
const OCTAVEDECR: char = '<';
const LENGTHINCR: char = '`';
const LENGTHDECR: char = '\'';
const VOLUMEINCR: char = '^';
const VOLUMEDECR: char = '_';
const LOOP_IN: char = '(';
const LOOP_OUT: char = ')';
const TUP_IN: char = '[';
const TUP_OUT: char = ']';

type R<'a> = Result<Atom, Err<Error<&'a str>>>;
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

fn atom<'a>(noteset: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, Atom> {
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
            r#loop(noteset),
            tuplet(noteset),
        )),
    )
}

fn close(in_tag: char, out_tag: char) -> impl FnMut(&str) -> IResult<&str, ()> {
    move |i| {
        let mut lvl = 1u8;
        let mut input = i;
        while let Ok((r, _)) =
            take_till1::<_, &str, Error<&str>>(|c| c == in_tag || c == out_tag)(input)
        {
            {
                let ch = r
                    .chars()
                    .next()
                    .ok_or(Err::Error(Error::new("", ErrorKind::Complete)))?;
                if ch == in_tag {
                    lvl += 1
                } else {
                    lvl -= 1;
                    if lvl == 0 {
                        return Ok((&r[1..], ()));
                    }
                }
            }
            input = r;
        }
        Err(Err::Error(Error::new("", ErrorKind::Complete)))
    }
}

fn r#loop<'a>(noteset: &'a str) -> impl FnMut(&'a str) -> LeResult + 'a {
    map_res(
        preceded(
            char(LOOP_IN),
            pair(
                opt(map_opt(
                    verify(u16, |res| NonZeroU16::new(*res).is_some()),
                    NonZeroU16::new,
                )),
                consumed(close(LOOP_IN, LOOP_OUT)),
            ),
        ),
        move |(repeat, (inner, _))| {
            R::Ok(Atom::Loop(
                repeat.unwrap_or(NonZeroU16::new(2).unwrap()),
                many0(atom(noteset))(inner)?.1,
            ))
        },
    )
}

fn tuplet<'a>(noteset: &'a str) -> impl FnMut(&'a str) -> LeResult + 'a {
    map_res(
        preceded(char(TUP_IN), consumed(close(TUP_IN, TUP_OUT))),
        move |(inner, _)| {
            R::Ok(Atom::Tuplet(
                verify(many0(atom(noteset)), |res: &Vec<Atom>| !res.is_empty())(inner)?.1,
            ))
        },
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
