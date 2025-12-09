mod audio_file;
mod event;
mod mapinfo;
mod move_route;
mod parameter_type;
mod script;

pub use audio_file::*;
pub use event::*;
pub use mapinfo::*;
pub use move_route::*;
pub use parameter_type::*;
pub use script::Script;

use crate::structured::NilPadded;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Default, Hash)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[repr(u8)]
#[serde(into = "u8")]
#[serde(try_from = "u8")]
#[marshal(into = "u8")]
#[marshal(try_from = "u8")]
pub enum BlendMode {
    #[default]
    Normal = 0,
    Add = 1,
    Subtract = 2,
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
    OneEnemy = 1,
    AllEnemies = 2,
    OneAlly = 3,
    AllAllies = 4,
    OneAllyHP0 = 5,
    AllAlliesHP0 = 6,
    User = 7,
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
pub enum Occasion {
    #[default]
    Always = 0,
    OnlyBattle = 1,
    OnlyMenu = 2,
    Never = 3,
}

pub type MapInfos = std::collections::BTreeMap<usize, MapInfo>;
pub type CommonEvents = NilPadded<CommonEvent>;
