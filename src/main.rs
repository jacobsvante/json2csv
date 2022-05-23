use std::{fs::File, io::{stdout, Write}};
use json2csv::{cli, json2csv};

fn main() -> anyhow::Result<()> {
    let opts = cli::parse();
    let csv_data = json2csv(
        File::open(opts.input_file)?,
        Some(opts.delimiter),
    )?;
    stdout().write_all(&csv_data)?;
    Ok(())
}
