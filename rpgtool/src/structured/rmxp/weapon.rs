#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Weapon")]
pub struct Weapon {
    pub id: usize,
    pub name: String,
    pub icon_name: String,
    pub description: String,
    pub animation1_id: usize,
    pub animation2_id: usize,
    pub price: i32,
    pub atk: i32,
    pub pdef: i32,
    pub mdef: i32,
    pub str_plus: i32,
    pub dex_plus: i32,
    pub agi_plus: i32,
    pub int_plus: i32,
    pub element_set: Vec<usize>,
    pub plus_state_set: Vec<usize>,
    pub minus_state_set: Vec<usize>,
}
