//! This module provides the types to read from and write into buckets.
//! However, the query language `flux` is not fully supported yet.

pub mod functions;
mod read_query;
mod write_query;

use std::fmt::Display;

pub use read_query::ReadQuery;
pub use write_query::WriteQuery;

#[allow(non_camel_case_types)]
pub enum Precision {
    h,
    s,
    ms,
    us,
    ns,
}

impl Display for Precision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Precision::h => "h",
                Precision::s => "s",
                Precision::ms => "ms",
                Precision::us => "us",
                Precision::ns => "ns",
            }
        )
    }
}
