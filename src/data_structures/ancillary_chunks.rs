use crate::prelude::*;
use std::fmt::{Display, Formatter};

struct AncillaryChunks(Vec<Chunk>);

impl Display for AncillaryChunks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for chunk in &self.0 {
            result.push_str(&format!("{}\n", chunk));
        }
        write!(f, "{}", result)
    }
}
