

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::State")]
pub struct State {
    pub id: usize,
    pub name: String,
    pub animation_id: usize,
    pub restriction: Restriction,
    pub nonresistance: bool,
    pub zero_hp: bool,
    pub cant_get_exp: bool,
    pub cant_evade: bool,
    pub slip_damage: bool,
    pub rating: i32,
    pub hit_rate: i32,
    pub maxhp_rate: i32,
    pub maxsp_rate: i32,
    pub str_rate: i32,
    pub dex_rate: i32,
    pub agi_rate: i32,
    pub int_rate: i32,
    pub atk_rate: i32,
    pub pdef_rate: i32,
    pub mdef_rate: i32,
    pub eva: i32,
    pub battle_only: bool,
    pub hold_turn: i32,
    pub auto_release_prob: i32,
    pub shock_release_prob: i32,
    pub guard_element_set: Vec<usize>,
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
pub enum Restriction {
    #[default]
    None = 0,
    NoMagic = 1,
    AttackEnemies = 2,
    AttackAllies = 3,
    NoMove = 4,
}
