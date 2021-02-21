use std::{collections::BTreeMap, fmt::Display, time::Instant};

/// Use this struct to write data to a bucket.
pub struct WriteQuery<'a, T: Display> {
    pub name: &'a str,
    pub tags: BTreeMap<&'a str, &'a str>,
    pub field_name: &'a str,
    pub value: T,
    pub timestamp: Option<Instant>,
}

impl<'a, T: Display> Display for WriteQuery<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}={}", self.name, self.field_name, self.value)
    }
}
