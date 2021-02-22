use std::{collections::BTreeMap, fmt::Display, time::Instant};

/// Use this struct to write data to a bucket.
pub struct WriteQuery<'a, T: Display> {
    pub name: &'a str,
    pub tags: BTreeMap<&'a str, &'a str>,
    pub field_name: &'a str,
    pub value: T,
    pub timestamp: Option<Instant>,
}

impl<'a, T: Display> WriteQuery<'a, T> {
    pub fn new(
        name: &'a str,
        tags: BTreeMap<&'a str, &'a str>,
        field_name: &'a str,
        value: T,
        timestamp: Option<Instant>,
    ) -> Self {
        Self {
            name,
            tags,
            field_name,
            value,
            timestamp,
        }
    }

    fn format_tags(&self) -> String {
        if self.tags.is_empty() {
            String::new()
        } else {
            self.tags
                .iter()
                .map(|(k, v)| format!(",{}={}", k, v))
                .collect()
        }
    }
}

impl<'a, T: Display> Display for WriteQuery<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}={}",
            self.name,
            self.format_tags(),
            self.field_name,
            self.value
        )
    }
}
