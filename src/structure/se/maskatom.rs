use super::MaskAtom;

impl MaskAtom {
    pub(crate) fn serialize(&self, notes: &str) -> String {
        match self {
            Self::Octave(n) => format!("@{n}"),
            Self::Length(n) => format!("${n}"),
            Self::Volume(n) => format!("!{n}"),
            Self::Note(n) => (notes.as_bytes()[*n as usize] as char).into(),
            Self::Rest => '.'.to_string(),
        }
    }
}
