use super::MaskAtom;

impl MaskAtom {
    pub(crate) fn serialize(&self, notes: &str) -> String {
        match self {
            Self::Octave(n) => format!("@{n}"),
            Self::Length(n) => format!("${n}"),
            Self::Volume(n) => format!("!{n}"),
            Self::Note(n) => (notes.as_bytes()[*n as usize] as char).into(),
            Self::Rest => '.'.to_string(),
            Self::OctaveIncr => "<".to_string(),
            Self::OctaveDecr => ">".to_string(),
            Self::VolumeIncr => "^".to_string(),
            Self::VolumeDecr => "_".to_string(),
            Self::LengthIncr => "\\".to_string(),
            Self::LengthDecr => "/".to_string(),
            Self::Loop(v) => format!(
                "({})",
                v.iter()
                    .map(|atom| atom.serialize(notes))
                    .collect::<String>()
            ),
            Self::Tuplet(v) => format!(
                "[{}]",
                v.iter()
                    .map(|atom| atom.serialize(notes))
                    .collect::<String>()
            ),
        }
    }
}
