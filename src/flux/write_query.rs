use std::{
    collections::HashMap,
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::Precision;

/// Use this struct to write data to a bucket.
pub struct WriteQuery<'a, T: Display> {
    pub name: &'a str,
    pub tags: HashMap<&'a str, &'a str>,
    pub field_name: &'a str,
    pub value: T,
    pub timestamp: Option<(SystemTime, Precision)>,
}

impl<'a, T: Display> WriteQuery<'a, T> {
    pub fn new(
        name: &'a str,
        tags: HashMap<&'a str, &'a str>,
        field_name: &'a str,
        value: T,
        timestamp: Option<(SystemTime, Precision)>,
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
            self.tags.iter().map(|(k, v)| format!(",{k}={v}")).collect()
        }
    }

    fn format_timestamp(&self) -> String {
        if let Some((system_time, precision)) = &self.timestamp {
            // We are just going to assume that the system time is after the UNIX Epoch
            let ts = system_time
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime before UNIX Epoch");
            format!(
                " {}",
                match precision {
                    Precision::h => (ts.as_secs() as u128) / 3600,
                    Precision::s => ts.as_secs() as u128,
                    Precision::ms => ts.as_millis(),
                    Precision::us => ts.as_micros(),
                    Precision::ns => ts.as_nanos(),
                }
            )
        } else {
            String::new()
        }
    }
}

impl<'a, T: Display> Display for WriteQuery<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}={}{}",
            self.name,
            self.format_tags(),
            self.field_name,
            self.value,
            self.format_timestamp(),
        )
    }
}
