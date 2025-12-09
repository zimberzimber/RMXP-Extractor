use crate::structured::{BlendMode, MoveRoute, ParameterType};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event")]
pub struct Event {
    pub id: usize,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub pages: Vec<EventPage>,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::CommonEvent")]
pub struct CommonEvent {
    pub id: usize,
    pub name: String,
    pub trigger: usize,
    pub switch_id: usize,
    pub list: Vec<EventCommand>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event::Page")]
pub struct EventPage {
    pub condition: EventCondition,
    pub graphic: Graphic,
    pub move_type: MoveType,
    pub move_speed: MoveSpeed,
    pub move_frequency: MoveFreq,
    pub move_route: MoveRoute,
    pub walk_anime: bool,
    pub step_anime: bool,
    pub direction_fix: bool,
    pub through: bool,
    pub always_on_top: bool,
    pub trigger: EventTrigger,
    pub list: Vec<EventCommand>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum EventTrigger {
    ActionButton,
    PlayerTouch,
    EventTouch,
    Autorun,
    Parallel,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum MoveType {
    Fixed,
    Random,
    Approach,
    Custom,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum MoveFreq {
    Lowest = 1,
    Lower,
    Low,
    High,
    Higher,
    Highest,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum MoveSpeed {
    Slowest = 1,
    Slower,
    Slow,
    Fast,
    Faster,
    Fastest,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event::Page::Graphic")]
pub struct Graphic {
    pub tile_id: usize,
    pub character_name: String,
    pub character_hue: i32,
    pub direction: i32,
    pub pattern: i32,
    pub opacity: i32,
    pub blend_type: BlendMode,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event::Page::Condition")]
pub struct EventCondition {
    pub switch1_valid: bool,
    pub switch2_valid: bool,
    pub variable_valid: bool,
    pub self_switch_valid: bool,

    pub switch1_id: usize,

    pub switch2_id: usize,

    pub variable_id: usize,
    pub variable_value: i32,
    pub self_switch_ch: SelfSwitch,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[serde(from = "String", into = "String")]
#[marshal(from = "String", into = "String")]
pub enum SelfSwitch {
    A,
    B,
    C,
    D,
}

impl From<String> for SelfSwitch {
    fn from(value: String) -> Self {
        match value.as_str() {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            _ => panic!("wrong value for self switch"),
        }
    }
}

impl From<SelfSwitch> for String {
    fn from(val: SelfSwitch) -> Self {
        match val {
            SelfSwitch::A => "A".to_string(),
            SelfSwitch::B => "B".to_string(),
            SelfSwitch::C => "C".to_string(),
            SelfSwitch::D => "D".to_string(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[allow(missing_docs)]
#[marshal(class = "RPG::EventCommand")]
pub struct EventCommand {
    pub code: u16,
    pub indent: usize,
    pub parameters: Vec<ParameterType>,
}
