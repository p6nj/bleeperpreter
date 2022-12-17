use crate::doc;
use anyhow::{Context, Result};
use std::process::exit;

#[derive(Debug)]
enum Word {
    L(u8),
    O(u8),
    N(u8),
}

#[derive(Debug)]
struct Channel {
    title: String,
    words: Vec<Word>,
}

#[derive(Debug)]
/// A song with some info to include in the exported audio file metadata.
pub struct Song {
    /// The title of the song (for metadata)
    title: String,
    /// The author of the song (for metadata)
    author: String,
    /// The scale of the song (for pitch rendering)
    ///
    /// This vector of strings contain every symbol found in the scale header.
    /// Each symbol represents a frequency from a shared and evenly distributed interval (from X to 2X)
    scale: Vec<&'static str>,
    channels: Vec<Channel>,
    /// Beats Per Minute (**for metadata only**)
    bpm: u16,
    /// Basic scale tuning, provide a reference for the first note of the first octave (~C1).
    ///
    /// Note that 440Hz/442Hz is the current standard tuning for A4;
    /// in this piece of code a scale may not have a tenth note like A,
    /// but it will have at least one note so the tuning is based on the first one.
    tuning: f64,
}

/// `ensure(condition: bool)`
///
/// Returns `()` if the condition is true, else returns nothing.
///
/// Useful for anyhow assertions (triggers the following context when the condition is false):
///
/// <br>
///
/// ```rust
/// use anyhow::{Context, Result};
///
/// fn get_second_argument -> Result<String> {
///     let args: Vec<String> = std::env::args().collect();
///     ensure(args.len() == 2).context("Expected two arguments")?;
///     Ok(String::from(&args[1]))
/// }
/// ```
fn ensure(condition: bool) -> Option<()> {
    match condition {
        true => Some(()),
        false => None,
    }
}

pub fn get_filename() -> Result<String> {
    let args: Vec<String> = std::env::args().collect();
    ensure(args.len() == 2).context("Expected two arguments")?;
    if args[1] == "-h" || args[1] == "--help" {
        println!("{}", doc::USAGE);
        exit(0);
    }
    let filename: String = String::from(&args[1]);
    Ok(filename)
}

pub fn serialize(data: String) -> Result<Song> {
    let chars = &mut data.chars();
    let mut character = chars.next();
    let mut line = 0u32;

    let mut song = Song {
        title: String::new(),
        author: String::new(),
        scale: Vec::from(["c", "C", "d", "D", "e", "f", "F", "g", "G", "a", "A", "b"]), // Blip defaults
        channels: Vec::new(),
        bpm: 60,                                 // 1 beat per second
        tuning: 55f64 * 2f64.powf(1f64 / 12f64), // occidental default for C1
    };

    let mut tempo = 60u16; // real tempo, in quarter note per second

    while character.is_some() {
        match character.unwrap() {
            'a' => {
                // author
                let mut chars = chars
                    .as_str()
                    .strip_prefix("uthor:")
                    .with_context(|| format!("Expected author header: {line}"))?
                    .trim_start()
                    .chars();
                let mut character = chars.next();
                ensure(character.unwrap() != '\n')
                    .with_context(|| format!("Expected author value: {line}"))?;
                while character.unwrap() != '\n' {
                    song.author.push(character.unwrap());
                    character = chars.next();
                }
                line += 1;
            }
            't' => {
                let character = &chars.next();
                if character.is_none() {
                    break;
                }
                match character.unwrap() {
                    'i' => {
                        // title
                        let mut chars = chars
                            .as_str()
                            .strip_prefix("tle:")
                            .with_context(|| format!("Expected title header: {line}"))?
                            .trim_start()
                            .chars();
                        let mut character = chars.next();
                        ensure(character.unwrap() != '\n')
                            .with_context(|| format!("Expected title value: {line}"))?;
                        while character.unwrap() != '\n' {
                            song.title.push(match character.unwrap() {
                                '\\' => match chars.next().unwrap() {
                                    'n' => '\n',
                                    other => other,
                                },
                                anything => anything,
                            });
                            character = chars.next();
                        }
                        line += 1;
                    }
                    'e' => {
                        // tempo
                        let mut chars = chars
                            .as_str()
                            .strip_prefix("mpo:")
                            .with_context(|| format!("Expected tempo header: {line}"))?
                            .trim_start()
                            .chars();
                        let mut character = chars.next();
                        ensure(character.unwrap() != '\n')
                            .with_context(|| format!("Expected tempo value: {line}"))?;
                        let mut numerator = 0u16; // why isn't it called nominator?
                        let mut denominator = 4u16;
                        while character.unwrap() != '\n' {
                            match character.unwrap() {
                                '/' => {
                                    ensure(numerator != 0).with_context(|| {
                                        format!("Expected valid numerator first: {line}")
                                    })?;
                                    denominator = 0;
                                    character = chars.next();
                                    while character.unwrap() != '\n' {
                                        denominator = denominator * 10
                                            + (character
                                                .unwrap()
                                                .to_digit(10)
                                                .with_context(|| format!("Cannot convert denominator character {character:?} to digit: {line}"))?
                                                as u16);
                                        character = chars.next();
                                    }
                                    break;
                                }
                                default => {
                                    numerator =
                                        numerator * 10 + (default.to_digit(10).unwrap() as u16)
                                }
                            }
                            character = chars.next();
                            line += 1;
                        }
                        // the bpm is always the numerator, the denominator will be used for real tempo
                        // for example, 'tempo: 120/2' means that the BPM is 120 but one beat is a duple note (x2 faster)
                        song.bpm = numerator;
                        tempo = (numerator * denominator) / 4;
                    }
                    _ => (),
                }
            }
            '\n' => line += 1,
            _ => {
                // anything else is seen as a comment, especially if it starts by a capital letter or '/'
                while chars.next().is_some() && chars.next() != Some('\n') {}
                line += 1;
            }
        };
        character = chars.next();
    }
    Ok(song)
}
