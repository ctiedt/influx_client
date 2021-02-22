//! An unofficial library to read from and write to [InfluxDB](https://www.influxdata.com/) databases.
//! This library supports the 2.x API.
//! It is still very early in development, so features may be missing or not working.

pub mod flux;
use std::{fmt::Display, io::Read};

pub use flux::{Precision, ReadQuery, WriteQuery};

#[derive(Debug)]
pub struct InfluxError {
    msg: Option<String>,
}

/// Use a Client to connect to your influx database and execute queries.
pub struct Client<'a> {
    url: &'a str,
    token: String,
    reqwest_client: reqwest::blocking::Client,
}

impl<'a> Client<'a> {
    /// Create a client with a given database URL and token.
    pub fn new(url: &'a str, token: &'a str) -> Self {
        Self {
            url,
            token: token.to_owned(),
            reqwest_client: reqwest::blocking::Client::new(),
        }
    }

    /// Create a client that reads its token from the
    /// `INFLUXDB_TOKEN` environment variable
    pub fn from_env(url: &'a str) -> Result<Self, InfluxError> {
        Ok(Self {
            url,
            token: std::env::var("INFLUXDB_TOKEN").map_err(|e| InfluxError {
                msg: Some(e.to_string()),
            })?,
            reqwest_client: reqwest::blocking::Client::new(),
        })
    }

    /// Insert a new value into a bucket.
    /// Note that not all attributes on `WriteQuery` are supported yet.
    pub fn insert<T: Display>(
        &self,
        bucket: &'a str,
        org: &'a str,
        precision: Precision,
        query: WriteQuery<T>,
    ) -> Result<(), InfluxError> {
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
            .map_err(|e| InfluxError {
                msg: Some(e.to_string()),
            })?;
        Ok(())
    }

    /// Retrieve a value from a bucket based on certain filters.
    pub fn get(&self, org: &'a str, query: ReadQuery) -> Result<String, InfluxError> {
        self.get_raw(org, &format!("{}", query))
    }

    /// If you prefer to write your own `flux` queries, use this method.
    /// As `flux` support is not complete yet, this is currently the only
    /// way to use the full `flux` language.
    pub fn get_raw(&self, org: &'a str, query: &'a str) -> Result<String, InfluxError> {
        let mut buf = String::new();
        self.reqwest_client
            .post(&format!("{}/api/v2/query?org={}", self.url, org))
            .header("Accept", "application/csv")
            .header("Content-Type", "application/vnd.flux")
            .header("Authorization", &format!("Token {}", self.token))
            .body(query.to_owned())
            .send()
            .map_err(|e| InfluxError {
                msg: Some(e.to_string()),
            })?
            .read_to_string(&mut buf)
            .map_err(|e| InfluxError {
                msg: Some(e.to_string()),
            })?;

        Ok(buf)
    }
}
