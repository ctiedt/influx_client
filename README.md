# influx_client

A Rust library to interact with [InfluxDB](https://www.influxdata.com/) databases.
It is still early in development, so expect bugs and missing features.

## Things that work

- Writing data to a bucket
- Querying data in a certain time range (only relative so far)
- Using filters in queries

## Examples

Reading from a bucket:

```rust
use influx_client::{
    Client, Precision, WriteQuery,
};

fn main() {
    let client = Client::from_env("http://localhost:8086").expect("INFLUXDB_TOKEN not set");
    let mut tags = BTreeMap::new();
    tags.insert("t1", "v1");
    tags.insert("t2", "v2");
    let data = WriteQuery {
        name: "test",
        tags,
        field_name: "i",
        value: 42,
        timestamp: None,
    };

    client.insert("home", "home", Precision::ms, data);
}
```

Writing to a bucket:

```rust
use std::collections::BTreeMap;

use influx_client::{
    flux::functions::{NumericFilter, Range, StringFilter},
    Client, Precision, ReadQuery,
};

fn main() {
    let client = Client::from_env("http://localhost:8086").expect("INFLUXDB_TOKEN not set");

    let q = ReadQuery::new("home")
        .range(Range::new(Some((-12, Precision::h)), None))
        .filter(StringFilter::Eq("_measurement", "test"))
        .filter(NumericFilter::Lt("_value", 99));

    println!("{}", client.get("home", q).unwrap());
}
```