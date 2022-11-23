use clap::Parser;
use std::path::PathBuf;

pub fn parse() -> Opts {
    Opts::parse()
}

/// Convert JSON data to CSV
#[derive(Parser, Debug)]
pub struct Opts {
    #[arg(short, long, default_value = "/dev/stdin")]
    pub input_file: PathBuf,
    /// Where to write output. Defaults to standard output.
    #[arg(short, long, default_value = "/dev/stdout")]
    pub output_file: PathBuf,
    #[arg(short, long, default_value = ",", value_parser = unescape_char)]
    pub delimiter: char,
}

fn unescape_chars(src: &str) -> anyhow::Result<String> {
    let collected: String = unescape::unescape(src)
        .ok_or_else(|| anyhow::anyhow!("Failed to unescape delimiter"))?
        .chars()
        .collect();
    Ok(collected)
}

fn unescape_char(src: &str) -> anyhow::Result<char> {
    let chars: Vec<char> = unescape_chars(src)?.chars().collect();
    match &chars[..] {
        [c] => Ok(c.to_owned()),
        arr => anyhow::bail!("Needs to be exactly 1 character, not {}", arr.len()),
    }
}
