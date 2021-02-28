# influx_client
[![crates-badge](https://img.shields.io/crates/v/influx_client?style=for-the-badge)](https://crates.io/crates/influx_client) [![docs-badge](https://img.shields.io/docsrs/influx-client/latest?style=for-the-badge)](https://docs.rs/influx_client)

A Rust library to interact with [InfluxDB](https://www.influxdata.com/) databases.
It is still early in development, so expect bugs and missing features.

## Things that work

- Writing data to a bucket
- Querying data in a certain time range (only relative so far)
- Using filters in queries

## Examples

Writing to a bucket:

```rust
use std::{collections::HashMap, time::SystemTime};

use influx_client::{
    Client, InfluxError, Precision, WriteQuery,
};

fn main() -> Result<(), InfluxError> {
    let client = Client::from_env("http://localhost:8086").expect("INFLUXDB_TOKEN not set");
    let mut tags = HashMap::new();
    tags.insert("t1", "v1");
    tags.insert("t2", "v2");
    let data = WriteQuery {
        name: "test",
        tags,
        field_name: "i",
        value: 42,
        timestamp: Some((SystemTime::now(), Precision::ns)),
    };

    client.insert("home", "home", Precision::ms, data)?;
}

```

Reading from a bucket:

```rust
use influx_client::{
    flux::functions::{NumericFilter, Range, StringFilter},
    Client, InfluxError, Precision, ReadQuery,
};

fn main() -> Result<(), InfluxError> {
    let client = Client::from_env("http://localhost:8086").expect("INFLUXDB_TOKEN not set");
    
    let q = ReadQuery::new("home")
        .range(Range::new(Some((-12, Precision::h)), None))
        .filter(StringFilter::Eq("_measurement", "test"))
        .filter(NumericFilter::Lt("_value", 99));

    println!("{}", client.get("home", q)?);
    Ok(())
}

```