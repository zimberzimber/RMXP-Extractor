pub use crate::structured::{AudioFile, Occasion, Scope};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Skill")]
pub struct Skill {
    pub id: usize,
    pub name: String,
    pub icon_name: String,
    pub description: String,
    pub scope: Scope,
    pub occasion: Occasion,
    pub animation1_id: usize,
    pub animation2_id: usize,
    pub menu_se: AudioFile,
    pub common_event_id: usize,
    pub sp_cost: i32,
    pub power: i32,
    pub atk_f: i32,
    pub eva_f: i32,
    pub str_f: i32,
    pub dex_f: i32,
    pub agi_f: i32,
    pub int_f: i32,
    pub hit: i32,
    pub pdef_f: i32,
    pub mdef_f: i32,
    pub variance: i32,
    pub element_set: Vec<usize>,
    pub plus_state_set: Vec<usize>,
    pub minus_state_set: Vec<usize>,
}
