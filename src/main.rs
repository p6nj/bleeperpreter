#![feature(iterator_try_reduce)]

mod cli;
mod mixing;
mod playing;
mod saving;
mod structure;

use anyhow::Result;
use cli::Cli;

fn main() -> Result<()> {
    Cli::look_what_to_do_and_do_it()
}

/* Help for expressions:
 * https://thewolfsound.com/sine-saw-square-triangle-pulse-basic-waveforms-in-synthesis/
 * https://github.com/rekka/meval-rs#supported-expressions
 * https://www.desmos.com/calculator
 * sine with vibrato (FM): sin((2*pi*f*t)-cos(2*pi*8*t))
 * easy sine-like triangular wave: (2/pi)*signum(sin(t))*(t%pi)-signum(sin(t))
 * FM for any 2pi-periodic (sine-like) function: https://www.desmos.com/calculator/hfxv6h1n9n
 */
