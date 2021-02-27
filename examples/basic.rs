// This is the simple example of inserting data into InfluxDB and reading it again.

use std::{collections::HashMap, time::SystemTime};

use influx_client::{
    flux::functions::{NumericFilter, Range, StringFilter},
    Client, InfluxError, Precision, ReadQuery, WriteQuery,
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

    let q = ReadQuery::new("home")
        .range(Range::new(Some((-12, Precision::h)), None))
        .filter(StringFilter::Eq("_measurement", "test"))
        .filter(NumericFilter::Lt("_value", 99));

    println!("{}", client.get("home", q)?);
    Ok(())
}
