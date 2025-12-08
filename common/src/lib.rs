mod ser;
pub use ser::SerializeValue;

mod de;
pub use de::DeserializeValue;

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
