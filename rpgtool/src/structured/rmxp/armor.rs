#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Armor")]
pub struct Armor {
    pub id: usize,
    pub name: String,
    pub icon_name: String,
    pub description: String,
    pub kind: Kind,
    pub auto_state_id: usize,
    pub price: i32,
    pub pdef: i32,
    pub mdef: i32,
    pub eva: i32,
    pub str_plus: i32,
    pub dex_plus: i32,
    pub agi_plus: i32,
    pub int_plus: i32,
    pub guard_element_set: Vec<usize>,
    pub guard_state_set: Vec<usize>,
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
    Shield = 0,
    Helmet = 1,
    BodyArmor = 2,
    Accessory = 3,
}
