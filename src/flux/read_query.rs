use std::fmt::Display;

use super::filter;
use super::Precision;
use filter::Filter;

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

    pub fn range(
        mut self,
        start: Option<(i32, Precision)>,
        stop: Option<(i32, Precision)>,
    ) -> Self {
        if let (Some((start_time, start_precision)), Some((stop_time, stop_precision))) =
            (&start, &stop)
        {
            self.range.replace(format!(
                "range(start: {}{}, stop: {}{}",
                start_time, start_precision, stop_time, stop_precision
            ));
        } else if let (Some((start_time, start_precision)), None) = (&start, &stop) {
            self.range
                .replace(format!("range(start: {}{})", start_time, start_precision));
        } else if let (None, Some((stop_time, stop_precision))) = (&start, &stop) {
            self.range
                .replace(format!("range(stop: {}{})", stop_time, stop_precision));
        }
        self
    }

    pub fn filter<T: Filter>(mut self, kind: T) -> Self {
        self.filters.push(format!("{}", kind));
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
