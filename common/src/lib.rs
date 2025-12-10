#![warn(clippy::unwrap_used, clippy::pedantic, rust_2018_idioms)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation
)]

mod de;
mod ser;

pub struct Value(pub alox_48::Value);

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        de::DeserializeValue::deserialize(deserializer).map(|v| Self(v.0))
    }
}

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser::SerializeValue(&self.0).serialize(serializer)
    }
}

impl<'de> alox_48::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> alox_48::DeResult<Self>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        alox_48::Value::deserialize(deserializer).map(Self)
    }
}

impl alox_48::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> alox_48::SerResult<S::Ok>
    where
        S: alox_48::SerializerTrait,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, clap::ValueEnum)]
#[non_exhaustive]
pub enum Format {
    Json,
    Marshal,
    Ron,
    Yaml,
    // Saphyr,
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
            Format::Yaml /* | Format::Saphyr */ => "yaml",
        })
    }
}

// TODO provide trace
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
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
    // #[error("{0}")]
    // SaphyrDe(#[from] serde_saphyr::Error),
    // #[error("{0}")]
    // SaphyrSer(#[from] serde_saphyr::ser_error::Error),
    // #[error("No YAML document was present")]
    // SaphyrNoDocument,
    #[error("{0}")]
    Yaml(#[from] serde_yaml_ng::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

// FIXME shitty hack because serde_yaml doesn't support binary and we have no way to recover from serialization errors
thread_local! {
    static BYTES_ALLOWED: std::cell::Cell<bool> = const { std::cell::Cell::new(true) };
}

fn set_binary_bytes_allowed(allowed: bool) {
    BYTES_ALLOWED.set(allowed);
}

fn binary_bytes_allowed() -> bool {
    BYTES_ALLOWED.get()
}

pub fn conv_read<R, T>(from: Format, mut input: R) -> Result<T, ConvError>
where
    R: std::io::Read,
    T: for<'de> serde::Deserialize<'de> + for<'de> alox_48::Deserialize<'de>,
{
    set_binary_bytes_allowed(from != Format::Yaml);

    let value = match from {
        Format::Marshal => {
            let mut data = vec![];
            input.read_to_end(&mut data)?;
            alox_48::from_bytes::<T>(&data)?
        }
        Format::Json => serde_json::from_reader(input)?,
        Format::Ron => ron::Options::default().from_reader(input)?,
        // Format::Saphyr => {
        //     let mut iter = serde_saphyr::read(&mut input);
        //     iter.next().ok_or(ConvError::SaphyrNoDocument)??
        // }
        Format::Yaml => serde_yaml_ng::from_reader(input)?,
    };
    Ok(value)
}

pub fn conv_write<W, T>(value: T, to: Format, mut output: W) -> Result<(), ConvError>
where
    W: std::io::Write,
    T: serde::Serialize + alox_48::Serialize,
{
    set_binary_bytes_allowed(to != Format::Yaml);

    match to {
        Format::Marshal => {
            let data = alox_48::to_bytes(value)?;
            output.write_all(&data)?;
        }
        Format::Json => {
            let mut ser = serde_json::Serializer::pretty(output);
            serde::Serialize::serialize(&value, &mut ser)?;
        }
        Format::Ron => {
            let config = ron::ser::PrettyConfig::default();
            ron::Options::default().to_io_writer_pretty(output, &value, config)?;
        }
        // Format::Saphyr => {
        //     serde_saphyr::to_io_writer(&mut output, &value)?;
        // }
        Format::Yaml => {
            let mut ser = serde_yaml_ng::Serializer::new(output);
            serde::Serialize::serialize(&value, &mut ser)?;
        }
    }

    Ok(())
}
