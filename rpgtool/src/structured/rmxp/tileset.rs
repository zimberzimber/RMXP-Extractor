

use crate::structured::{BlendMode, Table1};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Tileset")]
pub struct Tileset {
    pub id: usize,
    pub name: String,
    pub tileset_name: String,
    pub autotile_names: [String; 7],
    pub panorama_name: String,
    pub panorama_hue: i32,
    pub fog_name: String,
    pub fog_hue: i32,
    pub fog_opacity: i32,
    pub fog_blend_type: BlendMode,
    pub fog_zoom: i32,
    pub fog_sx: i32,
    pub fog_sy: i32,
    pub battleback_name: String,
    pub passages: Table1,
    pub priorities: Table1,
    pub terrain_tags: Table1,
}
