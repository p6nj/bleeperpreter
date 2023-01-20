use crate::proc::ensure;
use crate::structs::{Channel, Song, Symbol, Tempo};
use anyhow::{Context, Result};

fn parse_author(
    mut character: Option<u8>,
    mut chars: Vec<u8>,
    song: &mut Song,
    line: &mut u32,
) -> Result<(Option<u8>, Vec<u8>)> {
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
    Ok((character, chars))
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
        tuning: 442f32, // occidental default
    };

    for s in ['c', 'C', 'd', 'D', 'e', 'f', 'F', 'g', 'G', 'a', 'A', 'b'] {
        song.scale.push(s);
    }

    while character.is_some() {
        // dbg!(character.unwrap() as char);
        match character.unwrap() as char {
            'a' => {
                // author
                (character, chars) = parse_author(character, chars, &mut song, &mut line)
                    .context("Cannot parse author information")?;
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
                        // dbg!(character.unwrap() as char);
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
                song.scale.clear();
                let mut word = ' '; // using the space character ensures that no note is a space.
                while character.unwrap() != b'\n' {
                    match character.unwrap() as char {
                        ',' => {
                            ensure(word != ' ')
                                .with_context(|| format!("Unexpected separator: {line}"))?;
                            song.scale.push(word);
                            word = ' ';
                        }
                        c => {
                            ensure(word == ' ').with_context(|| {
                                format!("Sorry, one char per note (for readability): {line}")
                            })?;
                            word = c;
                        }
                    }
                    character = chars.pop();
                }
                if word != ' ' {
                    song.scale.push(word);
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
                            let mut octave = 0u8;
                            character = chars.pop();
                            while character.unwrap().is_ascii_digit() {
                                octave = octave * 10
                                    + (character.unwrap() as char).to_digit(10).unwrap() as u8;
                                character = chars.pop();
                            }
                            current_channel.symbols.push(Symbol::O(octave));
                        }
                        ' ' => {
                            current_channel.symbols.push(Symbol::R);
                            character = chars.pop();
                        }
                        '_' => {
                            let mut symbols = current_channel
                                .symbols
                                .iter()
                                .filter(|s| matches!(s, Symbol::N(_, _)))
                                .rev();
                            current_channel.symbols.push(Symbol::N(
                                match symbols.next().with_context(|| {
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
                                    .position(|s| s == &note)
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
