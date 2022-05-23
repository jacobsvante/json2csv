use csv::StringRecord;
use serde_json::{Map, Value};
use std::io::Read;

pub mod cli;

fn parse_delimiter(d: char) -> anyhow::Result<u8> {
    u8::try_from(u32::from(d)).map_err(anyhow::Error::from)
}

/// Deserializes input as JSON and serializes it to output as CSV
pub fn json2csv<I: Read>(input: I, delimiter: Option<char>) -> anyhow::Result<Vec<u8>> {

    let delimiter = parse_delimiter(delimiter.unwrap_or(','))?;
    let output = Vec::new();

    let mut writer = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(output);

    let entries: Vec<Map<String, Value>> = serde_json::from_reader(input)?;

    if entries.len() > 0 {
        let columns = {
            let r0 = &entries[0];
            r0.keys().collect::<Vec<_>>()
        };
        writer.write_record(&columns)?;
        for (idx, record) in entries.iter().enumerate() {
            if record.keys().collect::<Vec<_>>() != columns {
                eprintln!(
                    "Record #{} contains different fields than record #1",
                    idx + 1
                );
                std::process::exit(1);
            }
        }
        for entry in entries {
            let mut record = StringRecord::new();
            for value in entry.values() {
                record.push_field(value.as_str().unwrap());
            }
            writer.write_record(&record)?;
        }
    }
    Ok(writer.into_inner()?)
}
