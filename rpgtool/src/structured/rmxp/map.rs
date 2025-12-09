use crate::structured::{AudioFile, Event, Table3};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Map")]
pub struct Map {
    pub tileset_id: usize,
    pub width: usize,
    pub height: usize,
    pub autoplay_bgm: bool,
    pub bgm: AudioFile,
    pub autoplay_bgs: bool,
    pub bgs: AudioFile,
    pub encounter_list: Vec<i32>,
    pub encounter_step: i32,
    pub data: Table3,
    pub events: std::collections::BTreeMap<usize, Event>,
}
