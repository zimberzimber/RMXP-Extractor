use std::{io::Write, path::PathBuf};

use alox_48::Value;
use clap::Parser;
use serde::Serialize;

/// Converts Ruby marshal files to other formats, and vice versa.
#[derive(Parser)]
struct Args {
    /// The input file.
    ///
    /// Leave blank to read from STDIN
    #[arg(long, visible_short_alias = 'i')]
    input: PathBuf,
    /// The output file.
    ///
    /// Leave blank to write to STDOUT
    #[arg(long, visible_short_alias = 'o')]
    output: PathBuf,
    /// The formats to convert from/to.
    ///
    /// Input comes first.
    #[arg(long, visible_short_alias = 'f', number_of_values = 2, required = true)]
    format: Vec<Format>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Format {
    Json,
    Marshal,
    Ron,
    Yaml,
    Toml,
}

fn main() {
    let Args {
        input,
        output,
        format,
    } = Args::parse();

    #[allow(unused)]
    let [to, from] = format
        .try_into()
        .expect("should only be 2 (clap limitation)");

    let input = std::fs::read(input).unwrap();
    let value: Value = match to {
        Format::Marshal => alox_48::from_bytes(&input).unwrap(),
        Format::Json => {
            let value: common::DeserializeValue = serde_json::from_slice(&input).unwrap();
            value.0
        }
        _ => todo!(),
    };

    let mut output = std::fs::File::create(output).unwrap();
    match from {
        Format::Marshal => {
            let data = alox_48::to_bytes(value).unwrap();
            output.write_all(&data).unwrap();
        }
        Format::Json => {
            let mut ser = serde_json::Serializer::pretty(output);
            common::SerializeValue(&value).serialize(&mut ser).unwrap();
        }
        _ => todo!(),
    }
}
