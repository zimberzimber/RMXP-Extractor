
pub use crate::structured::Table1;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Class")]
pub struct Class {
    pub id: usize,
    pub name: String,
    pub position: Position,

    pub weapon_set: Vec<usize>,

    pub armor_set: Vec<usize>,
    pub element_ranks: Table1,
    pub state_ranks: Table1,
    pub learnings: Vec<Learning>,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Class::Learning")]
pub struct Learning {
    pub level: i32,

    pub skill_id: usize,
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
    #[default]
    Front = 0,
    Middle = 1,
    Rear = 2,
}
