//! An unofficial library to read from and write to [InfluxDB](https://www.influxdata.com/) databases.
//! This library supports the 2.x API.
//! It is still very early in development, so features may be missing or not working.

pub mod blocking;
pub mod flux;
use std::fmt::Display;

use csv::StringRecord;
pub use flux::{Precision, ReadQuery, WriteQuery};
use futures::TryFutureExt;

#[derive(Debug)]
pub struct InfluxError {
    pub msg: Option<String>,
}

/// Use a Client to connect to your influx database and execute queries.
pub struct Client<'a> {
    url: &'a str,
    token: String,
    reqwest_client: reqwest::Client,
}

impl<'a> Client<'a> {
    /// Create a client with a given database URL and token.
    pub fn new(url: &'a str, token: &'a str) -> Self {
        Self {
            url,
            token: token.to_owned(),
            reqwest_client: reqwest::Client::new(),
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
            reqwest_client: reqwest::Client::new(),
        })
    }

    /// Insert a new value into a bucket.
    /// Note that not all attributes on `WriteQuery` are supported yet.
    pub async fn insert<T: Display>(
        &self,
        bucket: &'a str,
        org: &'a str,
        precision: Precision,
        query: WriteQuery<'a, T>,
    ) -> Result<(), InfluxError> {
        self.reqwest_client
            .post(&format!(
                "{}/api/v2/write?org={}&bucket={}&precision={}",
                self.url, org, bucket, precision
            ))
            .header("Authorization", &format!("Token {}", self.token))
            .body(query.to_string())
            .send()
            .map_err(|e| InfluxError {
                msg: Some(e.to_string()),
            })
            .await?;
        Ok(())
    }

    /// Retrieve a value from a bucket based on certain filters.
    pub async fn get(&self, org: &'a str, query: ReadQuery<'a>) -> Result<String, InfluxError> {
        self.get_raw(org, &query.to_string()).await
    }

    pub async fn get_csv(
        &self,
        org: &'a str,
        query: ReadQuery<'a>,
    ) -> Result<Vec<StringRecord>, InfluxError> {
        let res = self.get(org, query).await?;
        let reader = csv::ReaderBuilder::new().from_reader(res.as_bytes());
        Ok(reader.into_records().map(|r| r.unwrap()).collect())
    }

    /// If you prefer to write your own `flux` queries, use this method.
    /// As `flux` support is not complete yet, this is currently the only
    /// way to use the full `flux` language.
    pub async fn get_raw(&self, org: &'a str, query: &'a str) -> Result<String, InfluxError> {
        self.reqwest_client
            .post(&format!("{}/api/v2/query?org={}", self.url, org))
            .header("Accept", "application/csv")
            .header("Content-Type", "application/vnd.flux")
            .header("Authorization", &format!("Token {}", self.token))
            .body(query.to_owned())
            .send()
            .await
            .map_err(|e| InfluxError {
                msg: Some(e.to_string()),
            })?
            .text()
            .await
            .map_err(|e| InfluxError {
                msg: Some(e.to_string()),
            })
    }
}
