use anyhow::{Context, Result};

use crate::audio::{silence, sine};

#[derive(Debug)]
enum Symbol {
    /// Length symbol: the length of a duple note if the number is 2, a quarter note if 4 etc...
    L(u8),
    /// Octava symbol: the pitch of the next notes will depend on it
    O(u8),
    /// Note symbol: the number coresponds to the index of the note in the scale (starting from 1 for the frequency calculation).
    /// A boolean is given to know if the enveloppe should be recreated or not, depending on if it's an '_' or not.
    N(u8, bool),
    /// Rest symbol: a silent note with no additional information.
    R,
}

#[derive(Debug)]
/// The bpm is always the numerator, the denominator will be used for real tempo.
/// For example, 'tempo: 120/2' means that the BPM is 120 but one beat is a duple note (x2 faster).
struct Tempo {
    numerator: u16,
    denominator: u16,
}

trait GetReal {
    fn get_real(&self) -> f64;
}

impl GetReal for Tempo {
    /// Retruns the real tempo in BPM using the numerator and the denominator.
    fn get_real(&self) -> f64 {
        (self.numerator * self.denominator) as f64 / 4f64
    }
}

#[derive(Debug)]
struct Channel {
    title: String,
    symbols: Vec<Symbol>,
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
    /// This vector of strings contain every symbol found in the scale header; it's a string so the symbols can be made of multiple letters as in French.
    /// Each symbol represents a frequency from a shared and evenly distributed interval (from X to 2X).
    scale: Vec<String>,
    channels: Vec<Channel>,
    /// Tempo (for metadata and length calculation)
    tempo: Tempo,
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

pub fn get_filenames() -> Result<(String, String)> {
    let args: Vec<String> = std::env::args().collect();
    Ok((
        args.get(1).context("Expected two arguments")?.clone(),
        args.get(2).context("Expected two arguments")?.clone(),
    ))
}

pub fn serialize(data: String) -> Result<Song> {
    let mut chars = Vec::from(data);
    chars.reverse();
    let mut character = chars.pop();
    let mut line = 1u32;

    let mut song = Song {
        title: String::new(),
        author: String::new(),
        scale: Vec::new(),
        channels: Vec::new(),
        tempo: Tempo {
            numerator: 60,
            denominator: 4,
        }, // 1 quarter note per second
        tuning: 55f64 * 2f64.powf(1f64 / 12f64), // occidental default for C1
    };

    for s in ["c", "C", "d", "D", "e", "f", "F", "g", "G", "a", "A", "b"] {
        song.scale.push(String::from(s));
    }

    while character.is_some() {
        dbg!(character.unwrap() as char);
        match character.unwrap() as char {
            'a' => {
                // author
                chars.reverse();
                chars.drain(..7);
                chars.reverse();
                character = chars.pop();
                loop {
                    match character.unwrap() as char {
                        ' ' => (),
                        '\t' => (),
                        _ => break,
                    };
                    character = chars.pop();
                }
                ensure(character.unwrap() != b'\n')
                    .with_context(|| format!("Expected author value: {line}"))?;
                while character.unwrap() != b'\n' {
                    song.author.push(character.unwrap() as char);
                    character = chars.pop();
                }
            }
            't' => {
                character = chars.pop();
                if character.is_none() {
                    break;
                }
                match character.unwrap() as char {
                    'i' => {
                        // title
                        chars.reverse();
                        chars.drain(..5);
                        chars.reverse();
                        character = chars.pop();
                        loop {
                            match character.unwrap() as char {
                                ' ' => (),
                                '\t' => (),
                                _ => break,
                            };
                            character = chars.pop();
                        }
                        ensure(character.unwrap() != b'\n')
                            .with_context(|| format!("Expected title value: {line}"))?;
                        while character.unwrap() != b'\n' {
                            song.title.push(match character.unwrap() as char {
                                '\\' => match chars.pop().unwrap() as char {
                                    'n' => '\n',
                                    other => other,
                                },
                                anything => anything,
                            });
                            character = chars.pop();
                        }
                    }
                    'e' => {
                        // tempo
                        // dbg!(*chars.last().unwrap() as char);
                        chars.reverse();
                        chars.drain(..5);
                        chars.reverse();
                        character = chars.pop();
                        loop {
                            match character.unwrap() as char {
                                ' ' => (),
                                '\t' => (),
                                _ => break,
                            };
                            character = chars.pop();
                        }
                        dbg!(character.unwrap() as char);
                        ensure(character.unwrap() != b'\n')
                            .with_context(|| format!("Expected tempo value: {line}"))?;
                        let mut tempo = Tempo {
                            numerator: 0,
                            denominator: 4,
                        };
                        while character.unwrap() != b'\n' {
                            match character.unwrap() as char {
                                '/' => {
                                    ensure(tempo.numerator != 0).with_context(|| {
                                        format!("Expected valid numerator first: {line}")
                                    })?;
                                    tempo.denominator = 0;
                                    character = chars.pop();
                                    while character.unwrap() != b'\n' {
                                        tempo.denominator = tempo.denominator * 10
                                            + ((character
                                                .unwrap() as char)
                                                .to_digit(10)
                                                .with_context(|| format!("Cannot convert denominator character {character:?} to digit: {line}"))?
                                                as u16);
                                        character = chars.pop();
                                    }
                                    break;
                                }
                                default => {
                                    tempo.numerator = tempo.numerator * 10
                                        + (default.to_digit(10).unwrap() as u16)
                                }
                            }
                            character = chars.pop();
                        }
                        song.tempo = tempo;
                    }
                    _ => (),
                }
            }
            's' => {
                // scale
                chars.reverse();
                chars.drain(..6);
                chars.reverse();
                character = chars.pop();
                loop {
                    match character.unwrap() as char {
                        ' ' => (),
                        '\t' => (),
                        _ => break,
                    };
                    character = chars.pop();
                }
                ensure(character.unwrap() != b'\n')
                    .with_context(|| format!("Expected scale value: {line}"))?;
                let mut word = String::new();
                while character.unwrap() != b'\n' {
                    match character.unwrap() as char {
                        ',' => {
                            ensure(!word.is_empty())
                                .with_context(|| format!("Unexpected separator: {line}"))?;
                            song.scale.push(String::from(word.trim()));
                            word.clear();
                        }
                        c => word.push(c),
                    }
                    character = chars.pop();
                }
                if !word.is_empty() {
                    song.scale.push(String::from(word.trim()));
                    drop(word);
                }
            }
            '#' => {
                // channel
                song.channels.push(Channel {
                    title: String::new(),
                    symbols: Vec::new(),
                });
                let nbchannels = song.channels.len() - 1;
                let current_channel = song
                    .channels
                    .get_mut(nbchannels)
                    .context("Cannot find the current channel (serious bug)")?;
                character = chars.pop();
                loop {
                    match character.unwrap() as char {
                        ' ' => (),
                        '\t' => (),
                        _ => break,
                    };
                    character = chars.pop();
                }
                ensure(character.unwrap() != b'\n')
                    .with_context(|| format!("Expected channel identifier: {line}"))?;
                while character.unwrap() != b'\n' {
                    current_channel.title.push(character.unwrap() as char);
                    character = chars.pop();
                }
                line += 1; // we're now in the channel body
                character = chars.pop();
                while character.is_some() && character.unwrap() != b'#' {
                    match character.unwrap() as char {
                        'l' => {
                            let mut length = 0u8;
                            character = chars.pop();
                            while character.unwrap().is_ascii_digit() {
                                length = length * 10
                                    + (character.unwrap() as char).to_digit(10).unwrap() as u8;
                                character = chars.pop();
                            }
                            current_channel.symbols.push(Symbol::L(length));
                        }
                        'o' => {
                            let mut octava = 0u8;
                            character = chars.pop();
                            while character.unwrap().is_ascii_digit() {
                                octava = octava * 10
                                    + (character.unwrap() as char).to_digit(10).unwrap() as u8;
                                character = chars.pop();
                            }
                            current_channel.symbols.push(Symbol::O(octava));
                        }
                        ' ' => {
                            current_channel.symbols.push(Symbol::R);
                            character = chars.pop();
                        }
                        '_' => {
                            let mut symbs = current_channel
                                .symbols
                                .iter()
                                .filter(|s| matches!(s, Symbol::N(_, _)))
                                .rev();
                            current_channel.symbols.push(Symbol::N(
                                match symbs.next().with_context(|| {
                                    format!("No previous note to repeat with '_': {line}")
                                })? {
                                    &Symbol::N(d, _) => d,
                                    _ => 0u8, // will never happen
                                },
                                false,
                            ));
                            character = chars.pop();
                        }
                        '/' => {
                            character = chars.pop();
                            if character
                                .with_context(|| format!("Unmatched comment prefix: {line}"))?
                                == b'*'
                            {
                                // comment
                                while character.is_some() {
                                    match character.unwrap() as char {
                                        '*' => {
                                            character = chars.pop();
                                            if character.unwrap() == b'/' {
                                                character = chars.pop();
                                                break;
                                            }
                                        }
                                        '\n' => line += 1,
                                        _ => (),
                                    }
                                    character = chars.pop();
                                }
                            }
                        }
                        '\n' => character = chars.pop(),
                        note => {
                            current_channel.symbols.push(Symbol::N(
                                song.scale
                                    .iter()
                                    .position(|s| s == &note.to_string())
                                    .with_context(|| {
                                        format!("Unknown character {note:?}: {line}")
                                    })? as u8, // as U-wish
                                true,
                            ));
                            character = chars.pop();
                        }
                    }
                }
            }
            '\n' => (),
            _ => while chars.pop().is_some() && chars.pop() != Some(b'\n') {}, // anything else is seen as a comment, especially if it starts by a capital letter or '/'
        };
        character = chars.pop();
        line += 1;
    }
    Ok(song)
}

/// Render audio from song symbols. Returns a vector of channels, vectors of samples ready to be written in a wave file.
///
/// Only uses a linear iterator.
pub fn render(song: Song) -> Result<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let (mut length, mut octave) = (4u8, 4u8);
    let tempo = song.tempo.get_real();
    for (i, chan) in song.channels.iter().enumerate() {
        result.push(Vec::new());
        for symb in chan.symbols.iter() {
            match symb {
                Symbol::L(n) => length = *n,
                Symbol::N(n, _) => sine(
                    4f64 * (60f64 / tempo as f64) / length as f64,
                    octave as f64
                        * song.tuning
                        * 2f64.powf(*n as f64 / song.scale.len() as f64 + 1f64),
                )
                .iter()
                .for_each(|x| result[i].push(*x)),
                Symbol::O(n) => octave = *n,
                Symbol::R => silence(4f64 * (60f64 / tempo as f64) / length as f64)
                    .iter()
                    .for_each(|x| result[i].push(*x)),
            };
        }
    }
    // dbg!(&result);
    Ok(merge(result))
}

pub fn merge(channels: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut sums: Vec<Vec<i32>> = Vec::new();
    let mut cursum: i64;
    let mut len: usize;
    for i in 0..channels.len() {
        len = channels[i].len();
        if sums.len() < len {
            for _ in 0..len - sums.len() {
                sums.push(Vec::new());
            }
        }
        for j in 0..len {
            sums[j].push(channels[i][j]);
        }
    }
    for mut i in sums {
        len = i.len();
        if i.is_empty() {
            break;
        }
        cursum = 0;
        for _ in 0..len {
            cursum += i.pop().unwrap() as i64;
        }
        result.push((cursum / len as i64) as i32);
    }
    result
}
