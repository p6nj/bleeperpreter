<img src="https://raw.githubusercontent.com/p6nj/bleeperpreter/main/icon/iconx4.png?sanitize=true" alt="bppt logo" align="right">

<!-- [![Crates.io](https://img.shields.io/crates/v/bleeperpreter.svg)](https://crates.io/crates/bleeperpreter)
[![Docs.rs](https://docs.rs/bleeperpreter/badge.svg)](https://docs.rs/bleeperpreter) -->
[![dependency status](https://deps.rs/repo/github/p6nj/bleeperpreter/status.svg)](https://deps.rs/repo/github/p6nj/bleeperpreter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Bleeperpreter <!-- omit in toc -->
> *A simple language for generating audio from text*

Table of Contents:
- [Intro](#intro)
- [Quick Setup](#quick-setup)
- [Workspace](#workspace)
- [Syntax](#syntax)
- [TODO](#todo)

## Intro
Bleeperpreter is an interpreter for a simple language that can be used to generate audio from text. The core of this project is a library with a string deserializer. One of the goals of this project is to make it easy to add new backends for the interpreter.

## Quick Setup
- get Rust from [rustup.rs](https://rustup.rs/)
- clone this repo:
```bash
git clone https://github.com/p6nj/bleeperpreter.git
```
- cd into the Wave implementation:
```bash
cd bppt-wav
```
- build and run:
```bash
cargo run -- play json/poc.json # for the default example
cargo run -- try "sin(2*pi*f*t)" # to try a signal on the fly
```

## Workspace
This project is a Rust workspace with the following members:
- [bppt](bppt) - the core library
- [bppt-wav](bppt-wav) - a pure sound backend with WAV output
- [bppt-midi](bppt-midi) - a MIDI oriented backend with soundfont support
  
Each implementation has examples (`./toml` or `./json`) and tests. The core library has a few tests as well. You can run all tests from the root of the project with:
```bash
cargo test
```

## Syntax
The language is composed of notes (one letter each), rests (dots) and parametters (a special character followed by a number).
Each implementation has a way of describing which notes are available and what their letter is in a "set". The score is where the notes and parametters are used to generate audio.

Parametters change the octave, length, and volume (or velocity) of notes. Length are in the time signature format: a 4 is a quarter note, a 1 is a whole note. The character associated with each parametter can change over time but you can view (or modify) the current setup in [bppt/src/structure/de/atoms.rs](bppt/src/structure/de/atoms.rs) from line 17. Some parametters don't take any arguments, like the "octave increase" character and other short parametter modifiers. You can also loop a part by putting it in loop delimiters and adding a number after the opening delimiter, or make it a tuplet using the tuplet delimiter.

## TODO
- [ ] finish MIDI backend
- [ ] add documentation
- [ ] publish to crates.io
- [ ] add better error reporting
- [ ] switch `bppt-wav` file input type to `toml`