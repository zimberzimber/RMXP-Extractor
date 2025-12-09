

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::AudioFile")]
pub struct AudioFile {
    pub name: String,
    pub volume: u8,
    pub pitch: u8,
}
