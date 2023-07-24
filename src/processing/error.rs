use std::fmt::Display;

use derive_new::new;

#[derive(Debug, new)]
pub struct Error<'a> {
    pub kind: ErrorKind,
    pub msg: &'a str,
}

#[derive(Debug)]
pub enum ErrorKind {
    Unimplemented,
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.msg)
    }
}

impl std::error::Error for Error<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}
