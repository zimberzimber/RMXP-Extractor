use std::{io::Write, path::PathBuf};

use alox_48::Value;
use clap::{CommandFactory, Parser, error::ErrorKind};
use serde::Serialize;

use common::{DeserializeValue, Format, SerializeValue};

// TODO stdin?
/// Converts Ruby marshal files to other formats, and vice versa.
#[derive(Parser)]
struct Args {
    /// The input file.
    #[arg(long, visible_short_alias = 'i')]
    input: PathBuf,
    /// The output file.
    #[arg(long, visible_short_alias = 'o')]
    output: PathBuf,
    /// The formats to convert from/to.
    ///
    /// Input comes first.
    ///
    /// Required if the format cannot be determined via file extensions.
    #[arg(long, visible_short_alias = 'f', number_of_values = 2)]
    format: Option<Vec<Format>>,
}

fn main() {
    let Args {
        input,
        output,
        format,
    } = Args::parse();

    let [to, from] = match format.as_deref() {
        Some(&[to, from]) => [to, from],
        None => {
            let Some((to, from)) = Format::guess(&input).zip(Format::guess(&output)) else {
                let mut command = Args::command();
                command
                    .error(ErrorKind::DisplayHelp, "unable to determine format")
                    .exit()
            };
            [to, from]
        }
        _ => unreachable!(), // we enforce the number of values in clap
    };

    let input = std::fs::read(input).unwrap();
    let value: Value = match to {
        Format::Marshal => alox_48::from_bytes(&input).unwrap(),
        Format::Json => {
            let value: DeserializeValue = serde_json::from_slice(&input).unwrap();
            value.0
        }
        Format::Ron => {
            let value: DeserializeValue = ron::Options::default().from_bytes(&input).unwrap();
            value.0
        }
        Format::Yaml => {
            let value: DeserializeValue = serde_yaml_ng::from_slice(&input).unwrap();
            value.0
        }
    };

    let mut output = std::fs::File::create(output).unwrap();
    match from {
        Format::Marshal => {
            let data = alox_48::to_bytes(value).unwrap();
            output.write_all(&data).unwrap();
        }
        Format::Json => {
            let mut ser = serde_json::Serializer::pretty(output);
            SerializeValue(&value).serialize(&mut ser).unwrap();
        }
        Format::Ron => {
            let config = ron::ser::PrettyConfig::default();
            ron::Options::default()
                .to_io_writer_pretty(output, &SerializeValue(&value), config)
                .unwrap();
        }
        Format::Yaml => {
            let mut ser = serde_yaml_ng::Serializer::new(output);
            SerializeValue(&value).serialize(&mut ser).unwrap();
        }
    }
}
