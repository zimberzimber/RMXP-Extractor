#[derive(Default, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::MapInfo")]
pub struct MapInfo {
    pub name: String,
    pub parent_id: usize,
    pub order: i32,
    pub expanded: bool,
    pub scroll_x: i32,
    pub scroll_y: i32,
}

impl PartialOrd for MapInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MapInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}
