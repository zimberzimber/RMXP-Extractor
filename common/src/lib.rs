mod ser;
pub use ser::SerializeValue;

mod de;
pub use de::DeserializeValue;

use serde::Serialize;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Format {
    Json,
    Marshal,
    Ron,
    Yaml,
}

impl Format {
    pub fn guess(path: &std::path::Path) -> Option<Format> {
        let ext = path.extension()?;
        Self::guess_from_ext(ext)
    }

    pub fn guess_from_ext(ext: impl AsRef<std::ffi::OsStr>) -> Option<Format> {
        let ext = ext.as_ref().to_str()?;
        match ext {
            "json" | "jsonc" => Some(Format::Json),
            "rxdata" | "rvdata" | "rvdata2" => Some(Format::Marshal),
            "ron" => Some(Format::Ron),
            "yaml" | "yml" => Some(Format::Yaml),
            _ => None,
        }
    }

    pub fn file_ext(self) -> &'static std::path::Path {
        std::path::Path::new(match self {
            Format::Json => "json",
            Format::Marshal => "rxdata",
            Format::Ron => "ron",
            Format::Yaml => "yaml",
        })
    }
}

pub fn conv_io<R, W>(from: Format, to: Format, mut input: R, mut output: W)
where
    R: Read,
    W: Write,
{
    let value: alox_48::Value = match from {
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
