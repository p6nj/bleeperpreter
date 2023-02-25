use crate::proc::ensure;
use crate::structs::{get_real_length, get_real_tempo, Channel, Effect, Song, Symbol};
use anyhow::{Context, Result};

#[allow(dead_code)]
fn printvec(v: &Vec<u8>) -> String {
    let mut r = String::new();
    for n in v {
        r.push(*n as char);
    }
    r
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
        tempo: get_real_tempo(60, 4), // 1 quarter note per second
        tuning: 442f32,               // occidental default
    };

    for s in ['c', 'C', 'd', 'D', 'e', 'f', 'F', 'g', 'G', 'a', 'A', 'b'] {
        song.scale.push(s);
    }

    while character.is_some() {
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
                        let mut numerator = 0u16;
                        let mut denominator = 0u16;
                        while character.unwrap() != b'\n' {
                            match character.unwrap() as char {
                                '/' => {
                                    ensure(numerator != 0).with_context(|| {
                                        format!("Expected valid numerator first: {line}")
                                    })?;
                                    denominator = 0;
                                    character = chars.pop();
                                    while character.unwrap() != b'\n' {
                                        denominator = denominator * 10
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
                                    numerator =
                                        numerator * 10 + (default.to_digit(10).unwrap() as u16)
                                }
                            }
                            character = chars.pop();
                        }
                        song.tempo = get_real_tempo(numerator, denominator);
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
                            current_channel
                                .symbols
                                .push(Symbol::L(get_real_length(length, song.tempo)));
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
                            current_channel.symbols.push(
                                symbols
                                    .next()
                                    .with_context(|| {
                                        format!("No previous note to repeat with '_': {line}")
                                    })?
                                    .clone(),
                            );
                            character = chars.pop();
                        }
                        '-' => {
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
                                Some(Effect::SG(0)),
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
                        '{' => {
                            // found a loop
                            character = chars.pop();
                            ensure(character.is_some()).context("Last character cannot be '{'.")?;
                            let mut midstring: Vec<u8> = vec![];
                            let mut level = 1u8;
                            let mut n = 0u8;
                            while character.unwrap().is_ascii_digit() {
                                n = n * 10
                                    + (character.unwrap() as char).to_digit(10).unwrap() as u8;
                                character = chars.pop();
                                ensure(character.is_some()).with_context(|| {
                                    format!("Unfinished loop with declared counter: {line}")
                                })?;
                            }
                            if n == 0 {
                                n = 2
                            };
                            let n = n; // freeze!
                            loop {
                                midstring.push(character.unwrap());
                                match character.unwrap() as char {
                                    '{' => level += 1,
                                    '}' => {
                                        if level == 1 {
                                            break;
                                        } else {
                                            level -= 1
                                        }
                                    }
                                    _ => (),
                                }
                                character = chars.pop();
                                ensure(character.is_some())
                                    .with_context(|| format!("Unfinished loop: {line}"))?;
                            }
                            midstring.pop(); // remove last '}'
                            midstring = midstring.repeat(n as usize);
                            midstring.reverse();
                            chars = [chars, midstring].concat();
                            character = chars.pop();
                        }
                        '(' => todo!(),
                        '~' => todo!(),
                        '[' => todo!(),
                        '.' => todo!(),
                        ',' => todo!(),
                        note => {
                            current_channel.symbols.push(Symbol::N(
                                song.scale
                                    .iter()
                                    .position(|s| s == &note)
                                    .with_context(|| {
                                        format!("Unknown character {note:?}: {line}")
                                    })? as u8,
                                None,
                            ));
                            character = chars.pop();
                        }
                    }
                }
            }
            '\n' => character = chars.pop(), // every other token case will take you there apart from the channel token which will make a loop (channel > something, channel > something...)
            _ => {
                while character.is_some() && character != Some(b'\n') {
                    character = chars.pop()
                }
            } // anything else is seen as a comment, especially if it starts by a capital letter or '/'
        };
        line += 1;
    }
    Ok(song)
}
