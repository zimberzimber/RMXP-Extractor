
use crate::structured::Table2;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Actor")]
pub struct Actor {
    pub id: usize,
    pub name: String,
    pub class_id: usize,
    pub initial_level: i32,
    pub final_level: i32,
    pub exp_basis: i32,
    pub exp_inflation: i32,
    pub character_name: String,
    pub character_hue: i32,
    pub battler_name: String,
    pub battler_hue: i32,
    pub parameters: Table2,
    pub weapon_id: usize,
    pub armor1_id: usize,
    pub armor2_id: usize,
    pub armor3_id: usize,
    pub armor4_id: usize,
    pub weapon_fix: bool,
    pub armor1_fix: bool,
    pub armor2_fix: bool,
    pub armor3_fix: bool,
    pub armor4_fix: bool,
}
