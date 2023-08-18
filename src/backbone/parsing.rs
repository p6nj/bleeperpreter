use crate::backbone::Mask;

use super::Notes;
use ::logos::Logos;
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use text_lines::TextLines;
mod logos;
use self::logos::Extras;
pub(crate) use self::logos::MaskAtom;
mod parsing_errors;
#[cfg(test)]
mod tests;

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
                let score = Mask(
                    MaskAtom::lexer_with_extras(
                        score_str,
                        Extras::new(set.clone(), TextLines::new(score_str)),
                    )
                    .inspect(|result| {
                        if let Err(e) = result {
                            eprintln!("Score syntax error: {:?}", e);
                        }
                    })
                    .flatten()
                    .collect::<Vec<MaskAtom>>(),
                );
                Ok(Self::Value::new(set, score))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut set = None;
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
                            score = Some(Mask(
                                MaskAtom::lexer_with_extras(
                                    score_str,
                                    Extras::new(
                                        match set.clone() {
                                            Some(s) => s,
                                            None => {
                                                let mut set = None;
                                                while let Some(key) = map.next_key::<Field>()? {
                                                    match key {
                                                        Field::Set => set = Some(map.next_value()?),
                                                        _ => (),
                                                    }
                                                }
                                                set
                                            }
                                            .ok_or_else(|| Error::missing_field("set"))?,
                                        },
                                        TextLines::new(score_str),
                                    ),
                                )
                                .inspect(|result| {
                                    if let Err(e) = result {
                                        eprintln!("Score syntax error: {:?}", e);
                                    }
                                })
                                .flatten()
                                .collect::<Vec<MaskAtom>>(),
                            ));
                        }
                    }
                }
                let set = set.ok_or_else(|| Error::missing_field("set"))?;
                let score = score.ok_or_else(|| Error::missing_field("score"))?;
                Ok(Self::Value::new(set, score))
            }
        }

        const FIELDS: &'static [&'static str] = &["set", "score"];
        deserializer.deserialize_struct("Notes", FIELDS, NotesVisitor)
    }
}
