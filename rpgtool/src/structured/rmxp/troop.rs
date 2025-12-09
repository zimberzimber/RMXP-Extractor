
use crate::structured::EventCommand;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Troop")]
pub struct Troop {
    pub id: usize,
    pub name: String,
    pub members: Vec<Member>,
    pub pages: Vec<Page>,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Troop::Member")]
pub struct Member {
    pub enemy_id: usize,
    pub x: i32,
    pub y: i32,
    pub hidden: bool,
    pub immortal: bool,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Troop::Page")]
pub struct Page {
    pub condition: Condition,
    pub span: i32,
    pub list: Vec<EventCommand>,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Troop::Page::Condition")]
pub struct Condition {
    pub turn_valid: bool,
    pub enemy_valid: bool,
    pub actor_valid: bool,
    pub switch_valid: bool,
    pub turn_a: i32,
    pub turn_b: i32,
    pub enemy_index: usize,
    pub enemy_hp: i32,
    pub actor_id: usize,
    pub actor_hp: i32,
    pub switch_id: usize,
}
