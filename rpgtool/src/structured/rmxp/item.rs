
pub use crate::structured::{AudioFile, Occasion, Scope};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Item")]
pub struct Item {
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
    pub price: i32,
    pub consumable: bool,
    pub parameter_type: ParameterType,
    pub parameter_points: i32,
    pub recover_hp_rate: i32,
    pub recover_hp: i32,
    // These fields are missing in rmxp data *sometimes*.
    // Why? Who knows!
    #[marshal(default)]
    #[serde(default)]
    pub recover_sp_rate: i32,
    #[marshal(default)]
    #[serde(default)]
    pub recover_sp: i32,
    pub hit: i32,
    pub pdef_f: i32,
    pub mdef_f: i32,
    pub variance: i32,

    pub element_set: Vec<usize>,

    pub plus_state_set: Vec<usize>,

    pub minus_state_set: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[repr(u8)]
#[serde(into = "u8")]
#[serde(try_from = "u8")]
#[marshal(into = "u8")]
#[marshal(try_from = "u8")]
pub enum ParameterType {
    #[default]
    None = 0,
    MaxHP = 1,
    MaxSP = 2,
    Str = 3,
    Dex = 4,
    Agi = 5,
    Int = 6,
}
