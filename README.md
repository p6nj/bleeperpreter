# BLELEEPPEERERRRTRTPPPRRETETTTTTERRRR 🤖
## Another custom MML interpreter, this time in Rust?  
Python 🐍 is cute but Rust's mascot 🦀 is cuter! It's also faster and safe.
In a text file full of typos and other obstacles, making sense means making rules.
Rust is perfect for this kind of project (error reporting, `match` trees, zero-cost abstractions...).
## Where is it??
Here's a directory tree:
```
bleeperpreter
├─ AST
├─ AST.png
├─ Cargo.lock
├─ Cargo.toml
├─ LICENSE.txt
├─ notes
├─ src
│  ├─ audio.rs
│  ├─ doc.rs
│  ├─ file.rs
│  ├─ main.rs
│  └─ proc.rs
└─ target
   ...
   └─ debug
      ...
      ├─ examples
      │  └─ cool tune
      ...
```
While I'm building the code, I use `cargo` (comes with [`rust`](https://rustup.rs/)) to run it like so:
```bash
cargo run -q "target/debug/examples/cool tune" test.wav
```
where [`"target/debug/examples/cool tune"`](target/debug/examples/cool%20tune) is a sample file containing some bleeeeeep code for a very cool tune, and `test.wav` is the name of the wave file
that the interpreter will generate.  
Feel free to look at that sample file as it should contain every feature currently supported; edit it as you wish or make another one and use your own `cargo run`.
## There's more though??
The picture [`AST.png`](AST.png) is an early **A**bstract **S**yntax **T**ree generated with the script named [`AST`](AST) using [`dprebyl`](https://github.com/dprebyl)'s [fork](https://dprebyl.github.io/syntree/) of [`mshang`](https://github.com/mshang)'s [web tools](https://github.com/mshang/syntree).  
There's also deez [`notes`](notes).