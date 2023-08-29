use super::*;
use lazy_regex::{regex_captures, regex_replace_all};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Signal(pub(crate) Expr);

impl<'de> Deserialize<'de> for Signal {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SignalVisitor;
        impl<'de> serde::de::Visitor<'de> for SignalVisitor {
            type Value = Signal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an expression as a string or an array of strings")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Signal(
                    Expr::from_str(v).map_err(|err| E::custom(err.to_string()))?,
                ))
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Self::Value, A::Error> {
                let mut exprs = Vec::new();
                while let Some(s) = seq.next_element::<String>()? {
                    exprs.push(s);
                }
                let expr = exprs
                    .iter()
                    .rev()
                    .cloned()
                    .try_reduce(|expr, context| -> Result<String, A::Error> {
                        let (_, left, right) = regex_captures!(
                            r"^(?P<left>[A-z|(|)]+) *= *(?P<right>.+)$"s,
                            context.as_str()
                        )
                        .ok_or_else(|| {
                            A::Error::custom(format!(r#"invalid expression: "{}""#, context))
                        })?;
                        Ok(match left.contains('(') {
                            true => {
                                let fname =
                                    regex_captures!(r"^[A-z]+"s, left).ok_or_else(|| {
                                        A::Error::custom(format!(
                                            r#"invalid function name: "{left}""#
                                        ))
                                    })?;
                                regex_replace_all!(
                                    r"(?P<f>[A-z]+)\((?P<arg>[^()]+)\)"s,
                                    &expr,
                                    move |_, f, arg| {
                                        if f == fname {
                                            return format!("{arg}");
                                        }
                                        format!("{f}({arg})")
                                    }
                                )
                                .to_string()
                            }
                            false => {
                                regex_replace_all!(r"\b[A-z]+\b"s, &expr, move |word: &str| {
                                    if word == left {
                                        return format!("({right})");
                                    }
                                    word.to_string()
                                })
                                .to_string()
                            }
                        })
                    })?
                    .ok_or(A::Error::custom("no expressions found"))?;
                Ok(Signal(
                    Expr::from_str(expr.as_str())
                        .map_err(|err| A::Error::custom(err.to_string()))?,
                ))
            }
        }
        deserializer.deserialize_any(SignalVisitor)
    }
}
