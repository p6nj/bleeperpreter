use crate::structure::Atom;

use super::*;

impl<'de> Deserialize<'de> for Notes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Set,
            Score,
        }

        struct NotesVisitor;

        impl<'de> Visitor<'de> for NotesVisitor {
            type Value = Notes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("set and score")
            }

            fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let set: String = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let score_str = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;
                let score = Atom::parse(score_str, &set)
                    .map_err(|err| Error::custom(format!("Syntax error: {}", err)))?;
                Ok(Self::Value::new(set.len() as u8, score))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut set: Option<String> = None;
                let mut score = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Set => {
                            if set.is_some() {
                                return Err(Error::duplicate_field("set"));
                            }
                            set = Some(map.next_value()?);
                        }
                        Field::Score => {
                            if score.is_some() {
                                return Err(Error::duplicate_field("score"));
                            }
                            let score_str = map.next_value()?;
                            score = Some(
                                Atom::parse(
                                    score_str,
                                    match set.clone() {
                                        Some(s) => s,
                                        None => {
                                            let mut set: Option<String> = None;
                                            while let Some(key) = map.next_key::<Field>()? {
                                                if let Field::Set = key {
                                                    set = Some(map.next_value()?)
                                                }
                                            }
                                            set
                                        }
                                        .ok_or_else(|| Error::missing_field("set"))?,
                                    }
                                    .as_str(),
                                )
                                .map_err(|err| Error::custom(format!("Syntax error: {}", err)))?,
                            );
                        }
                    }
                }
                let set = set.ok_or_else(|| Error::missing_field("set"))?;
                let score = score.ok_or_else(|| Error::missing_field("score"))?;
                Ok(Self::Value::new(set.len() as u8, score))
            }
        }

        const FIELDS: &[&str] = &["set", "score"];
        deserializer.deserialize_struct("Notes", FIELDS, NotesVisitor)
    }
}
