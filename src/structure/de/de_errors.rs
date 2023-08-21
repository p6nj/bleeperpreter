use derive_new::new;
use std::fmt::Debug;

#[derive(Clone, PartialEq, new)]
pub(crate) struct ParseError {
    pub(crate) msg: String,
    pub(crate) at: (usize, usize),
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} just before {:?}", self.msg, self.at)
    }
}

impl Default for ParseError {
    fn default() -> Self {
        ParseError::new("Dummy error".to_string(), (0, 0))
    }
}
