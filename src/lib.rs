//! An unofficial library to read from and write to [InfluxDB](https://www.influxdata.com/) databases.
//! This library supports the 2.x API.
//! It is still very early in development, so features may be missing or not working.

mod flux;
use std::{fmt::Display, io::Read};

pub use flux::{Filter, NumericFilter, Precision, ReadQuery, StringFilter, WriteQuery};

/// Use a Client to connect to your influx database and execute queries.
pub struct Client<'a> {
    url: &'a str,
    token: &'a str,
    reqwest_client: reqwest::blocking::Client,
}

impl<'a> Client<'a> {
    /// Create a client with a given database URL and token.
    pub fn new(url: &'a str, token: &'a str) -> Self {
        Self {
            url,
            token,
            reqwest_client: reqwest::blocking::Client::new(),
        }
    }

    /// Insert a new value into a bucket.
    /// Note that not all attributes on `WriteQuery` are supported yet.
    pub fn insert<T: Display>(
        &self,
        bucket: &'a str,
        org: &'a str,
        precision: Precision,
        query: WriteQuery<T>,
    ) {
        self.reqwest_client
            .post(&format!(
                "{}/api/v2/write?org={}&bucket={}&precision={}",
                self.url, org, bucket, precision
            ))
            .header("Authorization", &format!("Token {}", self.token))
            .body(format!(
                "{} {}={}",
                query.name, query.field_name, query.value
            ))
            .send()
            .unwrap();
    }

    /// Retrieve a value from a bucket based on certain filters.
    pub fn get(&self, org: &'a str, query: ReadQuery) -> String {
        self.get_raw(org, &format!("{}", query))
    }

    /// If you prefer to write your own `flux` queries, use this method.
    /// As `flux` support is not complete yet, this is currently the only
    /// way to use the full `flux` language.
    pub fn get_raw(&self, org: &'a str, query: &'a str) -> String {
        let mut buf = String::new();
        self.reqwest_client
            .post(&format!("{}/api/v2/query?org={}", self.url, org))
            .header("Accept", "application/csv")
            .header("Content-Type", "application/vnd.flux")
            .header("Authorization", &format!("Token {}", self.token))
            .body(query.to_owned())
            .send()
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        buf
    }
}
