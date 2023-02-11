use std::fmt::Display;

#[derive(PartialEq)]
pub enum Symbol {
    /// Length symbol: the length of a duple note if the number is 2, a quarter note if 4 etc...
    L(u8),
    /// Octave symbol: the pitch of the next notes will depend on it
    O(u8),
    /// Note symbol: the number corresponds to the index of the note in the scale (starting from 1 for the frequency calculation).
    N(u8),
    /// Rest symbol: a silent note with no additional information.
    R,
}

#[derive(PartialEq)]
pub struct Channel {
    pub title: String,
    pub symbols: Vec<Symbol>,
}

/// The bpm is always the numerator, the denominator will be used for real tempo.
/// For example, 'tempo: 120/2' means that the BPM is 120 but one beat is a duple note (x2 faster).
#[derive(PartialEq)]
pub struct Tempo {
    pub numerator: u16,
    pub denominator: u16,
}

pub trait GetReal {
    fn get_real(&self) -> f64;
}

impl GetReal for Tempo {
    /// Returns the real tempo in BPM using the numerator and the denominator.
    fn get_real(&self) -> f64 {
        (self.numerator * self.denominator) as f64 / 4f64
    }
}

#[derive(PartialEq)]
/// A song with some info to include in the exported audio file metadata.
pub struct Song {
    /// The title of the song (for metadata)
    pub title: String,
    /// The author of the song (for metadata)
    pub author: String,
    /// The scale of the song (for pitch rendering)
    ///
    /// This vector of strings contain every symbol found in the scale header; it's a string so the symbols can be made of multiple letters as in French.
    /// Each symbol represents a frequency from a shared and evenly distributed interval (from X to 2X).
    pub scale: Vec<char>,
    pub channels: Vec<Channel>,
    /// Tempo (for metadata and length calculation)
    pub tempo: Tempo,
    /// Basic scale tuning, provide a reference for the first note of the first octave (~C1).
    ///
    /// Note that 440Hz/442Hz is the current standard tuning for A4;
    /// in this piece of code a scale may not have a tenth note like A,
    /// but it will have at least one note so the tuning is based on the first one.
    pub tuning: f32,
}

impl Display for Tempo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Symbol::L(l) => format!("l{l}"),
                Symbol::N(n) => ['c', 'C', 'd', 'D', 'e', 'f', 'F', 'g', 'G', 'a', 'A', 'b']
                    .get(*n as usize)
                    .unwrap()
                    .to_string(),
                Symbol::O(o) => format!("o{o}"),
                Symbol::R => String::from(" "),
            }
        )
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all = String::new();
        for ele in &self.symbols {
            all.push_str(format!("{ele}").as_str());
        }
        write!(f, "# {0}\n{1}\n", self.title, all)
    }
}

impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all = String::new();
        for ele in &self.channels {
            all.push_str(format!("{ele}").as_str());
        }
        let mut allchar = String::new();
        for ele in &self.scale {
            allchar.push_str(format!("{ele},").as_str());
        }
        allchar.pop();
        write!(
            f,
            "tempo: {}\ntitle: {}\nauthor: {}\nscale: {}\n{}",
            self.tempo, self.title, self.author, allchar, all
        )
    }
}
