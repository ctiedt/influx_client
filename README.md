# influx_client

A Rust library to interact with [InfluxDB](https://www.influxdata.com/) databases.
It is still early in development, so expect bugs and missing features.

## Examples

Reading from a bucket:

```rust
use influx_client::{Client, NumericFilter, Precision, ReadQuery, StringFilter};

fn main() {
    let client = Client::new("http://localhost:8086", "YOUR TOKEN HERE");

    let q = ReadQuery::new("home")
        .range(Some((-12, Precision::h)), None)
        .filter(StringFilter::Eq("_measurement", "test"))
        .filter(NumericFilter::Lt("_value", 99));
    println!("{}", q);

    dbg!(client.get("home", q));
}

```

Writing to a bucket:

```rust
use std::collections::BTreeMap;

use influx_client::{Client, Precision, WriteQuery};

fn main() {
    let client = Client::new("http://localhost:8086", "YOUR TOKEN HERE");
    let data = WriteQuery {
        name: "test",
        tags: BTreeMap::new(),
        field_name: "i",
        value: 42,
        timestamp: None,
    };

    client.insert("home", "home", Precision::ms, data);
}

```