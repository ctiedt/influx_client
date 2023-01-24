// You can also read CSV records from InfluxDB.
// This crate treats all records as string records.
// If you want them parsed, take a look at the csv_records example.

use influx_client::{
    blocking::Client,
    flux::functions::{NumericFilter, Range, StringFilter},
    InfluxError, Precision, ReadQuery,
};

fn main() -> Result<(), InfluxError> {
    let client = Client::from_env("http://localhost:8086").expect("INFLUXDB_TOKEN not set");

    let q = ReadQuery::new("home")
        .range(Range::new(Some((-12, Precision::h)), None))
        .filter(StringFilter::Eq("_measurement", "test"))
        .filter(NumericFilter::Lt("_value", 99));

    let csv = client.get_csv("home", q)?;
    for rec in &csv {
        for (i, v) in rec.iter().enumerate() {
            println!("{i}: {v}");
        }
    }
    Ok(())
}
