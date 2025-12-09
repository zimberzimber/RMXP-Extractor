
use crate::structured::{AudioFile, Color, Table2};

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Animation")]
pub struct Animation {
    pub id: usize,
    pub name: String,
    pub animation_name: String,
    pub animation_hue: i32,
    pub position: Position,
    pub frame_max: usize,
    pub frames: Vec<Frame>,
    pub timings: Vec<Timing>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Animation::Timing")]
pub struct Timing {
    pub frame: usize,
    pub se: AudioFile,
    pub flash_scope: Scope,
    pub flash_color: Color,
    pub flash_duration: usize,
    pub condition: Condition,
}
#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Animation::Frame")]
pub struct Frame {
    pub cell_max: usize,
    pub cell_data: Table2,
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
pub enum Position {
    Top = 0,
    #[default]
    Middle = 1,
    Bottom = 2,
    Screen = 3,
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
pub enum Scope {
    #[default]
    None = 0,
    Target = 1,
    Screen = 2,
    HideTarget = 3,
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
pub enum Condition {
    #[default]
    None = 0,
    Hit = 1,
    Miss = 2,
}
