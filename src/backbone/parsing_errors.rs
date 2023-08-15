use std::fmt::Debug;

use derive_new::new;

#[derive(PartialEq, Clone)]
pub enum ErrorKind {
    Incomprehensible(char),
    SomethingMissing { what: String, needs: String },
    MismatchedOpening(char),
    MismatchedClosing(char),
    Dummy,
}

#[derive(Clone, PartialEq, Default, new)]
pub struct ParseError {
    kind: ErrorKind,
    at: (usize, usize),
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Incomprehensible(c) => write!(f, r#"Unknown character "{}""#, c),
            Self::SomethingMissing { what, needs } => write!(f, r#"{}s need {}"#, what, needs),
            Self::MismatchedOpening(c) => write!(f, r#""{}" closure is never closed"#, c),
            Self::MismatchedClosing(c) => {
                write!(f, r#"This closing "{}" needs an opening friend"#, c)
            }
            Self::Dummy => unreachable!("Dummy error kind! Do not touch!"),
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} at {:?}", self.kind, self.at)
    }
}

impl Default for ErrorKind {
    fn default() -> Self {
        ErrorKind::Dummy
    }
}
