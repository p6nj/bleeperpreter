use crate::structure::{self, Atom};
use anyhow::Result;

pub(crate) type Samples = Vec<f32>;

mod processing;
#[cfg(test)]
mod tests;

impl structure::Track {
    pub fn mix(&mut self) -> Result<Samples> {
        let mut sorted = self.process()?;
        sorted.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
        Ok(sorted.iter().cloned().fold(vec![], |acc, v| {
            v.iter()
                .zip(acc.iter().chain([0f32].iter().cycle()))
                .map(|(s, acc)| *s / (sorted.len() as f32) + acc)
                .collect()
        }))
    }
}
