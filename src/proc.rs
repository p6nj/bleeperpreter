use crate::{
    audio::{signal, silence, Signal},
    structs::{get_real_length, Song, Symbol},
};
use anyhow::{Context, Result};

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
pub fn ensure(condition: bool) -> Option<()> {
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

/// Render audio from song symbols. Returns a vector of channels, vectors of samples ready to be written in a wave file.
///
/// Only uses a linear iterator.
pub fn render(song: Song) -> Result<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let (mut length, mut octave) = (get_real_length(4u8, song.tempo), 4u8);
    for (i, chan) in song.channels.iter().enumerate() {
        result.push(Vec::new());
        for symbol in chan.symbols.iter() {
            match symbol {
                Symbol::L(n) => length = *n,
                Symbol::N(n, _) => signal(
                    Signal::Sine,
                    length,
                    song.tuning
                        * 2f32.powf((*n + 12 * (octave - 1)) as f32 / song.scale.len() as f32),
                )
                .iter()
                .for_each(|x| result[i].push(*x)),
                Symbol::O(n) => octave = *n,
                Symbol::R => silence(length).iter().for_each(|x| result[i].push(*x)),
            };
        }
    }
    // dbg!(&result);
    Ok(merge(result))
}

#[allow(dead_code)]
pub fn merge(channels: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut sums: Vec<Vec<i32>> = Vec::new();
    let mut sum: i64;
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
        sum = 0;
        for _ in 0..len {
            sum += i.pop().unwrap() as i64;
        }
        result.push((sum / len as i64) as i32);
    }
    result
}

#[allow(dead_code)]
fn interleave(mut channels: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut maxsize = 0usize;
    let mut x: usize;
    for channel in &channels {
        x = channel.len();
        if x > maxsize {
            maxsize = x;
        }
    }
    for _ in 0..maxsize - 1 {
        for channel in &mut channels {
            result.push(match channel.pop() {
                Some(sample) => sample,
                None => 0i32,
            });
        }
    }
    result
}
