#![warn(clippy::unwrap_used, clippy::pedantic, rust_2018_idioms)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation
)]

mod ser;
use std::sync::atomic::{AtomicBool, Ordering};

pub use ser::SerializeValue;

mod de;
pub use de::DeserializeValue;

use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Copy, clap::ValueEnum)]
pub enum Format {
    Json,
    Marshal,
    Ron,
    Yaml,
    Saphyr,
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
            Format::Yaml | Format::Saphyr => "yaml",
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
    SaphyrDe(#[from] serde_saphyr::Error),
    #[error("{0}")]
    SaphyrSer(#[from] serde_saphyr::ser_error::Error),
    #[error("No YAML document was present")]
    SaphyrNoDocument,
    #[error("{0}")]
    Yaml(#[from] serde_yaml_ng::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

static BYTES_ALLOWED: AtomicBool = AtomicBool::new(true);

fn set_binary_bytes_allowed(allowed: bool) {
    BYTES_ALLOWED.store(allowed, Ordering::Relaxed);
}

fn binary_bytes_allowed() -> bool {
    BYTES_ALLOWED.load(Ordering::Relaxed)
}

pub fn conv_read<R>(from: Format, mut input: R) -> Result<alox_48::Value, ConvError>
where
    R: std::io::Read,
{
    set_binary_bytes_allowed(from != Format::Yaml);

    let value = match from {
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
        Format::Saphyr => {
            let mut iter = serde_saphyr::read(&mut input);
            let value: DeserializeValue = iter.next().ok_or(ConvError::SaphyrNoDocument)??;
            value.0
        }
        Format::Yaml => {
            let value: DeserializeValue = serde_yaml_ng::from_reader(input)?;
            value.0
        }
    };
    Ok(value)
}

pub fn conv_write<W>(value: alox_48::Value, to: Format, mut output: W) -> Result<(), ConvError>
where
    W: std::io::Write,
{
    set_binary_bytes_allowed(to != Format::Yaml);

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
        Format::Saphyr => {
            serde_saphyr::to_io_writer(&mut output, &SerializeValue(&value))?;
        }
        Format::Yaml => {
            let mut ser = serde_yaml_ng::Serializer::new(output);
            SerializeValue(&value).serialize(&mut ser)?;
        }
    }

    Ok(())
}
