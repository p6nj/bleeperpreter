use super::de_errors::ParseError;
use logos::{Lexer, Logos, Skip};
use std::num::NonZeroU8;
use text_lines::TextLines as TextPosition;
#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Logos)]
#[logos(extras = Extras)]
#[logos(error = ParseError)]
pub(crate) enum MaskAtom {
    #[regex(r"@\d{1,2}", octave)]
    Octave(NonZeroU8),
    #[regex(r"\$\d{1,3}", (normal_cmd_callback_generator("length")))]
    Length(u8),
    #[regex(r"!\d{1,3}", (normal_cmd_callback_generator("volume")))]
    Volume(u8),
    #[regex(r"\p{L}", note)]
    Note(u8),
    #[token(".")]
    #[regex(r"[ \t\n\f\r]+", junk)]
    Rest,
    #[token(">")]
    OctaveIncr,
    #[token("<")]
    OctaveDecr,
    #[token("\\")]
    LengthIncr,
    #[token("/")]
    LengthDecr,
    #[token("^")]
    VolumeIncr,
    #[token("_")]
    VolumeDecr,
    #[regex(r"\([\s\S]*\)", wrapped)]
    Loop(Vec<MaskAtom>),
    #[regex(r"\[[\s\S]*\]", wrapped)]
    Tuplet(Vec<MaskAtom>),
}

pub(crate) struct Extras {
    notes: String,
    position: TextPosition,
    index: usize,
}

impl Extras {
    pub(super) fn new(notes: String, position: TextPosition) -> Self {
        Extras {
            notes,
            position,
            index: 0,
        }
    }
}

fn increment(lex: &mut Lexer<MaskAtom>) {
    lex.extras.index += lex.slice().chars().count();
}

fn at(lex: &Lexer<MaskAtom>) -> (usize, usize) {
    let at = lex
        .extras
        .position
        .line_and_column_display(lex.extras.index);
    (at.line_number, at.column_number)
}

fn normal_cmd_callback_generator(
    for_: &str,
) -> impl Fn(&mut Lexer<MaskAtom>) -> Result<u8, ParseError> + '_ {
    move |lex| {
        increment(lex);
        lex.slice()[1..]
            .parse()
            .map_err(|_| ParseError::new(format!("Expected a {} number", for_), at(lex)))
    }
}

fn junk(lex: &mut Lexer<MaskAtom>) -> Skip {
    increment(lex);
    Skip
}

fn octave(lex: &mut Lexer<MaskAtom>) -> Result<NonZeroU8, ParseError> {
    increment(lex);
    NonZeroU8::new(
        lex.slice()[1..]
            .parse()
            .map_err(|_| ParseError::new("Expected an octave number".to_string(), at(lex)))?,
    )
    .ok_or(ParseError::new(
        "Octave 0 does not exist".to_string(),
        at(lex),
    ))
}

fn note(lex: &mut Lexer<MaskAtom>) -> Result<u8, ParseError> {
    increment(lex);
    Ok(lex
        .extras
        .notes
        .find(lex.slice().chars().next().unwrap())
        .ok_or(ParseError::new("Unkown note".to_string(), at(lex)))? as u8)
}

fn wrapped(lex: &mut Lexer<MaskAtom>) -> Vec<MaskAtom> {
    let source = &lex.slice()[1..lex.slice().len()];
    let result = MaskAtom::lexer_with_extras(
        &source,
        Extras::new(lex.extras.notes.clone(), TextPosition::new(&source)),
    )
    .into_iter()
    .inspect(|result| {
        if let Err(e) = result {
            let updated_error = ParseError {
                at: {
                    let current = at(lex);
                    (e.at.0 + current.0, e.at.1 + current.1)
                },
                ..e.clone()
            };
            eprintln!("Score syntax error: {:?}", updated_error);
        }
    })
    .flatten()
    .collect::<Vec<MaskAtom>>();
    increment(lex);
    result
}
