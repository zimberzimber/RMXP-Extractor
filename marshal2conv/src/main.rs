use std::{io::Read, io::Write, path::PathBuf};

use alox_48::Value;
use clap::{CommandFactory, Parser, error::ErrorKind};
use serde::Serialize;

use common::{DeserializeValue, Format, SerializeValue};

// TODO stdin?
/// Converts Ruby marshal files to other formats, and vice versa.
#[derive(Parser)]
struct Args {
    /// The source file.
    src: PathBuf,
    /// The destination file.
    dest: PathBuf,
    /// The formats to convert from/to.
    ///
    /// Input comes first.
    ///
    /// Required if the format cannot be determined via file extensions.
    #[arg(long, visible_short_alias = 'f', number_of_values = 2)]
    format: Option<Vec<Format>>,
}

fn main() {
    let Args { src, dest, format } = Args::parse();

    let [from, to] = match format.as_deref() {
        Some(&[from, to]) => [from, to],
        None => {
            let Some((from, to)) = Format::guess(&src).zip(Format::guess(&dest)) else {
                let mut command = Args::command();
                command
                    .error(
                        ErrorKind::DisplayHelp,
                        "unable to determine conversion formats, please specify with --format",
                    )
                    .exit()
            };
            [from, to]
        }
        _ => unreachable!(), // we enforce the number of values in clap
    };

    let mut input = std::fs::File::open(src).unwrap();
    let mut output = std::fs::File::create(dest).unwrap();

    let value: Value = match from {
        Format::Marshal => {
            let mut data = vec![];
            input.read_to_end(&mut data).unwrap();
            alox_48::from_bytes(&data).unwrap()
        }
        Format::Json => {
            let value: DeserializeValue = serde_json::from_reader(input).unwrap();
            value.0
        }
        Format::Ron => {
            let value: DeserializeValue = ron::Options::default().from_reader(input).unwrap();
            value.0
        }
        Format::Yaml => {
            let value: DeserializeValue = serde_yaml_ng::from_reader(input).unwrap();
            value.0
        }
    };

    match to {
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
