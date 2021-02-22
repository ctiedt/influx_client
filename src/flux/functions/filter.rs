use std::fmt::Display;

use num_traits::Num;

/// This trait is used for types that can be used to filter with in a `ReadQuery`.
pub trait Filter: Display {}

pub enum NumericFilter<'a, T: Num> {
    Eq(&'a str, T),
    Ne(&'a str, T),
    Lt(&'a str, T),
    Gt(&'a str, T),
    Le(&'a str, T),
    Ge(&'a str, T),
}

impl<'a, T: Num + Display> Filter for NumericFilter<'a, T> {}

pub enum StringFilter<'a> {
    Eq(&'a str, &'a str),
    Ne(&'a str, &'a str),
}

impl<'a> Filter for StringFilter<'a> {}

impl<'a> Display for StringFilter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringFilter::Eq(name, val) => {
                write!(f, "filter(fn: (r) => r.{} == \"{}\")", name, val)
            }
            StringFilter::Ne(name, val) => {
                write!(f, "filter(fn: (r) => r.{} != \"{}\")", name, val)
            }
        }
    }
}

impl<'a, T: Num + Display> Display for NumericFilter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericFilter::Eq(name, val) => write!(f, "filter(fn: (r) => r.{} == {})", name, val),
            NumericFilter::Ne(name, val) => write!(f, "filter(fn: (r) => r.{} != {})", name, val),
            NumericFilter::Lt(name, val) => write!(f, "filter(fn: (r) => r.{} < {})", name, val),
            NumericFilter::Gt(name, val) => write!(f, "filter(fn: (r) => r.{} > {})", name, val),
            NumericFilter::Le(name, val) => write!(f, "filter(fn: (r) => r.{} <= {})", name, val),
            NumericFilter::Ge(name, val) => write!(f, "filter(fn: (r) => r.{} >= {})", name, val),
        }
    }
}
