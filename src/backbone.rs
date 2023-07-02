use std::collections::HashMap;

use anyhow::{Context, Error};
use json::JsonValue;
use meval::{tokenizer::Operation, Expr};
use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::multi::many0;
use nom::{
    character::complete::one_of, combinator::map_res, number::complete::u8, sequence::preceded,
    IResult,
};

pub struct Album {
    name: String,
    artist: String,
    tracks: Vec<Track>,
}

struct Track {
    name: String,
    bpm: u16,
    instruments: HashMap<String, Instrument>,
    effects: HashMap<String, Effect>,
    channels: Vec<Channel>,
}

enum Instrument {
    Sample {
        data: Vec<f32>,
        r#loops: bool,
        resets: bool,
    },
    Expression {
        expr: Expr,
        resets: bool,
    },
    Mix {
        first: Box<Instrument>,
        second: Box<Instrument>,
        operator: Operation,
    },
}

// TODO: complete effect structure
struct Effect {
    base: String,
    mask: Mask,
}

struct Channel {
    name: String,
    instrument: Instrument,
    effects: Vec<Effect>,
    tuning: u16,
    mask: Mask,
}

#[derive(PartialEq, Debug)]
enum MaskAtoms {
    Octave(u8),
    Length(u8),
    Volume(u8),
    Note(u8),
    Rest,
}

fn note<'a>(notes: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(one_of(notes), move |c| {
        Ok::<MaskAtoms, Error>(MaskAtoms::Note(
            notes
                .find(c)
                .with_context(|| format!("unknown note: {}", c))?
                .try_into()
                .context("converting note index to u8 (shouldn't fail)")?,
        ))
    })
}

fn rest<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(char('.'), move |_| Ok::<MaskAtoms, Error>(MaskAtoms::Rest))
}

fn length<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(preceded(char('$'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtoms, Error>(MaskAtoms::Length(n))
    })
}

fn octave<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(preceded(char('@'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtoms, Error>(MaskAtoms::Octave(n))
    })
}

fn volume<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, MaskAtoms> {
    map_res(preceded(char('!'), map_res(digit1, str::parse)), move |n| {
        Ok::<MaskAtoms, Error>(MaskAtoms::Volume(n))
    })
}

struct Mask(Vec<MaskAtoms>);

impl From<JsonValue> for Channel {
    fn from(value: JsonValue) -> Self {
        todo!()
    }
}

/// From the string mask and a string of allowed notes
impl TryFrom<(String, String)> for Mask {
    type Error = Error;
    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        Ok(Mask(
            many0(preceded(
                space0,
                alt((note(value.1.as_str()), rest(), length(), octave(), volume())),
            ))(value.0.as_str())
            .context("cannot parse mask input")?
            .1,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn note_parser() {
        assert_eq!(Ok(("iueg", MaskAtoms::Note(2))), note("abcde")("ciueg"));
    }
    #[test]
    pub fn length_parser() {
        assert_eq!(Ok(("iueg", MaskAtoms::Length(16))), length()("$16iueg"));
    }
    #[test]
    pub fn octave_parser() {
        assert_eq!(Ok(("iueg", MaskAtoms::Octave(4))), octave()("@4iueg"));
    }
    #[test]
    pub fn rest_parser() {
        assert_eq!(Ok(("iueg", MaskAtoms::Rest)), rest()(".iueg"));
    }
}
