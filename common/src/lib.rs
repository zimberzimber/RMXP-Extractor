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
        let ext = path.extension()?.to_str()?;
        match ext {
            "json" | "jsonc" => Some(Format::Json),
            "rxdata" | "rvdata" | "rvdata2" => Some(Format::Marshal),
            "ron" => Some(Format::Ron),
            "yaml" | "yml" => Some(Format::Yaml),
            _ => None,
        }
    }
}
