// This example shows how you might deserialize CSV records read from InfluxDB.

use std::{convert::TryFrom, fmt::Display, num::ParseIntError};

use csv::StringRecord;
use influx_client::{
    blocking::Client,
    flux::functions::{NumericFilter, Range, StringFilter},
    InfluxError, Precision, ReadQuery,
};

// We define our own record type with all the data we want.
// In a real-world scenario, you might want to parse the timestamp using e.g. chrono.

#[derive(Debug)]
struct Record<T>
where
    T: Display,
{
    time: String,
    measurement: String,
    field: String,
    value: T,
}

// This crate treats all records as string records.
// If you want to work with the data, you have to convert it yourself,
// e.g. using the TryFrom trait.

// If you don't want to implement this yourself, consider using the serde feature in the csv crate.

impl TryFrom<&StringRecord> for Record<u32> {
    type Error = InfluxError;

    fn try_from(value: &StringRecord) -> Result<Self, Self::Error> {
        let time = value
            .get(5)
            .ok_or_else(|| InfluxError {
                msg: Some("No Timestamp".to_owned()),
            })?
            .to_owned();
        let measurement = value
            .get(8)
            .ok_or_else(|| InfluxError {
                msg: Some("No Timestamp".to_owned()),
            })?
            .to_owned();
        let field = value
            .get(7)
            .ok_or_else(|| InfluxError {
                msg: Some("No Timestamp".to_owned()),
            })?
            .to_owned();
        let value = value.get(6).ok_or_else(|| InfluxError {
            msg: Some("No Timestamp".to_owned()),
        })?;
        Ok(Self {
            time,
            measurement,
            field,
            value: value.parse().map_err(|e: ParseIntError| InfluxError {
                msg: Some(e.to_string()),
            })?,
        })
    }
}

fn main() -> Result<(), InfluxError> {
    let client = Client::from_env("http://localhost:8086").expect("INFLUXDB_TOKEN not set");

    let q = ReadQuery::new("home")
        .range(Range::new(Some((-12, Precision::h)), None))
        .filter(StringFilter::Eq("_measurement", "test"))
        .filter(NumericFilter::Lt("_value", 99));

    let csv = client.get_csv("home", q)?;
    for rec in &csv {
        println!("{:?}", Record::try_from(rec)?);
    }
    Ok(())
}
