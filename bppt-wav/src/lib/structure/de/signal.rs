use lazy_regex::{regex_captures, regex_replace_all};
use meval::Expr;
use serde::de::Error;
use serde::Deserialize;
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Clone)]
pub struct Signal(pub Expr);

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

                let mut result_expr = None;

                for context in exprs.iter().rev() {
                    let (_, left, right) = regex_captures!(
                        r"^(?P<left>[A-z|(|)]+) *= *(?P<right>.+)$"s,
                        context.as_str()
                    )
                    .ok_or_else(|| {
                        A::Error::custom(format!(r#"invalid expression: "{}""#, context))
                    })?;

                    let expr = match left.contains('(') {
                        true => {
                            let fname = regex_captures!(r"^[A-z]+"s, left).ok_or_else(|| {
                                A::Error::custom(format!(r#"invalid function name: "{left}""#))
                            })?;

                            regex_replace_all!(
                                r"(?P<f>[A-z]+)\((?P<arg>[^()]+)\)"s,
                                &exprs.join(";"),
                                move |_, f, arg: &str| {
                                    if f == fname {
                                        return arg.to_string();
                                    }
                                    format!("{f}({arg})")
                                }
                            )
                            .to_string()
                        }
                        false => regex_replace_all!(
                            r"\b[A-z]+\b"s,
                            &exprs.join(";"),
                            move |word: &str| {
                                if word == left {
                                    return format!("({right})");
                                }
                                word.to_string()
                            }
                        )
                        .to_string(),
                    };

                    result_expr = Some(expr);
                }

                let expr = result_expr.ok_or_else(|| A::Error::custom("no expressions found"))?;
                Ok(Signal(
                    Expr::from_str(expr.as_str())
                        .map_err(|err| A::Error::custom(err.to_string()))?,
                ))
            }
        }
        deserializer.deserialize_any(SignalVisitor)
    }
}
