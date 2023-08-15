use derive_new::new;
use std::fmt::Debug;

#[derive(Clone, PartialEq, Default, new)]
pub struct ParseError {
    msg: String,
    at: (usize, usize),
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} just before {:?}", self.msg, self.at)
    }
}
