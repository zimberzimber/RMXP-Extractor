#![warn(clippy::unwrap_used)]

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

#[derive(Debug, thiserror::Error)]
pub enum ConvError {
    #[error("{0}")]
    MarshalDe(#[from] alox_48::DeError),
    #[error("{0}")]
    MarshalSer(#[from] alox_48::SerError),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    RonDe(#[from] ron::de::SpannedError),
    #[error("{0}")]
    Ron(#[from] ron::Error),
    #[error("{0}")]
    YamlDe(#[from] serde_saphyr::Error),
    #[error("{0}")]
    YamlSer(#[from] serde_saphyr::ser_error::Error),
    #[error("No YAML document was present")]
    YamlNoDocument,
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

pub fn conv_io<R, W>(from: Format, to: Format, mut input: R, mut output: W) -> Result<(), ConvError>
where
    R: Read,
    W: Write,
{
    let value: alox_48::Value = match from {
        Format::Marshal => {
            let mut data = vec![];
            input.read_to_end(&mut data)?;
            alox_48::from_bytes(&data)?
        }
        Format::Json => {
            let value: DeserializeValue = serde_json::from_reader(input)?;
            value.0
        }
        Format::Ron => {
            let value: DeserializeValue = ron::Options::default().from_reader(input)?;
            value.0
        }
        Format::Yaml => {
            let mut iter = serde_saphyr::read(&mut input);
            let value: DeserializeValue = iter.next().ok_or(ConvError::YamlNoDocument)??;
            value.0
        }
    };

    match to {
        Format::Marshal => {
            let data = alox_48::to_bytes(value)?;
            output.write_all(&data)?;
        }
        Format::Json => {
            let mut ser = serde_json::Serializer::pretty(output);
            SerializeValue(&value).serialize(&mut ser)?;
        }
        Format::Ron => {
            let config = ron::ser::PrettyConfig::default();
            ron::Options::default().to_io_writer_pretty(output, &SerializeValue(&value), config)?;
        }
        Format::Yaml => {
            serde_saphyr::to_io_writer(&mut output, &SerializeValue(&value))?;
        }
    }

    Ok(())
}
