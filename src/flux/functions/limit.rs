use std::fmt::Display;

/// Only read up to n data points starting from offset
pub struct Limit {
    n: u32,
    offset: u32,
}

impl Limit {
    pub fn new(n: u32, offset: u32) -> Self {
        Self { n, offset }
    }
}

impl Display for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "limit(n: {}, offset: {})", self.n, self.offset)
    }
}
