use std::fmt::Display;

use super::functions::{Filter, Range};

/// Use this struct to read from a bucket.
pub struct ReadQuery<'a> {
    bucket: &'a str,
    range: Option<String>,
    filters: Vec<String>,
}

impl<'a> ReadQuery<'a> {
    pub fn new(bucket: &'a str) -> Self {
        Self {
            bucket,
            range: None,
            filters: Vec::new(),
        }
    }

    pub fn range(mut self, range: Range) -> Self {
        self.range.replace(range.to_string());
        self
    }

    pub fn filter<T: Filter>(mut self, kind: T) -> Self {
        self.filters.push(kind.to_string());
        self
    }
}

impl<'a> Display for ReadQuery<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "from(bucket: \"{}\"){}{}",
            self.bucket,
            if let Some(range) = &self.range {
                format!(" |> {}", range)
            } else {
                String::new()
            },
            if !&self.filters.is_empty() {
                format!(" |> {}", self.filters.join(" |> "))
            } else {
                String::new()
            }
        )
    }
}
