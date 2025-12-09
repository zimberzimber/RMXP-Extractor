
use super::ParameterType;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::MoveRoute")]
pub struct MoveRoute {
    pub repeat: bool,
    pub skippable: bool,
    pub list: Vec<MoveCommand>,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::MoveCommand")]
#[allow(missing_docs)]
pub struct MoveCommand {
    pub code: u16,
    pub parameters: Vec<ParameterType>,
}
