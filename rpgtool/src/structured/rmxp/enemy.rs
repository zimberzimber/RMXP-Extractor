
pub use crate::structured::Table1;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Enemy")]
pub struct Enemy {
    pub id: usize,
    pub name: String,
    pub battler_name: String,
    pub battler_hue: i32,
    pub maxhp: i32,
    pub maxsp: i32,
    pub str: i32,
    pub dex: i32,
    pub agi: i32,
    pub int: i32,
    pub atk: i32,
    pub pdef: i32,
    pub mdef: i32,
    pub eva: i32,
    pub animation1_id: usize,
    pub animation2_id: usize,
    pub element_ranks: Table1,
    pub state_ranks: Table1,
    pub actions: Vec<Action>,
    pub exp: i32,
    pub gold: i32,
    pub item_id: usize,
    pub weapon_id: usize,
    pub armor_id: usize,
    pub treasure_prob: i32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Enemy::Action")]
pub struct Action {
    pub kind: Kind,
    pub basic: Basic,

    pub skill_id: usize,
    pub condition_turn_a: i32,
    pub condition_turn_b: i32,
    pub condition_hp: i32,
    pub condition_level: i32,
    pub condition_switch_id: usize,
    pub rating: i32,
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
pub enum Kind {
    #[default]
    Basic = 0,
    Skill = 1,
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
pub enum Basic {
    #[default]
    Attack = 0,
    Defend = 1,
    Escape = 2,
    DoNothing = 3,
}
