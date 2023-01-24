use std::fmt::Display;

use super::functions::{Filter, Limit, Range, Sort};

/// Use this struct to read from a bucket.
pub struct ReadQuery<'a> {
    bucket: &'a str,
    range: Option<String>,
    filters: Vec<String>,
    sort: Option<String>,
    limit: Option<String>,
}

impl<'a> ReadQuery<'a> {
    pub fn new(bucket: &'a str) -> Self {
        Self {
            bucket,
            range: None,
            filters: Vec::new(),
            sort: None,
            limit: None,
        }
    }

    /// Only read data in a certain time range
    pub fn range(mut self, range: Range) -> Self {
        self.range.replace(range.to_string());
        self
    }

    /// Filter the data based on its value
    pub fn filter<T: Filter>(mut self, kind: T) -> Self {
        self.filters.push(kind.to_string());
        self
    }

    /// Sort the data by the value of the given columns
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort.replace(sort.to_string());
        self
    }

    /// Only read a given number of data points
    pub fn limit(mut self, limit: Limit) -> Self {
        self.limit.replace(limit.to_string());
        self
    }
}

impl<'a> Display for ReadQuery<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "from(bucket: \"{}\"){}{}{}{}",
            self.bucket,
            if let Some(range) = &self.range {
                format!(" |> {range}")
            } else {
                String::new()
            },
            if !&self.filters.is_empty() {
                format!(" |> {}", self.filters.join(" |> "))
            } else {
                String::new()
            },
            if let Some(sort) = &self.sort {
                format!(" |> {sort}")
            } else {
                String::new()
            },
            if let Some(limit) = &self.limit {
                format!(" |> {limit}")
            } else {
                String::new()
            }
        )
    }
}
